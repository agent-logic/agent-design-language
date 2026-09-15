//! Deterministic syntactic graph. No compilation, macro expansion, or repository execution.
use super::syntax::References;
use crate::codefriend::evidence::{
    contracts::{Completion, Confidence, Finding, ReviewRecord, Run, Severity, CONTRACT},
    hash,
    store::Store,
    Admission,
};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use syn::visit::Visit;

pub const VERSION: &str = "codefriend.structure.v1";
const LIMIT: usize = 512;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoundaryPolicy {
    pub schema: String,
    /// Explicit single-crate Rust root, such as src/lib.rs. No path inference across crates.
    pub crate_root: String,
    /// Optional admitted Cargo manifest. Declared dependencies are facts, not resolved external code.
    pub manifest_path: Option<String>,
    /// Every analyzed Rust file must have an explicit layer assignment.
    pub layers: BTreeMap<String, String>,
    /// Cross-layer dependencies not listed here are forbidden; same-layer edges are allowed.
    pub allowed: BTreeSet<(String, String)>,
    pub coupling_threshold: usize,
}
impl BoundaryPolicy {
    pub fn validate(&self, a: &Admission) -> Result<()> {
        ensure!(
            self.schema == VERSION,
            "unsupported_boundary_policy_version"
        );
        ensure!(
            self.layers.len() <= LIMIT && (1..=LIMIT).contains(&self.coupling_threshold),
            "invalid_graph_policy_bounds"
        );
        ensure!(
            self.crate_root.ends_with(".rs") && a.packet.scope.analysis.contains(&self.crate_root),
            "crate_root_outside_analysis_scope"
        );
        if let Some(path) = &self.manifest_path {
            ensure!(
                path.ends_with("Cargo.toml") && a.packet.scope.paths().contains(&path.as_str()),
                "manifest_outside_scope"
            );
        }
        for (path, layer) in &self.layers {
            ensure!(
                a.packet.scope.analysis.contains(path) && path.ends_with(".rs"),
                "boundary_path_outside_rust_scope"
            );
            ensure!(
                !layer.is_empty()
                    && layer.len() <= 64
                    && layer
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
                "invalid_layer_identifier"
            );
        }
        let layers: BTreeSet<_> = self.layers.values().collect();
        ensure!(
            self.allowed.len() <= LIMIT
                && self
                    .allowed
                    .iter()
                    .all(|(f, t)| layers.contains(f) && layers.contains(t)),
            "unknown_allowed_layer"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub path: String,
    pub line: usize,
    pub evidence_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    pub module: String,
    pub kind: String,
    pub layer: Option<String>,
    pub location: Location,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub reference: String,
    pub kind: String,
    pub location: Location,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Unknown {
    pub location: Location,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StructureReport {
    pub schema: String,
    pub policy: BoundaryPolicy,
    pub policy_digest: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub unknowns: Vec<Unknown>,
    pub analysis_complete: bool,
    pub record: ReviewRecord,
    pub digest: String,
}
fn location(a: &Admission, path: &str, line: usize) -> Location {
    Location {
        path: path.into(),
        line,
        evidence_id: a
            .evidence
            .iter()
            .find(|e| e.path == path)
            .map(|e| e.id.clone())
            .unwrap_or_default(),
    }
}
fn unknown(out: &mut Vec<Unknown>, a: &Admission, path: &str, line: usize, reason: &str) {
    out.push(Unknown {
        location: location(a, path, line),
        reason: reason.into(),
    });
}
fn node_id(repository: &str, module: &str) -> Result<String> {
    hash(&(VERSION, repository, module))
}
fn add_finding(
    record: &mut ReviewRecord,
    rule: &str,
    anchor: &str,
    title: &str,
    rationale: String,
    evidence: Vec<String>,
) -> Result<()> {
    let evidence: Vec<_> = evidence
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut f = Finding { schema: CONTRACT.into(), id: String::new(), repository: record.run.repository.clone(), perspective: "architecture".into(), rule: rule.into(), semantic_anchor: anchor.into(), title: title.into(), severity: Severity::Medium, rationale, confidence: Confidence::Unknown, evidence, inference: "Syntactic references are observed; architectural risk is inferred against the explicit policy, without type checking or runtime proof.".into(), scope_digest: record.run.scope_digest.clone(), limitations: vec!["Bounded single-crate module/use/path analysis; no semantic resolution, macro expansion, dynamic dispatch or runtime coupling proof.".into()] };
    f.id = f.identity()?;
    f.validate(&record.run, &record.admission)?;
    record.findings.push(f);
    Ok(())
}

/// Retrieve through the production retention/deletion boundary, not an arbitrary JSON admission.
pub fn repository_structure_reporter(
    store: &Store,
    packet_id: &str,
    policy: BoundaryPolicy,
) -> Result<StructureReport> {
    analyze(store.get(packet_id)?, policy)
}

fn analyze(a: Admission, policy: BoundaryPolicy) -> Result<StructureReport> {
    a.validate()?;
    policy.validate(&a)?;
    ensure!(
        a.packet.scope.analysis.len() <= LIMIT,
        "graph_node_limit_exceeded"
    );
    let mut unknowns = Vec::new();
    let mut modules = BTreeMap::<String, String>::new();
    modules.insert("crate".into(), policy.crate_root.clone());
    let mut parsed = BTreeMap::new();
    for path in &a.packet.scope.analysis {
        let object = a
            .packet
            .objects
            .iter()
            .find(|o| &o.path == path)
            .ok_or_else(|| anyhow::anyhow!("analysis_object_missing"))?;
        let Some(content) = &object.content else {
            unknown(&mut unknowns, &a, path, 1, "source_omitted_or_redacted");
            continue;
        };
        if path.ends_with(".rs") {
            match syn::parse_file(content) {
                Ok(file) => {
                    let mut refs = References::default();
                    refs.visit_file(&file);
                    parsed.insert(path.clone(), refs);
                }
                Err(_) => unknown(&mut unknowns, &a, path, 1, "rust_parse_failed"),
            }
        } else if path.ends_with("Cargo.toml") {
            let manifest = content.parse::<toml::Value>();
            if manifest.is_err() {
                unknown(&mut unknowns, &a, path, 1, "manifest_parse_failed");
            }
            // Manifests are retained evidence; dependency configuration needs separate crate resolution.
            else {
                unknown(
                    &mut unknowns,
                    &a,
                    path,
                    1,
                    "manifest_dependency_resolution_outside_single_crate_slice",
                );
            }
        } else {
            unknown(&mut unknowns, &a, path, 1, "unsupported_analysis_language");
        }
    }
    // Discover only declared out-of-line modules. Arbitrary filename inference can invent topology.
    let mut visited = BTreeSet::new();
    loop {
        let next = modules
            .iter()
            .find(|(m, _)| !visited.contains(*m))
            .map(|(m, p)| (m.clone(), p.clone()));
        let Some((module, path)) = next else {
            break;
        };
        ensure!(modules.len() <= LIMIT, "graph_node_limit_exceeded");
        visited.insert(module.clone());
        let Some(refs) = parsed.get(&path) else {
            continue;
        };
        let file = std::path::Path::new(&path);
        let parent = file.parent().unwrap_or_else(|| std::path::Path::new(""));
        let dir = if path == policy.crate_root || file.file_name().is_some_and(|s| s == "mod.rs") {
            parent.to_path_buf()
        } else {
            parent.join(file.file_stem().unwrap_or_default())
        };
        for (name, line) in &refs.modules {
            let candidates = [
                dir.join(format!("{name}.rs")),
                dir.join(name).join("mod.rs"),
            ];
            let found: Vec<_> = candidates
                .iter()
                .filter_map(|p| p.to_str())
                .filter(|p| parsed.contains_key(*p))
                .collect();
            if found.len() != 1 {
                unknown(
                    &mut unknowns,
                    &a,
                    &path,
                    *line,
                    "missing_or_ambiguous_declared_module",
                );
                continue;
            }
            modules.insert(format!("{module}::{name}"), found[0].into());
        }
    }
    let mut nodes = Vec::new();
    for (module, path) in &modules {
        if !parsed.contains_key(path) {
            continue;
        }
        if !policy.layers.contains_key(path) {
            unknown(&mut unknowns, &a, path, 1, "missing_boundary_assignment");
        }
        nodes.push(Node {
            id: node_id(&a.packet.repository, module)?,
            module: module.clone(),
            kind: "rust_module".into(),
            layer: policy.layers.get(path).cloned(),
            location: location(&a, path, 1),
        });
    }
    for path in parsed.keys() {
        if !modules.values().any(|p| p == path) {
            unknown(
                &mut unknowns,
                &a,
                path,
                1,
                "unreachable_or_undeclared_module",
            );
        }
    }
    let mut edges = BTreeSet::new();
    for node in &nodes {
        let refs = &parsed[&node.location.path];
        for (line, reason) in &refs.unknowns {
            unknown(&mut unknowns, &a, &node.location.path, *line, reason);
        }
        for (parts, line) in &refs.paths {
            let mut normalized: Vec<String> = node.module.split("::").map(str::to_owned).collect();
            let mut rest = parts.as_slice();
            match rest.first().map(String::as_str) {
                Some("crate") => {
                    normalized = vec!["crate".into()];
                    rest = &rest[1..];
                }
                Some("self") => rest = &rest[1..],
                Some("super") => {
                    while rest.first().is_some_and(|s| s == "super") {
                        normalized.pop();
                        rest = &rest[1..];
                    }
                }
                _ => {
                    unknown(
                        &mut unknowns,
                        &a,
                        &node.location.path,
                        *line,
                        "unqualified_or_external_import_not_resolved",
                    );
                    continue;
                }
            }
            if normalized.is_empty() {
                unknown(
                    &mut unknowns,
                    &a,
                    &node.location.path,
                    *line,
                    "path_escapes_crate",
                );
                continue;
            }
            normalized.extend_from_slice(rest);
            let reference = normalized.join("::");
            let target = nodes
                .iter()
                .filter(|n| {
                    reference == n.module || reference.starts_with(&(n.module.clone() + "::"))
                })
                .max_by_key(|n| n.module.len());
            if let Some(target) = target.filter(|n| n.module != "crate" || normalized.len() == 1) {
                if target.id != node.id {
                    edges.insert(Edge {
                        from: node.id.clone(),
                        to: target.id.clone(),
                        reference,
                        kind: "syntactic_reference".into(),
                        location: location(&a, &node.location.path, *line),
                    });
                }
            } else {
                unknown(
                    &mut unknowns,
                    &a,
                    &node.location.path,
                    *line,
                    "reference_target_not_in_graph",
                );
            }
            ensure!(edges.len() <= 4096, "graph_edge_limit_exceeded");
        }
    }
    if let Some(path) = &policy.manifest_path {
        let content = a
            .packet
            .objects
            .iter()
            .find(|o| &o.path == path)
            .and_then(|o| o.content.as_deref());
        if let Some(content) = content {
            let manifest: toml::Value = content
                .parse()
                .map_err(|_| anyhow::anyhow!("manifest_parse_failed"))?;
            for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(table) = manifest.get(kind) {
                    let table = table
                        .as_table()
                        .ok_or_else(|| anyhow::anyhow!("invalid_manifest_dependency_table"))?;
                    for name in table.keys() {
                        ensure!(
                            !name.is_empty()
                                && name.len() <= 128
                                && name
                                    .bytes()
                                    .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b)),
                            "invalid_dependency_name"
                        );
                        let module = format!("external::{kind}::{name}");
                        let id = node_id(&a.packet.repository, &module)?;
                        // Manifest-table location is explicit; source text remains in its evidence object.
                        let line = content
                            .lines()
                            .position(|l| l.trim() == format!("[{kind}]"))
                            .map_or(1, |n| n + 1);
                        let loc = location(&a, path, line);
                        nodes.push(Node {
                            id: id.clone(),
                            module: module.clone(),
                            kind: "declared_external_dependency".into(),
                            layer: None,
                            location: loc.clone(),
                        });
                        let root_id = node_id(&a.packet.repository, "crate")?;
                        if nodes.iter().any(|n| n.id == root_id) {
                            edges.insert(Edge {
                                from: root_id,
                                to: id,
                                reference: module,
                                kind: "manifest_declaration".into(),
                                location: loc,
                            });
                        }
                    }
                }
            }
            if manifest.get("target").is_some() || manifest.get("workspace").is_some() {
                unknown(
                    &mut unknowns,
                    &a,
                    path,
                    1,
                    "conditional_or_workspace_dependencies_not_resolved",
                );
            }
        } else {
            unknown(&mut unknowns, &a, path, 1, "manifest_omitted_or_redacted");
        }
    }
    ensure!(
        nodes.len() <= LIMIT && edges.len() <= 4096,
        "graph_bounds_exceeded"
    );
    nodes.sort_by(|a, b| a.module.cmp(&b.module));
    unknowns.sort();
    unknowns.dedup();
    ensure!(unknowns.len() <= 4096, "graph_unknown_limit_exceeded");
    let complete = unknowns.is_empty() && a.packet.completeness == "complete_scoped_acquisition";
    let run = Run::new(
        &a,
        BTreeMap::from([
            ("architecture".into(), VERSION.into()),
            ("architecture-policy".into(), hash(&policy)?),
        ]),
        "none-local-syntax".into(),
        if complete {
            Completion::Complete
        } else {
            Completion::Incomplete
        },
        if complete {
            vec![]
        } else {
            vec!["partial_architecture_analysis".into()]
        },
    )?;
    let mut report = StructureReport {
        schema: VERSION.into(),
        policy_digest: hash(&policy)?,
        policy,
        nodes,
        edges: edges.into_iter().collect(),
        unknowns,
        analysis_complete: complete,
        record: ReviewRecord {
            admission: a,
            run,
            findings: vec![],
        },
        digest: String::new(),
    };
    findings(&mut report)?;
    report.record.findings.sort_by(|a, b| a.id.cmp(&b.id));
    report.record.validate()?;
    report.digest = hash(&report)?;
    Ok(report)
}

impl StructureReport {
    /// Recompute the deterministic product from the live admission and explicit policy.
    /// This rejects tampered topology, findings and deleted/expired source admissions.
    pub fn validate(&self, store: &Store) -> Result<()> {
        let expected = repository_structure_reporter(
            store,
            &self.record.admission.packet.packet_id,
            self.policy.clone(),
        )?;
        ensure!(*self == expected, "structure_artifact_mismatch");
        Ok(())
    }
}

pub fn write_report(report: &StructureReport, store: &Store, path: &std::path::Path) -> Result<()> {
    use std::io::Write;
    report.validate(store)?;
    safe_artifact_path(path)?;
    let bytes = serde_json::to_vec(report)?;
    ensure!(
        bytes.len() <= 16 * 1024 * 1024,
        "structure_artifact_too_large"
    );
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut file = opts
        .open(path)
        .map_err(|_| anyhow::anyhow!("structure_output_unavailable"))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}
pub fn read_report(store: &Store, path: &std::path::Path) -> Result<StructureReport> {
    use std::io::Read;
    safe_artifact_path(path)?;
    ensure!(
        std::fs::symlink_metadata(path)?.file_type().is_file(),
        "structure_input_not_regular"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 16 * 1024 * 1024,
        "structure_artifact_too_large"
    );
    let report: StructureReport = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow::anyhow!("invalid_structure_artifact"))?;
    report.validate(store)?;
    Ok(report)
}
fn safe_artifact_path(path: &std::path::Path) -> Result<()> {
    ensure!(
        !path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir)),
        "artifact_parent_traversal_rejected"
    );
    let mut current = std::path::PathBuf::new();
    for c in std::path::absolute(path)?.components() {
        current.push(c);
        if let Ok(m) = std::fs::symlink_metadata(&current) {
            ensure!(!m.file_type().is_symlink(), "artifact_symlink_rejected");
        }
    }
    Ok(())
}

