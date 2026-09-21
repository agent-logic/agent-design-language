//! PVF: deterministic component contracts; not installed Journey qualification.
use adl::codefriend::{
    architecture::four_plus_one::*,
    evidence::{Admission, Retention},
    ingestion::{local, Scope},
};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
// Native proof scrubs ambient TMPDIR; fixture artifacts remain in this checkout.
fn fixture_tempdir() -> tempfile::TempDir {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/codefriend-fixtures");
    fs::create_dir_all(&root).unwrap();
    tempfile::tempdir_in(root.canonicalize().unwrap()).unwrap()
}

fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().into()
}
fn fixture() -> (Admission, Package) {
    let temp = fixture_tempdir();
    let root = temp.path();
    git(root, &["init", "-b", "main"]);
    git(
        root,
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    fs::write(
        root.join("architecture.md"),
        "The gateway forwards requests to the worker.\n",
    )
    .unwrap();
    fs::write(root.join("LICENSE"), "MIT fixture\n").unwrap();
    git(root, &["add", "."]);
    git(
        root,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ],
    );
    let packet = local::acquire(
        root,
        "https://example.com/owner/repo",
        &git(root, &["rev-parse", "HEAD"]),
        Scope {
            analysis: vec!["architecture.md".into()],
            context: vec!["LICENSE".into()],
            max_files: 2,
            max_bytes: 4096,
            max_file_bytes: 2048,
        },
    )
    .unwrap();
    let a = Admission::new(packet, Retention { seconds: 100 }, 100).unwrap();
    let c = Citation {
        evidence_id: a
            .evidence
            .iter()
            .find(|e| e.path == "architecture.md")
            .unwrap()
            .id
            .clone(),
        path: "architecture.md".into(),
        first_line: 1,
        last_line: 1,
        excerpt: "The gateway forwards requests to the worker.".into(),
    };
    let entities = ["gateway", "worker"]
        .into_iter()
        .map(|id| Entity {
            id: id.into(),
            name: id.into(),
            responsibility: "Fixture declaration".into(),
            basis: Basis::SourceDeclaration,
            citations: vec![c.clone()],
        })
        .collect();
    let views = View::ALL
        .into_iter()
        .map(|v| {
            (
                v,
                ArchitectureView {
                    entities: vec!["gateway".into(), "worker".into()],
                    relationships: vec![Relationship {
                        from: "gateway".into(),
                        to: "worker".into(),
                        description: "forwards".into(),
                        basis: Basis::SourceDeclaration,
                        citations: vec![c.clone()],
                    }],
                    missing_inputs: vec![
                        "Fixture only: view-specific semantics not established".into()
                    ],
                },
            )
        })
        .collect();
    let mut p = Package {
        schema: SCHEMA.into(),
        repository: a.packet.repository.clone(),
        revision: a.packet.revision.clone(),
        packet_id: a.packet.packet_id.clone(),
        admission_digest: a.digest.clone(),
        entities,
        views,
        scenarios: vec![Scenario {
            id: "request".into(),
            description: "Request forwarding".into(),
            failure_recovery: false,
            basis: Basis::SourceDeclaration,
            citations: vec![c],
            trace: View::ALL
                .into_iter()
                .map(|v| (v, vec!["gateway".into(), "worker".into()]))
                .collect::<BTreeMap<_, _>>(),
        }],
        conflicts: vec![],
        missing_inputs: vec![],
        complete: false,
        digest: String::new(),
    };
    p.digest = p.expected_digest().unwrap();
    (a, p)
}
fn seal(p: &mut Package) {
    p.digest = p.expected_digest().unwrap();
}
#[test]
fn four_plus_one_accepts_honest_partial_and_rejects_false_complete() {
    let (a, mut p) = fixture();
    p.validate(&a, 101).unwrap();
    p.complete = true;
    seal(&mut p);
    assert!(p
        .validate(&a, 101)
        .unwrap_err()
        .to_string()
        .contains("false_completeness"));
}
#[test]
fn four_plus_one_rejects_forged_source_and_revision() {
    let (a, mut p) = fixture();
    p.entities[0].citations[0].excerpt = "Invented topology".into();
    seal(&mut p);
    assert!(p
        .validate(&a, 101)
        .unwrap_err()
        .to_string()
        .contains("excerpt_mismatch"));
    let (_, mut p) = fixture();
    p.revision = "0".repeat(40);
    seal(&mut p);
    assert!(p.validate(&a, 101).is_err());
}
#[test]
fn four_plus_one_rejects_dangling_trace_and_expired_admission() {
    let (a, mut p) = fixture();
    assert!(p
        .validate(&a, 200)
        .unwrap_err()
        .to_string()
        .contains("expired"));
    p.scenarios[0]
        .trace
        .insert(View::Deployment, vec!["invented".into()]);
    seal(&mut p);
    assert!(p
        .validate(&a, 101)
        .unwrap_err()
        .to_string()
        .contains("unknown_trace"));
}
#[test]
fn four_plus_one_preserves_evidenced_conflicts_as_partial() {
    let (a, mut p) = fixture();
    p.conflicts.push(Conflict {
        description: "Fixture conflicting interpretation".into(),
        entities: vec!["gateway".into()],
        citations: p.entities[0].citations.clone(),
    });
    seal(&mut p);
    p.validate(&a, 101).unwrap();
    assert!(!p.coverage_complete());
}

