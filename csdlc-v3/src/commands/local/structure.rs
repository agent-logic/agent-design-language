//! Validation of the registry-declared card structure, independent of rendered-value parity.
use markdown::{mdast::Node, to_mdast, ParseOptions};
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Deserialize, PartialEq, Eq, Debug)]
struct Heading {
    level: u8,
    text: Option<String>,
}
#[derive(Deserialize, PartialEq, Eq, Debug)]
struct Fence {
    ordinal: usize,
    info: String,
    heading_path: Vec<String>,
}
#[derive(Deserialize)]
struct LockedLine {
    heading_path: Vec<String>,
    text: String,
}
#[derive(Deserialize)]
struct Structure {
    schema: String,
    template_set: String,
    card_kind: String,
    headings: Vec<Heading>,
    fenced_blocks: Vec<Fence>,
    locked_lines: Vec<LockedLine>,
    frontmatter_keys: Vec<String>,
    // These fields describe allowed vocabulary, not mandatory line occurrences.
    scaffold_lines: Vec<String>,
    scaffold_line_prefixes: Vec<String>,
    rendered_value_line_prefixes: Vec<String>,
    editable_sections: Vec<String>,
}

#[derive(Default)]
struct StructureObservation {
    headings: Vec<Heading>,
    fences: Vec<Fence>,
    stack: Vec<(u8, String)>,
    locations: Vec<(usize, Vec<String>)>,
    code_lines: BTreeSet<usize>,
    frontmatter_keys: BTreeSet<String>,
}

impl StructureObservation {
    fn visit(&mut self, node: &Node, required_headings: &[Heading]) {
        match node {
            Node::Heading(heading) => {
                let text = node.to_string();
                while self
                    .stack
                    .last()
                    .is_some_and(|(depth, _)| *depth >= heading.depth)
                {
                    self.stack.pop();
                }
                self.stack.push((
                    heading.depth,
                    if required_headings
                        .get(self.headings.len())
                        .is_some_and(|heading| heading.text.is_none())
                    {
                        "<dynamic-heading>".into()
                    } else {
                        text.clone()
                    },
                ));
                self.headings.push(Heading {
                    level: heading.depth,
                    text: Some(text),
                });
                if let Some(pos) = node.position() {
                    self.locations.push((
                        pos.start.line,
                        self.stack.iter().map(|(_, s)| s.clone()).collect(),
                    ));
                }
            }
            Node::Code(code) => {
                if let Some(pos) = node.position() {
                    self.code_lines.extend(pos.start.line..=pos.end.line);
                }
                self.fences.push(Fence {
                    ordinal: self.fences.len(),
                    info: code.lang.clone().unwrap_or_default(),
                    heading_path: self.stack.iter().map(|(_, text)| text.clone()).collect(),
                });
            }
            Node::Yaml(yaml) => {
                // Schema paths cover block-mapping keys only, including nested pr_start keys.
                let mut parents: Vec<(usize, String)> = Vec::new();
                let mut sequence_indent = None;
                for line in yaml.value.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    let indent = line.len() - trimmed.len();
                    if sequence_indent.is_some_and(|depth| indent > depth) {
                        continue;
                    }
                    sequence_indent = None;
                    if trimmed.starts_with('-') {
                        sequence_indent = Some(indent);
                        continue;
                    }
                    let Some((key, _)) = trimmed.split_once(':') else {
                        continue;
                    };
                    if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || key.is_empty()
                    {
                        continue;
                    }
                    let indent = line.len() - trimmed.len();
                    while parents.last().is_some_and(|(depth, _)| *depth >= indent) {
                        parents.pop();
                    }
                    parents.push((indent, key.to_owned()));
                    self.frontmatter_keys.insert(
                        parents
                            .iter()
                            .map(|(_, key)| key.as_str())
                            .collect::<Vec<_>>()
                            .join("."),
                    );
                }
            }
            _ => {}
        }
        if let Some(children) = node.children() {
            for child in children {
                self.visit(child, required_headings);
            }
        }
    }
}

