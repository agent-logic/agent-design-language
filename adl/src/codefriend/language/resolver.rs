//! Deterministic module edges over admitted source only. Inputs are produced by
//! the pinned syntax traversal, never recovered by parsing printed imports.
use super::{AnalysisPolicy, AnalysisReport, Coverage, Diagnostic, Language, Span};
use crate::codefriend::evidence::Admission;
use anyhow::{ensure, Result};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub(crate) enum ImportForm {
    RustUse,
    RustExternCrate,
    RustOutOfLineModule,
    RustInlineModule,
    JavaImport,
    JavaStaticImport,
    JavaPackage,
    JavaType,
    PythonImport,
    PythonFrom,
    JavaScriptModule,
}
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ImportSpec {
    pub file: String,
    pub span: Span,
    pub form: ImportForm,
    pub components: Vec<String>,
    pub relative_depth: usize,
    pub alias: Option<String>,
    pub is_glob: bool,
    pub module_context: Vec<String>,
    pub unsupported: bool,
    /// Already decoded by the JavaScript syntax owner; not quoted source text.
    pub specifier: Option<String>,
    /// Python `from module import member`; module is in components.
    pub imported: Option<String>,
}
type Key = (Language, String, Vec<String>);
#[derive(Default)]
struct ModuleIndex {
    entries: BTreeMap<Key, BTreeSet<String>>,
}
impl ModuleIndex {
    fn add(&mut self, language: Language, root: &str, components: Vec<String>, path: &str) {
        self.entries
            .entry((language, root.into(), components))
            .or_default()
            .insert(path.into());
    }
    fn get(&self, language: Language, root: &str, components: &[String]) -> BTreeSet<String> {
        self.entries
            .get(&(language, root.into(), components.to_vec()))
            .cloned()
            .unwrap_or_default()
    }
}
// Every ancestor must be admitted and unique. A child discovered before a
// later ambiguity is never independently trusted at lookup time.
fn rust_context(
    index: &ModuleIndex,
    root: &str,
    base: &[String],
    context: &[String],
    owner: &str,
) -> bool {
    for n in 0..=base.len() {
        if index.get(Language::Rust, root, &base[..n]).len() != 1 {
            return false;
        }
    }
    let expected = BTreeSet::from([owner.to_owned()]);
    if index.get(Language::Rust, root, base) != expected {
        return false;
    }
    let mut module = base.to_vec();
    for component in context {
        module.push(component.clone());
        if index.get(Language::Rust, root, &module) != expected {
            return false;
        }
    }
    true
}
fn root<'a>(policy: &'a AnalysisPolicy, path: &str, language: Language) -> Option<&'a str> {
    policy
        .roots
        .iter()
        .filter(|r| {
            r.language == language && (r.root == "." || path.starts_with(&format!("{}/", r.root)))
        })
        .max_by_key(|r| r.root.len())
        .map(|r| r.root.as_str())
}
fn relative<'a>(path: &'a str, root: &str) -> &'a str {
    if root == "." {
        path
    } else {
        &path[root.len() + 1..]
    }
}
fn joined(root: &str, path: &str) -> String {
    if root == "." {
        path.into()
    } else {
        format!("{root}/{path}")
    }
}
fn identity(form: ImportForm) -> bool {
    matches!(
        form,
        ImportForm::JavaPackage | ImportForm::JavaType | ImportForm::RustInlineModule
    )
}
fn language(form: ImportForm) -> Language {
    match form {
        ImportForm::RustUse
        | ImportForm::RustExternCrate
        | ImportForm::RustOutOfLineModule
        | ImportForm::RustInlineModule => Language::Rust,
        ImportForm::JavaImport
        | ImportForm::JavaStaticImport
        | ImportForm::JavaPackage
        | ImportForm::JavaType => Language::Java,
        ImportForm::PythonImport | ImportForm::PythonFrom => Language::Python,
        ImportForm::JavaScriptModule => Language::JavaScript,
    }
}
fn python_module(path: &str) -> Option<Vec<String>> {
    let stem = path
        .strip_suffix(".pyi")
        .or_else(|| path.strip_suffix(".py"))?;
    let mut parts: Vec<String> = stem.split('/').map(str::to_owned).collect();
    if parts.last().is_some_and(|v| v == "__init__") {
        parts.pop();
    }
    Some(parts)
}
fn module_file_candidates(root: &str, owner: &str, context: &[String], name: &str) -> Vec<String> {
    let rel = relative(owner, root);
    let parent = rel.rsplit_once('/').map(|v| v.0).unwrap_or("");
    let filename = rel.rsplit('/').next().unwrap_or(rel);
    let base = if matches!(filename, "lib.rs" | "main.rs" | "mod.rs") {
        parent.to_owned()
    } else {
        rel.strip_suffix(".rs").unwrap_or(rel).to_owned()
    };
    let mut parts: Vec<&str> = if base.is_empty() {
        vec![]
    } else {
        base.split('/').collect()
    };
    parts.extend(context.iter().map(String::as_str));
    parts.push(name);
    let base = parts.join("/");
    vec![
        joined(root, &format!("{base}.rs")),
        joined(root, &format!("{base}/mod.rs")),
    ]
}

