use crate::{
    generator::Fork,
    visitors::{as_syn_ident, generics_to_arguments, ToGenericsVisitor, TypeNameVisitor},
};
use convert_case::{Case, Casing};
use std::{collections::HashSet, fs};
use syn::{
    parse_quote, token::Mut, visit::Visit, AngleBracketedGenericArguments, Arm, Expr, GenericParam,
    Generics, Ident, ImplItemMethod, Item, ItemStruct,
};

const SOURCE_ROOT: &str = "ethereum-consensus/src";

fn is_copy(ident: &str) -> bool {
    matches!(
        ident.to_string().as_str(),
        "Root" | "Slot" | "ValidatorIndex" | "WithdrawalIndex" | "u64"
    )
}

#[derive(Debug)]
enum Type {
    BeaconBlockBody,
    BeaconBlock,
    SignedBeaconBlock,
    BlindedBeaconBlock,
    BlindedBeaconBlockBody,
    SignedBlindedBeaconBlock,
    ExecutionPayload,
    ExecutionPayloadHeader,
    BeaconState,
}

impl Type {
    fn name(&self) -> String {
        format!("{self:?}")
    }

    // The source module to search for the corresponding definitions of this type.
    fn module(&self) -> &str {
        match self {
            Self::BeaconBlockBody => "beacon_block",
            Self::BeaconBlock => "beacon_block",
            Self::SignedBeaconBlock => "beacon_block",
            Self::BlindedBeaconBlockBody => "blinded_beacon_block",
            Self::BlindedBeaconBlock => "blinded_beacon_block",
            Self::SignedBlindedBeaconBlock => "blinded_beacon_block",
            Self::ExecutionPayload => "execution_payload",
            Self::ExecutionPayloadHeader => "execution_payload",
            Self::BeaconState => "beacon_state",
        }
    }

    // Should we generate "special" reference types for this type?
    // In addition to the usual polymorphic wrapping, `true` here indicates
    // we should *also* generate a `TypeRef` and a `TypeRefMut`.
    // These reference types hold a polymorphic reference to the underlying type,
    // and support `From::from` implementations to ease conversion into and out of
    // the reference wrapper.
    fn needs_ref_types(&self) -> bool {
        matches!(
            self,
            Self::BeaconBlockBody |
                Self::BlindedBeaconBlockBody |
                Self::BeaconBlock |
                Self::BlindedBeaconBlock |
                Self::ExecutionPayload |
                Self::ExecutionPayloadHeader |
                Self::BeaconState
        )
    }

    // Return any fields of this (polymorphic) type that are themselves polymorphic, as
    // they return special reference types in this type's field accessors.
    fn polymorphic_fields(&self) -> Vec<&str> {
        match self {
            Self::BeaconBlockBody => vec!["execution_payload"],
            Self::BlindedBeaconBlockBody => vec!["execution_payload_header"],
            Self::BeaconBlock => vec!["body"],
            Self::BlindedBeaconBlock => vec!["body"],
            Self::SignedBeaconBlock => vec!["message"],
            Self::SignedBlindedBeaconBlock => vec!["message"],
            Self::BeaconState => vec!["latest_execution_payload_header"],
            _ => vec![],
        }
    }

    // Return the name of the child type for the polymorphic field this type contains.
    // Only supports one polymorphic field per parent type for now.
    fn get_polymorphic_type(&self) -> &str {
        match self {
            Self::BeaconBlockBody => "ExecutionPayload",
            Self::BlindedBeaconBlockBody => "ExecutionPayloadHeader",
            Self::BeaconBlock => "BeaconBlockBody",
            Self::BlindedBeaconBlock => "BlindedBeaconBlockBody",
            Self::SignedBeaconBlock => "BeaconBlock",
            Self::SignedBlindedBeaconBlock => "BlindedBeaconBlock",
            Self::BeaconState => "ExecutionPayloadHeader",
            _ => unimplemented!("not supported"),
        }
    }

    // Generally this generator assumes types are only accretive, although
    // there are some exceptions to this rule. This method returns `Some` data
    // that indicates to the generation functions how to handle field deletions
    // if they exist for a given type.
    // The `Fork` indicates which fork the corresponding fields were dropped.
    fn field_deletions(&self) -> Option<(Fork, Vec<&str>)> {
        match self {
            Self::BeaconState => Some((
                Fork::Altair,
                vec!["previous_epoch_attestations", "current_epoch_attestations"],
            )),
            _ => None,
        }
    }

