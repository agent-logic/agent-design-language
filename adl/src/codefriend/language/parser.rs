//! Source-buffer parsing. No project configuration, code or build script is executed.
use super::*;
use std::sync::Mutex;
use tree_sitter::{Node, Parser};

// Do not queue an unbounded number of source-bearing parses in the application.
static PARSER: Mutex<()> = Mutex::new(());
const EXTRACTION_VERSION: &str = "codefriend.syntax_facts.v1";

fn grammar(language: Language) -> tree_sitter::Language {
    match language {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::Java => tree_sitter_java::LANGUAGE.into(),
        Language::Python => tree_sitter_python::LANGUAGE.into(),
        Language::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
    }
}

fn identity(language: Language) -> Result<ToolIdentity> {
    let revision = match language {
        Language::Rust => "tree-sitter-rust@0.24.2",
        Language::Java => "tree-sitter-java@0.23.5",
        Language::Python => "tree-sitter-python@0.25.0",
        Language::JavaScript => "tree-sitter-javascript@0.25.0",
    };
    Ok(ToolIdentity {
        name: "tree-sitter".into(),
        version: "0.26.13".into(),
        grammar_revision: revision.into(),
        // The actual extraction implementation, not an asserted arbitrary query name.
        query_digest: hash(&(
            EXTRACTION_VERSION,
            include_str!("parser.rs"),
            include_str!("imports.rs"),
            include_str!("resolver.rs"),
        ))?,
    })
}

fn span(node: Node<'_>) -> Span {
    Span {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
    }
}

fn diagnostic(code: &str, node: Option<Node<'_>>) -> Diagnostic {
    Diagnostic {
        code: code.into(),
        span: node.map(span),
    }
}

fn source_kind(path: &str, language: Language) -> bool {
    match language {
        Language::Rust => path.ends_with(".rs"),
        Language::Java => path.ends_with(".java"),
        Language::Python => path.ends_with(".py") || path.ends_with(".pyi"),
        Language::JavaScript => [".js", ".mjs", ".cjs", ".jsx"]
            .iter()
            .any(|e| path.ends_with(e)),
    }
}

fn is_declaration(language: Language, kind: &str) -> bool {
    match language {
        Language::Rust => matches!(
            kind,
            "function_item"
                | "function_signature_item"
                | "struct_item"
                | "enum_item"
                | "trait_item"
                | "type_item"
                | "mod_item"
                | "const_item"
                | "static_item"
                | "macro_definition"
        ),
        Language::Java => matches!(
            kind,
            "class_declaration"
                | "interface_declaration"
                | "enum_declaration"
                | "record_declaration"
                | "method_declaration"
                | "constructor_declaration"
                | "annotation_type_declaration"
                | "variable_declarator"
        ),
        Language::Python => matches!(kind, "function_definition" | "class_definition"),
        Language::JavaScript => matches!(
            kind,
            "function_declaration"
                | "generator_function_declaration"
                | "class_declaration"
                | "method_definition"
                | "variable_declarator"
        ),
    }
}

fn is_import(language: Language, kind: &str) -> bool {
    match language {
        Language::Rust => matches!(
            kind,
            "use_declaration" | "extern_crate_declaration" | "mod_item"
        ),
        Language::Java => matches!(kind, "import_declaration" | "package_declaration"),
        Language::Python => matches!(
            kind,
            "import_statement" | "import_from_statement" | "future_import_statement"
        ),
        Language::JavaScript => matches!(kind, "import_statement" | "export_statement"),
    }
}

