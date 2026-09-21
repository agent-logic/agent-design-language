use syn::{spanned::Spanned, visit::Visit};

#[derive(Default)]
pub(super) struct References {
    pub paths: Vec<(Vec<String>, usize)>,
    pub modules: Vec<(String, usize)>,
    pub unknowns: Vec<(usize, &'static str)>,
}
impl References {
    fn use_tree(&mut self, prefix: Vec<String>, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Path(p) => {
                let mut prefix = prefix;
                prefix.push(p.ident.to_string());
                self.use_tree(prefix, &p.tree);
            }
            syn::UseTree::Name(n) => {
                let mut p = prefix;
                p.push(n.ident.to_string());
                self.paths.push((p, n.span().start().line));
            }
            syn::UseTree::Group(g) => {
                for item in &g.items {
                    self.use_tree(prefix.clone(), item);
                }
            }
            syn::UseTree::Rename(r) => {
                let mut p = prefix;
                p.push(r.ident.to_string());
                self.paths.push((p, r.span().start().line));
                self.unknowns
                    .push((r.span().start().line, "alias_uses_not_resolved"));
            }
            syn::UseTree::Glob(g) => self
                .unknowns
                .push((g.span().start().line, "glob_import_not_resolved")),
        }
    }
}
impl<'ast> Visit<'ast> for References {
    fn visit_type_path(&mut self, path: &'ast syn::TypePath) {
        if path.qself.is_some()
            || (path.path.segments.len() == 1
                && !matches!(
                    path.path.segments[0].ident.to_string().as_str(),
                    "bool"
                        | "char"
                        | "str"
                        | "u8"
                        | "u16"
                        | "u32"
                        | "u64"
                        | "u128"
                        | "usize"
                        | "i8"
                        | "i16"
                        | "i32"
                        | "i64"
                        | "i128"
                        | "isize"
                        | "f32"
                        | "f64"
                ))
        {
            self.unknowns
                .push((path.span().start().line, "type_resolution_not_performed"));
        }
        syn::visit::visit_type_path(self, path);
    }
    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        self.unknowns
            .push((item.span().start().line, "extern_crate_not_resolved"));
    }
    fn visit_trait_bound(&mut self, bound: &'ast syn::TraitBound) {
        self.unknowns
            .push((bound.span().start().line, "trait_resolution_not_performed"));
        syn::visit::visit_trait_bound(self, bound);
    }
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        self.use_tree(Vec::new(), &item.tree);
        for attr in &item.attrs {
            self.visit_attribute(attr);
        }
    }
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        for attr in &item.attrs {
            self.visit_attribute(attr);
        }
        if item.content.is_some() {
            self.unknowns
                .push((item.span().start().line, "inline_module_not_resolved"));
        } else {
            self.modules
                .push((item.ident.to_string(), item.span().start().line));
        }
    }
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let parts: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        if matches!(
            parts.first().map(String::as_str),
            Some("crate" | "self" | "super")
        ) {
            self.paths.push((parts, path.span().start().line));
        } else if parts.len() > 1 {
            self.unknowns.push((
                path.span().start().line,
                "unqualified_or_external_path_not_resolved",
            ));
        }
        syn::visit::visit_path(self, path);
    }
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*call.func {
            if p.path.segments.len() == 1 {
                self.unknowns
                    .push((call.span().start().line, "unqualified_call_not_resolved"));
            }
        } else {
            self.unknowns
                .push((call.span().start().line, "dynamic_call_not_resolved"));
        }
        syn::visit::visit_expr_call(self, call);
    }
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        self.unknowns
            .push((call.span().start().line, "method_dispatch_not_resolved"));
        syn::visit::visit_expr_method_call(self, call);
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.unknowns
            .push((mac.span().start().line, "macro_expansion_not_analyzed"));
    }
    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        if !attr.path().is_ident("doc") {
            self.unknowns
                .push((attr.span().start().line, "attribute_semantics_not_analyzed"));
        }
    }
}