    fn imports(&self) -> syn::Item {
        // TODO: electra
        match self {
            Self::BeaconBlockBody => parse_quote! {
                use crate::{
                    altair::beacon_block as altair,
                    bellatrix::beacon_block as bellatrix,
                    capella::beacon_block as capella,
                    crypto::KzgCommitment,
                    deneb::{beacon_block as deneb},
                    phase0::{
                        beacon_block as phase0, Attestation, AttesterSlashing, Deposit, Eth1Data, ProposerSlashing,
                        SignedVoluntaryExit,
                    },
                    altair::SyncAggregate,
                    capella::SignedBlsToExecutionChange,
                    primitives::{BlsSignature, Bytes32},
                    ssz::prelude::*,
                    Fork as Version,
                    types::execution_payload::{ExecutionPayloadRef, ExecutionPayloadRefMut},
                };
            },
            Self::BlindedBeaconBlockBody => parse_quote! {
                use crate::{
                    bellatrix::blinded_beacon_block as bellatrix,
                    capella::blinded_beacon_block as capella,
                    crypto::KzgCommitment,
                    deneb::{blinded_beacon_block as deneb},
                    phase0::{
                        Attestation, AttesterSlashing, Deposit, Eth1Data, ProposerSlashing,
                        SignedVoluntaryExit,
                    },
                    altair::SyncAggregate,
                    capella::SignedBlsToExecutionChange,
                    primitives::{BlsSignature, Bytes32},
                    ssz::prelude::*,
                    Fork as Version,
                    types::execution_payload_header::{ExecutionPayloadHeaderRef, ExecutionPayloadHeaderRefMut},
                };
            },
            Self::BeaconBlock => parse_quote! {
                use crate::{
                    altair::beacon_block as altair,
                    bellatrix::beacon_block as bellatrix,
                    capella::beacon_block as capella,
                    deneb::beacon_block as deneb,
                    phase0::beacon_block as phase0,
                    primitives::{Slot, ValidatorIndex, Root},
                    ssz::prelude::*,
                    Fork as Version,
                    types::beacon_block_body::{BeaconBlockBodyRef, BeaconBlockBodyRefMut},
                };
            },
            Self::BlindedBeaconBlock => parse_quote! {
                use crate::{
                    bellatrix::blinded_beacon_block as bellatrix,
                    capella::blinded_beacon_block as capella,
                    deneb::blinded_beacon_block as deneb,
                    primitives::{Slot, ValidatorIndex, Root},
                    ssz::prelude::*,
                    Fork as Version,
                    types::blinded_beacon_block_body::{BlindedBeaconBlockBodyRef, BlindedBeaconBlockBodyRefMut},
                };
            },
            Self::SignedBeaconBlock => parse_quote! {
                use crate::{
                    altair::beacon_block as altair,
                    bellatrix::beacon_block as bellatrix,
                    capella::beacon_block as capella,
                    deneb::beacon_block as deneb,
                    phase0::beacon_block as phase0,
                    primitives::BlsSignature,
                    ssz::prelude::*,
                    Fork as Version,
                    types::beacon_block::{BeaconBlockRef, BeaconBlockRefMut},
                };
            },
            Self::SignedBlindedBeaconBlock => parse_quote! {
                use crate::{
                    bellatrix::blinded_beacon_block as bellatrix,
                    capella::blinded_beacon_block as capella,
                    deneb::blinded_beacon_block as deneb,
                    primitives::BlsSignature,
                    ssz::prelude::*,
                    Fork as Version,
                    types::blinded_beacon_block::{BlindedBeaconBlockRef, BlindedBeaconBlockRefMut},
                };
            },
            Self::ExecutionPayload => parse_quote! {
                use crate::{
                    bellatrix::execution_payload::{self as bellatrix, Transaction},
                    capella::{execution_payload as capella, withdrawal::Withdrawal},
                    deneb::execution_payload as deneb,
                    electra::execution_payload as electra,
                    electra::operations::{DepositRequest, WithdrawalRequest, ConsolidationRequest},
                    primitives::{Hash32, ExecutionAddress, Bytes32},
                    ssz::prelude::*,
                    Fork as Version,
                };
            },
            Self::ExecutionPayloadHeader => parse_quote! {
                use crate::{
                    bellatrix::execution_payload as bellatrix,
                    capella::execution_payload as capella,
                    deneb::execution_payload as deneb,
                    electra::execution_payload as electra,
                    primitives::{Hash32, Root, ExecutionAddress, Bytes32},
                    ssz::prelude::*,
                    Fork as Version,
                };
            },
            Self::BeaconState => parse_quote! {
                use crate::{
                    altair::beacon_state as altair,
                    altair::sync::SyncCommittee,
                    bellatrix::beacon_state as bellatrix,
                    capella::beacon_state::{HistoricalSummary, self as capella},
                    deneb::beacon_state as deneb,
                    phase0::beacon_state::{self as phase0, Fork},
                    phase0::{JUSTIFICATION_BITS_LENGTH, beacon_block::BeaconBlockHeader, validator::Validator, operations::{PendingAttestation, Checkpoint, Eth1Data}},
                    primitives::{Root, ValidatorIndex, WithdrawalIndex, ParticipationFlags, Slot, Gwei, Bytes32},
                    ssz::prelude::*,
                    Fork as Version,
                    types::execution_payload_header::{ExecutionPayloadHeaderRef, ExecutionPayloadHeaderRefMut},
                };
            },
        }
    }
}

