//! Normalized import intent from pinned grammar fields; never a textual import parser.
use super::{
    resolver::{ImportForm, ImportSpec},
    Language, Span,
};
use anyhow::{ensure, Result};
use std::cell::Cell;
use tree_sitter::Node;
struct Budget {
    bytes: Cell<usize>,
}
impl Budget {
    fn check(&self, bytes: usize) -> Result<()> {
        ensure!(bytes <= self.bytes.get(), "language_import_byte_limit");
        Ok(())
    }
}

fn span(node: Node<'_>) -> Span {
    Span {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
    }
}
fn text<'s>(source: &'s str, node: Node<'_>) -> Result<&'s str> {
    source
        .get(node.byte_range())
        .ok_or_else(|| anyhow::anyhow!("language_import_span_invalid"))
}
fn children(node: Node<'_>) -> impl DoubleEndedIterator<Item = Node<'_>> {
    (0..node.named_child_count())
        .filter_map(move |i| u32::try_from(i).ok().and_then(|i| node.named_child(i)))
}
fn components(source: &str, node: Node<'_>, budget: &Budget) -> Result<Option<Vec<String>>> {
    let mut out = Vec::new();
    let mut stack = vec![node];
    let mut visited = 0;
    let mut text_bytes = 0usize;
    while let Some(n) = stack.pop() {
        visited += 1;
        budget.check(text_bytes + stack.len() * std::mem::size_of::<Node>())?;
        ensure!(visited <= 4096, "language_import_component_limit");
        match n.kind() {
            "identifier" | "type_identifier" | "crate" | "self" | "super" => {
                let s = text(source, n)?;
                ensure!(s.len() <= 4096, "language_import_text_limit");
                text_bytes += s.len() + std::mem::size_of::<String>();
                budget.check(text_bytes)?;
                out.push(s.to_owned());
            }
            "scoped_identifier" | "dotted_name" => {
                if n.child(0).is_some_and(|c| c.kind() == "::") {
                    return Ok(None);
                }
                for child in children(n).rev() {
                    budget.check(text_bytes + (stack.len() + 1) * std::mem::size_of::<Node>())?;
                    ensure!(stack.len() < 4096, "language_import_component_limit");
                    stack.push(child);
                }
            }
            _ => return Ok(None),
        }
    }
    Ok(Some(out))
}
fn context(source: &str, node: Node<'_>, budget: &Budget) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut parent = node.parent();
    let mut depth = 0;
    while let Some(p) = parent {
        depth += 1;
        budget.check(depth * std::mem::size_of::<String>())?;
        ensure!(depth <= 512, "language_import_context_limit");
        if p.kind() == "mod_item" && p.child_by_field_name("body").is_some() {
            if let Some(n) = p.child_by_field_name("name") {
                let value = text(source, n)?;
                budget.check(
                    names.iter().map(String::len).sum::<usize>()
                        + value.len()
                        + depth * std::mem::size_of::<String>(),
                )?;
                names.push(value.into());
            }
        }
        parent = p.parent();
    }
    names.reverse();
    Ok(names)
}
fn base(file: &str, node: Node<'_>, form: ImportForm) -> ImportSpec {
    ImportSpec {
        file: file.into(),
        span: span(node),
        form,
        components: Vec::new(),
        relative_depth: 0,
        alias: None,
        is_glob: false,
        module_context: Vec::new(),
        unsupported: false,
        specifier: None,
        imported: None,
    }
}
fn uncertain_module(node: Node<'_>, budget: &Budget) -> Result<bool> {
    let mut current = Some(node);
    let mut depth = 0;
    while let Some(n) = current {
        depth += 1;
        budget.check(0)?;
        ensure!(depth <= 512, "language_import_context_limit");
        if n.kind() == "mod_item" {
            let mut previous = n.prev_named_sibling();
            while let Some(p) = previous {
                budget.check(0)?;
                match p.kind() {
                    "attribute_item" => return Ok(true),
                    "line_comment" | "block_comment" => previous = p.prev_named_sibling(),
                    _ => break,
                }
            }
        }
        current = n.parent();
    }
    Ok(false)
}
fn push(
    out: &mut Vec<ImportSpec>,
    mut spec: ImportSpec,
    max: usize,
    budget: &Budget,
    node: Node<'_>,
    language: Language,
) -> Result<()> {
    ensure!(out.len() < max, "language_import_spec_limit");
    if language == Language::Rust {
        spec.unsupported |= uncertain_module(node, budget)?;
    }
    let bytes = serde_json::to_vec(&spec)?.len() + std::mem::size_of::<ImportSpec>();
    budget.check(bytes)?;
    budget.bytes.set(budget.bytes.get() - bytes);
    out.push(spec);
    Ok(())
}

/// Called once per relevant node by the existing bounded parser traversal.
/// `max_specs` is remaining operation budget, not a fresh per-node allowance.
pub(crate) fn collect(
    file: &str,
    language: Language,
    source: &str,
    node: Node<'_>,
    max_specs: usize,
    remaining_bytes: usize,
) -> Result<Vec<ImportSpec>> {
    let budget = Budget {
        bytes: Cell::new(remaining_bytes),
    };
    budget.check(0)?;
    let mut out = Vec::new();
    if node.has_error() || node.is_missing() {
        return Ok(out);
    }
    if matches!(
        node.kind(),
        "use_declaration"
            | "mod_item"
            | "extern_crate_declaration"
            | "package_declaration"
            | "import_declaration"
            | "class_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "annotation_type_declaration"
            | "import_statement"
            | "import_from_statement"
            | "future_import_statement"
            | "export_statement"
    ) {
        budget.check(node.end_byte() - node.start_byte())?;
    }
    match language {
        Language::Rust => match node.kind() {
            "use_declaration" => {
                let Some(arg) = node.child_by_field_name("argument") else {
                    return Ok(out);
                };
                let mut stack = vec![(arg, Vec::<String>::new())];
                let mut visited = 0;
                while let Some((n, prefix)) = stack.pop() {
                    visited += 1;
                    budget.check(
                        stack
                            .iter()
                            .map(|(_, p)| {
                                p.iter()
                                    .map(|v| v.len() + std::mem::size_of::<String>())
                                    .sum::<usize>()
                                    + std::mem::size_of::<Node>()
                            })
                            .sum(),
                    )?;
                    ensure!(
                        visited <= 4096 && stack.len() <= 4096,
                        "language_import_component_limit"
                    );
                    if n.kind() == "use_list" {
                        let prefix_bytes = prefix
                            .iter()
                            .map(|v| v.len() + std::mem::size_of::<String>())
                            .sum::<usize>();
                        let mut pending = stack
                            .iter()
                            .map(|(_, p)| {
                                p.iter()
                                    .map(|v| v.len() + std::mem::size_of::<String>())
                                    .sum::<usize>()
                                    + std::mem::size_of::<Node>()
                            })
                            .sum::<usize>();
                        for c in children(n).rev() {
                            pending += prefix_bytes + std::mem::size_of::<Node>();
                            budget.check(pending)?;
                            ensure!(
                                stack.len() < max_specs.min(4096),
                                "language_import_spec_limit"
                            );
                            stack.push((c, prefix.clone()));
                        }
                        continue;
                    }
                    if n.kind() == "scoped_use_list" {
                        let mut p = prefix;
                        let mut supported = true;
                        if let Some(path) = n.child_by_field_name("path") {
                            if let Some(parts) = components(source, path, &budget)? {
                                p.extend(parts)
                            } else {
                                supported = false
                            }
                        }
                        if supported {
                            if let Some(list) = n.child_by_field_name("list") {
                                stack.push((list, p));
                                continue;
                            }
                        }
                        let mut s = base(file, node, ImportForm::RustUse);
                        s.unsupported = true;
                        s.module_context = context(source, node, &budget)?;
                        push(&mut out, s, max_specs, &budget, node, language)?;
                        continue;
                    }
                    let mut s = base(file, node, ImportForm::RustUse);
                    s.components = prefix;
                    s.module_context = context(source, node, &budget)?;
                    let target = match n.kind() {
                        "use_as_clause" => {
                            s.alias = n
                                .child_by_field_name("alias")
                                .map(|a| text(source, a).map(str::to_owned))
                                .transpose()?;
                            n.child_by_field_name("path")
                        }
                        "use_wildcard" => {
                            s.is_glob = true;
                            children(n).next()
                        }
                        _ => Some(n),
                    };
                    if let Some(target) = target {
                        match components(source, target, &budget)? {
                            Some(p) => s.components.extend(p),
                            None => s.unsupported = true,
                        }
                    }
                    push(&mut out, s, max_specs, &budget, node, language)?;
                }
            }
            "mod_item" => {
                let mut s = base(
                    file,
                    node,
                    if node.child_by_field_name("body").is_some() {
                        ImportForm::RustInlineModule
                    } else {
                        ImportForm::RustOutOfLineModule
                    },
                );
                s.module_context = context(source, node, &budget)?;
                s.unsupported = node
                    .prev_named_sibling()
                    .is_some_and(|n| n.kind() == "attribute_item");
                if let Some(name) = node.child_by_field_name("name") {
                    s.components = vec![text(source, name)?.into()];
                } else {
                    s.unsupported = true;
                }
                push(&mut out, s, max_specs, &budget, node, language)?;
            }
            "extern_crate_declaration" => {
                let mut s = base(file, node, ImportForm::RustExternCrate);
                let mut ids = children(node).filter(|n| n.kind() == "identifier");
                if let Some(n) = ids.next() {
                    budget.check(text(source, n)?.len())?;
                    s.components = vec![text(source, n)?.into()];
                } else {
                    s.unsupported = true;
                }
                if let Some(n) = ids.next() {
                    budget.check(text(source, n)?.len())?;
                    s.alias = Some(text(source, n)?.into());
                }
                push(&mut out, s, max_specs, &budget, node, language)?;
            }
            _ => {}
        },
        Language::Java => {
            if matches!(node.kind(), "package_declaration" | "import_declaration") {
                let mut cursor = node.walk();
                let static_import = node.children(&mut cursor).any(|n| n.kind() == "static");
                let mut s = base(
                    file,
                    node,
                    if node.kind() == "package_declaration" {
                        ImportForm::JavaPackage
                    } else if static_import {
                        ImportForm::JavaStaticImport
                    } else {
                        ImportForm::JavaImport
                    },
                );
                s.is_glob = children(node).any(|n| n.kind() == "asterisk");
                if let Some(n) =
                    children(node).find(|n| matches!(n.kind(), "identifier" | "scoped_identifier"))
                {
                    if let Some(p) = components(source, n, &budget)? {
                        s.components = p
                    } else {
                        s.unsupported = true
                    }
                } else {
                    s.unsupported = true;
                }
                push(&mut out, s, max_specs, &budget, node, language)?;
            } else if matches!(
                node.kind(),
                "class_declaration"
                    | "interface_declaration"
                    | "enum_declaration"
                    | "record_declaration"
                    | "annotation_type_declaration"
            ) && node.parent().is_some_and(|p| p.kind() == "program")
            {
                let mut s = base(file, node, ImportForm::JavaType);
                let program = node.parent().unwrap();
                for p in children(program) {
                    budget.check(0)?;
                    if p.kind() == "package_declaration" {
                        if let Some(n) = children(p)
                            .find(|n| matches!(n.kind(), "identifier" | "scoped_identifier"))
                        {
                            if let Some(parts) = components(source, n, &budget)? {
                                s.components = parts
                            } else {
                                s.unsupported = true
                            }
                        }
                    }
                }
                if let Some(name) = node.child_by_field_name("name") {
                    s.components.push(text(source, name)?.into())
                } else {
                    s.unsupported = true;
                }
                push(&mut out, s, max_specs, &budget, node, language)?;
            }
        }
        Language::Python => {
            if matches!(
                node.kind(),
                "import_statement" | "import_from_statement" | "future_import_statement"
            ) {
                let from = node.kind() != "import_statement";
                let mut common = base(
                    file,
                    node,
                    if from {
                        ImportForm::PythonFrom
                    } else {
                        ImportForm::PythonImport
                    },
                );
                if from {
                    if let Some(module) = node.child_by_field_name("module_name") {
                        if module.kind() == "relative_import" {
                            for n in children(module) {
                                budget.check(0)?;
                                if n.kind() == "import_prefix" {
                                    let token = text(source, n)?;
                                    if token.bytes().all(|c| c == b'.') {
                                        common.relative_depth = token.len()
                                    } else {
                                        common.unsupported = true
                                    }
                                } else if let Some(p) = components(source, n, &budget)? {
                                    common.components.extend(p)
                                } else {
                                    common.unsupported = true
                                }
                            }
                        } else if let Some(p) = components(source, module, &budget)? {
                            common.components = p
                        } else {
                            common.unsupported = true
                        }
                    } else {
                        common.unsupported = true
                    }
                }
                let mut cursor = node.walk();
                for name in node.children_by_field_name("name", &mut cursor) {
                    budget.check(serde_json::to_vec(&common)?.len())?;
                    let mut s = common.clone();
                    let target = if name.kind() == "aliased_import" {
                        s.alias = name
                            .child_by_field_name("alias")
                            .map(|n| text(source, n).map(str::to_owned))
                            .transpose()?;
                        name.child_by_field_name("name")
                    } else {
                        Some(name)
                    };
                    if let Some(n) = target {
                        if let Some(parts) = components(source, n, &budget)? {
                            if from {
                                s.imported = Some(parts.join("."))
                            } else {
                                s.components = parts
                            }
                        } else {
                            s.unsupported = true
                        }
                    } else {
                        s.unsupported = true
                    }
                    push(&mut out, s, max_specs, &budget, node, language)?;
                }
                if children(node).any(|n| n.kind() == "wildcard_import") {
                    common.is_glob = true;
                    push(&mut out, common, max_specs, &budget, node, language)?;
                } else if out.is_empty() {
                    common.unsupported = true;
                    push(&mut out, common, max_specs, &budget, node, language)?;
                }
            }
        }
        Language::JavaScript => {
            if matches!(node.kind(), "import_statement" | "export_statement") {
                if let Some(value) = node.child_by_field_name("source") {
                    let mut s = base(file, value, ImportForm::JavaScriptModule);
                    s.specifier = decode_js_string(source, value, &budget)?;
                    s.unsupported = s.specifier.is_none();
                    push(&mut out, s, max_specs, &budget, node, language)?;
                }
            }
        }
    }
    Ok(out)
}
/// Decode grammar-identified string fragments/escapes only. Unsupported escape
/// semantics stay unresolved; this never interprets source as a JS program.
fn decode_js_string(source: &str, node: Node<'_>, budget: &Budget) -> Result<Option<String>> {
    if node.kind() != "string" {
        return Ok(None);
    }
    budget.check(text(source, node)?.len())?;
    // JSON decoding also correctly combines adjacent UTF-16 surrogate escapes.
    if let Ok(value) = serde_json::from_str::<String>(text(source, node)?) {
        return Ok(Some(value));
    }
    let mut out = String::new();
    for part in children(node) {
        budget.check(out.len() + text(source, part)?.len())?;
        match part.kind() {
            "string_fragment" => out.push_str(text(source, part)?),
            "escape_sequence" => {
                let e = text(source, part)?;
                match e {
                    "\\'" => out.push('\''),
                    "\\v" => out.push('\u{000b}'),
                    "\\\n" | "\\\r\n" | "\\\r" => {}
                    _ => {
                        if e.starts_with("\\x") && e.len() == 4 {
                            if let Ok(v) = u8::from_str_radix(&e[2..], 16) {
                                out.push(char::from(v));
                            } else {
                                return Ok(None);
                            }
                        } else if e.starts_with("\\u{") && e.ends_with('}') {
                            if let Some(c) = u32::from_str_radix(&e[3..e.len() - 1], 16)
                                .ok()
                                .and_then(char::from_u32)
                            {
                                out.push(c)
                            } else {
                                return Ok(None);
                            }
                        } else {
                            let encoded = format!("\"{e}\"");
                            if let Ok(v) = serde_json::from_str::<String>(&encoded) {
                                out.push_str(&v)
                            } else {
                                return Ok(None);
                            }
                        }
                    }
                }
            }
            _ => return Ok(None),
        }
        ensure!(out.len() <= 4096, "language_import_text_limit");
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tree(source: &str) -> tree_sitter::Tree {
        let mut p = tree_sitter::Parser::new();
        p.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
        p.parse(source, None).unwrap()
    }
    fn find<'a>(tree: &'a tree_sitter::Tree, kind: &str) -> Node<'a> {
        let mut stack = vec![tree.root_node()];
        while let Some(n) = stack.pop() {
            if n.kind() == kind {
                return n;
            }
            stack.extend(children(n));
        }
        panic!("node absent")
    }
    #[test]
    fn conditional_ancestor_marks_nested_use_unsupported() {
        let source =
            "#[cfg(feature = \"x\")] mod conditional { mod nested { use crate::target; } }";
        let t = tree(source);
        let n = find(&t, "use_declaration");
        let out = collect("lib.rs", Language::Rust, source, n, 16, 65536).unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].unsupported);
        assert_eq!(out[0].module_context, vec!["conditional", "nested"]);
    }
    #[test]
    fn collector_honors_remaining_byte_and_spec_limits() {
        let source = "use crate::{alpha,beta,gamma,delta};";
        let t = tree(source);
        let n = find(&t, "use_declaration");
        assert!(collect("lib.rs", Language::Rust, source, n, 100, 1,)
            .unwrap_err()
            .to_string()
            .contains("byte_limit"));
        assert!(collect("lib.rs", Language::Rust, source, n, 2, 65536,).is_err());
    }
    #[test]
    fn retained_specs_fit_remaining_envelope_budget() {
        let source = "use crate::{alpha,beta};";
        let t = tree(source);
        let n = find(&t, "use_declaration");
        let out = collect("lib.rs", Language::Rust, source, n, 16, 4096).unwrap();
        let bytes: usize = out
            .iter()
            .map(|s| serde_json::to_vec(s).unwrap().len() + std::mem::size_of::<ImportSpec>())
            .sum();
        assert!(bytes <= 4096);
        assert!(collect("lib.rs", Language::Rust, source, n, 16, bytes - 1,).is_err());
    }
}