/// Only an authenticated syntax owner may supply `syntax` and `specs`. Their
/// self hashes establish consistency, not provenance. No new parse or external
/// execution is performed.
pub(crate) fn resolve(
    admission: &Admission,
    policy: &AnalysisPolicy,
    syntax: &AnalysisReport,
    specs: &[ImportSpec],
    now: u64,
) -> Result<AnalysisReport> {
    syntax.validate(admission, policy, now)?;
    ensure!(
        specs.len() <= policy.limits.max_facts,
        "language_resolution_spec_limit"
    );
    let mut admitted = BTreeMap::new();
    for file in &syntax.files {
        if file.coverage.syntax == Coverage::Complete && file.coverage.evidence_id.is_some() {
            let path = &file.coverage.path;
            if admission
                .packet
                .objects
                .iter()
                .any(|o| &o.path == path && o.content.is_some())
            {
                admitted.insert(path.clone(), file.coverage.language);
            }
        }
    }
    let mut spec_bytes = 0usize;
    for s in specs {
        ensure!(
            policy.files.get(&s.file) == Some(&language(s.form)),
            "language_resolution_spec_language"
        );
        ensure!(
            s.components.len() <= 512 && s.module_context.len() <= 512 && s.relative_depth <= 512,
            "language_resolution_component_limit"
        );
        let source = admission
            .packet
            .objects
            .iter()
            .find(|o| o.path == s.file)
            .and_then(|o| o.content.as_deref())
            .ok_or_else(|| anyhow::anyhow!("language_resolution_spec_source_missing"))?;
        s.span.validate(source)?;
        for value in s
            .components
            .iter()
            .chain(&s.module_context)
            .chain(s.alias.iter())
            .chain(s.specifier.iter())
            .chain(s.imported.iter())
        {
            ensure!(
                value.len() <= 4096 && !value.contains('\0'),
                "language_resolution_component_invalid"
            );
            spec_bytes = spec_bytes
                .checked_add(value.len())
                .ok_or_else(|| anyhow::anyhow!("language_resolution_spec_limit"))?;
        }
        ensure!(
            spec_bytes <= policy.limits.max_output_bytes,
            "language_resolution_spec_limit"
        );
        if !identity(s.form) {
            ensure!(
                syntax
                    .files
                    .iter()
                    .find(|f| f.coverage.path == s.file)
                    .is_some_and(|f| f.facts.imports.iter().any(|i| i.span == s.span)),
                "language_resolution_import_span_missing"
            );
        }
    }
    let mut index = ModuleIndex::default();
    let mut rust_files: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (path, lang) in &admitted {
        let r = root(policy, path, *lang)
            .ok_or_else(|| anyhow::anyhow!("language_resolution_root_missing"))?;
        let rel = relative(path, r);
        if *lang == Language::Python {
            if let Some(parts) = python_module(rel) {
                index.add(*lang, r, parts, path);
            }
        } else if *lang == Language::Rust && matches!(rel, "lib.rs" | "main.rs") {
            index.add(*lang, r, vec![], path);
            rust_files.insert(path.clone(), vec![]);
        }
    }
    for s in specs
        .iter()
        .filter(|s| s.form == ImportForm::JavaType && !s.unsupported)
    {
        if admitted.contains_key(&s.file) {
            let r = root(policy, &s.file, Language::Java).unwrap();
            index.add(Language::Java, r, s.components.clone(), &s.file);
        }
    }
    // Each pass must discover an admitted Rust file/module. The input and pass
    // count are bounded; conflicting candidates remain in the index.
    for _ in 0..=admitted.len().min(512) {
        let before = index.entries.len();
        for s in specs.iter().filter(|s| {
            matches!(
                s.form,
                ImportForm::RustOutOfLineModule | ImportForm::RustInlineModule
            ) && !s.unsupported
        }) {
            let Some(owner_module) = rust_files.get(&s.file).cloned() else {
                continue;
            };
            let Some(name) = s.components.last() else {
                continue;
            };
            let r = root(policy, &s.file, Language::Rust).unwrap();
            if !rust_context(&index, r, &owner_module, &s.module_context, &s.file) {
                continue;
            }
            let mut module = owner_module;
            module.extend(s.module_context.clone());
            module.push(name.clone());
            if s.form == ImportForm::RustInlineModule {
                index.add(Language::Rust, r, module, &s.file);
            } else {
                for path in module_file_candidates(r, &s.file, &s.module_context, name) {
                    if admitted.get(&path) == Some(&Language::Rust)
                        && root(policy, &path, Language::Rust) == Some(r)
                    {
                        index.add(Language::Rust, r, module.clone(), &path);
                        rust_files.entry(path).or_insert_with(|| module.clone());
                    }
                }
            }
        }
        if before == index.entries.len() {
            break;
        }
    }
    let mut report = syntax.clone();
    for file in &mut report.files {
        file.coverage.semantics = Coverage::Unsupported;
        file.coverage
            .diagnostics
            .retain(|d| d.code != "project_resolution_not_performed");
        for import in &mut file.facts.imports {
            import.target_path = None;
        }
        let file_specs: Vec<_> = specs
            .iter()
            .filter(|s| s.file == file.coverage.path && !identity(s.form))
            .collect();
        for import in &mut file.facts.imports {
            let matches: Vec<_> = file_specs
                .iter()
                .filter(|s| s.span == import.span)
                .collect();
            let mut targets = BTreeSet::new();
            let mut unresolved = matches.is_empty();
            for s in matches {
                let result = lookup(s, policy, &admitted, &index, &rust_files);
                unresolved |= result.is_empty() || s.unsupported || s.is_glob;
                targets.extend(result);
            }
            let code = if targets.len() > 1 {
                "module_resolution_ambiguous"
            } else if unresolved {
                "module_resolution_unresolved"
            } else {
                "module_resolution_admitted_only"
            };
            if !unresolved && targets.len() == 1 {
                import.target_path = targets.into_iter().next();
            }
            file.coverage.diagnostics.push(Diagnostic {
                code: code.into(),
                span: Some(import.span),
            });
        }
        // Module-edge resolution does not establish complete structural or
        // semantic coverage; preserve all existing parser limitations.
        if file.coverage.syntax == Coverage::Complete {
            file.coverage.structure = Coverage::Partial;
        }
        file.coverage.diagnostics.push(Diagnostic {
            code: "admitted_module_resolution_only".into(),
            span: None,
        });
    }
    report.digest = report.expected_digest()?;

    report.validate(admission, policy, now)?;

    Ok(report)
}

