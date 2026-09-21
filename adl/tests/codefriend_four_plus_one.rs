//! PVF: deterministic component contracts; not installed Journey qualification.
use adl::codefriend::{
    architecture::four_plus_one::*,
    evidence::{Admission, Retention},
    ingestion::{local, Scope},
};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
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
    let temp = tempfile::tempdir().unwrap();
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
    let temp = tempfile::tempdir().unwrap();
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
    fs::write(root.join("lib.rs"), "pub mod worker;\n").unwrap();
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
    let response = serde_json::to_string(&full_draft(&a)).unwrap();
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
    files.insert("response.json".into(), response.into_bytes());
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
