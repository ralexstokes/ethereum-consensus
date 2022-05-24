use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments,
    GenericArgument, Ident, Item, ItemFn, ItemUse, PathArguments, PathSegment, Type, UseGroup,
    UseTree,
};

const ATTESTATION_BOUND_IDENT: &str = "PENDING_ATTESTATIONS_BOUND";
const SYNC_COMMITTEE_SIZE_IDENT: &str = "SYNC_COMMITTEE_SIZE";
const SPEC_IMPORT: &str = "use crate::phase0 as spec;";

enum Pass {
    RemoveOverrides,
    FixGenerics,
    FixImports,
    Finalize,
}

impl Default for Pass {
    fn default() -> Self {
        Self::RemoveOverrides
    }
}

#[derive(Default)]
struct Generator {
    target_fork: String,
    module: String,
    source_path: PathBuf,
    dest_path: PathBuf,
    modification_path: PathBuf,

    overrides: HashSet<String>,
    fns_to_drop: HashSet<String>,
    imports_to_drop: HashSet<String>,
    pass: Pass,
    extend_for_block: bool,
}

impl Generator {
    fn assemble(&mut self, base_src: &str, overrides_src: Option<String>) -> syn::File {
        if let Some(overrides_src) = overrides_src {
            let overrides = syn::parse_str::<syn::File>(&overrides_src).unwrap();
            self.visit_file(&overrides);
        }

        // Remove overrides from base
        let mut base = syn::parse_str::<syn::File>(base_src).unwrap();
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
        let fn_name = &node.sig.ident;
        self.overrides.insert(fn_name.to_string());
    }
}

impl VisitMut for Generator {
    fn visit_use_group_mut(&mut self, node: &mut UseGroup) {
        match self.pass {
            Pass::FixImports => {
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
            _ => {}
        }
        syn::visit_mut::visit_use_group_mut(self, node);
    }

    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        node: &mut AngleBracketedGenericArguments,
    ) {
        match self.pass {
            Pass::FixGenerics => {
                if self.extend_for_block {
                    let arg: Type = syn::parse_str("SYNC_COMMITTEE_SIZE").unwrap();
                    node.args.push(GenericArgument::Type(arg));
                }
            }
            _ => {}
        }
        syn::visit_mut::visit_angle_bracketed_generic_arguments_mut(self, node);
    }

    // `sed` in `syn`
    fn visit_ident_mut(&mut self, node: &mut Ident) {
        match self.pass {
            Pass::FixGenerics => {
                let target_ident: Ident = syn::parse_str(&ATTESTATION_BOUND_IDENT).unwrap();
                if node == &target_ident {
                    let replacement: Ident = syn::parse_str(&SYNC_COMMITTEE_SIZE_IDENT).unwrap();
                    *node = replacement;
                }
            }
            _ => {}
        }
        syn::visit_mut::visit_ident_mut(self, node);
    }

    fn visit_path_segment_mut(&mut self, node: &mut PathSegment) {
        match self.pass {
            Pass::FixGenerics => {
                let ident = node.ident.to_string();
                if ident.contains("BeaconBlock") {
                    match &mut node.arguments {
                        PathArguments::AngleBracketed(args) => {
                            self.extend_for_block = true;
                            self.visit_angle_bracketed_generic_arguments_mut(args);
                            self.extend_for_block = false;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        syn::visit_mut::visit_path_segment_mut(self, node);
    }

    fn visit_file_mut(&mut self, node: &mut syn::File) {
        match self.pass {
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
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let mut removed = 0;
                for index in indices_to_remove {
                    node.items.remove(index - removed);
                    removed += 1;
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
                let insertion_index = iter.next().unwrap();
                for (offset, ident) in self.overrides.iter().enumerate() {
                    let target_fork = &self.target_fork;
                    let module = self.module.split("::").last().unwrap();
                    let use_item: ItemUse = syn::parse_str(&format!(
                        "pub use crate::{target_fork}::{module}::{ident} as {ident};"
                    ))
                    .unwrap();
                    node.items
                        .insert(insertion_index + offset, Item::Use(use_item.into()));
                }
            }
            Pass::Finalize => {
                let target: ItemUse = syn::parse_str(&SPEC_IMPORT).unwrap();
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
                if target_index.len() > 1 {
                    panic!("more than one 'spec import' found, please fix source");
                }
                let target_index = target_index[0];
                let target_fork = &self.target_fork;
                let replacement: ItemUse =
                    syn::parse_str(&format!("use crate::{target_fork} as spec;")).unwrap();
                node.items[target_index] = replacement.into();

                let warning = parse_quote! {
                    //! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
                };
                node.attrs.insert(0, warning);
            }
        }
        syn::visit_mut::visit_file_mut(self, node);
    }
}

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

    let forks_to_gen = vec!["phase0", "altair" /* "bellatrix"*/];

    for fork_pair in forks_to_gen.windows(2) {
        let (source_fork, target_fork) = (fork_pair[0], fork_pair[1]);
        let source_dir = root.join(source_fork);
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

            let fns_to_drop = HashSet::from_iter(
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
            );

            let imports_to_drop =
                HashSet::from_iter(["PendingAttestation"].into_iter().map(String::from));

            let mut generator = Generator {
                target_fork: target_fork.to_string(),
                module: module.replace("/", "::"),
                source_path,
                dest_path,
                modification_path,
                fns_to_drop,
                imports_to_drop,
                ..Default::default()
            };
            generator.render();
        }
    }
}
