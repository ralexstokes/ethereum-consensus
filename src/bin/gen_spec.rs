use quote::ToTokens;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, ConstParam,
    GenericArgument, Ident, Item, ItemFn, ItemUse, PathArguments, PathSegment, Signature, Type,
    UseGroup, UseName, UsePath, UseTree,
};

const ATTESTATION_BOUND_IDENT: &str = "PENDING_ATTESTATIONS_BOUND";
const SYNC_COMMITTEE_SIZE_IDENT: &str = "SYNC_COMMITTEE_SIZE";

const BELLATRIX_BEACON_STATE_BOUNDS: [&str; 4] = [
    "BYTES_PER_LOGS_BLOOM",
    "MAX_EXTRA_DATA_BYTES",
    "MAX_BYTES_PER_TRANSACTION",
    "MAX_TRANSACTIONS_PER_PAYLOAD",
];

// "passes" of the generator that should be performed
// in sequential order following their specification here
#[derive(Debug)]
enum Pass {
    // read phases
    IdentifyResolutions,
    IdentifyOverrides,
    // write phases
    ImportResolutions,
    RemoveOverrides,
    FixGenerics,
    FixImports,
    Finalize,
}

impl Default for Pass {
    fn default() -> Self {
        Self::IdentifyResolutions
    }
}

#[derive(Default, Debug)]
struct Generator {
    root: PathBuf,
    source_fork: String,
    target_fork: String,
    module: String,
    source_path: PathBuf,
    dest_path: PathBuf,
    modification_path: PathBuf,

    overrides: HashSet<String>,
    fns_to_drop: Rc<HashSet<String>>,
    imports_to_drop: Rc<HashSet<String>>,
    imports_to_add: Rc<HashSet<String>>,
    fns_to_resolve: HashMap<String, HashSet<String>>,
    pass: Pass,
    // these are essentially hard-coded to work for each fork
    // consider providing as input like `overrides` and `fns_to_drop`
    extend_for_block: bool,
    extend_for_state: bool,
}

impl Generator {
    fn assemble(&mut self, base_src: &str, overrides_src: Option<String>) -> syn::File {
        let mut base = syn::parse_str::<syn::File>(base_src).unwrap();

        // identify functions we need to import from
        // an earlier module outside of the source module
        self.visit_file(&base);

        // the target module can provide "overrides" that should
        // take preference over functions defined in the source module
        self.pass = Pass::IdentifyOverrides;
        if let Some(overrides_src) = overrides_src {
            let overrides = syn::parse_str::<syn::File>(&overrides_src).unwrap();
            self.visit_file(&overrides);
        }

        // Import anything missing from the "template" files
        // used in the source fork
        self.pass = Pass::ImportResolutions;
        self.visit_file_mut(&mut base);

        // Remove overrides from base
        self.pass = Pass::RemoveOverrides;
        self.visit_file_mut(&mut base);

        // Fix generics
        self.pass = Pass::FixGenerics;
        self.visit_file_mut(&mut base);

        // Import overrides from supplemental module
        self.pass = Pass::FixImports;
        self.visit_file_mut(&mut base);

        // Finalize any remaining edits...
        self.pass = Pass::Finalize;
        self.visit_file_mut(&mut base);

        base
    }

    fn get_modification_source(&self) -> Option<String> {
        match File::open(&self.modification_path) {
            Ok(mut file) => {
                let mut src = String::new();
                file.read_to_string(&mut src).unwrap();
                Some(src)
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => None,
                _ => panic!("{}", err),
            },
        }
    }

    fn render(&mut self) {
        let modification_src = self.get_modification_source();
        let source_src = fs::read_to_string(&self.source_path).unwrap();
        let target = self.assemble(&source_src, modification_src);
        let output = prettyplease::unparse(&target);
        let dir = &self.dest_path.parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        fs::write(&self.dest_path, output).unwrap();
    }
}