fn facts_for_node(file: &mut FileAnalysis, source: &str, node: Node<'_>) -> Result<()> {
    let language = file.coverage.language;
    if is_declaration(language, node.kind()) {
        if let Some(name) = node.child_by_field_name("name") {
            // Destructuring needs a dedicated binding adapter, not an invented scalar name.
            if matches!(
                name.kind(),
                "identifier" | "type_identifier" | "property_identifier"
            ) {
                let spelling = &source[name.byte_range()];
                if spelling.len() <= 4096 {
                    let evidence = file
                        .coverage
                        .evidence_id
                        .as_deref()
                        .ok_or_else(|| anyhow::anyhow!("language_source_evidence_missing"))?;
                    file.facts.declarations.push(Declaration {
                        id: declaration_id(evidence, language, span(name), spelling, node.kind())?,
                        name: spelling.into(),
                        kind: node.kind().into(),
                        span: span(name),
                    });
                } else {
                    file.coverage
                        .diagnostics
                        .push(diagnostic("fact_text_limit", Some(name)));
                }
            }
        }
    }
    if is_import(language, node.kind()) {
        // Export declarations without a module source are declarations, not imports.
        let candidate = if language == Language::JavaScript {
            node.child_by_field_name("source")
        } else {
            Some(node)
        };
        if let Some(value) = candidate {
            let spelling = &source[value.byte_range()];
            if spelling.len() <= 4096 {
                file.facts.imports.push(Import {
                    spelling: spelling.into(),
                    kind: node.kind().into(),
                    span: span(value),
                    target_path: None,
                });
            } else {
                file.coverage
                    .diagnostics
                    .push(diagnostic("fact_text_limit", Some(value)));
            }
        }
    }
    let declaration_name = node.parent().is_some_and(|parent| {
        is_declaration(language, parent.kind()) && parent.child_by_field_name("name") == Some(node)
    });
    if matches!(
        node.kind(),
        "identifier" | "type_identifier" | "field_identifier" | "property_identifier"
    ) && !declaration_name
    {
        let spelling = &source[node.byte_range()];
        if spelling.len() <= 4096 {
            file.facts.references.push(Reference {
                spelling: spelling.into(),
                span: span(node),
                target_declaration: None,
            });
        } else {
            file.coverage
                .diagnostics
                .push(diagnostic("fact_text_limit", Some(node)));
        }
    }
    Ok(())
}