fn generation_fixture() -> (
    tempfile::TempDir,
    adl::codefriend::evidence::store::Store,
    adl::codefriend::architecture::artifact::StructureArtifact,
) {
    generation_fixture_at(100)
}
fn generation_fixture_at(
    now: u64,
) -> (
    tempfile::TempDir,
    adl::codefriend::evidence::store::Store,
    adl::codefriend::architecture::artifact::StructureArtifact,
) {
    use adl::codefriend::architecture::{artifact::StructureArtifact, structure};
    use adl::codefriend::evidence::store::Store;
    let temp = fixture_tempdir();
    let root = temp.path();
    git(root, &["init", "-b", "main"]);
    git(
        root,
        &[
            "remote",
            "add",
            "origin",
            "https://example.com/owner/architecture-fixture",
        ],
    );
    fs::write(
        root.join("lib.rs"),
        "pub mod worker;\nuse crate::worker::execute;\n",
    )
    .unwrap();
    fs::write(root.join("worker.rs"), "pub fn execute() {}\n").unwrap();
    fs::write(root.join("LICENSE"), "MIT fixture\n").unwrap();
    fs::write(
        root.join("architecture.md"),
        concat!(
            "Gateway accepts job requests; Worker executes jobs.\n",
            "Gateway is implemented in lib.rs, which declares Worker in worker.rs.\n",
            "Gateway sends asynchronous jobs to Worker; Worker replies with a result.\n",
            "Deployment declares Gateway on the edge node and Worker on the compute node.\n",
            "A successful request enters Gateway, runs in Worker and returns a result.\n",
            "On Worker failure, Gateway returns an error; the caller may submit a new job.\n"
        ),
    )
    .unwrap();
    git(root, &["add", "."]);
    git(
        root,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ],
    );
    let packet = local::acquire(
        root,
        "https://example.com/owner/architecture-fixture",
        &git(root, &["rev-parse", "HEAD"]),
        Scope {
            analysis: vec!["lib.rs".into(), "worker.rs".into()],
            context: vec!["LICENSE".into(), "architecture.md".into()],
            max_files: 4,
            max_bytes: 8192,
            max_file_bytes: 4096,
        },
    )
    .unwrap();
    let store = Store::open(&root.join("store"), move || now).unwrap();
    let a = store.admit(packet, Retention { seconds: 3600 }).unwrap();
    let graph = structure::repository_structure_reporter(
        &store,
        &a.packet.packet_id,
        structure::BoundaryPolicy {
            schema: structure::VERSION.into(),
            crate_root: "lib.rs".into(),
            manifest_path: None,
            layers: BTreeMap::from([
                ("lib.rs".into(), "application".into()),
                ("worker.rs".into(), "application".into()),
            ]),
            allowed: Default::default(),
            coupling_threshold: 10,
        },
    )
    .unwrap();
    (temp, store, StructureArtifact::V1(graph))
}
fn full_draft(a: &Admission) -> adl::codefriend::architecture::four_plus_one::generation::Draft {
    use adl::codefriend::architecture::four_plus_one::generation::Draft;
    let object = a
        .packet
        .objects
        .iter()
        .find(|o| o.path == "architecture.md")
        .unwrap();
    let lines: Vec<_> = object.content.as_ref().unwrap().lines().collect();
    let cite = |line: usize| Citation {
        evidence_id: a
            .evidence
            .iter()
            .find(|e| e.path == "architecture.md")
            .unwrap()
            .id
            .clone(),
        path: "architecture.md".into(),
        first_line: line,
        last_line: line,
        excerpt: lines[line - 1].into(),
    };
    let entities = [
        ("gateway", "Gateway", "Accepts requests", 1),
        ("worker", "Worker", "Executes jobs", 1),
        ("edge", "edge node", "Hosts Gateway", 4),
        ("compute", "compute node", "Hosts Worker", 4),
    ]
    .into_iter()
    .map(|(id, name, responsibility, line)| Entity {
        id: id.into(),
        name: name.into(),
        responsibility: responsibility.into(),
        basis: Basis::SourceDeclaration,
        citations: vec![cite(line)],
    })
    .collect();
    let mut views = BTreeMap::new();
    for (v, line, description) in [
        (View::Logical, 1, "delegates job execution"),
        (View::Development, 2, "declares module"),
        (View::Process, 3, "sends asynchronous job"),
    ] {
        views.insert(
            v,
            ArchitectureView {
                entities: vec!["gateway".into(), "worker".into()],
                relationships: vec![Relationship {
                    from: "gateway".into(),
                    to: "worker".into(),
                    description: description.into(),
                    basis: Basis::SourceDeclaration,
                    citations: vec![cite(line)],
                }],
                missing_inputs: vec![],
            },
        );
    }
    views.insert(
        View::Deployment,
        ArchitectureView {
            entities: vec![
                "gateway".into(),
                "worker".into(),
                "edge".into(),
                "compute".into(),
            ],
            relationships: [("gateway", "edge"), ("worker", "compute")]
                .into_iter()
                .map(|(from, to)| Relationship {
                    from: from.into(),
                    to: to.into(),
                    description: "declared placement, not observed topology".into(),
                    basis: Basis::SourceDeclaration,
                    citations: vec![cite(4)],
                })
                .collect(),
            missing_inputs: vec![],
        },
    );
    let scenarios = [("success", 5, false), ("failure", 6, true)]
        .into_iter()
        .map(|(id, line, failure_recovery)| Scenario {
            id: id.into(),
            description: lines[line - 1].into(),
            failure_recovery,
            basis: Basis::SourceDeclaration,
            citations: vec![cite(line)],
            trace: View::ALL
                .into_iter()
                .map(|v| (v, vec!["gateway".into(), "worker".into()]))
                .collect(),
        })
        .collect();
    Draft {
        entities,
        views,
        scenarios,
        conflicts: vec![],
        missing_inputs: vec![],
    }
}
#[test]
fn four_plus_one_generation_populates_all_views_and_failure_scenario() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let response = serde_json::to_string(&full_draft(&a)).unwrap();
    let g = generation::generate(
        &store,
        &graph,
        || 101,
        |prompt| {
            assert!(prompt.contains("Deployment declares Gateway"));
            assert!(prompt.contains("No identity, digest, approval, or completeness fields"));
            Ok(response.clone())
        },
    )
    .unwrap();
    assert!(g.package.complete);
    assert_eq!(g.package.views.len(), 4);
    assert_eq!(g.package.scenarios.len(), 2);
    assert!(g.package.scenarios.iter().any(|s| s.failure_recovery));
    g.package.validate(&a, 101).unwrap();
    g.validate(&store, &graph, &response, 101).unwrap();
    let mut altered = g.clone();
    altered.package.entities[0].name = "tampered".into();
    altered.package.digest = altered.package.expected_digest().unwrap();
    assert!(altered.validate(&store, &graph, &response, 101).is_err());
}
#[test]
fn four_plus_one_generation_denies_deleted_source_after_execution() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let response = serde_json::to_string(&full_draft(&a)).unwrap();
    let result = generation::generate(
        &store,
        &graph,
        || 101,
        |_| {
            store.delete(&a.packet.packet_id)?;
            Ok(response)
        },
    );
    assert!(result.unwrap_err().to_string().contains("deleted"));
}
#[test]
fn four_plus_one_generation_rejects_model_identity_and_duplicate_keys() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let mut response = serde_json::to_value(full_draft(&a)).unwrap();
    response["complete"] = true.into();
    assert!(generation::accept_response(&a, &graph, &response.to_string(), 101).is_err());
    assert!(
        generation::accept_response(&a, &graph, "{\"entities\":[],\"entities\":[]}", 101).is_err()
    );
}