fn lookup(
    s: &ImportSpec,
    policy: &AnalysisPolicy,
    admitted: &BTreeMap<String, Language>,
    index: &ModuleIndex,
    rust_files: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let lang = language(s.form);
    let Some(r) = root(policy, &s.file, lang) else {
        return BTreeSet::new();
    };
    if s.unsupported || s.is_glob || !admitted.contains_key(&s.file) {
        return BTreeSet::new();
    }
    let mut parts = s.components.clone();
    match s.form {
        ImportForm::JavaImport => index.get(lang, r, &parts),
        ImportForm::JavaStaticImport => {
            parts.pop();
            index.get(lang, r, &parts)
        }
        ImportForm::PythonImport | ImportForm::PythonFrom => {
            if s.relative_depth > 0 {
                let Some(mut package) = python_module(relative(&s.file, r)) else {
                    return BTreeSet::new();
                };
                if !s.file.ends_with("/__init__.py")
                    && !s.file.ends_with("/__init__.pyi")
                    && s.file != "__init__.py"
                    && s.file != "__init__.pyi"
                {
                    package.pop();
                }
                if s.relative_depth > package.len() {
                    return BTreeSet::new();
                }
                package.truncate(package.len() - (s.relative_depth - 1));
                package.extend(parts);
                parts = package;
            }
            if s.form == ImportForm::PythonFrom {
                let Some(member) = &s.imported else {
                    return BTreeSet::new();
                };
                parts.push(member.clone());
            }
            // Missing __init__ is an unsupported namespace package, not an
            // assumed package. Duplicate .py/.pyi remains ambiguous.
            for n in 1..parts.len() {
                let parent = joined(r, &format!("{}/__init__.py", parts[..n].join("/")));
                let stub = format!("{}i", parent);
                if admitted.contains_key(&parent) == admitted.contains_key(&stub) {
                    return BTreeSet::new();
                }
            }
            index.get(lang, r, &parts)
        }
        ImportForm::RustUse | ImportForm::RustOutOfLineModule => {
            if index.get(Language::Rust, r, &[]).len() != 1 {
                return BTreeSet::new();
            }
            let Some(base) = rust_files.get(&s.file) else {
                return BTreeSet::new();
            };
            if !rust_context(index, r, base, &s.module_context, &s.file) {
                return BTreeSet::new();
            }
            let mut module = base.clone();
            module.extend(s.module_context.clone());
            match parts.first().map(String::as_str) {
                Some("crate") => {
                    module.clear();
                    parts.remove(0);
                }
                Some("self") => {
                    parts.remove(0);
                }
                Some("super") => {
                    while parts.first().is_some_and(|p| p == "super") {
                        if module.pop().is_none() {
                            return BTreeSet::new();
                        }
                        parts.remove(0);
                    }
                }
                _ => {}
            }
            module.extend(parts);
            if (0..module.len()).any(|n| index.get(lang, r, &module[..n]).len() != 1) {
                return BTreeSet::new();
            }
            index.get(lang, r, &module)
        }
        ImportForm::JavaScriptModule => {
            let Some(specifier) = &s.specifier else {
                return BTreeSet::new();
            };
            if !(specifier.starts_with("./") || specifier.starts_with("../"))
                || specifier.contains(['\\', '?', '#', '\0'])
            {
                return BTreeSet::new();
            }
            let rel = relative(&s.file, r);
            let mut segments: Vec<&str> = rel.split('/').collect();
            segments.pop();
            for segment in specifier.split('/') {
                match segment {
                    "." => {}
                    ".." => {
                        if segments.pop().is_none() {
                            return BTreeSet::new();
                        }
                    }
                    "" => return BTreeSet::new(),
                    other => segments.push(other),
                }
            }
            let base = joined(r, &segments.join("/"));
            // Loader-specific extension and directory-index inference requires
            // an explicit loader policy. Only exact source paths are supported.
            if ![".js", ".mjs", ".cjs"]
                .iter()
                .any(|ext| base.ends_with(ext))
            {
                return BTreeSet::new();
            }
            [base]
                .into_iter()
                .filter(|p| admitted.get(p) == Some(&lang) && root(policy, p, lang) == Some(r))
                .collect()
        }
        _ => BTreeSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn policy(files: &[(&str, Language)]) -> AnalysisPolicy {
        AnalysisPolicy {
            schema: super::super::VERSION.into(),
            files: files.iter().map(|(p, l)| (p.to_string(), *l)).collect(),
            roots: [
                Language::Rust,
                Language::Java,
                Language::Python,
                Language::JavaScript,
            ]
            .into_iter()
            .map(|language| super::super::ProjectRoot {
                language,
                root: ".".into(),
                manifest: None,
            })
            .collect(),
            layers: BTreeMap::new(),
            allowed: BTreeSet::new(),
            limits: super::super::Limits {
                max_nodes: 1000,
                max_depth: 32,
                max_facts: 1000,
                max_output_bytes: 100000,
            },
        }
    }
    fn spec(file: &str, form: ImportForm, components: &[&str]) -> ImportSpec {
        ImportSpec {
            file: file.into(),
            span: Span {
                start_byte: 0,
                end_byte: 0,
            },
            form,
            components: components.iter().map(|v| v.to_string()).collect(),
            relative_depth: 0,
            alias: None,
            is_glob: false,
            module_context: vec![],
            unsupported: false,
            specifier: None,
            imported: None,
        }
    }
    #[test]
    fn javascript_admitted_relative_paths_never_pick_ambiguous_or_external() {
        let p = policy(&[
            ("main.js", Language::JavaScript),
            ("dep.js", Language::JavaScript),
            ("dep/index.js", Language::JavaScript),
            ("other.py", Language::Python),
        ]);
        let mut s = spec("main.js", ImportForm::JavaScriptModule, &[]);
        let index = ModuleIndex::default();
        let rust = BTreeMap::new();
        s.specifier = Some("./dep".into());
        assert!(lookup(&s, &p, &p.files, &index, &rust).is_empty());
        s.specifier = Some("./dep.js".into());
        assert_eq!(
            lookup(&s, &p, &p.files, &index, &rust),
            BTreeSet::from(["dep.js".into()])
        );
        for bad in [
            "../dep.js",
            "dep.js",
            "./other.py",
            "./dep.js?raw",
            "./missing.js",
        ] {
            s.specifier = Some(bad.into());
            assert!(lookup(&s, &p, &p.files, &index, &rust).is_empty());
        }
        let mut nested = p.clone();
        nested.roots.push(super::super::ProjectRoot {
            language: Language::JavaScript,
            root: "dep".into(),
            manifest: None,
        });
        s.specifier = Some("./dep/index.js".into());
        assert!(lookup(&s, &nested, &nested.files, &index, &rust).is_empty());
    }
    #[test]
    fn python_packages_relative_members_and_stub_collisions_stay_explicit() {
        let p = policy(&[
            ("pkg/__init__.py", Language::Python),
            ("pkg/a.py", Language::Python),
            ("pkg/b.py", Language::Python),
            ("pkg/b.pyi", Language::Python),
        ]);
        let mut index = ModuleIndex::default();
        for path in p.files.keys() {
            index.add(Language::Python, ".", python_module(path).unwrap(), path);
        }
        let mut s = spec("pkg/a.py", ImportForm::PythonFrom, &[]);
        s.relative_depth = 1;
        s.imported = Some("b".into());
        assert_eq!(lookup(&s, &p, &p.files, &index, &BTreeMap::new()).len(), 2);
        s.imported = Some("symbol".into());
        assert!(lookup(&s, &p, &p.files, &index, &BTreeMap::new()).is_empty());
        s.relative_depth = 2;
        s.imported = Some("b".into());
        assert!(lookup(&s, &p, &p.files, &index, &BTreeMap::new()).is_empty());
        let mut collision = p.files.clone();
        collision.insert("pkg/__init__.pyi".into(), Language::Python);
        s.relative_depth = 1;
        assert!(lookup(&s, &p, &collision, &index, &BTreeMap::new()).is_empty());
        let mut absent = p.files.clone();
        absent.remove("pkg/__init__.py");
        s.relative_depth = 1;
        assert!(lookup(&s, &p, &absent, &index, &BTreeMap::new()).is_empty());
    }
    #[test]
    fn java_type_ambiguity_static_members_and_globs() {
        let p = policy(&[
            ("Main.java", Language::Java),
            ("A.java", Language::Java),
            ("duplicate.java", Language::Java),
        ]);
        let mut index = ModuleIndex::default();
        index.add(
            Language::Java,
            ".",
            vec!["pkg".into(), "A".into()],
            "A.java",
        );
        let mut s = spec(
            "Main.java",
            ImportForm::JavaStaticImport,
            &["pkg", "A", "member"],
        );
        assert_eq!(
            lookup(&s, &p, &p.files, &index, &BTreeMap::new()),
            BTreeSet::from(["A.java".into()])
        );
        s.is_glob = true;
        assert!(lookup(&s, &p, &p.files, &index, &BTreeMap::new()).is_empty());
        s.is_glob = false;
        index.add(
            Language::Java,
            ".",
            vec!["pkg".into(), "A".into()],
            "duplicate.java",
        );
        assert_eq!(lookup(&s, &p, &p.files, &index, &BTreeMap::new()).len(), 2);
    }
    #[test]
    fn rust_layout_context_escape_and_root_ambiguity() {
        assert_eq!(
            module_file_candidates("src", "src/foo.rs", &["inner".into()], "child"),
            vec!["src/foo/inner/child.rs", "src/foo/inner/child/mod.rs"]
        );
        let p = policy(&[
            ("lib.rs", Language::Rust),
            ("foo.rs", Language::Rust),
            ("main.rs", Language::Rust),
        ]);
        let mut index = ModuleIndex::default();
        index.add(Language::Rust, ".", vec![], "lib.rs");
        index.add(Language::Rust, ".", vec!["foo".into()], "foo.rs");
        let files = BTreeMap::from([
            ("lib.rs".into(), vec![]),
            ("foo.rs".into(), vec!["foo".into()]),
        ]);
        let mut s = spec("lib.rs", ImportForm::RustUse, &["crate", "foo"]);
        s.alias = Some("renamed".into());
        assert_eq!(
            lookup(&s, &p, &p.files, &index, &files),
            BTreeSet::from(["foo.rs".into()])
        );
        s.components = vec!["super".into(), "foo".into()];
        assert!(lookup(&s, &p, &p.files, &index, &files).is_empty());
        s.components = vec!["crate".into(), "foo".into()];
        s.unsupported = true;
        assert!(lookup(&s, &p, &p.files, &index, &files).is_empty());
        s.unsupported = false;
        index.add(Language::Rust, ".", vec![], "main.rs");
        assert!(lookup(&s, &p, &p.files, &index, &files).is_empty());
    }
    #[test]
    fn rust_excluded_or_ambiguous_ancestors_never_authorize_children() {
        let mut index = ModuleIndex::default();
        index.add(Language::Rust, ".", vec![], "lib.rs");
        let context = vec!["conditional".into()];
        assert!(!rust_context(&index, ".", &[], &context, "lib.rs"));
        index.add(Language::Rust, ".", context.clone(), "lib.rs");
        assert!(rust_context(&index, ".", &[], &context, "lib.rs"));
        index.add(Language::Rust, ".", context.clone(), "alternative.rs");
        assert!(!rust_context(&index, ".", &[], &context, "lib.rs"));
        index.add(
            Language::Rust,
            ".",
            vec!["conditional".into(), "child".into()],
            "child.rs",
        );
        let p = policy(&[("lib.rs", Language::Rust), ("child.rs", Language::Rust)]);
        let s = spec(
            "lib.rs",
            ImportForm::RustUse,
            &["crate", "conditional", "child"],
        );
        let files = BTreeMap::from([("lib.rs".into(), vec![])]);
        assert!(lookup(&s, &p, &p.files, &index, &files).is_empty());
    }
}