struct TypeDefn {
    fork: Fork,
    item: ItemStruct,
}

fn load_type_defns(target_type: &Type, forks: &[Fork]) -> Vec<TypeDefn> {
    let mut defns = vec![];
    for fork in forks {
        let fork_name = fork.name();
        let type_module = target_type.module();
        let source_path = format!("{SOURCE_ROOT}/{fork_name}/{type_module}.rs");
        let module_source = match fs::read_to_string(&source_path) {
            Ok(src) => src,
            Err(_) => continue,
        };
        let file = syn::parse_file(&module_source).unwrap();
        for item in file.items {
            match item {
                Item::Struct(item) => {
                    if item.ident == target_type.name() {
                        defns.push(TypeDefn { fork: *fork, item });
                    }
                }
                _ => continue,
            }
        }
    }
    defns
}

#[derive(Debug)]
struct MergeType {
    generics: Vec<Ident>,
    variants: Vec<Variant>,
    fields: Vec<FieldDefn>,
}

impl MergeType {
    fn supported_forks(&self) -> Vec<Fork> {
        self.variants.iter().map(|v| v.fork).collect()
    }
}

#[derive(Debug)]
struct Variant {
    fork: Fork,
    generics: Vec<Ident>,
}

#[derive(Debug)]
struct FieldDefn {
    fork: Fork,
    ident: Ident,
    type_def: syn::Type,
}

fn derive_merge_generics(variants: &[Variant]) -> Vec<Ident> {
    let mut all_generics = vec![];
    for variant in variants {
        for ident in &variant.generics {
            if !all_generics.contains(ident) {
                all_generics.push(ident.clone());
            }
        }
    }
    all_generics
}

fn derive_merge_type(fork_sequence: &[Fork], defns: Vec<TypeDefn>) -> MergeType {
    let mut variants = vec![];
    let mut fields: Vec<FieldDefn> = vec![];
    let mut seen_fields = HashSet::new();
    let iter = fork_sequence.iter().skip_while(|&fork| fork != &defns[0].fork);
    for (fork, defn) in iter.zip(defns.iter()) {
        let mut visitor = ToGenericsVisitor::default();
        visitor.visit_generics(&defn.item.generics);
        let variant = Variant { fork: *fork, generics: visitor.bounds };
        variants.push(variant);

        for field in &defn.item.fields {
            let ident = field.ident.clone().unwrap();
            let mut field_defn =
                FieldDefn { fork: *fork, ident: ident.clone(), type_def: field.ty.clone() };
            if seen_fields.contains(&ident) {
                let target = fields.iter().position(|field| field.ident == ident).unwrap();
                let field_to_update = &mut fields[target];
                field_defn.fork = field_to_update.fork;
                let _ = std::mem::replace(field_to_update, field_defn);
            } else {
                fields.push(field_defn);
            }
            seen_fields.insert(ident);
        }
    }

    let generics = derive_merge_generics(&variants);

    MergeType { generics, variants, fields }
}