#[test]
fn four_plus_one_render_preserves_sources_crosslinks_and_escapes_labels() {
    use adl::codefriend::architecture::four_plus_one::{generation, render};
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let mut draft = full_draft(&a);
    draft.entities[0].name = "Gateway <tag> & \"label\"".into();
    let g = generation::accept_response(&a, &graph, &serde_json::to_string(&draft).unwrap(), 101)
        .unwrap();
    let files = render::artifacts(&g.package, &a, 101).unwrap();
    let md = String::from_utf8(files["architecture.md"].clone()).unwrap();
    for name in ["logical", "development", "process", "deployment"] {
        assert!(md.contains(&format!("## {name}")));
        assert!(files.contains_key(&format!("{name}.mmd")));
        assert!(files.contains_key(&format!("{name}-000.svg")));
    }
    assert!(md.contains("[failure](#scenario-failure)"));
    assert!(md.contains("### scenario-failure"));
    assert!(md.contains(
        &a.evidence
            .iter()
            .find(|e| e.path == "architecture.md")
            .unwrap()
            .id
    ));
    let svg = String::from_utf8(files["logical-000.svg"].clone()).unwrap();
    assert!(svg.contains("&lt;tag&gt; &amp; &quot;label&quot;"));
    assert!(!svg.contains("<tag>"));
    assert_eq!(files, render::artifacts(&g.package, &a, 101).unwrap());
}
#[test]
fn four_plus_one_artifacts_are_create_only_and_private() {
    use adl::codefriend::architecture::four_plus_one::{generation, render};
    use std::os::unix::fs::PermissionsExt;
    let (temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let g = generation::accept_response(
        &a,
        &graph,
        &serde_json::to_string(&full_draft(&a)).unwrap(),
        101,
    )
    .unwrap();
    let output = temp.path().join("rendered");
    render::write_artifacts(&g.package, &a, 101, &output).unwrap();
    assert_eq!(
        fs::metadata(&output).unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(output.join("architecture.md"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    assert!(render::write_artifacts(&g.package, &a, 101, &output).is_err());
    assert_eq!(
        fs::read(output.join("package.json")).unwrap(),
        serde_json::to_vec_pretty(&g.package).unwrap()
    );
}

#[test]
fn four_plus_one_empty_views_and_traces_require_actionable_gaps() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let mut draft = full_draft(&a);
    draft
        .views
        .get_mut(&View::Deployment)
        .unwrap()
        .relationships
        .clear();
    draft.scenarios[0].trace.remove(&View::Process);
    let g = generation::accept_response(&a, &graph, &serde_json::to_string(&draft).unwrap(), 101)
        .unwrap();
    assert!(!g.package.complete);
    assert!(!g.package.views[&View::Deployment].missing_inputs.is_empty());
    assert!(g
        .package
        .missing_inputs
        .iter()
        .any(|gap| gap.contains("Process") && gap.contains(&draft.scenarios[0].id)));
    let mut forged = g.package;
    forged
        .views
        .get_mut(&View::Deployment)
        .unwrap()
        .missing_inputs
        .clear();
    forged.digest = forged.expected_digest().unwrap();
    assert!(forged
        .validate(&a, 101)
        .unwrap_err()
        .to_string()
        .contains("missing_view_explanation"));
}

#[test]
fn four_plus_one_render_retains_isolated_entities() {
    use adl::codefriend::architecture::four_plus_one::{generation, render};
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let mut draft = full_draft(&a);
    draft
        .views
        .get_mut(&View::Logical)
        .unwrap()
        .entities
        .push("edge".into());
    let g = generation::accept_response(&a, &graph, &serde_json::to_string(&draft).unwrap(), 101)
        .unwrap();
    let files = render::artifacts(&g.package, &a, 101).unwrap();
    let svg = String::from_utf8(files["logical-entity-002.svg"].clone()).unwrap();
    assert!(svg.contains("edge (no admitted relationship)"));
    let md = String::from_utf8(files["architecture.md"].clone()).unwrap();
    assert!(md.contains("logical-entity-002.svg"));
    let source = String::from_utf8(files["logical.mmd"].clone()).unwrap();
    assert!(source.contains("n2[\"entity_2\"]"));
}

#[test]
fn four_plus_one_approved_exports_preserve_diagrams_and_reject_tampering() {
    use adl::codefriend::{
        architecture::four_plus_one::{generation, render},
        integration::{prepare_publication_bundle_with_architecture, PublicationFormat},
        publication::*,
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (temp, store, graph) = generation_fixture_at(now);
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let response = format!(
        "```json\n{}\n```",
        serde_json::to_string(&full_draft(&a)).unwrap()
    );
    let g = generation::accept_response(&a, &graph, &response, now).unwrap();
    let mut files = render::artifacts(&g.package, &a, now).unwrap();
    files.insert(
        "generation.json".into(),
        serde_json::to_vec_pretty(&g).unwrap(),
    );
    files.insert(
        "graph.json".into(),
        serde_json::to_vec_pretty(&graph).unwrap(),
    );
    files.insert(
        "response.json".into(),
        serde_json::to_vec(&response).unwrap(),
    );
    // Synthetic complete lane records exercise export contracts only.
    use adl::codefriend::evidence::contracts::{Completion, ReviewRecord, Run};
    let review = ReviewRecord {
        admission: a.clone(),
        run: Run::new(
            &a,
            ["correctness", "security", "adversarial", "constitutional"]
                .into_iter()
                .map(|lane| (lane.into(), "codefriend.review_lane.v1".into()))
                .collect(),
            "fixture".into(),
            Completion::Complete,
            vec![],
        )
        .unwrap(),
        findings: vec![],
    };
    let review = &review;
    let review_path = temp.path().join("review.json");
    fs::write(&review_path, serde_json::to_vec_pretty(review).unwrap()).unwrap();
    let destination = temp.path().join("exports");
    fs::create_dir(&destination).unwrap();
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        let bundle = temp.path().join(format!("bundle-{}", format.key()));
        let publication = prepare_publication_bundle_with_architecture(
            &review_path,
            &bundle,
            &destination,
            format,
            Some(&files),
        )
        .unwrap();
        let approval_store = temp.path().join(format!("approval-{}", format.key()));
        append_decision(
            &approval_store,
            review,
            &publication,
            DecisionKind::Approved,
            "operator-fixture",
            "Approve exact fixture artifacts",
            now,
        )
        .unwrap();
        let out = destination.join(&publication.target);
        let options = MarkdownRenderOptions {
            review_record: review_path.clone(),
            publication: bundle.join("publication.json"),
            approval_store,
            artifact_root: bundle.join("artifacts"),
            synthesis: "synthesis/synthesis.json".into(),
            remediation_plan: "remediation/remediation-plan.json".into(),
            test_plan: "tests/test-plan.json".into(),
            destination_root: destination.clone(),
            out: out.clone(),
        };
        match format {
            PublicationFormat::Markdown => {
                render_markdown(options).unwrap();
            }
            PublicationFormat::Html => {
                render_html(HtmlRenderOptions {
                    review_record: options.review_record,
                    publication: options.publication,
                    approval_store: options.approval_store,
                    artifact_root: options.artifact_root,
                    synthesis: options.synthesis,
                    remediation_plan: options.remediation_plan,
                    test_plan: options.test_plan,
                    destination_root: options.destination_root,
                    out: options.out,
                })
                .unwrap();
            }
            PublicationFormat::Pdf => {
                let font = [
                    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                    "/System/Library/Fonts/Supplemental/Arial.ttf",
                    "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
                    "/Library/Fonts/Arial Unicode.ttf",
                ]
                .into_iter()
                .map(std::path::PathBuf::from)
                .find(|p| p.is_file())
                .expect("PDF fixture needs a Unicode font");
                let result = render_pdf(PdfRenderOptions {
                    review_record: options.review_record,
                    publication: options.publication,
                    approval_store: options.approval_store,
                    artifact_root: options.artifact_root,
                    synthesis: options.synthesis,
                    remediation_plan: options.remediation_plan,
                    test_plan: options.test_plan,
                    destination_root: options.destination_root,
                    out: options.out,
                    font,
                })
                .unwrap();
                assert!(result.page_count > files.keys().filter(|n| n.ends_with(".svg")).count());
            }
        }
        for (name, bytes) in &files {
            if name == "architecture.md" {
                let attached =
                    fs::read_to_string(out.join("architecture-architecture.md")).unwrap();
                assert!(attached.contains("](architecture-logical.mmd)"));
                assert!(attached.contains("](architecture-logical-000.svg)"));
                continue;
            }
            assert_eq!(
                fs::read(out.join(format!("architecture-{name}"))).unwrap(),
                *bytes
            );
        }
        if let PublicationFormat::Html = format {
            let html = fs::read_to_string(out.join("report.html")).unwrap();
            assert!(html.contains("id=\"logical\""));
            assert!(html.contains("<table>"));
            assert!(html.contains("<th>ID</th>"));
            assert!(!html.contains("| ID | Name and responsibility |"));
            assert!(html.contains("architecture-logical-000.svg"));
            assert!(html.contains("id=\"scenario-failure\""));
        }
    }
    files
        .get_mut("logical-000.svg")
        .unwrap()
        .extend_from_slice(b"tampered");
    assert!(prepare_publication_bundle_with_architecture(
        &review_path,
        &temp.path().join("bad-bundle"),
        &destination,
        PublicationFormat::Html,
        Some(&files)
    )
    .is_err());
}

#[test]
fn four_plus_one_accepts_complete_json_fence_but_not_truncated_or_extra_text() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let response = serde_json::to_string(&full_draft(&a)).unwrap();
    let fenced = format!("```json\n{response}\n```");
    let g = generation::accept_response(&a, &graph, &fenced, 101).unwrap();
    g.validate(&store, &graph, &fenced, 101).unwrap();
    assert!(g.validate(&store, &graph, &response, 101).is_err());
    assert!(
        generation::accept_response(&a, &graph, &format!("{fenced} trailing prose"), 101).is_err()
    );
    assert!(generation::accept_response(
        &a,
        &graph,
        &format!("```json\n{}\n```", &response[..response.len() / 2]),
        101
    )
    .is_err());
}

#[test]
fn four_plus_one_derives_membership_only_from_declared_relationship_endpoints() {
    use adl::codefriend::architecture::four_plus_one::generation;
    let (_temp, store, graph) = generation_fixture();
    let a = store.get(&graph.record().run.packet_id).unwrap();
    let mut draft = full_draft(&a);
    draft
        .views
        .get_mut(&View::Development)
        .unwrap()
        .entities
        .retain(|id| id != "worker");
    let response = serde_json::to_string(&draft).unwrap();
    let g = generation::accept_response(&a, &graph, &response, 101).unwrap();
    assert!(g.package.views[&View::Development]
        .entities
        .contains(&"worker".into()));
    g.validate(&store, &graph, &response, 101).unwrap();
    draft
        .views
        .get_mut(&View::Development)
        .unwrap()
        .relationships[0]
        .to = "invented_worker".into();
    assert!(
        generation::accept_response(&a, &graph, &serde_json::to_string(&draft).unwrap(), 101)
            .is_err()
    );
}

#[test]
fn four_plus_one_structure_seed_preserves_edges_without_inventing_runtime() {
    let (_temp, store, graph) = generation_fixture();
    let admission = store.get(&graph.record().run.packet_id).unwrap();
    let package = from_structure(&store, &graph, 100).unwrap();
    package.validate(&admission, 100).unwrap();
    assert!(!package.complete);
    assert_eq!(package.entities.len(), 2);
    let development = &package.views[&View::Development];
    assert_eq!(development.relationships.len(), 1);
    let edge = &development.relationships[0];
    assert!(development.entities.contains(&edge.from));
    assert!(development.entities.contains(&edge.to));
    edge.citations[0].validate(&admission).unwrap();
    for view in [View::Logical, View::Process, View::Deployment] {
        assert!(package.views[&view].entities.is_empty());
        assert!(package.views[&view].relationships.is_empty());
        assert!(!package.views[&view].missing_inputs.is_empty());
    }
    assert!(package.scenarios.is_empty());
    assert!(!package.missing_inputs.is_empty());
    assert!(from_structure(&store, &graph, admission.expires_at).is_err());
}

#[test]
fn four_plus_one_cli_generates_retrievable_package_and_refuses_overwrite() {
    use adl::codefriend::architecture::{artifact, four_plus_one::generation::Generation};
    use std::{
        io::{Read, Write},
        net::TcpListener,
        time::{Duration, Instant},
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let (temp, store, graph) = generation_fixture_at(now);
    let admission = store.get(&graph.record().run.packet_id).unwrap();
    let graph_path = temp.path().join("graph.json");
    artifact::write_report(&graph, &store, &graph_path, now).unwrap();
    drop(store);
    let payload = serde_json::to_string(&full_draft(&admission)).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let endpoint = format!("http://{}/v1/responses", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(30);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(
                        Instant::now() < deadline,
                        "architecture request did not arrive"
                    );
                    std::thread::sleep(Duration::from_millis(5));
                }
                Err(e) => panic!("{e}"),
            }
        };
        stream.set_nonblocking(false).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = Vec::new();
        loop {
            let mut chunk = [0; 4096];
            let n = stream.read(&mut chunk).unwrap();
            assert!(n > 0);
            bytes.extend_from_slice(&chunk[..n]);
            assert!(bytes.len() <= 1024 * 1024);
            if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&bytes[..end]);
                let len: usize = headers
                    .lines()
                    .find_map(|line| {
                        let (key, value) = line.split_once(':')?;
                        key.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse().unwrap())
                    })
                    .unwrap();
                if bytes.len() >= end + 4 + len {
                    break;
                }
            }
        }
        assert!(String::from_utf8_lossy(&bytes).contains("codefriend.four_plus_one.prompt.v1"));
        let body = serde_json::json!({"output_text":payload}).to_string();
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();
    });
    let mut request = four_plus_one_cli_provider_request();
    request.route.endpoint_ref = Some(endpoint);
    let request_path = temp.path().join("provider.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    let output = temp.path().join("generated");
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_adl"))
            .args(["codefriend", "architecture", "four-plus-one", "--store"])
            .arg(temp.path().join("store"))
            .arg("--graph")
            .arg(&graph_path)
            .arg("--provider-request")
            .arg(&request_path)
            .arg("--out")
            .arg(&output)
            .env(
                "ADL_CODEFRIEND_REVIEW_FIXTURE_KEY",
                "public-loopback-fixture",
            )
            .env("ADL_OBSERVABILITY_OTEL", "0")
            .output()
            .unwrap()
    };
    let result = invoke();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    server.join().unwrap();
    let summary: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(summary["complete"], true);
    assert_eq!(summary["views"], 4);
    let report_bytes = fs::read(output.join("four-plus-one.json")).unwrap();
    let report: Generation = serde_json::from_slice(&report_bytes).unwrap();
    report.package.validate(&admission, now).unwrap();
    assert_eq!(summary["digest"], report.package.digest);
    assert!(output.join("rendered").is_dir());
    let rejected = invoke();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("four_plus_one_output_exists"));
    assert_eq!(
        fs::read(output.join("four-plus-one.json")).unwrap(),
        report_bytes
    );
    request.input_text = Some("unapproved input".into());
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    assert!(String::from_utf8_lossy(&invoke().stderr)
        .contains("provider_request_must_not_preload_review_input"));
}

