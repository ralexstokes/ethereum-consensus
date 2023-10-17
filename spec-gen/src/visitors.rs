use syn::{
    parse_quote, visit, visit::Visit, visit_mut, visit_mut::VisitMut,
    AngleBracketedGenericArguments, Generics, Ident, ItemFn, Type,
};

pub fn as_syn_ident(s: String) -> Ident {
    syn::parse_str(&s).unwrap()
}

#[derive(Default)]
struct LifetimeTypeVisitor {
    items: Vec<syn::Lifetime>,
}

impl<'ast> Visit<'ast> for LifetimeTypeVisitor {
    fn visit_lifetime(&mut self, i: &'ast syn::Lifetime) {
        if !self.items.contains(i) {
            self.items.push(i.clone())
        }
        visit::visit_lifetime(self, i);
    }
}

pub fn collect_lifetimes(f: &ItemFn) -> Vec<syn::Lifetime> {
    let mut visitor = LifetimeTypeVisitor::default();
    visitor.visit_item_fn(f);
    visitor.items
}

#[derive(Default)]
struct TypeParamVisitor {
    in_context: bool,
    items: Vec<syn::TypeParam>,
    bounds: Vec<String>,
}

impl<'ast> Visit<'ast> for TypeParamVisitor {
    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        if self.in_context {
            self.bounds.push(i.ident.to_string());
        }
    }

    fn visit_type_param_bound(&mut self, i: &'ast syn::TypeParamBound) {
        self.in_context = true;
        visit::visit_type_param_bound(self, i);
        self.in_context = false;
    }

    fn visit_type_param(&mut self, i: &'ast syn::TypeParam) {
        if !self.items.contains(i) {
            // NOTE: only supports a single bound for now
            i.bounds.iter().for_each(|i| {
                visit::visit_type_param_bound(self, i);
            });
            self.items.push(i.clone());
        }
        visit::visit_type_param(self, i);
    }
}

// returns type params and the name of their trait bound
pub fn collect_type_params(g: &Generics) -> (Vec<syn::TypeParam>, Vec<String>) {
    let mut visitor = TypeParamVisitor::default();
    visitor.visit_generics(g);
    (visitor.items, visitor.bounds)
}

#[derive(Default)]
pub struct ToGenericsVisitor {
    pub bounds: Vec<Ident>,
    in_context: bool,
}

impl<'ast> Visit<'ast> for ToGenericsVisitor {
    fn visit_ident(&mut self, i: &'ast Ident) {
        if !self.bounds.contains(i) && i != "usize" && self.in_context {
            self.bounds.push(i.clone());
        }
        visit::visit_ident(self, i);
    }

    fn visit_angle_bracketed_generic_arguments(&mut self, i: &'ast AngleBracketedGenericArguments) {
        self.in_context = true;
        visit::visit_angle_bracketed_generic_arguments(self, i);
        self.in_context = false;
    }

    fn visit_generics(&mut self, i: &'ast Generics) {
        self.in_context = true;
        visit::visit_generics(self, i);
        self.in_context = false;
    }
}

pub fn collate_generics_from(
    arguments: &[AngleBracketedGenericArguments],
    lifetimes: &[syn::Lifetime],
    type_params: &[syn::TypeParam],
) -> Generics {
    let mut visitor = ToGenericsVisitor::default();

    for argument in arguments {
        visitor.visit_angle_bracketed_generic_arguments(argument);
    }

    let bounds = visitor.bounds;

    let mut generics: syn::Generics = parse_quote! {
        <
        #(const #bounds: usize),*
        >
    };

    if !lifetimes.is_empty() {
        let lifetimes: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> = parse_quote! {
            #(#lifetimes)*,
        };
        generics.params.extend(lifetimes);
    }

    if !type_params.is_empty() {
        let type_params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]> = parse_quote! {
            #(#type_params)*,
        };
        generics.params.extend(type_params);
    }

    generics
}

#[derive(Default)]
struct ToArgumentsVisitor {
    bounds: Vec<Ident>,
}

impl<'ast> Visit<'ast> for ToArgumentsVisitor {
    fn visit_const_param(&mut self, i: &'ast syn::ConstParam) {
        self.bounds.push(i.ident.clone());
        visit::visit_const_param(self, i);
    }
}

pub fn generics_to_arguments(generics: &Generics) -> AngleBracketedGenericArguments {
    let mut visitor = ToArgumentsVisitor::default();

    visitor.visit_generics(generics);

    let bounds = visitor.bounds;

    parse_quote! {
        <
        #(#bounds),*
        >
    }
}

#[derive(Default)]
pub struct TypeNameVisitor {
    in_context: bool,
    pub names: Vec<String>,
}

impl TypeNameVisitor {
    // NOTE: this type collects type names in the order they appear in the source
    // This assumption is used in other parts of the codebase, so DO NOT violate.
    pub fn analyze(&mut self, f: &ItemFn) {
        self.visit_item_fn(f)
    }

    pub fn analyze_type(&mut self, t: &Type) {
        self.in_context = true;
        self.visit_type(t);
        self.in_context = false;
    }
}

impl<'ast> Visit<'ast> for TypeNameVisitor {
    fn visit_return_type(&mut self, i: &'ast syn::ReturnType) {
        self.in_context = true;
        visit::visit_return_type(self, i);
        self.in_context = false;
    }

    fn visit_path_segment(&mut self, i: &'ast syn::PathSegment) {
        if self.in_context {
            let name = i.ident.to_string();
            self.names.push(name);
        }
        visit::visit_path_segment(self, i);
    }

    fn visit_pat_type(&mut self, i: &'ast syn::PatType) {
        match &*i.ty {
            Type::Reference(node) => {
                self.in_context = true;
                visit::visit_type_reference(self, node);
                self.in_context = false;
            }
            Type::Path(node) => {
                self.in_context = true;
                visit::visit_type_path(self, node);
                self.in_context = false;
            }
            ty => println!("skipping type during visit: {ty:?}"),
        }
        visit::visit_pat_type(self, i);
    }
}

pub struct ArgumentsEditor<'a> {
    ident: &'a str,
    edit: &'a AngleBracketedGenericArguments,
    in_context: bool,
}

impl<'a> ArgumentsEditor<'a> {
    pub fn new(ident: &'a str, edit: &'a AngleBracketedGenericArguments) -> Self {
        Self { ident, edit, in_context: false }
    }

    pub fn edit(&mut self, fragment: &mut syn::ItemFn) {
        self.visit_item_fn_mut(fragment);
    }
}

impl<'a> VisitMut for ArgumentsEditor<'a> {
    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        i: &mut syn::AngleBracketedGenericArguments,
    ) {
        if self.in_context {
            *i = self.edit.clone();
        }
        visit_mut::visit_angle_bracketed_generic_arguments_mut(self, i);
    }

    fn visit_path_segment_mut(&mut self, i: &mut syn::PathSegment) {
        let should_edit = i.ident == self.ident;
        self.in_context = should_edit;
        visit_mut::visit_path_segment_mut(self, i);
        self.in_context = false;
    }
}