impl<'ast> Visit<'ast> for Generator {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if matches!(self.pass, Pass::IdentifyOverrides) {
            let fn_name = &node.sig.ident;
            self.overrides.insert(fn_name.to_string());
        }
        syn::visit::visit_item_fn(self, node)
    }

    fn visit_file(&mut self, node: &'ast syn::File) {
        if matches!(self.pass, Pass::IdentifyResolutions) {
            for item in &node.items {
                if let Item::Use(node) = item {
                    if let UseTree::Path(node) = &node.tree {
                        if node.ident == "crate" {
                            if let UseTree::Path(node) = &*node.tree {
                                // if the import is `use crate::{source_fork}::...` then
                                // identify as something we need to import to patch downstream
                                if node.ident == self.source_fork {
                                    let mut fn_to_resolve = node.to_token_stream().to_string();
                                    fn_to_resolve.retain(|c| !c.is_whitespace());
                                    let parts = fn_to_resolve.split("::").collect::<Vec<_>>();
                                    let (fn_name, path_parts) = parts.split_last().unwrap();
                                    let target_module = path_parts.last().unwrap();
                                    let entry = self
                                        .fns_to_resolve
                                        .entry(target_module.to_string())
                                        .or_default();
                                    entry.insert(fn_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_file(self, node);
    }
}

impl VisitMut for Generator {
    fn visit_signature_mut(&mut self, node: &mut Signature) {
        if let Pass::FixGenerics = self.pass {
            if self.source_fork == "altair" && !node.generics.params.is_empty() {
                for bound in BELLATRIX_BEACON_STATE_BOUNDS {
                    let param: ConstParam =
                        syn::parse_str(&format!("const {bound}: usize")).unwrap();
                    node.generics.params.push(param.into());
                }
            }
        }
        syn::visit_mut::visit_signature_mut(self, node);
    }

    fn visit_use_group_mut(&mut self, node: &mut UseGroup) {
        if let Pass::FixImports = self.pass {
            let filtered = node
                .items
                .iter()
                .cloned()
                .filter(|item| match item {
                    UseTree::Name(node) => {
                        let ident = node.ident.to_string();
                        !self.imports_to_drop.contains(&ident)
                    }
                    _ => true,
                })
                .collect::<Vec<_>>();
            node.items = Punctuated::<UseTree, Comma>::from_iter(filtered);
        }
        syn::visit_mut::visit_use_group_mut(self, node);
    }

    fn visit_use_path_mut(&mut self, node: &mut UsePath) {
        if let Pass::FixImports = self.pass {
            if node.ident == "spec" {
                if let UseTree::Group(node) = &mut *node.tree {
                    for addition in self.imports_to_add.iter() {
                        let arg: Ident = syn::parse_str(addition).unwrap();
                        let arg = UseName { ident: arg };
                        node.items.push(arg.into());
                    }
                }
            }
        }
        syn::visit_mut::visit_use_path_mut(self, node);
    }

    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        node: &mut AngleBracketedGenericArguments,
    ) {
        if let Pass::FixGenerics = self.pass {
            if self.extend_for_block && self.source_fork == "phase0" {
                let arg: Type = syn::parse_str("SYNC_COMMITTEE_SIZE").unwrap();
                node.args.push(GenericArgument::Type(arg));
            } else if self.extend_for_state && self.source_fork == "altair" {
                for bound in BELLATRIX_BEACON_STATE_BOUNDS {
                    let arg: Type = syn::parse_str(bound).unwrap();
                    node.args.push(GenericArgument::Type(arg));
                }
            }
        }
        syn::visit_mut::visit_angle_bracketed_generic_arguments_mut(self, node);
    }

    // `sed` in `syn`
    fn visit_ident_mut(&mut self, node: &mut Ident) {
        if let Pass::FixGenerics = self.pass {
            let target_ident: Ident = syn::parse_str(ATTESTATION_BOUND_IDENT).unwrap();
            if node == &target_ident {
                let replacement: Ident = syn::parse_str(SYNC_COMMITTEE_SIZE_IDENT).unwrap();
                *node = replacement;
            }
        }
        syn::visit_mut::visit_ident_mut(self, node);
    }

    fn visit_path_segment_mut(&mut self, node: &mut PathSegment) {
        if let Pass::FixGenerics = self.pass {
            let ident = node.ident.to_string();
            if ident.contains("BeaconBlock") {
                if let PathArguments::AngleBracketed(args) = &mut node.arguments {
                    self.extend_for_block = true;
                    self.extend_for_state = true;
                    self.visit_angle_bracketed_generic_arguments_mut(args);
                    self.extend_for_block = false;
                    self.extend_for_state = false;
                }
            } else if ident.contains("BeaconState") {
                if let PathArguments::AngleBracketed(args) = &mut node.arguments {
                    self.extend_for_state = true;
                    self.visit_angle_bracketed_generic_arguments_mut(args);
                    self.extend_for_state = false;
                }
            }
        }
        syn::visit_mut::visit_path_segment_mut(self, node);
    }

    fn visit_file_mut(&mut self, node: &mut syn::File) {
        match self.pass {
            Pass::ImportResolutions => {
                let mod_name = self.module.split("::").last().unwrap().to_string();
                if let Some(fns_to_resolve) = self.fns_to_resolve.get(&mod_name) {
                    let target_path = self
                        .root
                        .join(&self.source_fork)
                        .join(mod_name)
                        .with_extension("rs");
                    let target_src = fs::read_to_string(target_path).unwrap();
                    let target = syn::parse_str::<syn::File>(&target_src).unwrap();
                    let resolutions = target
                        .items
                        .into_iter()
                        .filter(|item| match item {
                            Item::Fn(node) => fns_to_resolve.contains(&node.sig.ident.to_string()),
                            _ => false,
                        })
                        .collect::<Vec<_>>();

                    node.items.extend(resolutions);
                }
            }
            Pass::RemoveOverrides => {
                let indices_to_remove = node
                    .items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| match item {
                        Item::Fn(node) => {
                            let fn_name = node.sig.ident.to_string();
                            let is_overriden = self.overrides.contains(&fn_name);
                            let is_expired = self.fns_to_drop.contains(&fn_name);
                            let should_drop = is_overriden || is_expired;
                            if should_drop {
                                Some(i)
                            } else {
                                None
                            }
                        }
                        Item::Use(node) => {
                            if let UseTree::Path(node) = &node.tree {
                                // NOTE: this could probably be cleaned up and
                                // combined with the "resolutions" phase but
                                // it works for now
                                if node.ident == "crate" {
                                    if let UseTree::Path(node) = &*node.tree {
                                        if node.ident == self.source_fork {
                                            return Some(i);
                                        }
                                    }
                                }
                                return None;
                            }
                            None
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                for (removed, index) in indices_to_remove.iter().enumerate() {
                    node.items.remove(index - removed);
                }
            }
            Pass::FixGenerics => {}
            Pass::FixImports => {
                let mut iter = node
                    .items
                    .iter()
                    .enumerate()
                    .skip_while(|(_, item)| !matches!(item, Item::Use(..)))
                    .skip_while(|(_, item)| matches!(item, Item::Use(..)))
                    .map(|(i, _)| i);
                if let Some(insertion_index) = iter.next() {
                    for (offset, ident) in self.overrides.iter().enumerate() {
                        let target_fork = &self.target_fork;
                        let module = self.module.split("::").last().unwrap();
                        let use_item: ItemUse = syn::parse_str(&format!(
                            "pub use crate::{target_fork}::{module}::{ident};"
                        ))
                        .unwrap();
                        node.items
                            .insert(insertion_index + offset, Item::Use(use_item));
                    }
                } else if self.overrides.len() == 1 && self.overrides.remove("state_transition") {
                    // a bit kludgy but so far this means we are replacing the function
                    // `state_transition` inside a `state_transition/mod.rs` file
                    let target_fork = &self.target_fork;
                    let ident = "state_transition";
                    let use_item: ItemUse = syn::parse_str(&format!(
                        "pub use crate::{target_fork}::state_transition_{target_fork}::{ident};"
                    ))
                    .unwrap();
                    node.items.push(use_item.into());
                }
            }
            Pass::Finalize => {
                let import_target = format!("use crate::{} as spec;", self.source_fork);
                let target: ItemUse = syn::parse_str(&import_target).unwrap();
                let target_index = node
                    .items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, item)| {
                        if item == &Item::Use(target.clone()) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if target_index.len() != 1 {
                    panic!("ambiguous 'spec import' found, please fix source");
                }
                let target_index = target_index[0];
                let target_fork = &self.target_fork;
                let replacement: ItemUse =
                    syn::parse_str(&format!("use crate::{target_fork} as spec;")).unwrap();
                node.items[target_index] = replacement.into();

                // clear any previous comments added
                node.attrs.clear();
                let warning = parse_quote! {
                    //! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
                };
                node.attrs.insert(0, warning);
            }
            _ => {}
        }
        syn::visit_mut::visit_file_mut(self, node);
    }
}

type Patch = HashSet<String>;
type Patches = (Patch, Patch, Patch);

fn main() {
    let root = Path::new("./src");
    let source_modules_to_gen = vec![
        "state_transition/block_processing",
        "state_transition/epoch_processing",
        "state_transition/genesis",
        "state_transition/helpers",
        "state_transition/mod",
        "state_transition/slot_processing",
    ];

    let mut edits: HashMap<&'static str, Patches> = HashMap::from_iter([
        (
            "altair",
            (
                HashSet::from_iter(
                    [
                        "get_unslashed_attesting_indices",
                        "get_matching_source_attestations",
                        "get_matching_target_attestations",
                        "get_matching_head_attestations",
                        "get_source_deltas",
                        "get_target_deltas",
                        "get_head_deltas",
                        "get_inclusion_delay_deltas",
                        "get_inactivity_penalty_deltas",
                        "get_attestation_deltas",
                        "get_attestation_component_deltas",
                        "get_attesting_balance",
                        "process_participation_record_updates",
                    ]
                    .into_iter()
                    .map(String::from),
                ),
                HashSet::from_iter(["PendingAttestation"].into_iter().map(String::from)),
                HashSet::new(),
            ),
        ),
        (
            "bellatrix",
            (
                HashSet::new(),
                HashSet::new(),
                HashSet::from_iter(
                    [
                        "SyncAggregate",
                        "ParticipationFlags",
                        "TIMELY_TARGET_FLAG_INDEX",
                        "WEIGHT_DENOMINATOR",
                        "PROPOSER_WEIGHT",
                    ]
                    .into_iter()
                    .map(String::from),
                ),
            ),
        ),
    ]);

    let forks_to_gen = vec!["phase0", "altair", "bellatrix"];

    for fork_pair in forks_to_gen.windows(2) {
        let (source_fork, target_fork) = (fork_pair[0], fork_pair[1]);
        let source_dir = root.join(source_fork);
        let edits = edits.remove(target_fork).unwrap();
        let fns_to_drop = Rc::new(edits.0);
        let imports_to_drop = Rc::new(edits.1);
        let imports_to_add = Rc::new(edits.2);
        for module in &source_modules_to_gen {
            let source_path = source_dir.join(module).with_extension("rs");
            let dest_path = root.join(target_fork).join(module).with_extension("rs");
            let target_module = match dest_path.file_stem().unwrap().to_str().unwrap() {
                "mod" => format!("state_transition_{target_fork}"),
                other => other.to_string(),
            };

            let modification_path = root
                .join(target_fork)
                .join(target_module)
                .with_extension("rs");

            let mut generator = Generator {
                root: root.into(),
                source_fork: source_fork.to_string(),
                target_fork: target_fork.to_string(),
                module: module.replace('/', "::"),
                source_path,
                dest_path,
                modification_path,
                fns_to_drop: fns_to_drop.clone(),
                imports_to_drop: imports_to_drop.clone(),
                imports_to_add: imports_to_add.clone(),
                ..Default::default()
            };
            generator.render();
        }
    }
}