/// Produce source-backed syntax facts, retaining explicit unresolved structure and semantics.
/// This is not a substitute for the language-specific project and semantic resolvers.
pub(super) fn analyze(
    admission: &Admission,
    policy: &AnalysisPolicy,
    now: u64,
) -> Result<(AnalysisReport, Vec<resolver::ImportSpec>)> {
    policy.validate(admission)?;

    ensure!(
        now >= admission.admitted_at && now < admission.expires_at,
        "language_admission_not_live"
    );
    let _slot = PARSER
        .try_lock()
        .map_err(|_| anyhow::anyhow!("language_parser_busy"))?;
    let mut report = AnalysisReport {
        schema: VERSION.into(),
        admission_digest: admission.digest.clone(),
        policy_digest: policy.digest()?,
        toolchain: BTreeMap::new(),
        files: Vec::new(),
        digest: String::new(),
    };
    let mut nodes = 0usize;
    let mut facts = 0usize;
    let mut payload_bytes = 0usize;
    let mut specs = Vec::new();
    let mut spec_bytes = 0usize;
    for (path, language) in &policy.files {
        report
            .toolchain
            .entry(*language)
            .or_insert(identity(*language)?);
        let object = admission
            .packet
            .objects
            .iter()
            .find(|o| &o.path == path)
            .ok_or_else(|| anyhow::anyhow!("language_source_missing"))?;
        let mut file = FileAnalysis {
            coverage: FileCoverage {
                path: path.clone(),
                evidence_id: admission
                    .evidence
                    .iter()
                    .find(|e| &e.path == path)
                    .map(|e| e.id.clone()),
                content_digest: object.content_digest.clone(),
                language: *language,
                syntax: Coverage::Unsupported,
                structure: Coverage::Unsupported,
                semantics: Coverage::Unsupported,
                diagnostics: Vec::new(),
            },
            facts: SourceFacts::default(),
        };
        if let Some(source) = object
            .content
            .as_deref()
            .filter(|_| source_kind(path, *language))
        {
            let mut parser = Parser::new();
            parser.set_language(&grammar(*language))?;
            let tree = parser
                .parse(source.as_bytes(), None)
                .ok_or_else(|| anyhow::anyhow!("language_parser_interrupted"))?;
            file.coverage.syntax = Coverage::Complete;
            file.coverage.structure = Coverage::Partial;
            file.coverage
                .diagnostics
                .push(diagnostic("project_resolution_not_performed", None));
            file.coverage
                .diagnostics
                .push(diagnostic("semantic_resolution_not_performed", None));
            let mut cursor = tree.walk();
            let mut depth = 0usize;
            loop {
                nodes += 1;
                ensure!(
                    nodes <= policy.limits.max_nodes && depth <= policy.limits.max_depth,
                    "language_traversal_limit"
                );
                let node = cursor.node();
                let previous = (
                    file.facts.declarations.len(),
                    file.facts.imports.len(),
                    file.facts.references.len(),
                    file.coverage.diagnostics.len(),
                );
                if node.is_error() || node.is_missing() {
                    file.coverage.syntax = Coverage::Partial;
                    file.coverage.diagnostics.push(diagnostic(
                        if node.is_error() {
                            "parse_error"
                        } else {
                            "missing_syntax"
                        },
                        Some(node),
                    ));
                }
                // Facts inside recovered erroneous syntax are not admitted as declarations.
                if !node.has_error() && !node.is_missing() {
                    facts_for_node(&mut file, source, node)?;
                    for spec in imports::collect(
                        path,
                        *language,
                        source,
                        node,
                        policy.limits.max_facts.saturating_sub(specs.len()),
                        policy.limits.max_output_bytes.saturating_sub(spec_bytes),
                    )? {
                        spec_bytes += serde_json::to_vec(&spec)?.len();
                        ensure!(
                            spec_bytes <= policy.limits.max_output_bytes,
                            "language_resolution_spec_limit"
                        );
                        ensure!(
                            specs.len() < policy.limits.max_facts,
                            "language_resolution_spec_limit"
                        );
                        specs.push(spec);
                    }
                }
                // Count each new fact once. Never accumulate a full file of oversized
                // source text before checking the aggregate serialized-output budget.
                for value in &file.facts.declarations[previous.0..] {
                    payload_bytes += serde_json::to_vec(value)?.len();
                }
                for value in &file.facts.imports[previous.1..] {
                    payload_bytes += serde_json::to_vec(value)?.len();
                }
                for value in &file.facts.references[previous.2..] {
                    payload_bytes += serde_json::to_vec(value)?.len();
                }
                for value in &file.coverage.diagnostics[previous.3..] {
                    payload_bytes += serde_json::to_vec(value)?.len();
                }
                ensure!(
                    payload_bytes <= policy.limits.max_output_bytes,
                    "language_output_limit"
                );
                let current = file.facts.declarations.len()
                    + file.facts.imports.len()
                    + file.facts.references.len()
                    + file.coverage.diagnostics.len();
                ensure!(
                    facts + current <= policy.limits.max_facts,
                    "language_fact_limit"
                );
                if !node.is_error() && !node.is_missing() && cursor.goto_first_child() {
                    depth += 1;
                    continue;
                }
                loop {
                    if cursor.goto_next_sibling() {
                        break;
                    }
                    if !cursor.goto_parent() {
                        break;
                    }
                    depth -= 1;
                }
                if depth == 0 && cursor.node() == tree.root_node() {
                    break;
                }
            }
        } else {
            file.coverage.diagnostics.push(diagnostic(
                if object.content.is_none() {
                    "source_not_available"
                } else {
                    "unsupported_source_kind"
                },
                None,
            ));
        }
        facts += file.facts.declarations.len()
            + file.facts.imports.len()
            + file.facts.references.len()
            + file.coverage.diagnostics.len();
        report.files.push(file);
        // Enforce aggregate output incrementally, before retaining another file's facts.
        ensure!(
            serde_json::to_vec(&report)?.len() <= policy.limits.max_output_bytes,
            "language_output_limit"
        );
    }
    report.digest = report.expected_digest()?;
    report.validate(admission, policy, now)?;

    Ok((report, specs))
}
