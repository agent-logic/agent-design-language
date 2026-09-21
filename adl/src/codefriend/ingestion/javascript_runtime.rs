//! Narrow syntax proof for runtime references, never a credential-value allowlist.
use std::{collections::BTreeSet, sync::Mutex};
use tree_sitter::Node;
static PARSER: Mutex<()> = Mutex::new(());
const MAX_BYTES: usize = 1024 * 1024;
const MAX_NODES: usize = 100_000;
const MAX_DEPTH: usize = 128;
const MAX_PAIRS: usize = 4096;

fn member(node: Node<'_>, depth: usize) -> bool {
    if depth > MAX_DEPTH {
        return false;
    }
    if node.kind() != "member_expression" {
        return false;
    }
    let (Some(object), Some(property)) = (
        node.child_by_field_name("object"),
        node.child_by_field_name("property"),
    ) else {
        return false;
    };
    property.kind() == "property_identifier"
        && (object.kind() == "identifier" || member(object, depth + 1))
}
fn runtime(node: Node<'_>, source: &[u8]) -> bool {
    if member(node, 0) {
        return true;
    }
    if node.kind() != "binary_expression" {
        return false;
    }
    let (Some(left), Some(right), Some(operator)) = (
        node.child_by_field_name("left"),
        node.child_by_field_name("right"),
        node.child_by_field_name("operator"),
    ) else {
        return false;
    };
    member(left, 0)
        && right.kind() == "string"
        && matches!(right.utf8_text(source), Ok("\"\"") | Ok("''"))
        && matches!(operator.utf8_text(source), Ok("||") | Ok("??"))
}
pub(super) fn assignment_colons(path: &str, content: &str) -> BTreeSet<usize> {
    let empty = BTreeSet::new();
    if !matches!(
        std::path::Path::new(path)
            .extension()
            .and_then(|s| s.to_str()),
        Some("js" | "mjs" | "cjs")
    ) || content.len() > MAX_BYTES
    {
        return empty;
    }
    let Ok(_guard) = PARSER.lock() else {
        return empty;
    };
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .is_err()
    {
        return empty;
    }
    let Some(tree) = parser.parse(content, None) else {
        return empty;
    };
    if tree.root_node().has_error() {
        return empty;
    }
    let mut cursor = tree.walk();
    let (mut visited, mut depth) = (0, 0);
    let mut proven = BTreeSet::new();
    loop {
        visited += 1;
        if visited > MAX_NODES || depth > MAX_DEPTH {
            return empty;
        }
        let node = cursor.node();
        if node.kind() == "pair" {
            if let (Some(key), Some(value)) = (
                node.child_by_field_name("key"),
                node.child_by_field_name("value"),
            ) {
                // Computed keys and escaped keys are intentionally not normalized here.
                if matches!(key.kind(), "string" | "property_identifier")
                    && key
                        .utf8_text(content.as_bytes())
                        .is_ok_and(|key| !key.contains('\\') && super::credential_key(key))
                    && runtime(value, content.as_bytes())
                {
                    let mut children = node.walk();
                    for child in node.children(&mut children) {
                        if child.kind() == ":" {
                            proven.insert(child.start_byte());
                            if proven.len() > MAX_PAIRS {
                                return empty;
                            }
                        }
                    }
                }
            }
        }
        if cursor.goto_first_child() {
            depth += 1;
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return proven;
            }
            depth -= 1;
        }
    }
}