fn findings(r: &mut StructureReport) -> Result<()> {
    let nodes: BTreeMap<_, _> = r.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut pairs = BTreeMap::<(String, String), Vec<String>>::new();
    for e in &r.edges {
        pairs
            .entry((e.from.clone(), e.to.clone()))
            .or_default()
            .push(e.location.evidence_id.clone());
    }
    for ((from, to), evidence) in &pairs {
        let f = nodes[from];
        let t = nodes[to];
        if let (Some(a), Some(b)) = (&f.layer, &t.layer) {
            if a != b && !r.policy.allowed.contains(&(a.clone(), b.clone())) {
                add_finding(&mut r.record,"forbidden_boundary",&format!("{}->{}",f.module,t.module),"Dependency crosses a forbidden boundary",format!("Observed reference from {} to {} crosses {a} to {b}; remove the dependency or review the declared boundary policy.",f.module,t.module),evidence.clone())?;
            }
        }
    }
    for node in &r.nodes {
        let targets: BTreeSet<_> = pairs
            .keys()
            .filter(|(f, _)| f == &node.id)
            .map(|(_, t)| t)
            .collect();
        if targets.len() > r.policy.coupling_threshold {
            let evidence = r
                .edges
                .iter()
                .filter(|e| e.from == node.id)
                .map(|e| e.location.evidence_id.clone())
                .collect();
            add_finding(&mut r.record,"module_fanout",&node.module,"Module exceeds declared coupling threshold",format!("{} depends on {} scoped modules, above threshold {}; consider narrowing this module's responsibilities. This counts syntactic module references, not runtime coupling.",node.module,targets.len(),r.policy.coupling_threshold),evidence)?;
        }
        let mut reachable = BTreeSet::new();
        let mut todo: Vec<_> = targets.into_iter().cloned().collect();
        while let Some(next) = todo.pop() {
            if reachable.insert(next.clone()) {
                todo.extend(
                    pairs
                        .keys()
                        .filter(|(f, _)| f == &next)
                        .map(|(_, t)| t.clone()),
                );
            }
        }
        if reachable.contains(&node.id) {
            let evidence = r
                .edges
                .iter()
                .filter(|e| reachable.contains(&e.from) && reachable.contains(&e.to))
                .map(|e| e.location.evidence_id.clone())
                .collect();
            add_finding(&mut r.record,"dependency_cycle",&node.module,"Module participates in a dependency cycle",format!("{} reaches itself through scoped directed references; inspect the retained edges and break or explicitly justify the cycle.",node.module),evidence)?;
        }
    }
    let mut refs = BTreeMap::<String, Vec<&Edge>>::new();
    for e in &r.edges {
        if e.reference != nodes[&e.to].module {
            refs.entry(e.reference.clone()).or_default().push(e);
        }
    }
    for (symbol, edges) in refs {
        if edges.iter().map(|e| &e.from).collect::<BTreeSet<_>>().len() > 1 {
            add_finding(&mut r.record,"name_connascence",&symbol,"Multiple modules depend on the same referenced name",format!("Multiple scoped modules spell {symbol}; a rename may require coordinated edits. This is inferred name connascence only, without type, value, execution-order or semantic identity resolution."),edges.iter().map(|e|e.location.evidence_id.clone()).collect())?;
        }
    }
    Ok(())
}
