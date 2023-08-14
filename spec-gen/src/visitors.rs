use syn::{
    parse_quote, visit, visit::Visit, visit_mut, visit_mut::VisitMut,
    AngleBracketedGenericArguments, Generics, Ident, ItemFn, Type,
};

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
struct ToGenericsVisitor {
    bounds: Vec<Ident>,
}

impl<'ast> Visit<'ast> for ToGenericsVisitor {
    fn visit_ident(&mut self, i: &'ast Ident) {
        if !self.bounds.contains(i) {
            self.bounds.push(i.clone());
        }
        visit::visit_ident(self, i);
    }
}

pub fn collate_generics_from(
    arguments: &[AngleBracketedGenericArguments],
    lifetimes: &[syn::Lifetime],
) -> Generics {
    let mut visitor = ToGenericsVisitor::default();

    for argument in arguments {
        visitor.visit_angle_bracketed_generic_arguments(argument);
    }

    let bounds = visitor.bounds;

    // NOTE: macro failed when trying to combine both arms into one invocation
    // it also seems like this will only suppport one explicit lifetime
    if lifetimes.is_empty() {
        parse_quote! {
            <
            #(const #bounds: usize),*
            >
        }
    } else {
        parse_quote! {
            <
            #(#lifetimes)*,
            #(const #bounds: usize),*
            >
        }
    }
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
}

impl<'ast> Visit<'ast> for TypeNameVisitor {
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