pub(super) fn validate(bytes: &[u8], version: &str, kind: &str, source: &str) -> bool {
    let Ok(schema) = serde_json::from_slice::<Structure>(bytes) else {
        return false;
    };
    if schema.schema != "adl.csdlc.prompt_card_structure.v1"
        || schema.template_set != version
        || schema.card_kind != kind
        || schema.headings.is_empty()
    {
        return false;
    }
    let mut options = ParseOptions::gfm();
    options.constructs.frontmatter = true;
    let Ok(ast) = to_mdast(source, &options) else {
        return false;
    };
    let mut observation = StructureObservation::default();
    observation.visit(&ast, &schema.headings);
    let StructureObservation {
        headings,
        fences,
        locations: heading_at_line,
        code_lines,
        frontmatter_keys,
        ..
    } = observation;
    if headings.len() != schema.headings.len()
        || headings
            .iter()
            .zip(&schema.headings)
            .any(|(actual, required)| {
                actual.level != required.level
                    || required
                        .text
                        .as_ref()
                        .is_some_and(|text| actual.text.as_ref() != Some(text))
            })
    {
        return false;
    }
    if fences != schema.fenced_blocks
        || frontmatter_keys != schema.frontmatter_keys.iter().cloned().collect()
    {
        return false;
    }
    let lines: Vec<_> = source.lines().collect();
    schema.locked_lines.iter().all(|locked| {
        lines.iter().enumerate().any(|(offset, line)| {
            let line_number = offset + 1;
            if code_lines.contains(&line_number) || line.trim_end() != locked.text {
                return false;
            }
            let path = heading_at_line
                .iter()
                .rev()
                .find(|(start, _)| *start <= line_number)
                .map(|(_, path)| path.as_slice())
                .unwrap_or(&[]);
            path == locked.heading_path
        })
    }) && {
        // Explicitly read vocabulary fields without interpreting them as required content.
        let _vocabulary = (
            &schema.scaffold_lines,
            &schema.scaffold_line_prefixes,
            &schema.rendered_value_line_prefixes,
            &schema.editable_sections,
        );
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::local::{structure_valid, PromptRegistry};
    use std::{fs, path::PathBuf};

    // PVF: deterministic local contract regression; no network, small CPU/file reads;
    // release gate: focused csdlc-v3 library validation. Proves structure rejection,
    // not operational lifecycle authority or hosted publication.
    #[test]
    fn active_six_card_schemas_accept_templates_and_reject_structure_damage() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_owned();
        let mut registry = PromptRegistry::from_current_json(
            &fs::read(root.join("docs/templates/prompts/current.json")).unwrap(),
        )
        .unwrap();
        for path in registry.structure_schema_paths.values_mut() {
            *path = root.join(&*path).to_string_lossy().into_owned();
        }
        let mut missing_schema_ref: serde_json::Value = serde_json::from_slice(
            &fs::read(root.join("docs/templates/prompts/current.json")).unwrap(),
        )
        .unwrap();
        missing_schema_ref["templates"]["sip"]
            .as_object_mut()
            .unwrap()
            .remove("structure_schema_path");
        assert!(PromptRegistry::from_current_json(
            &serde_json::to_vec(&missing_schema_ref).unwrap()
        )
        .is_err());
        for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            let template = fs::read_to_string(root.join(&registry.template_paths[kind])).unwrap();
            assert!(structure_valid(&registry, kind, &template), "active {kind}");
            if kind == "sor" {
                for slug in ["issue-523-planning", "Verification Summary", "Summary"] {
                    let rendered = template.replacen("# <slug>", &format!("# {slug}"), 1);
                    assert!(
                        structure_valid(&registry, kind, &rendered),
                        "dynamic SOR heading {slug}"
                    );
                }
            }
            let bytes = fs::read(&registry.structure_schema_paths[kind]).unwrap();
            let schema: Structure = serde_json::from_slice(&bytes).unwrap();
            let heading = &schema.headings[0];
            let damaged = template.replacen(
                &format!(
                    "{} {}",
                    "#".repeat(heading.level as usize),
                    heading.text.as_deref().unwrap_or("<slug>")
                ),
                "removed heading",
                1,
            );
            assert!(
                !structure_valid(&registry, kind, &damaged),
                "missing heading {kind}"
            );
            for locked in &schema.locked_lines {
                let damaged = template
                    .lines()
                    .map(|line| {
                        if line == locked.text {
                            "removed locked line"
                        } else {
                            line
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                assert!(
                    !structure_valid(&registry, kind, &damaged),
                    "missing locked line {kind}: {}",
                    locked.text
                );
            }
            let mut missing = registry.clone();
            missing.structure_schema_paths.remove(kind);
            assert!(!structure_valid(&missing, kind, &template));
            missing.structure_schema_paths.insert(
                kind.into(),
                root.join("absent-structure-schema.json")
                    .to_string_lossy()
                    .into_owned(),
            );
            assert!(!structure_valid(&missing, kind, &template));
            assert!(!validate(b"{}", &registry.version, kind, &template));
            assert!(!validate(&bytes, "wrong-version", kind, &template));
            assert!(!validate(
                &bytes,
                &registry.version,
                "wrong-kind",
                &template
            ));
            if let Some(key) = schema
                .frontmatter_keys
                .iter()
                .find(|key| !key.contains('.'))
            {
                let damaged = template.replacen(&format!("{key}:"), "unknown_key:", 1);
                assert!(
                    !structure_valid(&registry, kind, &damaged),
                    "missing frontmatter key {kind}"
                );
            }
            if !schema.fenced_blocks.is_empty() {
                let damaged = template.replacen("```yaml", "```text", 1);
                assert!(
                    !structure_valid(&registry, kind, &damaged),
                    "wrong fence language {kind}"
                );
            }
        }
    }
}