fn four_plus_one_cli_provider_request() -> adl::provider_communication::ProviderInvocationRequestV1
{
    use adl::{model_identity::ModelIdentityStrengthV1, provider_communication::*};
    let route = ProviderRouteV1 {
        provider_kind: ProviderKindV1::Hosted,
        provider: "openai".to_string(),
        runtime_surface: RuntimeSurfaceV1::HostedApi,
        provider_model_id: "codefriend-fixture-model".to_string(),
        endpoint_ref: Some("http://127.0.0.1:1".to_string()),
        credential_ref: Some("env:ADL_CODEFRIEND_REVIEW_FIXTURE_KEY".to_string()),
        source_registry: Some("codefriend-review-fixture".to_string()),
    };
    let mut model_identity = hosted_model_identity(
        "openai",
        "codefriend-fixture-model",
        "codefriend-fixture-model",
        Some("codefriend-review-fixture".to_string()),
    );
    model_identity.identity_strength = ModelIdentityStrengthV1::ProviderAsserted;
    ProviderInvocationRequestV1 {
        route,
        model_identity,
        prompt_contract_ref: "template.replaced.by.runner".to_string(),
        lane_ref: "template".to_string(),
        run_id: None,
        request_id: None,
        attempt_policy: ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 5_000,
            retry_backoff_ms: Some(1),
        },
        input_text: None,
        max_output_tokens: Some(512),
        context_window_tokens: None,
        reasoning_effort: None,
        clear_thinking: Some(true),
        temperature: Some(0.0),
        top_p: None,
        local_keep_alive: None,
        inference_parameter_fingerprint: Some("temperature=0,max_output_tokens=512".into()),
        tool_surface: Some("none".into()),
        governance_surface: Some("read_only_findings_only".into()),
        evaluator_ref: None,
        benchmark_ref: None,
    }
}