fn derive_type_defn(target_type: &Type, merge_type: &MergeType) -> (Item, Generics) {
    let type_name = as_syn_ident(target_type.name());
    let bounds = &merge_type.generics;
    let generics: syn::Generics = parse_quote! {
        <
        #(const #bounds: usize),*
        >
    };
    let generics_copy = generics.clone();
    let variant_defns = merge_type
        .variants
        .iter()
        .map(|variant| {
            let fork = &variant.fork;
            let fork = format!("{fork:?}");
            let fork = as_syn_ident(fork);
            let fork_name = as_syn_ident(variant.fork.name());
            let bounds = &variant.generics;
            let arguments: AngleBracketedGenericArguments = parse_quote! {
                <
                #(#bounds),*
                >
            };
            parse_quote! {
                #fork (
                    #fork_name::#type_name #arguments
                )
            }
        })
        .collect::<Vec<syn::Variant>>();
    let enum_defn = parse_quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Serializable, HashTreeRoot, serde::Serialize)]
        #[ssz(transparent)]
        #[serde(untagged)]
        pub enum #type_name #generics {
            #(#variant_defns),*
        }
    };
    (enum_defn, generics_copy)
}

fn derive_method_set(
    target_type: &Type,
    field_defn: &FieldDefn,
    fork_sequence: &[Fork],
    ref_type: &Option<RefType>,
) -> Vec<ImplItemMethod> {
    let ident = &field_defn.ident;
    let deletion_fork = target_type.field_deletions().and_then(|(fork, fields)| {
        let identifier = ident.to_string();
        if fields.contains(&identifier.as_str()) {
            Some(fork)
        } else {
            None
        }
    });
    let field_defined_in_all_forks = field_defn.fork == fork_sequence[0];
    let is_optional = deletion_fork.is_some() || !field_defined_in_all_forks;
    let is_polymorphic = target_type.polymorphic_fields().contains(&ident.to_string().as_ref());
    let mut is_type_copy = false;
    let ty: syn::Type = if is_polymorphic {
        let base = target_type.get_polymorphic_type();
        let name = as_syn_ident(format!("{base}Ref"));

        let ty = &field_defn.type_def;
        let mut visitor = ToGenericsVisitor::default();
        visitor.visit_type(ty);
        let bounds = visitor.bounds;
        let arguments: AngleBracketedGenericArguments = parse_quote! {
            <
            #(#bounds),*
            >
        };
        parse_quote! {
            #name #arguments
        }
    } else {
        let type_def = &field_defn.type_def;
        let mut visitor = TypeNameVisitor::default();
        visitor.analyze_type(type_def);
        let idents = visitor.names;
        let maybe_copy = idents.len() == 1;
        let is_copy = if maybe_copy { is_copy(&idents[0]) } else { false };
        is_type_copy = is_copy;
        let ref_item: Option<syn::Token!(&)> = if is_copy { None } else { Some(parse_quote!(&)) };
        parse_quote! {
            #ref_item #type_def
        }
    };
    let ty: syn::Type = if is_optional { parse_quote!(Option<#ty>) } else { ty };

    let match_arms = derive_match_arms(
        &field_defn.fork,
        fork_sequence,
        ident,
        false,
        is_polymorphic,
        is_optional,
        is_type_copy,
        deletion_fork,
    );
    let immut_ref = parse_quote! {
        pub fn #ident(&self) -> #ty {
            match self {
                #(#match_arms)*
            }
        }
    };

    let mut_ident = as_syn_ident(format!("{ident}_mut"));
    let ty_mut: syn::Type = if is_polymorphic {
        let base = target_type.get_polymorphic_type();
        let name = as_syn_ident(format!("{base}RefMut"));

        let ty = &field_defn.type_def;
        let mut visitor = ToGenericsVisitor::default();
        visitor.visit_type(ty);
        let bounds = visitor.bounds;
        let arguments: AngleBracketedGenericArguments = parse_quote! {
            <
            #(#bounds),*
            >
        };
        parse_quote! {
            #name #arguments
        }
    } else {
        let type_def = &field_defn.type_def;
        let ref_item: syn::Token!(&) = parse_quote!(&);
        let mutability = Mut::default();
        parse_quote! {
            #ref_item #mutability #type_def
        }
    };
    let ty_mut: syn::Type = if is_optional { parse_quote!(Option<#ty_mut>) } else { ty_mut };
    let match_arms = derive_match_arms(
        &field_defn.fork,
        fork_sequence,
        ident,
        true,
        is_polymorphic,
        is_optional,
        // NOTE: make this false so we generate mutable refs, rather than just a copy
        false,
        deletion_fork,
    );
    let mut_ref = parse_quote! {
        pub fn #mut_ident(&mut self) -> #ty_mut {
            match self {
                #(#match_arms)*
            }
        }
    };
    match ref_type {
        Some(RefType::Immutable) => vec![immut_ref],
        Some(RefType::Mutable) | None => {
            let mut result = vec![immut_ref];
            result.push(mut_ref);
            result
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn derive_match_arms(
    source_fork: &Fork,
    fork_sequence: &[Fork],
    ident: &Ident,
    is_mut: bool,
    is_polymorphic: bool,
    is_optional: bool,
    is_copy: bool,
    deletion_fork: Option<Fork>,
) -> Vec<Arm> {
    let mutability = if is_mut && !is_copy { Some(Mut::default()) } else { None };
    let ref_item: Option<syn::Token!(&)> = if is_copy { None } else { Some(parse_quote!(&)) };
    let accessor_expr: syn::Expr = if is_polymorphic {
        parse_quote!(From::from(#ref_item #mutability inner.#ident))
    } else {
        parse_quote!(#ref_item #mutability inner.#ident)
    };
    fork_sequence
        .iter()
        .map(|fork| {
            let fork_variant = as_syn_ident(format!("{fork:?}"));
            let was_deleted = if let Some(deletion_fork) = deletion_fork.as_ref() {
                fork >= deletion_fork
            } else {
                false
            };
            let accessor_expr = if is_optional {
                parse_quote!(Some(#accessor_expr))
            } else {
                accessor_expr.clone()
            };
            if fork < source_fork || was_deleted {
                parse_quote! {
                    Self::#fork_variant(_) => None,
                }
            } else {
                parse_quote! {
                    Self::#fork_variant(inner) => #accessor_expr,
                }
            }
        })
        .collect()
}

enum RefType {
    Immutable,
    Mutable,
}

fn derive_fields_impl(
    target_type: &Type,
    type_name: &Ident,
    merge_type: &MergeType,
    ref_type: Option<RefType>,
) -> Vec<ImplItemMethod> {
    let mut fields = merge_type
        .variants
        .iter()
        .flat_map(|variant| {
            let fork = &variant.fork;
            let fork = as_syn_ident(format!("{fork:?}"));
            let fork_name = as_syn_ident(variant.fork.name());
            let bounds = &variant.generics;
            let arguments: AngleBracketedGenericArguments = parse_quote! {
                <
                #(#bounds),*
                >
            };
            let immut_ref = parse_quote! {
                pub fn #fork_name(&self) -> Option<&#fork_name::#type_name #arguments> {
                    match self {
                        Self::#fork(inner) => Some(inner),
                        _ => None,
                    }
                }
            };
            let mut_ident = as_syn_ident(format!("{fork_name}_mut"));
            let mut_ref = parse_quote! {
                pub fn #mut_ident(&mut self) -> Option<&mut #fork_name::#type_name #arguments> {
                    match self {
                        Self::#fork(inner) => Some(inner),
                        _ => None,
                    }
                }
            };
            match ref_type {
                Some(RefType::Immutable) => vec![immut_ref],
                Some(RefType::Mutable) => vec![immut_ref, mut_ref],
                None => vec![immut_ref, mut_ref],
            }
        })
        .collect::<Vec<ImplItemMethod>>();

    let fork_sequence = merge_type.supported_forks();
    let fork_accessor_arms = fork_sequence
        .iter()
        .map(|fork| {
            let fork = as_syn_ident(format!("{fork:?}"));
            parse_quote! {
                Self::#fork(_) => Version::#fork,
            }
        })
        .collect::<Vec<Arm>>();
    fields.push(parse_quote! {
        pub fn version(&self) -> Version {
            match self {
                #(#fork_accessor_arms)*
            }
        }
    });

    let field_accessors = merge_type.fields.iter().flat_map(|field_defn| {
        derive_method_set(target_type, field_defn, &fork_sequence, &ref_type)
    });
    fields.extend(field_accessors);
    fields
}

fn derive_impl_defn(target_type: &Type, merge_type: &MergeType, generics: &Generics) -> syn::Item {
    let type_name = as_syn_ident(target_type.name());
    let arguments = generics_to_arguments(generics);
    let fields_impl = derive_fields_impl(target_type, &type_name, merge_type, None);
    parse_quote! {
        impl #generics #type_name #arguments {
            #(#fields_impl)*
        }
    }
}

fn derive_ref_impl_type(
    target_type: &Type,
    merge_type: &MergeType,
    is_mut: bool,
) -> (Item, Generics) {
    let name_extension = if is_mut { "Mut" } else { "" };
    let base_type_name = as_syn_ident(target_type.name());
    let type_name = as_syn_ident(format!("{0}Ref{1}", target_type.name(), name_extension));
    let bounds = &merge_type.generics;
    let generics: syn::Generics = parse_quote! {
        <
        'a,
        #(const #bounds: usize),*
        >
    };
    let generics_copy = generics.clone();
    let mutability = if is_mut { Some(Mut::default()) } else { None };
    let variant_defns = merge_type
        .variants
        .iter()
        .map(|variant| {
            let fork = &variant.fork;
            let fork = format!("{fork:?}");
            let fork = as_syn_ident(fork);
            let fork_name = as_syn_ident(variant.fork.name());
            let bounds = &variant.generics;
            let arguments: AngleBracketedGenericArguments = parse_quote! {
                <
                #(#bounds),*
                >
            };
            parse_quote! {
                #fork (
                   &'a #mutability #fork_name::#base_type_name #arguments
                )
            }
        })
        .collect::<Vec<syn::Variant>>();
    (
        parse_quote! {
            #[derive(Debug, PartialEq, Eq, HashTreeRoot)]
            #[ssz(transparent)]
            pub enum #type_name #generics {
                #(#variant_defns,)*
            }
        },
        generics_copy,
    )
}

fn derive_ref_impl_impl(
    target_type: &Type,
    merge_type: &MergeType,
    is_mut: bool,
    generics: &Generics,
) -> Item {
    let name_extension = if is_mut { "Mut" } else { "" };
    let base_type_name = as_syn_ident(target_type.name());
    let type_name = as_syn_ident(format!("{0}Ref{1}", target_type.name(), name_extension));
    let mut arguments = generics_to_arguments(generics);
    let lifetime: syn::GenericArgument = parse_quote! {
        'a
    };
    arguments.args.insert(0, lifetime);
    let ref_type = if is_mut { Some(RefType::Mutable) } else { Some(RefType::Immutable) };
    let fields_impl = derive_fields_impl(target_type, &base_type_name, merge_type, ref_type);
    parse_quote! {
        impl #generics #type_name #arguments {
            #(#fields_impl)*
        }
    }
}

fn derive_ref_impl_from(
    target_type: &Type,
    merge_type: &MergeType,
    is_mut: bool,
    generics: &Generics,
) -> Vec<Item> {
    let name_extension = if is_mut { "Mut" } else { "" };
    let mut arguments = generics_to_arguments(generics);
    let lifetime: syn::GenericArgument = parse_quote! {
        'a
    };
    arguments.args.insert(0, lifetime);

    let type_name = as_syn_ident(format!("{0}Ref{1}", target_type.name(), name_extension));
    let base_type = as_syn_ident(target_type.name());
    let mutability = if is_mut { Some(Mut::default()) } else { None };
    merge_type
        .variants
        .iter()
        .map(|variant| {
            let fork = &variant.fork;
            let fork_name = as_syn_ident(fork.name());
            let fork = as_syn_ident(format!("{fork:?}"));
            let bounds = &variant.generics;
            let local_arguments: AngleBracketedGenericArguments = parse_quote! {
                <
                #(#bounds),*
                >
            };
            parse_quote! {
                impl #generics From<&'a #mutability #fork_name::#base_type #local_arguments> for #type_name #arguments {
                    fn from(value: &'a #mutability #fork_name::#base_type #local_arguments) -> Self {
                        Self::#fork(value)
                    }
                }
            }
        })
        .collect()
}

fn derive_ref_impl(target_type: &Type, merge_type: &MergeType, is_mut: bool) -> Vec<Item> {
    let (type_defn, generics) = derive_ref_impl_type(target_type, merge_type, is_mut);
    let impl_defn = derive_ref_impl_impl(target_type, merge_type, is_mut, &generics);
    let from_defn = derive_ref_impl_from(target_type, merge_type, is_mut, &generics);
    let mut result = vec![type_defn, impl_defn];
    result.extend(from_defn);
    result
}

fn derive_serde_de_impl(target_type: &Type, generics: &Generics, merge_type: &MergeType) -> Item {
    let type_name = as_syn_ident(target_type.name());
    let arguments = generics_to_arguments(generics);
    let de_by_fork: Vec<Expr> = merge_type
        .variants
        .iter()
        .rev()
        .map(|variant| {
            let fork_name = as_syn_ident(format!("{:?}", variant.fork));
            parse_quote! {
                if let Ok(inner) = <_ as serde::Deserialize>::deserialize(&value) {
                    return Ok(Self::#fork_name(inner))
                }
            }
        })
        .collect();
    let mut generics = generics.clone();
    let de_lifetime: GenericParam = parse_quote! {
        'de
    };
    generics.params.insert(0, de_lifetime);
    parse_quote! {
        impl #generics serde::Deserialize<'de> for #type_name #arguments {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
              D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                #(#de_by_fork)*
                Err(serde::de::Error::custom("no variant could be deserialized from input"))
            }
        }
    }
}

fn as_syn(target_type: &Type, merge_type: &MergeType) -> Vec<Item> {
    let (type_defn, generics) = derive_type_defn(target_type, merge_type);

    let impl_defn = derive_impl_defn(target_type, merge_type, &generics);

    let serde_de_impl = derive_serde_de_impl(target_type, &generics, merge_type);

    let imports = target_type.imports();
    let mut result = vec![imports, type_defn, impl_defn, serde_de_impl];

    if target_type.needs_ref_types() {
        result.extend(derive_ref_impl(target_type, merge_type, false));
        result.extend(derive_ref_impl(target_type, merge_type, true));
    }

    result
}

fn render(target_type: &Type, items: Vec<Item>) {
    let warning = parse_quote! {
        //! WARNING: This file was derived by the `spec-gen` utility. DO NOT EDIT MANUALLY.
    };
    let target_file = syn::File { shebang: None, attrs: vec![warning], items };
    let output = prettyplease::unparse(&target_file);

    let type_name = target_type.name().to_case(Case::Snake);
    let target_path = format!("{SOURCE_ROOT}/types/{type_name}.rs");
    fs::write(target_path, output).unwrap();
}

pub fn run() {
    let fork_sequence = &[Fork::Phase0, Fork::Altair, Fork::Bellatrix, Fork::Capella, Fork::Deneb];
    let types = [
        Type::BeaconBlockBody,
        Type::BeaconBlock,
        Type::SignedBeaconBlock,
        Type::ExecutionPayload,
        Type::ExecutionPayloadHeader,
        Type::BlindedBeaconBlockBody,
        Type::BlindedBeaconBlock,
        Type::SignedBlindedBeaconBlock,
        Type::BeaconState,
    ];

    for target_type in &types {
        let defns = load_type_defns(target_type, fork_sequence);
        let merge_type = derive_merge_type(fork_sequence, defns);
        let output = as_syn(target_type, &merge_type);
        render(target_type, output)
    }

    let fork_sequence =
        &[Fork::Phase0, Fork::Altair, Fork::Bellatrix, Fork::Capella, Fork::Deneb, Fork::Electra];
    let types = [Type::ExecutionPayload, Type::ExecutionPayloadHeader];

    for target_type in &types {
        let defns = load_type_defns(target_type, fork_sequence);
        let merge_type = derive_merge_type(fork_sequence, defns);
        let output = as_syn(target_type, &merge_type);
        render(target_type, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_fork_cmp() {
        assert_eq!(Fork::Phase0.cmp(&Fork::Altair), Ordering::Less);
        assert_eq!(Fork::Bellatrix.cmp(&Fork::Altair), Ordering::Greater);
        assert_eq!(Fork::Capella.cmp(&Fork::Bellatrix), Ordering::Greater);
        assert_eq!(Fork::Deneb.cmp(&Fork::Capella), Ordering::Greater);
        assert_eq!(Fork::Electra.cmp(&Fork::Deneb), Ordering::Greater);
    }
}
