use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use csdlc_v2::cards::{
    digest, render, CardContent, PlanStep, ResourceProfile, StepStatus, ValidationLane,
};
use csdlc_v2::{
    bind_issue, edit_issue, BindRequest, BootstrapRequest, CardKind, EditRequest, InitialCardInput,
    LifecyclePhase, PlanningProfile, SemanticOperation, Store,
};

fn command(root: &Path, program: &str, args: &[&str]) -> Output {
    Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"))
}

fn must_succeed(output: Output) -> String {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 output")
}

fn assert_only_pending_design_review_diagnostic(repo: &Path, issue: u64) {
    let output = command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "issue",
            "--issue",
            &issue.to_string(),
        ],
    );
    assert!(!output.status.success());
    let envelope: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("validation error envelope JSON");
    assert_eq!(envelope["code"], "card_invalid");
    let report: serde_json::Value = serde_json::from_str(
        envelope["message"]
            .as_str()
            .expect("embedded validation report"),
    )
    .expect("embedded validation report JSON");
    assert_eq!(report["schema"], "csdlc.doctor.report.v1");
    assert_eq!(report["issue"], issue);
    assert_eq!(report["status"], "block");
    assert_eq!(report["ready"], false);
    assert_eq!(
        report["findings"],
        serde_json::json!([{
            "code": "design_review_missing_or_stale",
            "message": "design review does not cover the current design digest"
        }])
    );
    assert_eq!(report["next_operation"], "repair_design_readiness");
}

fn git(root: &Path, args: &[&str]) -> String {
    must_succeed(command(root, "git", args))
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("copy destination");
    for entry in fs::read_dir(source).expect("copy source") {
        let entry = entry.expect("copy entry");
        let destination = destination.join(entry.file_name());
        if entry.file_type().expect("copy file type").is_dir() {
            copy_directory(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).expect("copy file");
        }
    }
}

fn install_worktree_policy(root: &Path) {
    let required_parent = root.parent().expect("fixture worktree parent");
    fs::create_dir_all(root.join(".adl")).expect("policy directory");
    fs::write(
        root.join(".adl/worktree-policy.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "adl.worktree_policy.v1",
            "required_parent": required_parent.to_string_lossy()
        }))
        .expect("serialize worktree policy"),
    )
    .expect("worktree policy fixture");
}

fn add_non_primary_main_worktree(primary: &Path, worktree: &Path) {
    git(primary, &["switch", "--detach", "HEAD"]);
    git(
        primary,
        &["worktree", "add", &worktree.to_string_lossy(), "main"],
    );
}

fn focused_fixture_repo(root: &Path) {
    let primary = root.with_file_name(format!(
        "{}-primary",
        root.file_name()
            .and_then(|name| name.to_str())
            .expect("fixture repo name")
    ));
    fs::create_dir_all(primary.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(primary.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(primary.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(primary.join("design")).expect("design directory");
    fs::write(
        primary.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        primary.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(primary.join("csdlc-v2/tests/gate2.rs"), "// fixture\n").expect("test fixture");
    install_worktree_policy(&primary);
    for issue in [42_u64, 43] {
        fs::write(
            primary.join(format!("design/issue-{issue}.md")),
            format!("# Approved design for issue {issue}\n"),
        )
        .expect("design fixture");
        fs::write(
            primary.join(format!("design/issue-{issue}.mmd")),
            "flowchart LR\n  Create --> Bind\n",
        )
        .expect("diagram fixture");
    }
    git(&primary, &["init", "-b", "main"]);
    git(&primary, &["config", "user.email", "test@example.com"]);
    git(&primary, &["config", "user.name", "C-SDLC Test"]);
    git(
        &primary,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(&primary, &["add", "."]);
    git(&primary, &["commit", "-m", "fixture"]);
    add_non_primary_main_worktree(&primary, root);
}

fn focused_request(issue: u64) -> BootstrapRequest {
    let mut value = request();
    value.issue = issue;
    value.design_path = format!("design/issue-{issue}.md");
    value.diagram_path = format!("design/issue-{issue}.mmd");
    value.initial.title = format!("Focused bind issue {issue}");
    value.initial.slug = format!("focused-bind-issue-{issue}");
    value.initial.repo_inputs = vec![value.design_path.clone()];
    value.initial.affected_areas =
        vec![value.design_path.clone(), "csdlc-v2/tests/gate2.rs".into()];
    value
}

fn create_focused_issue(repo: &Path, request_root: &Path, issue: u64) {
    let request_path = request_root.join(format!("create-{issue}.json"));
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(&focused_request(issue)).expect("serialize focused request"),
    )
    .expect("focused create request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "create",
            "--request",
            &request_path.to_string_lossy(),
        ],
    ));
}

fn apply_edit(
    repo: &Path,
    temp: &Path,
    issue: u64,
    card: &str,
    name: &str,
    operation: serde_json::Value,
) {
    let index: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(format!(".csdlc/issues/{issue}/index.json"))).expect("issue index"),
    )
    .expect("issue index JSON");
    let request = temp.join(format!("{issue}-{name}.json"));
    fs::write(
        &request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": issue,
            "card": card,
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": format!("issue 5795 shaped fixture: {name}"),
            "operation": operation,
        }))
        .expect("serialize edit request"),
    )
    .expect("edit request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &request.to_string_lossy(),
        ],
    ));
}

fn issue_projection_snapshot(repo: &Path, issue: u64) -> BTreeMap<String, Vec<u8>> {
    fn collect(root: &Path, current: &Path, snapshot: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(current).expect("read projection directory") {
            let entry = entry.expect("projection entry");
            if entry.file_type().expect("projection file type").is_dir() {
                collect(root, &entry.path(), snapshot);
            } else {
                snapshot.insert(
                    entry
                        .path()
                        .strip_prefix(root)
                        .expect("projection relative path")
                        .to_string_lossy()
                        .into_owned(),
                    fs::read(entry.path()).expect("projection bytes"),
                );
            }
        }
    }

    let root = repo.join(format!(".csdlc/issues/{issue}"));
    let mut snapshot = BTreeMap::new();
    collect(&root, &root, &mut snapshot);
    snapshot
}

fn restore_issue_projection(repo: &Path, issue: u64, snapshot: &BTreeMap<String, Vec<u8>>) {
    let root = repo.join(format!(".csdlc/issues/{issue}"));
    for (relative, bytes) in snapshot {
        fs::write(root.join(relative), bytes).expect("restore projection bytes");
    }
}

fn direct_edit(
    store: &Store,
    record: &csdlc_v2::IssueRecord,
    card: CardKind,
    operation: SemanticOperation,
    fail_after_backup: bool,
) -> csdlc_v2::Result<csdlc_v2::IssueRecord> {
    edit_issue(
        store,
        EditRequest {
            issue: record.issue,
            card,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            actor: "test-operator".into(),
            reason: "prove guarded pre-bind contract repair".into(),
            operation,
            fail_after_backup,
        },
    )
}

fn write_consistent_record(repo: &Path, record: &mut csdlc_v2::IssueRecord) {
    record.digest.clear();
    record.digest = digest(&serde_json::to_vec(&*record).expect("record digest serialization"));
    let mut bytes = serde_json::to_vec_pretty(&*record).expect("record projection serialization");
    bytes.push(b'\n');
    fs::write(
        repo.join(format!(".csdlc/issues/{}/index.json", record.issue)),
        bytes,
    )
    .expect("write consistent record projection");
}

fn write_consistent_card(
    repo: &Path,
    record: &mut csdlc_v2::IssueRecord,
    kind: CardKind,
    values: &csdlc_v2::CardValues,
) {
    let rendered = render(values).expect("render consistent card");
    let cards = repo.join(format!(".csdlc/issues/{}/cards", record.issue));
    let mut encoded = serde_json::to_vec_pretty(values).expect("card projection serialization");
    encoded.push(b'\n');
    fs::write(cards.join(format!("{kind}.values.json")), encoded)
        .expect("write consistent card values");
    fs::write(
        cards.join(format!("{kind}.md")),
        rendered.markdown.as_bytes(),
    )
    .expect("write consistent rendered card");
    record.cards.insert(
        kind,
        csdlc_v2::model::CardProjection {
            values_digest: rendered.values_digest,
            rendered_digest: rendered.rendered_digest,
            ast_digest: rendered.ast_digest,
        },
    );
    write_consistent_record(repo, record);
}

fn write_consistent_design_reference(
    repo: &Path,
    record: &mut csdlc_v2::IssueRecord,
    design_ref: &str,
) {
    let store = Store::new(repo);
    let mut cards = store
        .load_cards(record.issue)
        .expect("design reference cards");
    record.design_path = design_ref.to_owned();
    for kind in [CardKind::Spp, CardKind::Vpp] {
        match &mut cards.get_mut(&kind).expect("design-bearing card").content {
            CardContent::Spp(values) => values.design_ref = design_ref.to_owned(),
            CardContent::Vpp(values) => values.design_ref = design_ref.to_owned(),
            _ => unreachable!("design-bearing card"),
        }
        write_consistent_card(repo, record, kind, &cards[&kind]);
    }
}

fn assert_single_generation_preserves_lifecycle_shell(
    before: &csdlc_v2::IssueRecord,
    after: &csdlc_v2::IssueRecord,
) {
    assert_eq!(after.generation, before.generation + 1);
    assert_eq!(after.issue, before.issue);
    assert_eq!(after.repository, before.repository);
    assert_eq!(after.code_repository, before.code_repository);
    assert_eq!(after.initialization_digest, before.initialization_digest);
    assert_eq!(after.phase, before.phase);
    assert_eq!(after.branch, before.branch);
    assert_eq!(after.worktree, before.worktree);
    assert_eq!(after.transitions, before.transitions);
    assert_eq!(after.review_assignment, before.review_assignment);
    assert_eq!(after.review, before.review);
    assert_eq!(after.publication, before.publication);
    assert_eq!(after.readiness, before.readiness);
    assert_eq!(after.migration, before.migration);
    assert_eq!(after.terminal, before.terminal);
    assert_eq!(after.design_path, before.design_path);
    assert_eq!(after.diagram_path, before.diagram_path);
    assert_eq!(after.audit.len(), before.audit.len() + 1);
    assert_eq!(&after.audit[..before.audit.len()], before.audit.as_slice());
    let event = after.audit.last().expect("new audit event");
    assert_eq!(event.generation, after.generation);
    assert_eq!(event.sequence, after.audit.len() as u64);
}

fn assert_design_bindings_match_authored(
    repo: &Path,
    cards: &BTreeMap<CardKind, csdlc_v2::CardValues>,
) {
    let (spp_design, spp_diagram) = match &cards[&CardKind::Spp].content {
        CardContent::Spp(values) => (&values.design_digest, &values.diagram_digest),
        _ => unreachable!("SPP"),
    };
    let (vpp_design, vpp_diagram) = match &cards[&CardKind::Vpp].content {
        CardContent::Vpp(values) => (&values.design_digest, &values.diagram_digest),
        _ => unreachable!("VPP"),
    };
    assert_eq!(spp_design, vpp_design);
    assert_eq!(spp_diagram, vpp_diagram);
    assert_eq!(
        spp_design,
        &digest(&fs::read(repo.join("design/issue-42.md")).expect("authored design bytes"))
    );
    assert_eq!(
        spp_diagram,
        &digest(&fs::read(repo.join("design/issue-42.mmd")).expect("authored diagram bytes"))
    );
}

fn spp_design_digests(cards: &BTreeMap<CardKind, csdlc_v2::CardValues>) -> (String, String) {
    match &cards[&CardKind::Spp].content {
        CardContent::Spp(values) => (values.design_digest.clone(), values.diagram_digest.clone()),
        _ => unreachable!("SPP"),
    }
}

fn assert_last_audit_operation(record: &csdlc_v2::IssueRecord, expected: &SemanticOperation) {
    let actual: serde_json::Value =
        serde_json::from_str(&record.audit.last().expect("new audit event").operation)
            .expect("audit operation JSON");
    assert_eq!(
        actual,
        serde_json::to_value(expected).expect("expected operation JSON")
    );
}

fn assert_last_prebind_audit_operation(
    record: &csdlc_v2::IssueRecord,
    expected: &SemanticOperation,
    old_design_digest: &str,
    new_design_digest: &str,
    old_diagram_digest: &str,
    new_diagram_digest: &str,
) {
    let actual: serde_json::Value =
        serde_json::from_str(&record.audit.last().expect("new audit event").operation)
            .expect("pre-bind audit operation JSON");
    assert_eq!(
        actual,
        serde_json::json!({
            "operation": expected,
            "design_binding_refresh": {
                "design_ref": "design/issue-42.md",
                "old_design_digest": old_design_digest,
                "new_design_digest": new_design_digest,
                "diagram_ref": "design/issue-42.mmd",
                "old_diagram_digest": old_diagram_digest,
                "new_diagram_digest": new_diagram_digest,
                "prior_design_approval": null,
            }
        })
    );
}

fn assert_card_semantics_unchanged(before: &csdlc_v2::CardValues, after: &csdlc_v2::CardValues) {
    assert_eq!(after.identity.generation, before.identity.generation + 1);
    assert_eq!(
        after.identity.schema_version,
        before.identity.schema_version
    );
    assert_eq!(
        after.identity.template_version,
        before.identity.template_version
    );
    assert_eq!(after.identity.issue, before.identity.issue);
    assert_eq!(after.identity.repository, before.identity.repository);
    assert_eq!(after.identity.title, before.identity.title);
    assert_eq!(after.identity.slug, before.identity.slug);
    assert_eq!(after.identity.version, before.identity.version);
    assert_eq!(after.status, before.status);
    assert_eq!(after.content, before.content);
}

fn assert_card_identity_advanced(before: &csdlc_v2::CardValues, after: &csdlc_v2::CardValues) {
    assert_eq!(after.identity.generation, before.identity.generation + 1);
    assert_eq!(
        after.identity.schema_version,
        before.identity.schema_version
    );
    assert_eq!(
        after.identity.template_version,
        before.identity.template_version
    );
    assert_eq!(after.identity.issue, before.identity.issue);
    assert_eq!(after.identity.repository, before.identity.repository);
    assert_eq!(after.identity.title, before.identity.title);
    assert_eq!(after.identity.slug, before.identity.slug);
    assert_eq!(after.identity.version, before.identity.version);
    assert_eq!(after.status, before.status);
}

fn request() -> BootstrapRequest {
    BootstrapRequest {
        issue: 42,
        repository: "agent-logic/agent-design-language".into(),
        actor: "test-operator".into(),
        design_path: "design/issue-42.md".into(),
        diagram_path: "design/issue-42.mmd".into(),
        design_reviewer: "fresh-session:22222222-2222-4222-8222-222222222222".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "Claim-free issue workflow".into(),
            slug: "claim-free-issue-workflow".into(),
            version: "v0.92".into(),
            goal: "Prove the claim-free issue workflow.".into(),
            required_outcome: "Create, validate, diagnose, and bind one issue.".into(),
            declared_scope: vec!["claim-free workflow".into()],
            authority_boundary: vec!["local test repository".into()],
            operator_constraints: vec!["no network".into()],
            task_boundary: "Exercise only the focused binary path.".into(),
            deliverables: vec![
                "bound issue record".into(),
                "csdlc-v2/tests/gate2.rs".into(),
            ],
            acceptance_criteria: vec![
                "issue creation is claim-free".into(),
                "binding is atomic and idempotent".into(),
            ],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["design/issue-42.md".into()],
            non_goals: vec!["publication".into()],
            plan_summary: "Create, validate, diagnose, and bind.".into(),
            steps: vec![PlanStep {
                id: "step-1".into(),
                action: "run the focused workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec![
                "design/issue-42.md".into(),
                "csdlc-v2/tests/gate2.rs".into(),
            ],
            invariants: vec!["Git topology is binding authority".into()],
            risks: vec!["conflicting worktree".into()],
            planning_profile: PlanningProfile::Small,
            stop_conditions: vec!["topology conflict".into()],
            validation_lanes: vec![ValidationLane {
                lane: "focused".into(),
                proof_role: "actual binary workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1_000,
                argv: vec![
                    "cargo".into(),
                    "test".into(),
                    "--manifest-path".into(),
                    "csdlc-v2/Cargo.toml".into(),
                    "--test".into(),
                    "gate2".into(),
                ],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed on invalid input or topology conflict.".into(),
            review_prompts: vec!["Review atomicity and idempotence.".into()],
            review_scope: "claim-free issue creation and binding".into(),
        },
    }
}

fn distributed_deferred_request(issue: u64, source: &str, test_target: &str) -> BootstrapRequest {
    let test_name = Path::new(test_target)
        .file_stem()
        .and_then(|value| value.to_str())
        .expect("named Rust test target");
    let mut value = request();
    value.issue = issue;
    value.repository = "danielbaustin/agent-design-language".into();
    value.design_path = format!("design/issue-{issue}.md");
    value.diagram_path = format!("design/issue-{issue}.mmd");
    value.initial.title = format!("Distributed deferred target issue {issue}");
    value.initial.slug = format!("distributed-deferred-target-{issue}");
    value.initial.repo_inputs = vec![value.design_path.clone()];
    value.initial.affected_areas = vec![source.into(), test_target.into()];
    value.initial.deliverables = vec![
        source.into(),
        test_target.into(),
        "Focused positive and negative tests".into(),
        "Digest-bound execution proof".into(),
        "Reviewed rollback evidence".into(),
    ];
    value.initial.validation_lanes = vec![ValidationLane {
        lane: "exact-child-tests".into(),
        proof_role: "Exact nextest target proves the issue-owned distributed behavior.".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
        deterministic: true,
        resource_profile: ResourceProfile::Small,
        budget_seconds: 120,
        budget_tokens: 1_000,
        argv: vec![
            "cargo".into(),
            "nextest".into(),
            "run".into(),
            "--manifest-path".into(),
            "adl-runtime/Cargo.toml".into(),
            "--test".into(),
            test_name.into(),
            "--no-tests=fail".into(),
        ],
        parallel_group: "child".into(),
        defer_reason: Some(format!(
            "The issue-owned temporary #[path = \"../src/distributed/...\"] harness will route {source} until integration registration."
        )),
    }];
    value.initial.failure_policy =
        "Fail closed on missing targets, zero tests, invalid evidence, or absent proof.".into();
    value
}

fn create_distributed_deferred_issue(repo: &Path, request_root: &Path, request: &BootstrapRequest) {
    fs::write(
        repo.join(&request.design_path),
        format!("# Approved design for issue {}\n", request.issue),
    )
    .expect("distributed design");
    fs::write(
        repo.join(&request.diagram_path),
        "flowchart LR\n  Prepare --> Bind\n",
    )
    .expect("distributed diagram");
    let request_path = request_root.join(format!("distributed-{}.json", request.issue));
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(request).expect("serialize distributed request"),
    )
    .expect("distributed request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "create",
            "--request",
            &request_path.to_string_lossy(),
        ],
    ));
}

fn distributed_fixture_repo(root: &Path) {
    focused_fixture_repo(root);
    fs::create_dir_all(root.join("adl-runtime/src/distributed"))
        .expect("distributed source directory");
    fs::create_dir_all(root.join("adl-runtime/tests")).expect("distributed tests directory");
    fs::write(
        root.join("adl-runtime/Cargo.toml"),
        "[package]\nname = \"adl-runtime\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("runtime manifest");
    fs::write(
        root.join("adl-runtime/src/lib.rs"),
        "// no distributed route\n",
    )
    .expect("runtime crate root");
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "distributed fixture"]);
}

type DeferredRequestMutation = fn(&mut BootstrapRequest);

#[test]
fn initialized_deferred_distributed_targets_bind_only_through_exact_path_harnesses() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    distributed_fixture_repo(&repo);

    for (issue, source, test_target) in [
        (
            5866,
            "adl-runtime/src/distributed/discovery.rs",
            "adl-runtime/tests/distributed_discovery.rs",
        ),
        (
            5871,
            "adl-runtime/src/distributed/capability_advertisement.rs",
            "adl-runtime/tests/distributed_capability_advertisement.rs",
        ),
        (
            5872,
            "adl-runtime/src/distributed/resource_weather.rs",
            "adl-runtime/tests/distributed_resource_weather.rs",
        ),
    ] {
        let mut request = distributed_deferred_request(issue, source, test_target);
        let exact_deliverables = request.initial.deliverables.clone();
        let exact_lanes = request.initial.validation_lanes.clone();
        request.initial.deliverables = vec![
            "Implement the bounded distributed behavior.".into(),
            "Focused positive and negative tests".into(),
            "Digest-bound execution proof".into(),
            "Reviewed rollback evidence".into(),
        ];
        request.initial.validation_lanes[0].defer_reason = None;
        create_distributed_deferred_issue(&repo, temp.path(), &request);

        let live_shape = csdlc_v2::doctor::diagnose_with_code_repository(
            &Store::new(&repo),
            issue,
            Some("agent-logic/agent-design-language"),
        );
        assert_eq!(live_shape.status, csdlc_v2::doctor::DoctorStatus::Block);
        for code in [
            "owned_rust_module_unroutable",
            "validator_target_missing",
            "issue_specific_denominator_missing",
        ] {
            assert!(
                live_shape
                    .findings
                    .iter()
                    .any(|finding| finding.code == code),
                "live child shape {issue} lacked {code}: {:?}",
                live_shape.findings
            );
        }
        for prose in [
            "Focused positive and negative tests",
            "Digest-bound execution proof",
            "Reviewed rollback evidence",
        ] {
            assert!(!live_shape
                .findings
                .iter()
                .any(|finding| finding.message.contains(prose)));
        }

        apply_edit(
            &repo,
            temp.path(),
            issue,
            "stp",
            "declare-exact-future-deliverables",
            serde_json::json!({
                "operation": "replace_planning_collection",
                "field": "deliverables",
                "values": exact_deliverables
            }),
        );
        apply_edit(
            &repo,
            temp.path(),
            issue,
            "vpp",
            "declare-temporary-path-harness",
            serde_json::json!({
                "operation": "replace_validation_lanes",
                "lanes": exact_lanes
            }),
        );
        let initialized = csdlc_v2::doctor::diagnose_with_code_repository(
            &Store::new(&repo),
            issue,
            Some("agent-logic/agent-design-language"),
        );
        assert_eq!(initialized.status, csdlc_v2::doctor::DoctorStatus::Pass);
        assert!(initialized.ready);
        assert!(initialized.findings.is_empty());
        assert_eq!(initialized.next_operation.as_deref(), Some("advance_ready"));

        apply_edit(
            &repo,
            temp.path(),
            issue,
            "spp",
            "follow-advertised-ready-transition",
            serde_json::json!({
                "operation": "advance_phase",
                "phase": "ready"
            }),
        );
        let ready = csdlc_v2::doctor::diagnose_with_code_repository(
            &Store::new(&repo),
            issue,
            Some("agent-logic/agent-design-language"),
        );
        assert_eq!(ready.status, csdlc_v2::doctor::DoctorStatus::Pass);
        assert!(!ready.ready);
        assert!(ready.findings.is_empty());
        assert_eq!(ready.next_operation.as_deref(), Some("inspect_phase"));

        let worktree = temp.path().join(format!("worktrees/issue-{issue}"));
        bind_issue(
            &Store::new(&repo),
            BindRequest {
                issue,
                base_branch: "main".into(),
                branch: format!("codex/{issue}-distributed-deferred"),
                worktree: worktree.to_string_lossy().into_owned(),
                code_repository: Some("agent-logic/agent-design-language".into()),
            },
        )
        .expect("bind initialized deferred targets");

        let bound = csdlc_v2::diagnose(&Store::new(&worktree), issue);
        assert_eq!(bound.status, csdlc_v2::doctor::DoctorStatus::Block);
        for code in [
            "owned_rust_module_unroutable",
            "validator_target_missing",
            "issue_specific_denominator_missing",
        ] {
            assert!(bound.findings.iter().any(|finding| finding.code == code));
        }

        fs::create_dir_all(
            worktree
                .join(source)
                .parent()
                .expect("source parent directory"),
        )
        .expect("materialized source directory");
        fs::create_dir_all(
            worktree
                .join(test_target)
                .parent()
                .expect("test parent directory"),
        )
        .expect("materialized test directory");
        fs::write(worktree.join(source), "pub fn implemented() {}\n").expect("implemented source");
        fs::write(
            worktree.join(test_target),
            format!(
                "#[path = \"../src/distributed/{}\"]\nmod subject;\n#[test]\nfn implemented_target_runs() {{ subject::implemented(); }}\n",
                Path::new(source)
                    .file_name()
                    .and_then(|value| value.to_str())
                    .expect("source filename")
            ),
        )
        .expect("implemented focused test");
        let implemented_targets = csdlc_v2::diagnose(&Store::new(&worktree), issue);
        assert_eq!(
            implemented_targets.status,
            csdlc_v2::doctor::DoctorStatus::Pass
        );
        assert!(implemented_targets.findings.is_empty());
    }
}

#[test]
fn deferred_rust_path_harness_admission_fails_closed_for_each_missing_predicate() {
    let cases: &[(&str, DeferredRequestMutation)] = &[
        ("source-not-deliverable", |request| {
            request.initial.deliverables.remove(0);
        }),
        ("test-not-owned", |request| {
            request.initial.affected_areas.pop();
        }),
        ("test-not-deliverable", |request| {
            request.initial.deliverables.remove(1);
        }),
        ("missing-deferral", |request| {
            request.initial.validation_lanes[0].defer_reason = None;
        }),
        ("placeholder-deferral", |request| {
            request.initial.validation_lanes[0].defer_reason = Some("TBD".into());
        }),
        ("no-path-harness", |request| {
            request.initial.validation_lanes[0].defer_reason =
                Some("The future focused test will be added during execution.".into());
        }),
        ("not-fail-closed", |request| {
            request.initial.failure_policy =
                "Fail closed unless the future validator is missing.".into();
        }),
        ("zero-test-not-enforced", |request| {
            request.initial.validation_lanes[0]
                .argv
                .retain(|argument| argument != "--no-tests=fail");
        }),
        ("multiple-unroutable-sources", |request| {
            let sibling = "adl-runtime/src/distributed/unrelated.rs".to_string();
            request.initial.affected_areas.push(sibling.clone());
            request.initial.deliverables.push(sibling);
        }),
        ("validator-deliverable-unowned", |request| {
            request
                .initial
                .deliverables
                .push("adl-runtime/tests/unowned_validator.rs".into());
        }),
    ];

    for (offset, (name, mutate)) in cases.iter().enumerate() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo = temp.path().join("repo");
        distributed_fixture_repo(&repo);
        let issue = 6000 + offset as u64;
        let source = "adl-runtime/src/distributed/discovery.rs";
        let test_target = "adl-runtime/tests/distributed_discovery.rs";
        let mut request = distributed_deferred_request(issue, source, test_target);
        mutate(&mut request);
        create_distributed_deferred_issue(&repo, temp.path(), &request);
        let diagnosis = csdlc_v2::doctor::diagnose_with_code_repository(
            &Store::new(&repo),
            issue,
            Some("agent-logic/agent-design-language"),
        );
        assert_eq!(
            diagnosis.status,
            csdlc_v2::doctor::DoctorStatus::Block,
            "case {name} unexpectedly passed: {:?}",
            diagnosis.findings
        );
        assert!(
            diagnosis.findings.iter().any(|finding| matches!(
                finding.code.as_str(),
                "owned_rust_module_unroutable"
                    | "validator_target_missing"
                    | "validator_deliverable_unowned"
                    | "issue_specific_denominator_missing"
            )),
            "case {name} lacked a false-readiness finding: {:?}",
            diagnosis.findings
        );
    }
}

fn intentional_deletion_request(issue: u64, target: &str) -> BootstrapRequest {
    let mut value = request();
    value.issue = issue;
    value.design_path = format!("design/issue-{issue}.md");
    value.diagram_path = format!("design/issue-{issue}.mmd");
    value.initial.title = format!("Intentional deletion validator issue {issue}");
    value.initial.slug = format!("intentional-deletion-validator-{issue}");
    value.initial.repo_inputs = vec![value.design_path.clone()];
    value.initial.affected_areas = vec![target.into()];
    value.initial.deliverables = vec![
        target.into(),
        format!("intentional deletion: {target}"),
        "Focused deletion readiness proof".into(),
    ];
    value.initial.validation_lanes = vec![ValidationLane {
        lane: "deleted-validator".into(),
        proof_role: "Classify the issue-owned intentionally deleted validator.".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
        deterministic: true,
        resource_profile: ResourceProfile::Small,
        budget_seconds: 120,
        budget_tokens: 1_000,
        argv: vec![
            "cargo".into(),
            "test".into(),
            "--manifest-path".into(),
            "csdlc-v2/Cargo.toml".into(),
            "--test".into(),
            Path::new(target)
                .file_stem()
                .and_then(|value| value.to_str())
                .expect("validator target stem")
                .into(),
        ],
        parallel_group: "local".into(),
        defer_reason: None,
    }];
    value.initial.failure_policy =
        "Fail closed on missing targets, invalid deletion proof, or absent review.".into();
    value
}

fn create_intentional_deletion_issue(repo: &Path, request_root: &Path, request: &BootstrapRequest) {
    fs::write(
        repo.join(&request.design_path),
        format!("# Approved design for issue {}\n", request.issue),
    )
    .expect("intentional deletion design");
    fs::write(
        repo.join(&request.diagram_path),
        "flowchart LR\n  BasePath --> DeletedCandidate\n",
    )
    .expect("intentional deletion diagram");
    let request_path = request_root.join(format!("intentional-deletion-{}.json", request.issue));
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(request).expect("serialize intentional deletion request"),
    )
    .expect("intentional deletion request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "create",
            "--request",
            &request_path.to_string_lossy(),
        ],
    ));
}

type IntentionalDeletionMutation = fn(&mut BootstrapRequest);

#[test]
fn intentional_deletion_deliverable_satisfies_readiness_with_git_proof() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    focused_fixture_repo(&repo);
    fs::write(
        repo.join("csdlc-v2/Cargo.toml"),
        "[package]\nname = \"csdlc-v2\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("csdlc manifest");
    let target = "csdlc-v2/tests/deleted_validator.rs";
    fs::write(
        repo.join(target),
        "// validator intentionally removed by issue\n",
    )
    .expect("deleted validator baseline");
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "add deletion validator baseline"]);
    git(&repo, &["switch", "-c", "codex/intentional-deletion"]);

    let request = intentional_deletion_request(42100, target);
    create_intentional_deletion_issue(&repo, temp.path(), &request);
    git(&repo, &["rm", target]);

    let diagnosis = csdlc_v2::doctor::diagnose_with_code_repository(
        &Store::new(&repo),
        42100,
        Some("agent-logic/agent-design-language"),
    );
    assert_eq!(
        diagnosis.status,
        csdlc_v2::doctor::DoctorStatus::Pass,
        "unexpected findings: {:?}",
        diagnosis.findings
    );
    assert!(diagnosis.findings.is_empty());
}

#[test]
fn intentional_deletion_deliverable_fails_closed_without_exact_predicates() {
    let cases: &[(&str, IntentionalDeletionMutation, bool)] = &[
        (
            "missing-marker",
            |request| {
                request
                    .initial
                    .deliverables
                    .retain(|deliverable| !deliverable.starts_with("intentional deletion:"));
            },
            true,
        ),
        (
            "not-fail-closed",
            |request| {
                request.initial.failure_policy =
                    "Fail closed unless intentional deletion evidence is absent.".into();
            },
            true,
        ),
        (
            "unowned-target",
            |request| {
                request.initial.affected_areas = vec!["csdlc-v2/src/cards.rs".into()];
            },
            true,
        ),
        ("base-absent", |_request| {}, false),
    ];

    for (offset, (name, mutate, create_base_file)) in cases.iter().enumerate() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo = temp.path().join("repo");
        focused_fixture_repo(&repo);
        fs::write(
            repo.join("csdlc-v2/Cargo.toml"),
            "[package]\nname = \"csdlc-v2\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .expect("csdlc manifest");
        let target = "csdlc-v2/tests/deleted_validator.rs";
        if *create_base_file {
            fs::write(
                repo.join(target),
                "// validator intentionally removed by issue\n",
            )
            .expect("deleted validator baseline");
        }
        git(&repo, &["add", "."]);
        git(
            &repo,
            &["commit", "-m", "prepare intentional deletion baseline"],
        );
        git(
            &repo,
            &[
                "switch",
                "-c",
                &format!("codex/intentional-deletion-{name}"),
            ],
        );

        let issue = 42110 + offset as u64;
        let mut request = intentional_deletion_request(issue, target);
        mutate(&mut request);
        create_intentional_deletion_issue(&repo, temp.path(), &request);
        if *create_base_file {
            git(&repo, &["rm", target]);
        }

        let diagnosis = csdlc_v2::doctor::diagnose_with_code_repository(
            &Store::new(&repo),
            issue,
            Some("agent-logic/agent-design-language"),
        );
        assert_eq!(
            diagnosis.status,
            csdlc_v2::doctor::DoctorStatus::Block,
            "case {name} unexpectedly passed"
        );
        assert!(
            diagnosis.findings.iter().any(|finding| matches!(
                finding.code.as_str(),
                "validator_target_missing"
                    | "validator_deliverable_unowned"
                    | "issue_specific_denominator_missing"
            )),
            "case {name} lacked a deletion-readiness finding: {:?}",
            diagnosis.findings
        );
    }
}

#[test]
fn intentional_deletion_deliverable_rejects_branch_local_add_then_delete() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    focused_fixture_repo(&repo);
    fs::write(
        repo.join("csdlc-v2/Cargo.toml"),
        "[package]\nname = \"csdlc-v2\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("csdlc manifest");
    git(&repo, &["add", "."]);
    git(
        &repo,
        &["commit", "-m", "prepare governed base without validator"],
    );
    git(&repo, &["switch", "-c", "codex/add-then-delete-validator"]);

    let target = "csdlc-v2/tests/deleted_validator.rs";
    fs::write(repo.join(target), "// branch-local validator\n").expect("branch-local validator");
    git(&repo, &["add", target]);
    git(&repo, &["commit", "-m", "add branch-local validator"]);

    let request = intentional_deletion_request(42120, target);
    create_intentional_deletion_issue(&repo, temp.path(), &request);
    git(&repo, &["rm", target]);

    let diagnosis = csdlc_v2::doctor::diagnose_with_code_repository(
        &Store::new(&repo),
        42120,
        Some("agent-logic/agent-design-language"),
    );
    assert_eq!(
        diagnosis.status,
        csdlc_v2::doctor::DoctorStatus::Block,
        "branch-local add-then-delete unexpectedly passed"
    );
    assert!(
        diagnosis.findings.iter().any(|finding| matches!(
            finding.code.as_str(),
            "validator_target_missing" | "issue_specific_denominator_missing"
        )),
        "branch-local add-then-delete lacked deletion proof finding: {:?}",
        diagnosis.findings
    );
}

#[test]
fn actual_binaries_create_validate_doctor_and_bind_without_claims() {
    let temp = tempfile::tempdir().expect("tempdir");
    let primary = temp.path().join("repo-primary");
    let repo = temp.path().join("repo");
    let worktree = temp.path().join("worktrees/issue-42");
    let conflict = temp.path().join("worktrees/conflict");
    fs::create_dir_all(primary.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(primary.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(primary.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(primary.join("design")).expect("design directory");
    fs::write(
        primary.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        primary.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(
        primary.join("csdlc-v2/tests/gate2.rs"),
        "// focused fixture\n",
    )
    .expect("gate2 fixture");
    install_worktree_policy(&primary);
    fs::write(
        primary.join("csdlc-v2/tests/gate4.rs"),
        "// unrelated fixture\n",
    )
    .expect("unrelated gate4 fixture");
    fs::write(primary.join("design/issue-42.md"), "# Approved design\n").expect("design");
    fs::write(
        primary.join("design/issue-42.mmd"),
        "flowchart LR\n  Create --> Bind\n",
    )
    .expect("diagram");

    git(&primary, &["init", "-b", "main"]);
    git(&primary, &["config", "user.email", "test@example.com"]);
    git(&primary, &["config", "user.name", "C-SDLC Test"]);
    git(
        &primary,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(&primary, &["add", "."]);
    git(&primary, &["commit", "-m", "fixture"]);
    add_non_primary_main_worktree(&primary, &repo);

    let invalid_create_request = temp.path().join("invalid-create.json");
    let mut invalid_create = request();
    invalid_create.issue = 40;
    invalid_create.design_path = "generated/invalid-design.md".into();
    invalid_create.diagram_path = "generated/invalid-diagram.mmd".into();
    invalid_create.initial.affected_areas.clear();
    fs::write(
        &invalid_create_request,
        serde_json::to_vec_pretty(&invalid_create).expect("serialize invalid create request"),
    )
    .expect("invalid create request");
    let repo_text = repo.to_string_lossy();
    let invalid_create_text = invalid_create_request.to_string_lossy();
    let invalid_created = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &invalid_create_text,
        ],
    );
    assert!(!invalid_created.status.success());
    assert!(!repo.join("generated/invalid-design.md").exists());
    assert!(!repo.join("generated/invalid-diagram.mmd").exists());

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let outside = temp.path().join("outside");
        fs::create_dir_all(&outside).expect("outside directory");
        symlink(&outside, repo.join("escape")).expect("symlinked authored parent");
        let symlink_request = temp.path().join("symlink-create.json");
        let mut symlinked = request();
        symlinked.issue = 38;
        symlinked.design_path = "escape/design.md".into();
        symlinked.diagram_path = "escape/diagram.mmd".into();
        fs::write(
            &symlink_request,
            serde_json::to_vec_pretty(&symlinked).expect("serialize symlink request"),
        )
        .expect("symlink request");
        let symlink_text = symlink_request.to_string_lossy();
        let symlink_result = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &["--root", &repo_text, "create", "--request", &symlink_text],
        );
        assert!(!symlink_result.status.success());
        assert!(!outside.join("design.md").exists());
        assert!(!outside.join("diagram.mmd").exists());
    }

    let non_proving_request = temp.path().join("non-proving-create.json");
    let mut non_proving = request();
    non_proving.issue = 41;
    non_proving.design_path = "generated/non-proving-design.md".into();
    non_proving.diagram_path = "generated/non-proving-diagram.mmd".into();
    non_proving.initial.validation_lanes[0].argv = vec!["true".into()];
    fs::write(
        &non_proving_request,
        serde_json::to_vec_pretty(&non_proving).expect("serialize non-proving request"),
    )
    .expect("non-proving request");
    let non_proving_text = non_proving_request.to_string_lossy();
    let non_proving_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &non_proving_text,
        ],
    );
    assert!(!non_proving_result.status.success());
    assert!(!repo.join("generated/non-proving-design.md").exists());
    assert!(!repo.join("generated/non-proving-diagram.mmd").exists());

    #[cfg(unix)]
    {
        fs::create_dir_all(repo.join("generated")).expect("generated directory");
        fs::write(repo.join("generated/not-executable"), "exit 0\n")
            .expect("non-executable validator");
        let non_executable_request = temp.path().join("non-executable-create.json");
        let mut non_executable = request();
        non_executable.issue = 39;
        non_executable.design_path = "generated/non-executable-design.md".into();
        non_executable.diagram_path = "generated/non-executable-diagram.mmd".into();
        non_executable.initial.validation_lanes[0].argv = vec!["generated/not-executable".into()];
        fs::write(
            &non_executable_request,
            serde_json::to_vec_pretty(&non_executable)
                .expect("serialize non-executable create request"),
        )
        .expect("non-executable create request");
        let non_executable_text = non_executable_request.to_string_lossy();
        must_succeed(command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &[
                "--root",
                &repo_text,
                "create",
                "--request",
                &non_executable_text,
            ],
        ));
        let non_executable_validation = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-validate"),
            &["--root", &repo_text, "issue", "--issue", "39"],
        );
        assert!(!non_executable_validation.status.success());
    }

    let create_request = temp.path().join("create.json");
    let mut create = serde_json::to_value(request()).expect("serialize create request");
    create["claim"] = serde_json::json!({"id": "ignored-legacy-create-claim"});
    fs::write(
        &create_request,
        serde_json::to_vec_pretty(&create).expect("serialize create request"),
    )
    .expect("create request");
    let create_text = create_request.to_string_lossy();
    let legacy_create = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &create_text],
    );
    assert!(!legacy_create.status.success());
    fs::write(
        &create_request,
        serde_json::to_vec_pretty(&request()).expect("serialize claim-free create request"),
    )
    .expect("claim-free create request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &create_text],
    ));

    let validated = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    ));
    assert!(validated.contains("\"status\":\"pass\""));
    let diagnosed = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));
    assert!(diagnosed.contains("\"ready\": true"));
    let same_repository_index: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(".csdlc/issues/42/index.json")).expect("same-repository index"),
    )
    .expect("same-repository index JSON");
    assert!(same_repository_index.get("code_repository").is_none());

    let split_worktree = temp.path().join("worktrees/issue-43");
    let mut split = request();
    split.issue = 43;
    split.repository = "danielbaustin/agent-design-language".into();
    split.design_path = "design/issue-43.md".into();
    split.diagram_path = "design/issue-43.mmd".into();
    split.initial.affected_areas[0] = "design/issue-43.md".into();
    split.initial.repo_inputs[0] = "design/issue-43.md".into();
    fs::write(repo.join("design/issue-43.md"), "# Approved split design\n").expect("split design");
    fs::write(
        repo.join("design/issue-43.mmd"),
        "flowchart LR\n  Issue --> Code\n",
    )
    .expect("split diagram");
    let split_create_request = temp.path().join("split-create.json");
    fs::write(
        &split_create_request,
        serde_json::to_vec_pretty(&split).expect("serialize split create request"),
    )
    .expect("split create request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &split_create_request.to_string_lossy(),
        ],
    ));
    let split_without_contract = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "43"],
    );
    assert!(!split_without_contract.status.success());
    assert!(String::from_utf8_lossy(&split_without_contract.stdout)
        .contains("no explicit code repository was declared"));
    let split_bind_request = temp.path().join("split-bind.json");
    fs::write(
        &split_bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "issue-43",
            "worktree": split_worktree,
            "code_repository": "agent-logic/agent-design-language",
        }))
        .expect("serialize split bind request"),
    )
    .expect("split bind request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &split_bind_request.to_string_lossy(),
        ],
    ));
    let split_repo_text = split_worktree.to_string_lossy();
    let split_diagnosis = must_succeed(command(
        &split_worktree,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &split_repo_text, "--issue", "43"],
    ));
    assert!(!split_diagnosis.contains("repository_identity_drift"));
    let split_index: serde_json::Value = serde_json::from_slice(
        &fs::read(split_worktree.join(".csdlc/issues/43/index.json")).expect("split issue index"),
    )
    .expect("split issue index JSON");
    assert_eq!(
        split_index["code_repository"],
        "agent-logic/agent-design-language"
    );
    let mismatched_split = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("other-owner/other-repository"),
    );
    let mismatch = mismatched_split
        .findings
        .iter()
        .find(|finding| finding.code == "repository_identity_drift")
        .expect("mismatched explicit split must fail closed");
    assert!(mismatch
        .message
        .contains("requested code repository other-owner/other-repository"));
    assert!(mismatch
        .message
        .contains("recorded code repository agent-logic/agent-design-language"));
    git(&repo, &["remote", "remove", "origin"]);
    let missing_origin = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("agent-logic/agent-design-language"),
    );
    assert!(missing_origin.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift" && finding.message.contains("none is available")
    }));
    git(
        &repo,
        &["remote", "add", "origin", "file:///not-a-github-repository"],
    );
    let non_github_origin = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("agent-logic/agent-design-language"),
    );
    assert!(non_github_origin.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift" && finding.message.contains("none is available")
    }));
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/other-owner/other-repository.git",
        ],
    );
    let substituted_bind_request = temp.path().join("substituted-split-bind.json");
    fs::write(
        &substituted_bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "issue-43",
            "worktree": split_worktree,
            "code_repository": "other-owner/other-repository",
        }))
        .expect("serialize substituted split bind request"),
    )
    .expect("substituted split bind request");
    let substituted_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &substituted_bind_request.to_string_lossy(),
        ],
    );
    assert!(!substituted_bind.status.success());
    let post_substitution = csdlc_v2::diagnose(&csdlc_v2::Store::new(&split_worktree), 43);
    assert!(post_substitution.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift"
            && finding
                .message
                .contains("declared code repository agent-logic/agent-design-language")
            && finding
                .message
                .contains("origin repository other-owner/other-repository")
    }));
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    let restored_split = csdlc_v2::diagnose(&csdlc_v2::Store::new(&split_worktree), 43);
    assert!(!restored_split
        .findings
        .iter()
        .any(|finding| finding.code == "repository_identity_drift"));

    let mut issue_5795_shape = request();
    issue_5795_shape.issue = 44;
    issue_5795_shape.repository = "danielbaustin/agent-design-language".into();
    issue_5795_shape.design_path = "design/issue-44.md".into();
    issue_5795_shape.diagram_path = "design/issue-44.mmd".into();
    issue_5795_shape.initial.affected_areas[0] = "design/issue-44.md".into();
    issue_5795_shape.initial.repo_inputs[0] = "design/issue-44.md".into();
    fs::write(repo.join("design/issue-44.md"), "# Approved design\n")
        .expect("issue 5795 shaped design");
    fs::write(
        repo.join("design/issue-44.mmd"),
        "flowchart LR\n  Plan --> Block\n",
    )
    .expect("issue 5795 shaped diagram");
    let issue_5795_request = temp.path().join("issue-5795-shape-create.json");
    fs::write(
        &issue_5795_request,
        serde_json::to_vec_pretty(&issue_5795_shape).expect("serialize issue 5795 shaped request"),
    )
    .expect("issue 5795 shaped request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &issue_5795_request.to_string_lossy(),
        ],
    ));
    fs::create_dir_all(repo.join("adl-runtime-kernel/src")).expect("runtime-kernel source");
    fs::create_dir_all(repo.join("adl-runtime-kernel/tests")).expect("future crate test directory");
    fs::write(
        repo.join("adl-runtime-kernel/Cargo.toml"),
        "[package]\nname = \"adl-runtime-kernel\"\nversion = \"0.1.0\"\n",
    )
    .expect("runtime-kernel manifest");
    let future_module = "adl-runtime-kernel/src/shepherd.rs";
    let present_validator = "adl-runtime-kernel/tests/shepherd_unit.rs";
    let missing_validator = "adl-runtime-kernel/tests/shepherd_live.rs";
    fs::write(repo.join(present_validator), "// present but unselected\n")
        .expect("present issue-owned validator");
    apply_edit(
        &repo,
        temp.path(),
        44,
        "spp",
        "unroutable-owned-module",
        serde_json::json!({
            "operation": "replace_planning_collection",
            "field": "affected_areas",
            "values": [future_module, present_validator, missing_validator]
        }),
    );
    apply_edit(
        &repo,
        temp.path(),
        44,
        "stp",
        "missing-validator-deliverable",
        serde_json::json!({
            "operation": "replace_planning_collection",
            "field": "deliverables",
            "values": [future_module, present_validator, missing_validator]
        }),
    );
    apply_edit(
        &repo,
        temp.path(),
        44,
        "vpp",
        "zero-denominator-lane",
        serde_json::json!({
            "operation": "replace_validation_lanes",
            "lanes": [{
                "lane": "unrelated-existing-gate",
                "proof_role": "reproduce the unrelated existing test denominator",
                "acceptance_ids": ["AC-1", "AC-2"],
                "deterministic": true,
                "resource_profile": "small",
                "budget_seconds": 120,
                "budget_tokens": 1000,
                "argv": ["cargo", "test", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "gate4"],
                "parallel_group": "local",
                "defer_reason": null
            }]
        }),
    );
    let issue_5795_diagnosis = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "44"],
    );
    assert!(!issue_5795_diagnosis.status.success());
    let issue_5795_diagnosis =
        String::from_utf8(issue_5795_diagnosis.stdout).expect("UTF-8 diagnosis");
    for code in [
        "repository_identity_drift",
        "owned_rust_module_unroutable",
        "validator_target_missing",
        "issue_specific_denominator_missing",
    ] {
        assert!(issue_5795_diagnosis.contains(code), "missing {code}");
    }

    fs::create_dir_all(repo.join("workspace/member/src/foo")).expect("nested product source");
    fs::create_dir_all(repo.join("workspace/member/tests")).expect("product tests");
    fs::create_dir_all(repo.join("docs/src")).expect("non-crate src directory");
    fs::write(
        repo.join("Cargo.toml"),
        "[workspace]\nmembers = [\"workspace/member\"]\nresolver = \"2\"\n",
    )
    .expect("workspace manifest");
    fs::write(
        repo.join("workspace/member/Cargo.toml"),
        "[package]\nname = \"product\"\nversion = \"0.1.0\"\n",
    )
    .expect("product manifest");
    fs::write(repo.join("workspace/member/src/lib.rs"), "pub mod foo;\n")
        .expect("product crate root");
    fs::write(
        repo.join("workspace/member/src/foo/mod.rs"),
        "// parent module\n",
    )
    .expect("parent module");
    fs::write(
        repo.join("workspace/member/tests/focused.rs"),
        "// focused validator\n",
    )
    .expect("focused validator");
    let mut valid_edge_shapes = request();
    valid_edge_shapes.issue = 45;
    valid_edge_shapes.design_path = "design/issue-45.md".into();
    valid_edge_shapes.diagram_path = "design/issue-45.mmd".into();
    valid_edge_shapes.initial.repo_inputs[0] = "design/issue-45.md".into();
    valid_edge_shapes.initial.affected_areas = vec![
        "design/issue-45.md".into(),
        "workspace/member/src/foo/mod.rs".into(),
        "workspace/member/src/foo/bar/mod.rs".into(),
        "workspace/member/src/server.py".into(),
        "docs/src/example.rs".into(),
        "workspace/member/tests/focused.rs".into(),
    ];
    valid_edge_shapes.initial.deliverables = vec![
        "workspace/member/src/foo/bar/mod.rs".into(),
        "workspace/member/src/server.py".into(),
        "docs/src/example.rs".into(),
        "workspace/member/tests/focused.rs".into(),
    ];
    valid_edge_shapes.initial.validation_lanes[0].argv = vec![
        "cargo".into(),
        "test".into(),
        "--manifest-path=workspace/member/Cargo.toml".into(),
        "--test=focused".into(),
    ];
    fs::write(repo.join("design/issue-45.md"), "# Approved design\n").expect("edge-shape design");
    fs::write(
        repo.join("design/issue-45.mmd"),
        "flowchart LR\n  Plan --> Ready\n",
    )
    .expect("edge-shape diagram");
    let valid_edge_request = temp.path().join("valid-edge-shapes-create.json");
    fs::write(
        &valid_edge_request,
        serde_json::to_vec_pretty(&valid_edge_shapes).expect("serialize valid edge request"),
    )
    .expect("valid edge request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &valid_edge_request.to_string_lossy(),
        ],
    ));
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", ".", "--issue", "45"],
    ));
    apply_edit(
        &repo,
        temp.path(),
        45,
        "vpp",
        "package-selected-validator",
        serde_json::json!({
            "operation": "replace_validation_lanes",
            "lanes": [{
                "lane": "package-selected-focused",
                "proof_role": "exercise the issue-owned package validator",
                "acceptance_ids": ["AC-1", "AC-2"],
                "deterministic": true,
                "resource_profile": "small",
                "budget_seconds": 120,
                "budget_tokens": 1000,
                "argv": ["cargo", "test", "-p", "product", "--test", "focused"],
                "parallel_group": "local",
                "defer_reason": null
            }]
        }),
    );
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", ".", "--issue", "45"],
    ));

    fs::create_dir_all(repo.join("validators")).expect("validator directory");
    for (issue, failure_policy, should_pass) in [
        (
            46_u64,
            "Do not under any circumstances ever fail closed when proof is absent.",
            false,
        ),
        (47_u64, "Fail closed when proof is absent.", true),
        (
            48_u64,
            "Fail closed unless the validator is missing.",
            false,
        ),
    ] {
        let design = format!("design/issue-{issue}.md");
        let diagram = format!("design/issue-{issue}.mmd");
        fs::write(repo.join(&design), "# Approved design\n").expect("deferred design");
        fs::write(repo.join(&diagram), "flowchart LR\n  Plan --> Deferred\n")
            .expect("deferred diagram");
        let mut deferred = request();
        deferred.issue = issue;
        deferred.design_path = design.clone();
        deferred.diagram_path = diagram;
        deferred.initial.repo_inputs[0] = design.clone();
        deferred.initial.affected_areas = vec![design, "validators/future-check.sh".into()];
        deferred.initial.deliverables = vec!["validators/future-check.sh".into()];
        deferred.initial.failure_policy = failure_policy.into();
        deferred.initial.validation_lanes[0].argv =
            vec!["bash".into(), "validators/future-check.sh".into()];
        deferred.initial.validation_lanes[0].defer_reason =
            Some("validator is an explicit execution deliverable".into());
        let deferred_request = temp.path().join(format!("deferred-{issue}-create.json"));
        fs::write(
            &deferred_request,
            serde_json::to_vec_pretty(&deferred).expect("serialize deferred request"),
        )
        .expect("deferred request");
        must_succeed(command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &[
                "--root",
                &repo_text,
                "create",
                "--request",
                &deferred_request.to_string_lossy(),
            ],
        ));
        let diagnosis = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-doctor"),
            &["--repo", &repo_text, "--issue", &issue.to_string()],
        );
        assert_eq!(diagnosis.status.success(), should_pass);
        if !should_pass {
            let diagnosis = String::from_utf8(diagnosis.stdout).expect("UTF-8 diagnosis");
            assert!(diagnosis.contains("validator_target_missing"));
            assert!(diagnosis.contains("issue_specific_denominator_missing"));
        }
    }

    let index_path = repo.join(".csdlc/issues/42/index.json");
    let current_index = || -> serde_json::Value {
        serde_json::from_slice(&fs::read(&index_path).expect("issue index"))
            .expect("issue index JSON")
    };
    let planning_edit_path = temp.path().join("initialized-planning-edit.json");
    let index = current_index();
    fs::write(
        &planning_edit_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove initialized planning repair",
            "operation": {
                "operation": "replace_planning_collection",
                "field": "affected_areas",
                "values": ["design/future/deep.rs", "csdlc-v2/tests/gate2.rs"]
            }
        }))
        .expect("serialize initialized planning edit"),
    )
    .expect("initialized planning edit");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &planning_edit_path.to_string_lossy(),
        ],
    ));
    let stale_edit = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &planning_edit_path.to_string_lossy(),
        ],
    );
    assert!(!stale_edit.status.success());

    let before_invalid_contract = fs::read(repo.join(".csdlc/issues/42/index.json"))
        .expect("index before invalid pre-bind contract repair");
    let invalid_contract_path = temp
        .path()
        .join("invalid-initialized-acceptance-repair.json");
    let index = current_index();
    fs::write(
        &invalid_contract_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "stp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "reject changed pre-bind denominator",
            "operation": {
                "operation": "replace_acceptance_criteria",
                "values": ["AC-1: denominator shrink must fail"]
            }
        }))
        .expect("serialize invalid pre-bind contract repair"),
    )
    .expect("invalid pre-bind contract repair");
    let invalid_contract = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &invalid_contract_path.to_string_lossy(),
        ],
    );
    assert!(!invalid_contract.status.success());
    assert_eq!(
        fs::read(repo.join(".csdlc/issues/42/index.json"))
            .expect("index after invalid pre-bind contract repair"),
        before_invalid_contract
    );

    let non_planning_path = temp.path().join("initialized-non-planning-edit.json");
    let index = current_index();
    fs::write(
        &non_planning_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove non-planning edit remains blocked",
            "operation": {
                "operation": "replace_plan_steps",
                "steps": [{
                    "id": "step-1",
                    "action": "must remain blocked before binding",
                    "acceptance_ids": ["AC-1", "AC-2"],
                    "status": "pending"
                }]
            }
        }))
        .expect("serialize non-planning edit"),
    )
    .expect("non-planning edit");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &non_planning_path.to_string_lossy(),
        ],
    ));
    let approve_initialized_path = temp.path().join("approve-initialized-plan-repair.json");
    let index = current_index();
    fs::write(
        &approve_initialized_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "reviewer": "fresh-session:33333333-3333-4333-8333-333333333333"
        }))
        .expect("serialize initialized reapproval"),
    )
    .expect("initialized reapproval");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "approve-design",
            "--request",
            &approve_initialized_path.to_string_lossy(),
        ],
    ));

    let ready_path = temp.path().join("advance-ready.json");
    let index = current_index();
    fs::write(
        &ready_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove ready planning repair",
            "operation": {"operation": "advance_phase", "phase": "ready"}
        }))
        .expect("serialize ready transition"),
    )
    .expect("ready transition");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &ready_path.to_string_lossy(),
        ],
    ));

    let ready_edit_path = temp.path().join("ready-planning-edit.json");
    let index = current_index();
    fs::write(
        &ready_edit_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove ready planning repair",
            "operation": {
                "operation": "replace_planning_collection",
                "field": "invariants",
                "values": ["Git topology remains binding authority"]
            }
        }))
        .expect("serialize ready planning edit"),
    )
    .expect("ready planning edit");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &ready_edit_path.to_string_lossy(),
        ],
    ));
    fs::write(
        repo.join("design/issue-42.md"),
        "# Repaired approved design\n",
    )
    .expect("change design before ready AC repair");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Repair --> Reapprove\n",
    )
    .expect("change diagram before ready AC repair");
    let ready_acceptance_path = temp.path().join("ready-acceptance-repair.json");
    let index = current_index();
    fs::write(
        &ready_acceptance_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "stp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "repair exact ready acceptance contract",
            "operation": {
                "operation": "replace_acceptance_criteria",
                "values": [
                    "AC-1: issue creation is claim-free",
                    "AC-2: binding is atomic and idempotent"
                ]
            }
        }))
        .expect("serialize ready acceptance repair"),
    )
    .expect("ready acceptance repair");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &ready_acceptance_path.to_string_lossy(),
        ],
    ));
    let repaired = current_index();
    assert_eq!(repaired["phase"], "ready");
    assert_eq!(repaired["design_review"], "pending");
    assert!(repaired["branch"].is_null());
    assert!(repaired["worktree"].is_null());
    let approve_ready_path = temp.path().join("approve-ready-contract-repair.json");
    fs::write(
        &approve_ready_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "expected_generation": repaired["generation"],
            "expected_digest": repaired["digest"],
            "reviewer": "fresh-session:44444444-4444-4444-8444-444444444444"
        }))
        .expect("serialize ready reapproval"),
    )
    .expect("ready reapproval");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "approve-design",
            "--request",
            &approve_ready_path.to_string_lossy(),
        ],
    ));
    let ready_diagnosis = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));
    assert!(ready_diagnosis.contains("\"status\": \"pass\""));
    assert!(ready_diagnosis.contains("\"phase\": \"ready\""));

    let duplicate_request = temp.path().join("duplicate-create.json");
    let mut duplicate = request();
    duplicate.design_path = "generated/duplicate-design.md".into();
    duplicate.diagram_path = "generated/duplicate-diagram.mmd".into();
    fs::write(
        &duplicate_request,
        serde_json::to_vec_pretty(&duplicate).expect("serialize duplicate request"),
    )
    .expect("duplicate request");
    let duplicate_text = duplicate_request.to_string_lossy();
    let duplicate_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &duplicate_text],
    );
    assert!(!duplicate_result.status.success());
    assert!(!repo.join("generated/duplicate-design.md").exists());
    assert!(!repo.join("generated/duplicate-diagram.mmd").exists());
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n")
        .expect("restore committed design after duplicate-create rollback proof");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Create --> Bind\n",
    )
    .expect("restore committed diagram after duplicate-create rollback proof");
    let restore_binding_path = temp.path().join("restore-ready-design-binding.json");
    let index = current_index();
    fs::write(
        &restore_binding_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "stp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "restore reviewed committed design binding",
            "operation": {
                "operation": "replace_acceptance_criteria",
                "values": [
                    "AC-1: issue creation is claim-free",
                    "AC-2: binding is atomic and idempotent"
                ]
            }
        }))
        .expect("serialize restored binding repair"),
    )
    .expect("restored binding repair");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &restore_binding_path.to_string_lossy(),
        ],
    ));
    let restore_approval_path = temp.path().join("approve-restored-ready-binding.json");
    let index = current_index();
    fs::write(
        &restore_approval_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "reviewer": "fresh-session:55555555-5555-4555-8555-555555555555"
        }))
        .expect("serialize restored binding approval"),
    )
    .expect("restored binding approval");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "approve-design",
            "--request",
            &restore_approval_path.to_string_lossy(),
        ],
    ));
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));

    let bind_request = temp.path().join("bind.json");
    let bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42",
        "worktree": worktree,
        "claim": {"id": "ignored-legacy-bind-claim"},
    });
    fs::write(
        &bind_request,
        serde_json::to_vec_pretty(&bind).expect("serialize bind request"),
    )
    .expect("bind request");
    let bind_text = bind_request.to_string_lossy();

    let legacy_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!legacy_bind.status.success());
    fs::write(
        &bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "issue-42",
            "worktree": worktree,
        }))
        .expect("serialize claim-free bind request"),
    )
    .expect("claim-free bind request");

    fs::write(repo.join("design/issue-42.md"), "# Stale design\n").expect("stale design");
    let invalid_validation = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    );
    assert!(!invalid_validation.status.success());
    let invalid_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!invalid_bind.status.success());
    assert!(!worktree.exists());
    assert!(!git(&repo, &["branch", "--list", "issue-42"]).contains("issue-42"));
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("restore design");

    let existing = temp.path().join("worktrees/existing");
    git(
        &repo,
        &[
            "worktree",
            "add",
            "-b",
            "issue-42-existing",
            &existing.to_string_lossy(),
            "main",
        ],
    );
    fs::create_dir_all(existing.join(".csdlc/locks/42.lock")).expect("invalid target lock");
    let existing_request = temp.path().join("existing-bind.json");
    fs::write(
        &existing_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "issue-42-existing",
            "worktree": existing,
        }))
        .expect("serialize existing bind request"),
    )
    .expect("existing bind request");
    let existing_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &existing_request.to_string_lossy(),
        ],
    );
    assert!(!existing_result.status.success());
    assert!(!existing.join(".csdlc/issues/42").exists());
    git(
        &repo,
        &["worktree", "remove", "--force", &existing.to_string_lossy()],
    );
    git(&repo, &["branch", "-D", "issue-42-existing"]);

    let first = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(first.contains("\"created\":true"));
    let topology = git(&repo, &["worktree", "list", "--porcelain"]);
    assert!(topology.contains(&format!(
        "worktree {}",
        worktree.canonicalize().unwrap().display()
    )));
    assert!(topology.contains("branch refs/heads/issue-42"));

    let second = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(second.contains("\"created\":false"));

    let index = worktree.join(".csdlc/issues/42/index.json");
    let original_index = fs::read(&index).expect("bound index");
    let mut unsigned: serde_json::Value =
        serde_json::from_slice(&original_index).expect("index JSON");
    unsigned["unsigned_topology"] = serde_json::json!("must fail");
    fs::write(
        &index,
        serde_json::to_vec_pretty(&unsigned).expect("serialize unsigned index"),
    )
    .expect("unsigned index");
    let unsigned_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!unsigned_result.status.success());
    fs::write(&index, &original_index).expect("restore bound index");

    let audit = worktree.join(".csdlc/issues/42/audit.jsonl");
    let original_audit = fs::read(&audit).expect("bound audit");
    let mut forged_audit = original_audit.clone();
    forged_audit.extend_from_slice(b"{}\n");
    fs::write(&audit, forged_audit).expect("forged audit");
    let audit_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!audit_result.status.success());
    fs::write(&audit, original_audit).expect("restore bound audit");

    let mut contradictory: serde_json::Value =
        serde_json::from_slice(&fs::read(&index).expect("bound index")).expect("index JSON");
    contradictory["branch"] = serde_json::json!("different-branch");
    fs::write(
        &index,
        serde_json::to_vec_pretty(&contradictory).expect("serialize contradictory index"),
    )
    .expect("contradictory index");
    let inconsistent = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!inconsistent.status.success());

    let conflict_request = temp.path().join("conflict.json");
    let conflict_bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42-conflict",
        "worktree": conflict,
    });
    fs::write(
        &conflict_request,
        serde_json::to_vec_pretty(&conflict_bind).expect("serialize conflict request"),
    )
    .expect("conflict request");
    let conflict_text = conflict_request.to_string_lossy();
    let rejected = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &conflict_text],
    );
    assert!(!rejected.status.success());
    let rejection = String::from_utf8_lossy(&rejected.stdout);
    assert!(rejection.contains("reconciliation_required") || rejection.contains("corrupt_record"));

    git(
        &repo,
        &["worktree", "remove", "--force", &worktree.to_string_lossy()],
    );
}

#[test]
fn prebind_operator_constraints_correction_is_exact_and_fail_closed() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    focused_fixture_repo(&repo);
    let store = Store::new(&repo);
    let mut record = csdlc_v2::initialize_native_json(
        &store,
        &serde_json::to_vec(&request()).expect("serialize bootstrap"),
    )
    .expect("bootstrap initialized issue");
    let initial_snapshot = issue_projection_snapshot(&repo, 42);
    let replacement = vec!["use only current typed v2 binaries".into()];
    let operation = SemanticOperation::CorrectOperatorConstraintsBeforeBind {
        values: replacement.clone(),
    };

    for (actor, reason) in [("", "reason"), ("actor", " ")] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 42,
                card: CardKind::Sip,
                expected_generation: record.generation,
                expected_digest: record.digest.clone(),
                actor: actor.into(),
                reason: reason.into(),
                operation: operation.clone(),
                fail_after_backup: false,
            },
        )
        .expect_err("empty actor or reason must fail");
        assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
        assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);
    }

    let wrong_card = direct_edit(&store, &record, CardKind::Stp, operation.clone(), false)
        .expect_err("wrong card must fail");
    assert_eq!(wrong_card.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);

    for invalid in [Vec::<String>::new(), vec![" ".into()]] {
        let error = direct_edit(
            &store,
            &record,
            CardKind::Sip,
            SemanticOperation::CorrectOperatorConstraintsBeforeBind { values: invalid },
            false,
        )
        .expect_err("empty constraints must fail");
        assert_eq!(error.code, csdlc_v2::ErrorCode::CardInvalid);
        assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);
    }
    for (generation, digest, expected) in [
        (
            record.generation + 1,
            record.digest.clone(),
            csdlc_v2::ErrorCode::StaleGeneration,
        ),
        (
            record.generation,
            "0".repeat(64),
            csdlc_v2::ErrorCode::StaleDigest,
        ),
    ] {
        let error = edit_issue(
            &store,
            EditRequest {
                issue: 42,
                card: CardKind::Sip,
                expected_generation: generation,
                expected_digest: digest,
                actor: "operator".into(),
                reason: "prove stale CAS rejection".into(),
                operation: operation.clone(),
                fail_after_backup: false,
            },
        )
        .expect_err("stale CAS must fail");
        assert_eq!(error.code, expected);
        assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);
    }

    let original_design = fs::read(repo.join("design/issue-42.md")).expect("original design");
    fs::write(repo.join("design/issue-42.md"), "# unreviewed drift\n").expect("drift design");
    let drift = direct_edit(&store, &record, CardKind::Sip, operation.clone(), false)
        .expect_err("authored drift must fail");
    assert_eq!(drift.code, csdlc_v2::ErrorCode::CardInvalid);
    assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);
    fs::write(repo.join("design/issue-42.md"), original_design).expect("restore design");
    let original_diagram = fs::read(repo.join("design/issue-42.mmd")).expect("original diagram");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Drift --> Reject\n",
    )
    .expect("drift diagram");
    let drift = direct_edit(&store, &record, CardKind::Sip, operation.clone(), false)
        .expect_err("authored diagram drift must fail");
    assert_eq!(drift.code, csdlc_v2::ErrorCode::CardInvalid);
    assert_eq!(issue_projection_snapshot(&repo, 42), initial_snapshot);
    fs::write(repo.join("design/issue-42.mmd"), original_diagram).expect("restore diagram");

    let mut retained_record = record.clone();
    let mut retained_cards = store.load_cards(42).expect("retained truth cards");
    let CardContent::Sor(sor) = &mut retained_cards.get_mut(&CardKind::Sor).expect("SOR").content
    else {
        panic!("SOR")
    };
    sor.integration_state = csdlc_v2::cards::IntegrationState::WorktreeOnly;
    write_consistent_card(
        &repo,
        &mut retained_record,
        CardKind::Sor,
        &retained_cards[&CardKind::Sor],
    );
    let retained_snapshot = issue_projection_snapshot(&repo, 42);
    let retained = direct_edit(
        &store,
        &retained_record,
        CardKind::Sip,
        operation.clone(),
        false,
    )
    .expect_err("retained SOR truth must fail");
    assert_eq!(retained.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(issue_projection_snapshot(&repo, 42), retained_snapshot);
    restore_issue_projection(&repo, 42, &initial_snapshot);

    let clean_record = record.clone();
    record.migration = Some(csdlc_v2::MigrationEvidence {
        schema: "csdlc.migration.v1".into(),
        imported_unix_seconds: 1,
        sunset_unix_seconds: 2,
        source_digest: "migration-source".into(),
        authored_sources: BTreeMap::new(),
        authored_sections: BTreeMap::new(),
        compatibility_view: "retained".into(),
    });
    write_consistent_record(&repo, &mut record);
    let migrated_snapshot = issue_projection_snapshot(&repo, 42);
    let migration = direct_edit(&store, &record, CardKind::Sip, operation.clone(), false)
        .expect_err("migration evidence must fail");
    assert_eq!(migration.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(issue_projection_snapshot(&repo, 42), migrated_snapshot);
    restore_issue_projection(&repo, 42, &initial_snapshot);
    record = clean_record;

    let before_cards = store.load_cards(42).expect("cards before correction");
    let (design_digest, diagram_digest) = spp_design_digests(&before_cards);
    let interrupted = direct_edit(&store, &record, CardKind::Sip, operation.clone(), true)
        .expect_err("injected interruption must fail");
    assert_eq!(
        interrupted.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    let corrected = direct_edit(&store, &record, CardKind::Sip, operation.clone(), false)
        .expect("correct constraints after transaction recovery");
    assert_single_generation_preserves_lifecycle_shell(&record, &corrected);
    assert!(matches!(
        corrected.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    let after_cards = store.load_cards(42).expect("cards after correction");
    let (after_design_digest, after_diagram_digest) = spp_design_digests(&after_cards);
    assert_eq!(after_design_digest, design_digest);
    assert_eq!(after_diagram_digest, diagram_digest);
    for kind in [
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(&before_cards[&kind], &after_cards[&kind]);
    }
    let CardContent::Sip(after_sip) = &after_cards[&CardKind::Sip].content else {
        panic!("SIP")
    };
    assert_eq!(after_sip.operator_constraints, replacement);
    let audit: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("structured correction audit");
    assert_eq!(
        audit,
        serde_json::json!({
            "operation": "correct_operator_constraints_before_bind",
            "previous_values": ["no network"],
            "new_values": ["use only current typed v2 binaries"],
        })
    );

    for bind_before_correction in [false, true] {
        let temp = tempfile::tempdir().expect("ready tempdir");
        let repo = temp.path().join("repo");
        focused_fixture_repo(&repo);
        let store = Store::new(&repo);
        let initialized = csdlc_v2::initialize_native_json(
            &store,
            &serde_json::to_vec(&request()).expect("serialize ready bootstrap"),
        )
        .expect("bootstrap ready fixture");
        let ready = edit_issue(
            &store,
            EditRequest {
                issue: 42,
                card: CardKind::Sip,
                expected_generation: initialized.generation,
                expected_digest: initialized.digest,
                actor: "operator".into(),
                reason: "advance ready fixture".into(),
                operation: SemanticOperation::AdvancePhase {
                    phase: LifecyclePhase::Ready,
                },
                fail_after_backup: false,
            },
        )
        .expect("advance ready fixture");
        let (store, current) = if bind_before_correction {
            let worktree = temp.path().join("worktrees/issue-42");
            bind_issue(
                &store,
                BindRequest {
                    issue: 42,
                    base_branch: "main".into(),
                    branch: "issue-42".into(),
                    worktree: worktree.to_string_lossy().into_owned(),
                    code_repository: None,
                },
            )
            .expect("bind rejection fixture");
            let bound_store = Store::new(&worktree);
            let current = bound_store.load_record(42).expect("bound rejection record");
            (bound_store, current)
        } else {
            (store, ready)
        };
        let result = direct_edit(&store, &current, CardKind::Sip, operation.clone(), false);
        if bind_before_correction {
            assert_eq!(
                result.expect_err("bound correction must fail").code,
                csdlc_v2::ErrorCode::InvalidTransition
            );
            let executed = edit_issue(
                &store,
                EditRequest {
                    issue: 42,
                    card: CardKind::Sor,
                    expected_generation: current.generation,
                    expected_digest: current.digest,
                    actor: "operator".into(),
                    reason: "create later-phase rejection fixture".into(),
                    operation: SemanticOperation::RecordExecution {
                        summary: "implemented fixture".into(),
                        changes: vec!["csdlc-v2/tests/gate2.rs".into()],
                        artifacts: vec!["fixture".into()],
                    },
                    fail_after_backup: false,
                },
            )
            .expect("record later-phase execution");
            let implemented = edit_issue(
                &store,
                EditRequest {
                    issue: 42,
                    card: CardKind::Sor,
                    expected_generation: executed.generation,
                    expected_digest: executed.digest,
                    actor: "operator".into(),
                    reason: "advance later-phase fixture".into(),
                    operation: SemanticOperation::AdvancePhase {
                        phase: LifecyclePhase::Implemented,
                    },
                    fail_after_backup: false,
                },
            )
            .expect("advance later-phase fixture");
            assert_eq!(
                direct_edit(
                    &store,
                    &implemented,
                    CardKind::Sip,
                    operation.clone(),
                    false,
                )
                .expect_err("implemented correction must fail")
                .code,
                csdlc_v2::ErrorCode::InvalidTransition
            );
        } else {
            let corrected = result.expect("ready correction must succeed");
            let approved = csdlc_v2::store::approve_design(
                &store,
                csdlc_v2::store::ApproveDesignRequest {
                    issue: 42,
                    expected_generation: corrected.generation,
                    expected_digest: corrected.digest,
                    reviewer: "fresh-session:66666666-6666-4666-8666-666666666666".into(),
                },
            )
            .expect("reapprove corrected ready design");
            assert!(matches!(
                approved.design_review,
                csdlc_v2::DesignReview::Approved { .. }
            ));
        }
    }
}

#[test]
fn prebind_contract_repair_is_exact_atomic_and_fail_closed() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    focused_fixture_repo(&repo);
    let store = Store::new(&repo);
    let mut record = csdlc_v2::initialize_native_json(
        &store,
        &serde_json::to_vec(&request()).expect("serialize bootstrap"),
    )
    .expect("bootstrap initialized issue");

    let exact_acceptance = || SemanticOperation::ReplaceAcceptanceCriteria {
        values: vec![
            "AC-1: issue creation is claim-free".into(),
            "AC-2: binding is atomic and idempotent".into(),
        ],
    };
    let plan = |action: &str, acceptance_ids: Vec<&str>, status: StepStatus| {
        SemanticOperation::ReplacePlanSteps {
            steps: vec![PlanStep {
                id: "S1".into(),
                action: action.into(),
                acceptance_ids: acceptance_ids.into_iter().map(str::to_owned).collect(),
                status,
            }],
        }
    };
    for (name, values) in [
        (
            "malformed",
            vec![
                "issue creation is claim-free",
                "AC-2: binding remains exact",
            ],
        ),
        (
            "reordered",
            vec![
                "AC-2: binding remains exact",
                "AC-1: creation remains exact",
            ],
        ),
        (
            "renumbered",
            vec![
                "AC-1: creation remains exact",
                "AC-3: binding remains exact",
            ],
        ),
    ] {
        let before = issue_projection_snapshot(&repo, 42);
        let error = direct_edit(
            &store,
            &record,
            CardKind::Stp,
            SemanticOperation::ReplaceAcceptanceCriteria {
                values: values.into_iter().map(str::to_owned).collect(),
            },
            false,
        )
        .expect_err(name);
        assert_eq!(error.code, csdlc_v2::ErrorCode::CardInvalid);
        assert_eq!(issue_projection_snapshot(&repo, 42), before);
    }

    let initialized_before = record.clone();
    let initialized_cards_before = store
        .load_cards(42)
        .expect("initialized cards before repair");
    fs::write(
        repo.join("design/issue-42.md"),
        "# Independently repaired design\n",
    )
    .expect("repair design");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Repair --> Review\n",
    )
    .expect("repair diagram");
    let authored_design_before_initialized_stp =
        fs::read(repo.join("design/issue-42.md")).expect("design before initialized STP");
    let authored_diagram_before_initialized_stp =
        fs::read(repo.join("design/issue-42.mmd")).expect("diagram before initialized STP");
    let audit_before_acceptance =
        fs::read(repo.join(".csdlc/issues/42/audit.jsonl")).expect("initial audit");
    let (old_design_digest, old_diagram_digest) = spp_design_digests(&initialized_cards_before);
    let initialized_operation = exact_acceptance();
    record = direct_edit(
        &store,
        &record,
        CardKind::Stp,
        initialized_operation.clone(),
        false,
    )
    .expect("initialized acceptance repair");
    assert_single_generation_preserves_lifecycle_shell(&initialized_before, &record);
    assert_eq!(record.phase, LifecyclePhase::Initialized);
    assert!(matches!(
        record.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    let audit_after_acceptance =
        fs::read(repo.join(".csdlc/issues/42/audit.jsonl")).expect("repaired audit");
    assert!(audit_after_acceptance.starts_with(&audit_before_acceptance));
    let initialized_cards_after = store
        .load_cards(42)
        .expect("initialized cards after repair");
    for kind in [CardKind::Sip, CardKind::Srp, CardKind::Sor] {
        assert_card_semantics_unchanged(
            &initialized_cards_before[&kind],
            &initialized_cards_after[&kind],
        );
    }
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_eq!(
            initialized_cards_after[&kind].status,
            initialized_cards_before[&kind].status
        );
    }
    for kind in [CardKind::Stp, CardKind::Spp, CardKind::Vpp] {
        assert_card_identity_advanced(
            &initialized_cards_before[&kind],
            &initialized_cards_after[&kind],
        );
    }
    let mut expected_initialized_stp = match &initialized_cards_before[&CardKind::Stp].content {
        CardContent::Stp(values) => values.clone(),
        _ => unreachable!("STP"),
    };
    let SemanticOperation::ReplaceAcceptanceCriteria { values } = &initialized_operation else {
        unreachable!("initialized STP operation")
    };
    expected_initialized_stp.acceptance_criteria = values.clone();
    match &initialized_cards_after[&CardKind::Stp].content {
        CardContent::Stp(values) => assert_eq!(values, &expected_initialized_stp),
        _ => unreachable!("STP"),
    }
    assert_eq!(
        fs::read(repo.join("design/issue-42.md")).expect("design after initialized STP"),
        authored_design_before_initialized_stp
    );
    assert_eq!(
        fs::read(repo.join("design/issue-42.mmd")).expect("diagram after initialized STP"),
        authored_diagram_before_initialized_stp
    );
    assert_design_bindings_match_authored(&repo, &initialized_cards_after);
    let (new_design_digest, new_diagram_digest) = spp_design_digests(&initialized_cards_after);
    assert_ne!(new_design_digest, old_design_digest);
    assert_ne!(new_diagram_digest, old_diagram_digest);
    assert_last_prebind_audit_operation(
        &record,
        &initialized_operation,
        &old_design_digest,
        &new_design_digest,
        &old_diagram_digest,
        &new_diagram_digest,
    );
    let clean_initialized_repair = issue_projection_snapshot(&repo, 42);
    for (name, invalid_design_ref) in [
        ("absolute", "/tmp/outside-design.md"),
        ("traversal", "../design/issue-42.md"),
        ("empty", ""),
        ("drive-relative", "C:relative.md"),
        ("backslash", "design\\issue-42.md"),
        ("dot-segment", "design/./issue-42.md"),
        ("trailing-separator", "design/issue-42.md/"),
        ("noncanonical", "design//issue-42.md"),
    ] {
        let mut invalid_record = record.clone();
        write_consistent_design_reference(&repo, &mut invalid_record, invalid_design_ref);
        let injected = issue_projection_snapshot(&repo, 42);

        let repair_error = direct_edit(
            &store,
            &invalid_record,
            CardKind::Stp,
            initialized_operation.clone(),
            false,
        )
        .expect_err(name);
        assert_eq!(repair_error.code, csdlc_v2::ErrorCode::CorruptRecord);
        assert_eq!(
            repair_error.message,
            "authored artifact path must be nonempty, clean, canonical, and repository-relative"
        );
        assert_eq!(issue_projection_snapshot(&repo, 42), injected);

        let approval_error = csdlc_v2::store::approve_design(
            &store,
            csdlc_v2::store::ApproveDesignRequest {
                issue: 42,
                expected_generation: invalid_record.generation,
                expected_digest: invalid_record.digest.clone(),
                reviewer: "fresh-session:77777777-7777-4777-8777-777777777777".into(),
            },
        )
        .expect_err(name);
        assert_eq!(approval_error.code, csdlc_v2::ErrorCode::CorruptRecord);
        assert_eq!(
            approval_error.message,
            "authored artifact path must be nonempty, clean, canonical, and repository-relative"
        );
        assert_eq!(issue_projection_snapshot(&repo, 42), injected);

        let diagnosis = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-doctor"),
            &["--repo", &repo.to_string_lossy(), "--issue", "42"],
        );
        assert!(!diagnosis.status.success(), "{name} doctor succeeded");
        let diagnosis_stdout = String::from_utf8_lossy(&diagnosis.stdout);
        assert!(
            diagnosis_stdout.contains("corrupt_record"),
            "{name}: {diagnosis_stdout}"
        );
        assert!(
            diagnosis_stdout.contains(
                "authored artifact path must be nonempty, clean, canonical, and repository-relative"
            ),
            "{name}: {diagnosis_stdout}"
        );
        assert_eq!(issue_projection_snapshot(&repo, 42), injected);

        let validation = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-validate"),
            &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
        );
        assert!(!validation.status.success(), "{name} validation succeeded");
        let validation_stdout = String::from_utf8_lossy(&validation.stdout);
        assert!(
            validation_stdout.contains("corrupt_record"),
            "{name}: {validation_stdout}"
        );
        assert!(
            validation_stdout.contains(
                "authored artifact path must be nonempty, clean, canonical, and repository-relative"
            ),
            "{name}: {validation_stdout}"
        );
        assert_eq!(issue_projection_snapshot(&repo, 42), injected);
        restore_issue_projection(&repo, 42, &clean_initialized_repair);
    }
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        assert!(repo
            .join(format!(".csdlc/issues/42/cards/{card}.values.json"))
            .is_file());
        assert!(repo
            .join(format!(".csdlc/issues/42/cards/{card}.md"))
            .is_file());
    }
    assert_only_pending_design_review_diagnostic(&repo, 42);

    let initialized_spp_before = record.clone();
    let initialized_spp_cards_before = store
        .load_cards(42)
        .expect("initialized cards before SPP repair");
    let (old_design_digest, old_diagram_digest) = spp_design_digests(&initialized_spp_cards_before);
    let initialized_spp_operation = plan(
        "exercise exact initialized plan repair",
        vec!["AC-1", "AC-2"],
        StepStatus::Pending,
    );
    record = direct_edit(
        &store,
        &record,
        CardKind::Spp,
        initialized_spp_operation.clone(),
        false,
    )
    .expect("initialized SPP repair");
    assert_single_generation_preserves_lifecycle_shell(&initialized_spp_before, &record);
    assert!(matches!(
        record.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    let initialized_spp_cards_after = store
        .load_cards(42)
        .expect("initialized cards after SPP repair");
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(
            &initialized_spp_cards_before[&kind],
            &initialized_spp_cards_after[&kind],
        );
    }
    assert_card_identity_advanced(
        &initialized_spp_cards_before[&CardKind::Spp],
        &initialized_spp_cards_after[&CardKind::Spp],
    );
    let mut expected_initialized_spp = match &initialized_spp_cards_before[&CardKind::Spp].content {
        CardContent::Spp(values) => values.clone(),
        _ => unreachable!("SPP"),
    };
    let SemanticOperation::ReplacePlanSteps { steps } = &initialized_spp_operation else {
        unreachable!("initialized SPP operation")
    };
    expected_initialized_spp.plan_revision += 1;
    expected_initialized_spp.steps = steps.clone();
    match &initialized_spp_cards_after[&CardKind::Spp].content {
        CardContent::Spp(values) => assert_eq!(values, &expected_initialized_spp),
        _ => unreachable!("SPP"),
    }
    assert_design_bindings_match_authored(&repo, &initialized_spp_cards_after);
    let (new_design_digest, new_diagram_digest) = spp_design_digests(&initialized_spp_cards_after);
    assert_last_prebind_audit_operation(
        &record,
        &initialized_spp_operation,
        &old_design_digest,
        &new_design_digest,
        &old_diagram_digest,
        &new_diagram_digest,
    );
    assert_only_pending_design_review_diagnostic(&repo, 42);

    let wrong_card_before = issue_projection_snapshot(&repo, 42);
    assert!(direct_edit(&store, &record, CardKind::Spp, exact_acceptance(), false,).is_err());
    assert_eq!(issue_projection_snapshot(&repo, 42), wrong_card_before);

    let index_path = repo.join(".csdlc/issues/42/index.json");
    let clean_initialized = issue_projection_snapshot(&repo, 42);
    let mut later_evidence = record.clone();
    later_evidence.review_assignment = Some(csdlc_v2::ReviewAssignment {
        reviewer: "too-early".into(),
        assigned_by: "test-operator".into(),
        revision: "0".repeat(40),
        scope: vec!["later evidence".into()],
    });
    write_consistent_record(&repo, &mut later_evidence);
    let injected_later_evidence = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(
        &store,
        &later_evidence,
        CardKind::Stp,
        exact_acceptance(),
        false,
    )
    .expect_err("later evidence guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(
        error.message,
        "pre-bind contract repair requires unbound topology and no later lifecycle evidence"
    );
    assert_eq!(
        issue_projection_snapshot(&repo, 42),
        injected_later_evidence
    );
    restore_issue_projection(&repo, 42, &clean_initialized);

    let mut reference_drift = record.clone();
    let mut drifted_spp =
        store.load_cards(42).expect("cards for reference drift")[&CardKind::Spp].clone();
    match &mut drifted_spp.content {
        CardContent::Spp(values) => values.design_ref = "design/wrong.md".into(),
        _ => unreachable!("SPP"),
    }
    write_consistent_card(&repo, &mut reference_drift, CardKind::Spp, &drifted_spp);
    let injected_reference_drift = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(
        &store,
        &reference_drift,
        CardKind::Stp,
        exact_acceptance(),
        false,
    )
    .expect_err("reference drift guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CardInvalid);
    assert_eq!(
        error.message,
        "pre-bind repair design/diagram references disagree with issue authority"
    );
    assert_eq!(
        issue_projection_snapshot(&repo, 42),
        injected_reference_drift
    );
    restore_issue_projection(&repo, 42, &clean_initialized);

    let design_path = repo.join("design/issue-42.md");
    let design_bytes = fs::read(&design_path).expect("design bytes");
    fs::remove_file(&design_path).expect("remove design for path drift");
    fs::create_dir(&design_path).expect("replace design with directory");
    let path_drift_projection = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(&store, &record, CardKind::Stp, exact_acceptance(), false)
        .expect_err("authored path drift guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
    assert_eq!(
        error.message,
        "authored artifact target must be a regular single-link file: design/issue-42.md"
    );
    assert_eq!(issue_projection_snapshot(&repo, 42), path_drift_projection);
    fs::remove_dir(&design_path).expect("remove drifted design directory");
    fs::write(&design_path, &design_bytes).expect("restore design");

    let mut wrong_identity = record.clone();
    wrong_identity.issue = 43;
    wrong_identity.digest.clear();
    wrong_identity.digest =
        digest(&serde_json::to_vec(&wrong_identity).expect("wrong identity digest serialization"));
    let mut wrong_identity_bytes =
        serde_json::to_vec_pretty(&wrong_identity).expect("wrong identity projection");
    wrong_identity_bytes.push(b'\n');
    fs::write(&index_path, wrong_identity_bytes).expect("inject identity drift");
    let injected_identity_drift = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(&store, &record, CardKind::Stp, exact_acceptance(), false)
        .expect_err("record identity guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::CorruptRecord);
    assert_eq!(
        error.message,
        "issue projection namespace mismatch: requested 42, embedded 43"
    );
    assert_eq!(
        issue_projection_snapshot(&repo, 42),
        injected_identity_drift
    );
    restore_issue_projection(&repo, 42, &clean_initialized);

    let approval = csdlc_v2::store::approve_design(
        &store,
        csdlc_v2::store::ApproveDesignRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            reviewer: "fresh-session:33333333-3333-4333-8333-333333333333".into(),
        },
    )
    .expect("reapprove initialized repair");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    ));
    record = direct_edit(
        &store,
        &approval,
        CardKind::Spp,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Ready,
        },
        false,
    )
    .expect("advance repaired issue to ready");

    let ready_stp_before = record.clone();
    let ready_stp_cards_before = store.load_cards(42).expect("ready cards before STP repair");
    let (old_design_digest, old_diagram_digest) = spp_design_digests(&ready_stp_cards_before);
    let ready_acceptance = vec![
        "AC-1: ready acceptance repair preserves unbound topology".to_owned(),
        "AC-2: ready acceptance repair requires reapproval".to_owned(),
    ];
    let ready_stp_operation = SemanticOperation::ReplaceAcceptanceCriteria {
        values: ready_acceptance.clone(),
    };
    record = direct_edit(
        &store,
        &record,
        CardKind::Stp,
        ready_stp_operation.clone(),
        false,
    )
    .expect("ready STP repair");
    assert_single_generation_preserves_lifecycle_shell(&ready_stp_before, &record);
    assert!(matches!(
        record.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    let ready_stp_cards_after = store.load_cards(42).expect("ready cards after STP repair");
    for kind in [
        CardKind::Sip,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(
            &ready_stp_cards_before[&kind],
            &ready_stp_cards_after[&kind],
        );
    }
    assert_card_identity_advanced(
        &ready_stp_cards_before[&CardKind::Stp],
        &ready_stp_cards_after[&CardKind::Stp],
    );
    let mut expected_ready_stp = match &ready_stp_cards_before[&CardKind::Stp].content {
        CardContent::Stp(values) => values.clone(),
        _ => unreachable!("STP"),
    };
    expected_ready_stp.acceptance_criteria = ready_acceptance;
    match &ready_stp_cards_after[&CardKind::Stp].content {
        CardContent::Stp(values) => assert_eq!(values, &expected_ready_stp),
        _ => unreachable!("STP"),
    }
    assert_design_bindings_match_authored(&repo, &ready_stp_cards_after);
    let (new_design_digest, new_diagram_digest) = spp_design_digests(&ready_stp_cards_after);
    assert_last_prebind_audit_operation(
        &record,
        &ready_stp_operation,
        &old_design_digest,
        &new_design_digest,
        &old_diagram_digest,
        &new_diagram_digest,
    );
    assert_only_pending_design_review_diagnostic(&repo, 42);

    for (name, operation) in [
        (
            "nonpending",
            plan(
                "reject nonpending ready plan",
                vec!["AC-1", "AC-2"],
                StepStatus::Completed,
            ),
        ),
        (
            "duplicate",
            plan(
                "reject duplicate ready coverage",
                vec!["AC-1", "AC-1", "AC-2"],
                StepStatus::Pending,
            ),
        ),
        (
            "missing",
            plan(
                "reject missing ready coverage",
                vec!["AC-1"],
                StepStatus::Pending,
            ),
        ),
        (
            "extra",
            plan(
                "reject extra ready coverage",
                vec!["AC-1", "AC-2", "AC-3"],
                StepStatus::Pending,
            ),
        ),
    ] {
        let before = issue_projection_snapshot(&repo, 42);
        let error = direct_edit(&store, &record, CardKind::Spp, operation, false).expect_err(name);
        assert_eq!(error.code, csdlc_v2::ErrorCode::CardInvalid);
        assert_eq!(issue_projection_snapshot(&repo, 42), before);
    }

    let audit_before_plan =
        fs::read(repo.join(".csdlc/issues/42/audit.jsonl")).expect("audit before plan");
    let ready_before = record.clone();
    let ready_cards_before = store
        .load_cards(42)
        .expect("ready cards before plan repair");
    let authored_design_before_ready_spp =
        fs::read(repo.join("design/issue-42.md")).expect("design before ready SPP");
    let authored_diagram_before_ready_spp =
        fs::read(repo.join("design/issue-42.mmd")).expect("diagram before ready SPP");
    let (old_design_digest, old_diagram_digest) = spp_design_digests(&ready_cards_before);
    let ready_operation = plan(
        "exercise exact ready plan repair",
        vec!["AC-1", "AC-2"],
        StepStatus::Pending,
    );
    record = direct_edit(
        &store,
        &record,
        CardKind::Spp,
        ready_operation.clone(),
        false,
    )
    .expect("ready plan repair");
    assert_single_generation_preserves_lifecycle_shell(&ready_before, &record);
    assert_eq!(record.phase, LifecyclePhase::Ready);
    assert!(matches!(
        record.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    let audit_after_plan =
        fs::read(repo.join(".csdlc/issues/42/audit.jsonl")).expect("audit after plan");
    assert!(audit_after_plan.starts_with(&audit_before_plan));
    let ready_cards_after = store.load_cards(42).expect("ready cards after plan repair");
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(&ready_cards_before[&kind], &ready_cards_after[&kind]);
    }
    assert_card_identity_advanced(
        &ready_cards_before[&CardKind::Spp],
        &ready_cards_after[&CardKind::Spp],
    );
    let mut expected_ready_spp = match &ready_cards_before[&CardKind::Spp].content {
        CardContent::Spp(values) => values.clone(),
        _ => unreachable!("SPP"),
    };
    let SemanticOperation::ReplacePlanSteps { steps } = &ready_operation else {
        unreachable!("ready SPP operation")
    };
    expected_ready_spp.plan_revision += 1;
    expected_ready_spp.steps = steps.clone();
    match &ready_cards_after[&CardKind::Spp].content {
        CardContent::Spp(values) => assert_eq!(values, &expected_ready_spp),
        _ => unreachable!("SPP"),
    }
    assert_eq!(
        fs::read(repo.join("design/issue-42.md")).expect("design after ready SPP"),
        authored_design_before_ready_spp
    );
    assert_eq!(
        fs::read(repo.join("design/issue-42.mmd")).expect("diagram after ready SPP"),
        authored_diagram_before_ready_spp
    );
    assert_design_bindings_match_authored(&repo, &ready_cards_after);
    let (new_design_digest, new_diagram_digest) = spp_design_digests(&ready_cards_after);
    assert_last_prebind_audit_operation(
        &record,
        &ready_operation,
        &old_design_digest,
        &new_design_digest,
        &old_diagram_digest,
        &new_diagram_digest,
    );
    assert_only_pending_design_review_diagnostic(&repo, 42);
    let clean_ready = issue_projection_snapshot(&repo, 42);
    let mut ready_with_topology = record.clone();
    ready_with_topology.branch = Some("premature-ready-branch".into());
    ready_with_topology.worktree = Some("/tmp/premature-ready-worktree".into());
    write_consistent_record(&repo, &mut ready_with_topology);
    let injected_ready_topology = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(
        &store,
        &ready_with_topology,
        CardKind::Spp,
        plan(
            "reject ready topology evidence",
            vec!["AC-1", "AC-2"],
            StepStatus::Pending,
        ),
        false,
    )
    .expect_err("ready topology guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(
        error.message,
        "pre-bind contract repair requires unbound topology and no later lifecycle evidence"
    );
    assert_eq!(
        issue_projection_snapshot(&repo, 42),
        injected_ready_topology
    );
    restore_issue_projection(&repo, 42, &clean_ready);
    let audit_len_before_ready_approval = record.audit.len();
    record = csdlc_v2::store::approve_design(
        &store,
        csdlc_v2::store::ApproveDesignRequest {
            issue: 42,
            expected_generation: record.generation,
            expected_digest: record.digest.clone(),
            reviewer: "fresh-session:44444444-4444-4444-8444-444444444444".into(),
        },
    )
    .expect("reapprove ready plan repair");
    assert_eq!(record.audit.len(), audit_len_before_ready_approval + 1);
    let ready_approval_event = record.audit.last().expect("ready approval audit event");
    assert_eq!(
        ready_approval_event.actor,
        "fresh-session:44444444-4444-4444-8444-444444444444"
    );
    assert_eq!(
        ready_approval_event.reason,
        "reapprove repaired ready issue design"
    );
    let ready_approval_operation: serde_json::Value =
        serde_json::from_str(&ready_approval_event.operation).expect("approval audit JSON");
    assert_eq!(ready_approval_operation["operation"], "approve_design");
    assert_eq!(ready_approval_operation["design_ref"], "design/issue-42.md");
    assert_eq!(
        ready_approval_operation["diagram_ref"],
        "design/issue-42.mmd"
    );
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    ));

    let before_interruption = issue_projection_snapshot(&repo, 42);
    let interrupted = direct_edit(&store, &record, CardKind::Stp, exact_acceptance(), true)
        .expect_err("injected interruption");
    assert_eq!(
        interrupted.code,
        csdlc_v2::ErrorCode::InterruptedTransaction
    );
    let mut stale = record.clone();
    stale.digest = "0".repeat(64);
    assert!(direct_edit(&store, &stale, CardKind::Stp, exact_acceptance(), false).is_err());
    assert_eq!(issue_projection_snapshot(&repo, 42), before_interruption);

    let clean_ready = issue_projection_snapshot(&repo, 42);
    let mut reviewed = record.clone();
    let mut from = LifecyclePhase::Ready;
    for to in [
        LifecyclePhase::Bound,
        LifecyclePhase::Implemented,
        LifecyclePhase::Reviewed,
    ] {
        reviewed.transitions.push(csdlc_v2::model::TransitionEvent {
            sequence: reviewed.transitions.len() as u64 + 1,
            from,
            to,
            actor: "test-operator".into(),
            reason: "build consistent unsupported-phase fixture".into(),
        });
        from = to;
    }
    reviewed.phase = LifecyclePhase::Reviewed;
    reviewed.branch = Some("reviewed-fixture".into());
    reviewed.worktree = Some("/tmp/reviewed-fixture".into());
    write_consistent_record(&repo, &mut reviewed);
    let injected_reviewed = issue_projection_snapshot(&repo, 42);
    let error = direct_edit(&store, &reviewed, CardKind::Stp, exact_acceptance(), false)
        .expect_err("unsupported reviewed phase guard");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidTransition);
    assert_eq!(error.message, "stp mutation is not allowed during reviewed");
    assert_eq!(issue_projection_snapshot(&repo, 42), injected_reviewed);
    restore_issue_projection(&repo, 42, &clean_ready);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "repair ready fixture"]);

    let worktree = temp.path().join("worktrees/issue-42");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: "issue-42".into(),
            worktree: worktree.to_string_lossy().into_owned(),
            code_repository: None,
        },
    )
    .expect("bind repaired issue for compatibility proof");
    let repo = worktree;
    let store = Store::new(&repo);
    record = store.load_record(42).expect("bound record");
    assert_eq!(record.phase, LifecyclePhase::Bound);
    assert_eq!(record.branch.as_deref(), Some("issue-42"));
    let canonical_repo = fs::canonicalize(&repo).expect("canonical fixture worktree");
    assert_eq!(
        record.worktree.as_deref(),
        Some(canonical_repo.to_string_lossy().as_ref())
    );

    let bound_stp_before = record.clone();
    let bound_stp_cards_before = store.load_cards(42).expect("bound cards before STP repair");
    let bound_acceptance = vec![
        "AC-1: bound acceptance repair preserves topology".to_owned(),
        "AC-2: bound acceptance repair preserves outputs".to_owned(),
    ];
    let bound_stp_operation = SemanticOperation::ReplaceAcceptanceCriteria {
        values: bound_acceptance.clone(),
    };
    record = direct_edit(
        &store,
        &record,
        CardKind::Stp,
        bound_stp_operation.clone(),
        false,
    )
    .expect("bound STP compatibility edit");
    assert_single_generation_preserves_lifecycle_shell(&bound_stp_before, &record);
    assert_eq!(record.design_review, bound_stp_before.design_review);
    let bound_stp_cards_after = store.load_cards(42).expect("bound cards after STP repair");
    for kind in [
        CardKind::Sip,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(
            &bound_stp_cards_before[&kind],
            &bound_stp_cards_after[&kind],
        );
    }
    assert_card_identity_advanced(
        &bound_stp_cards_before[&CardKind::Stp],
        &bound_stp_cards_after[&CardKind::Stp],
    );
    let mut expected_bound_stp = match &bound_stp_cards_before[&CardKind::Stp].content {
        CardContent::Stp(values) => values.clone(),
        _ => unreachable!("STP"),
    };
    expected_bound_stp.acceptance_criteria = bound_acceptance;
    match &bound_stp_cards_after[&CardKind::Stp].content {
        CardContent::Stp(values) => assert_eq!(values, &expected_bound_stp),
        _ => unreachable!("STP"),
    }
    assert_design_bindings_match_authored(&repo, &bound_stp_cards_after);
    assert_last_audit_operation(&record, &bound_stp_operation);

    let compatibility_plan = |action: &str| SemanticOperation::ReplacePlanSteps {
        steps: vec![PlanStep {
            id: "S1".into(),
            action: action.into(),
            acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
            status: StepStatus::Pending,
        }],
    };
    let bound_spp_before = record.clone();
    let bound_spp_cards_before = store.load_cards(42).expect("bound cards before SPP repair");
    let bound_spp_operation = compatibility_plan("prove bound compatibility outputs");
    record = direct_edit(
        &store,
        &record,
        CardKind::Spp,
        bound_spp_operation.clone(),
        false,
    )
    .expect("bound SPP compatibility edit");
    assert_single_generation_preserves_lifecycle_shell(&bound_spp_before, &record);
    assert_eq!(record.design_review, bound_spp_before.design_review);
    let bound_spp_cards_after = store.load_cards(42).expect("bound cards after SPP repair");
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(
            &bound_spp_cards_before[&kind],
            &bound_spp_cards_after[&kind],
        );
    }
    assert_card_identity_advanced(
        &bound_spp_cards_before[&CardKind::Spp],
        &bound_spp_cards_after[&CardKind::Spp],
    );
    let mut expected_bound_spp = match &bound_spp_cards_before[&CardKind::Spp].content {
        CardContent::Spp(values) => values.clone(),
        _ => unreachable!("SPP"),
    };
    let SemanticOperation::ReplacePlanSteps { steps } = &bound_spp_operation else {
        unreachable!("bound SPP operation")
    };
    expected_bound_spp.plan_revision += 1;
    expected_bound_spp.steps = steps.clone();
    match &bound_spp_cards_after[&CardKind::Spp].content {
        CardContent::Spp(values) => assert_eq!(values, &expected_bound_spp),
        _ => unreachable!("SPP"),
    }
    assert_design_bindings_match_authored(&repo, &bound_spp_cards_after);
    assert_last_audit_operation(&record, &bound_spp_operation);

    record = direct_edit(
        &store,
        &record,
        CardKind::Sor,
        SemanticOperation::RecordExecution {
            summary: "implemented compatibility fixture".into(),
            changes: vec!["csdlc-v2/tests/gate2.rs".into()],
            artifacts: vec!["bound and implemented repair proof".into()],
        },
        false,
    )
    .expect("record compatibility execution");
    record = direct_edit(
        &store,
        &record,
        CardKind::Spp,
        SemanticOperation::AdvancePhase {
            phase: LifecyclePhase::Implemented,
        },
        false,
    )
    .expect("advance compatibility fixture to implemented");
    let implemented_before = record.clone();
    let implemented_cards_before = store
        .load_cards(42)
        .expect("implemented cards before SPP repair");
    let implemented_spp_operation = compatibility_plan("prove implemented compatibility outputs");
    record = direct_edit(
        &store,
        &record,
        CardKind::Spp,
        implemented_spp_operation.clone(),
        false,
    )
    .expect("implemented SPP compatibility edit");
    assert_single_generation_preserves_lifecycle_shell(&implemented_before, &record);
    assert_eq!(record.phase, LifecyclePhase::Implemented);
    assert_eq!(record.design_review, implemented_before.design_review);
    let implemented_cards_after = store
        .load_cards(42)
        .expect("implemented cards after SPP repair");
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_semantics_unchanged(
            &implemented_cards_before[&kind],
            &implemented_cards_after[&kind],
        );
    }
    assert_card_identity_advanced(
        &implemented_cards_before[&CardKind::Spp],
        &implemented_cards_after[&CardKind::Spp],
    );
    let mut expected_implemented_spp = match &implemented_cards_before[&CardKind::Spp].content {
        CardContent::Spp(values) => values.clone(),
        _ => unreachable!("SPP"),
    };
    let SemanticOperation::ReplacePlanSteps { steps } = &implemented_spp_operation else {
        unreachable!("implemented SPP operation")
    };
    expected_implemented_spp.plan_revision += 1;
    expected_implemented_spp.steps = steps.clone();
    match &implemented_cards_after[&CardKind::Spp].content {
        CardContent::Spp(values) => assert_eq!(values, &expected_implemented_spp),
        _ => unreachable!("SPP"),
    }
    assert_design_bindings_match_authored(&repo, &implemented_cards_after);
    assert_last_audit_operation(&record, &implemented_spp_operation);
}

#[test]
fn implemented_sip_scope_correction() {
    let target = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    fs::create_dir_all(&target).expect("target temp parent");
    let temp = tempfile::Builder::new()
        .prefix("issue-63-scope-")
        .tempdir_in(&target)
        .expect("repo-local tempdir");
    let primary = temp.path().join("repo-primary");
    let repo = temp.path().join("repo");
    fs::create_dir_all(primary.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(primary.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(primary.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(primary.join("design")).expect("design directory");
    fs::write(
        primary.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        primary.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(
        primary.join("csdlc-v2/tests/gate2.rs"),
        "// focused fixture\n",
    )
    .expect("gate2 fixture");
    fs::write(primary.join("design/issue-42.md"), "# Approved design\n").expect("design");
    fs::write(
        primary.join("design/issue-42.mmd"),
        "flowchart LR\n  Edit --> Validate\n",
    )
    .expect("diagram");
    git(&primary, &["init", "-b", "main"]);
    git(&primary, &["config", "user.email", "test@example.com"]);
    git(&primary, &["config", "user.name", "C-SDLC Test"]);
    git(&primary, &["add", "."]);
    git(&primary, &["commit", "-m", "fixture"]);
    add_non_primary_main_worktree(&primary, &repo);

    let store = Store::new(&repo);
    let record = csdlc_v2::initialize_native_json(
        &store,
        &serde_json::to_vec(&request()).expect("serialize bootstrap"),
    )
    .expect("bootstrap");
    let _ready = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test-operator".into(),
            reason: "fixture ready".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .expect("ready");
    install_worktree_policy(&repo);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "install worktree policy"]);
    let worktree = temp.path().join("worktrees/issue-42");
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: "issue-42".into(),
            worktree: worktree.to_string_lossy().into_owned(),
            code_repository: None,
        },
    )
    .expect("bind");
    let repo = worktree;
    let store = Store::new(&repo);
    let bound = store.load_record(42).expect("bound record");
    assert_eq!(bound.phase, LifecyclePhase::Bound);

    let request_path = |name: &str| temp.path().join(format!("{name}.json"));
    let write_edit = |path: &Path,
                      index: &serde_json::Value,
                      card: &str,
                      actor: &str,
                      reason: &str,
                      values: Vec<&str>| {
        fs::write(
            path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "issue": 42,
                "card": card,
                "expected_generation": index["generation"],
                "expected_digest": index["digest"],
                "actor": actor,
                "reason": reason,
                "operation": {
                    "operation": "correct_declared_scope_before_publication",
                    "values": values
                }
            }))
            .expect("serialize correction request"),
        )
        .expect("write correction request");
    };
    let current_index = || -> serde_json::Value {
        serde_json::from_slice(
            &fs::read(repo.join(".csdlc/issues/42/index.json")).expect("issue index"),
        )
        .expect("issue index JSON")
    };
    let bound_request = request_path("bound-correction");
    write_edit(
        &bound_request,
        &current_index(),
        "sip",
        "test-operator",
        "bound is too early",
        vec!["src/new.rs"],
    );
    let bound_rejection = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &bound_request.to_string_lossy(),
        ],
    );
    assert!(!bound_rejection.status.success());

    let executed = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            actor: "test-operator".into(),
            reason: "record fixture implementation".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "implemented fixture".into(),
                changes: vec!["src/old.rs".into()],
                artifacts: vec!["fixture".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("execution");
    let implemented = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: executed.generation,
            expected_digest: executed.digest,
            actor: "test-operator".into(),
            reason: "advance implemented fixture".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Implemented,
            },
            fail_after_backup: false,
        },
    )
    .expect("implemented");

    for (name, card, actor, reason, values) in [
        ("empty-actor", "sip", "", "reason", vec!["src/new.rs"]),
        ("empty-reason", "sip", "operator", "", vec!["src/new.rs"]),
        ("empty-scope", "sip", "operator", "reason", vec![]),
        (
            "wrong-card",
            "stp",
            "operator",
            "reason",
            vec!["src/new.rs"],
        ),
    ] {
        let path = request_path(name);
        write_edit(&path, &current_index(), card, actor, reason, values);
        let output = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-edit"),
            &[
                "--repo",
                &repo.to_string_lossy(),
                "apply",
                "--request",
                &path.to_string_lossy(),
            ],
        );
        assert!(!output.status.success(), "{name} unexpectedly succeeded");
        assert_eq!(store.load_record(42).unwrap().digest, implemented.digest);
    }

    let stale_digest_path = request_path("stale-digest");
    let mut stale_index = current_index();
    stale_index["digest"] = serde_json::Value::String("0".repeat(64));
    write_edit(
        &stale_digest_path,
        &stale_index,
        "sip",
        "operator",
        "stale digest",
        vec!["src/new.rs"],
    );
    assert!(!command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &stale_digest_path.to_string_lossy(),
        ],
    )
    .status
    .success());

    let correction_path = request_path("correction");
    write_edit(
        &correction_path,
        &current_index(),
        "sip",
        "review-fix-operator",
        "replace stale declared scope path",
        vec!["src/new.rs", "tests/new.rs"],
    );
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &correction_path.to_string_lossy(),
        ],
    ));
    let corrected = store.load_record(42).expect("corrected record");
    assert_eq!(corrected.generation, implemented.generation + 1);
    assert_ne!(corrected.digest, implemented.digest);
    let operation: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("audit operation JSON");
    assert_eq!(
        operation["previous_values"],
        serde_json::json!(["claim-free workflow"])
    );
    assert_eq!(
        operation["new_values"],
        serde_json::json!(["src/new.rs", "tests/new.rs"])
    );
    let event = corrected.audit.last().expect("correction audit");
    assert_eq!(event.actor, "review-fix-operator");
    assert_eq!(event.reason, "replace stale declared scope path");
    let sip_values: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(".csdlc/issues/42/cards/sip.values.json")).expect("SIP values"),
    )
    .expect("SIP values JSON");
    assert_eq!(
        sip_values["content"]["values"]["declared_scope"],
        serde_json::json!(["src/new.rs", "tests/new.rs"])
    );
    let sip_path = repo.join(".csdlc/issues/42/cards/sip.md");
    let rendered = fs::read_to_string(&sip_path).expect("rendered SIP");
    assert!(rendered.contains("src/new.rs"));
    assert!(rendered.contains("tests/new.rs"));
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    ));

    let stale_generation = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &correction_path.to_string_lossy(),
        ],
    );
    assert!(!stale_generation.status.success());
    let before_drift = fs::read(&sip_path).expect("SIP before drift");
    fs::write(
        &sip_path,
        [before_drift.as_slice(), b"\nmanual drift\n"].concat(),
    )
    .expect("inject Markdown drift");
    assert!(!command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    )
    .status
    .success());
    fs::write(sip_path, before_drift).expect("restore rendered SIP");
}

#[test]
fn bind_topology_scan_uses_canonical_record_identity() {
    let temp = tempfile::tempdir().expect("tempdir");

    // Reproduce the reported failure through the real bind binary: an unrelated
    // retained record stores `worktree: "."` and references intentionally absent
    // local artifacts in the issue worktree being bound.
    let success_repo = temp.path().join("success-repo");
    let success_worktree = temp.path().join("success-worktree");
    focused_fixture_repo(&success_repo);
    create_focused_issue(&success_repo, temp.path(), 42);

    // Reproduce the original report exactly: another registered worktree
    // retains a pre-topology projection for the same issue. It contains the
    // retired claim field but declares no branch or worktree authority.
    let stale_same_issue_worktree = temp.path().join("same-issue-stale-worktree");
    git(
        &success_repo,
        &[
            "worktree",
            "add",
            "-b",
            "stale-same-issue-42",
            &stale_same_issue_worktree.to_string_lossy(),
            "main",
        ],
    );
    copy_directory(
        &success_repo.join(".csdlc/issues/42"),
        &stale_same_issue_worktree.join(".csdlc/issues/42"),
    );
    let stale_same_issue_index = stale_same_issue_worktree.join(".csdlc/issues/42/index.json");
    let mut stale_same_issue: serde_json::Value = serde_json::from_slice(
        &fs::read(&stale_same_issue_index).expect("read stale same-issue index"),
    )
    .expect("parse stale same-issue index");
    stale_same_issue
        .as_object_mut()
        .expect("same-issue index object")
        .remove("branch");
    stale_same_issue
        .as_object_mut()
        .expect("same-issue index object")
        .remove("worktree");
    stale_same_issue["claim"] = serde_json::json!({
        "owner": "retired-preparation-session",
        "lease": "released"
    });
    let stale_same_issue_with_claim =
        serde_json::to_vec_pretty(&stale_same_issue).expect("serialize stale same-issue index");
    fs::write(&stale_same_issue_index, &stale_same_issue_with_claim)
        .expect("write stale same-issue index");

    let retained_5791 = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
        .join(".csdlc/issues/5791");
    copy_directory(&retained_5791, &success_repo.join(".csdlc/issues/5791"));
    let foreign_index_path = success_repo.join(".csdlc/issues/5791/index.json");
    let mut foreign_index: serde_json::Value =
        serde_json::from_slice(&fs::read(&foreign_index_path).expect("read foreign issue index"))
            .expect("parse foreign issue index");
    foreign_index["claim"] = serde_json::json!({
        "owner": "retired-session",
        "lease": "stale",
        "heartbeat": "2026-01-01T00:00:00Z"
    });
    let foreign_index_with_claim =
        serde_json::to_vec_pretty(&foreign_index).expect("serialize claim-bearing index");
    fs::write(&foreign_index_path, &foreign_index_with_claim)
        .expect("write claim-bearing foreign issue index");
    assert!(!success_repo
        .join(".adl/local-artifacts/5791-bootstrap/design.md")
        .exists());
    let success_bind = temp.path().join("success-bind.json");
    fs::write(
        &success_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "candidate-42",
            "worktree": success_worktree,
        }))
        .expect("serialize success bind"),
    )
    .expect("success bind request");
    let success_diagnosis = csdlc_v2::diagnose(&Store::new(&success_repo), 42);
    assert_eq!(
        success_diagnosis.status,
        csdlc_v2::doctor::DoctorStatus::Pass,
        "focused source issue must be bindable: {:?}",
        success_diagnosis.findings
    );
    let success = must_succeed(command(
        &success_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_repo.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    ));
    let success: serde_json::Value = serde_json::from_str(&success).expect("bind result JSON");
    assert_eq!(success["branch"], "candidate-42");
    assert_eq!(
        success["worktree"],
        success_worktree
            .canonicalize()
            .expect("canonical worktree")
            .to_string_lossy()
            .as_ref()
    );
    assert_eq!(
        fs::read(&foreign_index_path).expect("reread foreign issue index"),
        foreign_index_with_claim,
        "bind must not rewrite an unrelated legacy claim-bearing projection"
    );
    assert_eq!(
        fs::read(&stale_same_issue_index).expect("reread stale same-issue index"),
        stale_same_issue_with_claim,
        "bind must not rewrite a same-issue projection with no topology authority"
    );

    // A present but malformed topology field is not equivalent to absent
    // topology and must still receive strict current-record decoding.
    let mut malformed_topology = stale_same_issue.clone();
    malformed_topology
        .as_object_mut()
        .expect("malformed topology object")
        .remove("claim");
    malformed_topology["branch"] = serde_json::json!(42);
    fs::write(
        &stale_same_issue_index,
        serde_json::to_vec_pretty(&malformed_topology).expect("serialize malformed topology"),
    )
    .expect("write malformed topology");
    let malformed_topology_result = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!malformed_topology_result.status.success());
    fs::write(&stale_same_issue_index, &stale_same_issue_with_claim)
        .expect("restore stale same-issue index");

    // The same retired field remains corruption when it appears on a relevant
    // record; relevance-first scanning must not weaken strict IssueRecord
    // verification for the issue being bound.
    let relevant_index_path = success_worktree.join(".csdlc/issues/42/index.json");
    let relevant_index_before = fs::read(&relevant_index_path).expect("read relevant issue index");
    let mut relevant_index: serde_json::Value =
        serde_json::from_slice(&relevant_index_before).expect("parse relevant issue index");
    relevant_index["claim"] = serde_json::json!({"owner": "retired-session"});
    fs::write(
        &relevant_index_path,
        serde_json::to_vec_pretty(&relevant_index).expect("serialize malformed relevant index"),
    )
    .expect("write malformed relevant issue index");
    let claim_on_relevant_record = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!claim_on_relevant_record.status.success());
    fs::write(&relevant_index_path, relevant_index_before).expect("restore relevant issue index");

    // A genuinely relevant same-issue record is still fully verified.
    fs::remove_file(success_worktree.join("design/issue-42.md")).expect("remove issue design");
    let corrupt_same_issue = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!corrupt_same_issue.status.success());
    fs::write(
        success_worktree.join("design/issue-42.md"),
        "# Approved design for issue 42\n",
    )
    .expect("restore issue design");

    // An exact projection must not short-circuit a later conflicting copy of
    // the same issue. Build that conflicting projection through the real bind
    // binary, then place it in a worktree listed after the exact projection.
    let conflicting_source_repo = temp.path().join("conflicting-source-repo");
    let conflicting_source_worktree = temp.path().join("conflicting-source-worktree");
    focused_fixture_repo(&conflicting_source_repo);
    create_focused_issue(&conflicting_source_repo, temp.path(), 42);
    let conflicting_source_bind = temp.path().join("conflicting-source-bind.json");
    fs::write(
        &conflicting_source_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "other-42",
            "worktree": conflicting_source_worktree,
        }))
        .expect("serialize conflicting source bind"),
    )
    .expect("conflicting source bind request");
    must_succeed(command(
        &conflicting_source_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflicting_source_repo.to_string_lossy(),
            "--request",
            &conflicting_source_bind.to_string_lossy(),
        ],
    ));
    let later_projection = temp.path().join("later-projection");
    git(
        &success_repo,
        &[
            "worktree",
            "add",
            "-b",
            "later-projection",
            &later_projection.to_string_lossy(),
            "main",
        ],
    );
    copy_directory(
        &conflicting_source_worktree.join(".csdlc/issues/42"),
        &later_projection.join(".csdlc/issues/42"),
    );
    let later_conflict = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!later_conflict.status.success());
    assert!(String::from_utf8_lossy(&later_conflict.stdout).contains("reconciliation_required"));

    // A retained exact record is not sufficient for idempotence when Git no
    // longer registers the stored branch/worktree pair.
    let stale_repo = temp.path().join("stale-repo");
    let stale_worktree = temp.path().join("stale-worktree");
    focused_fixture_repo(&stale_repo);
    create_focused_issue(&stale_repo, temp.path(), 42);
    let stale_bind = temp.path().join("stale-bind.json");
    fs::write(
        &stale_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "stale-42",
            "worktree": stale_worktree,
        }))
        .expect("serialize stale bind"),
    )
    .expect("stale bind request");
    must_succeed(command(
        &stale_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &stale_repo.to_string_lossy(),
            "--request",
            &stale_bind.to_string_lossy(),
        ],
    ));
    copy_directory(
        &stale_worktree.join(".csdlc/issues/42"),
        &stale_repo.join(".csdlc/issues/42"),
    );
    fs::create_dir_all(stale_repo.join("design")).expect("stale design directory");
    fs::copy(
        stale_worktree.join("design/issue-42.md"),
        stale_repo.join("design/issue-42.md"),
    )
    .expect("retain stale design");
    git(
        &stale_repo,
        &[
            "worktree",
            "remove",
            "--force",
            &stale_worktree.to_string_lossy(),
        ],
    );
    let stale_result = command(
        &stale_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &stale_repo.to_string_lossy(),
            "--request",
            &stale_bind.to_string_lossy(),
        ],
    );
    assert!(!stale_result.status.success());
    assert!(String::from_utf8_lossy(&stale_result.stdout).contains("reconciliation_required"));

    // Build one valid owner record, retain its projection in an unrelated
    // primary checkout, and unregister its original worktree. Collision
    // decisions must use the stored branch/worktree predicates, not the branch
    // or path of the projection being scanned.
    let conflict_repo = temp.path().join("conflict-repo");
    let owner_worktree = temp.path().join("owner-worktree");
    focused_fixture_repo(&conflict_repo);
    create_focused_issue(&conflict_repo, temp.path(), 43);
    let owner_bind = temp.path().join("owner-bind.json");
    fs::write(
        &owner_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "claimed-branch",
            "worktree": owner_worktree,
        }))
        .expect("serialize owner bind"),
    )
    .expect("owner bind request");
    must_succeed(command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &owner_bind.to_string_lossy(),
        ],
    ));
    copy_directory(
        &owner_worktree.join(".csdlc/issues/43"),
        &conflict_repo.join(".csdlc/issues/43"),
    );
    git(
        &conflict_repo,
        &[
            "worktree",
            "remove",
            "--force",
            &owner_worktree.to_string_lossy(),
        ],
    );
    create_focused_issue(&conflict_repo, temp.path(), 42);

    let branch_collision = temp.path().join("branch-collision.json");
    fs::write(
        &branch_collision,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "claimed-branch",
            "worktree": temp.path().join("branch-candidate"),
        }))
        .expect("serialize branch collision"),
    )
    .expect("branch collision request");
    fs::remove_file(conflict_repo.join("design/issue-43.md")).expect("remove owner design");
    let contextual_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &branch_collision.to_string_lossy(),
        ],
    );
    assert!(!contextual_rejection.status.success());
    let contextual_output = format!(
        "{}{}",
        String::from_utf8_lossy(&contextual_rejection.stdout),
        String::from_utf8_lossy(&contextual_rejection.stderr)
    );
    assert!(contextual_output.contains("issue 43 topology"));
    assert!(contextual_output.contains("design/issue-43.md"));
    fs::write(
        conflict_repo.join("design/issue-43.md"),
        "# Approved design for issue 43\n",
    )
    .expect("restore owner design");
    let branch_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &branch_collision.to_string_lossy(),
        ],
    );
    assert!(!branch_rejection.status.success());
    assert!(String::from_utf8_lossy(&branch_rejection.stdout).contains("reconciliation_required"));

    let worktree_collision = temp.path().join("worktree-collision.json");
    fs::write(
        &worktree_collision,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "different-candidate-branch",
            "worktree": owner_worktree,
        }))
        .expect("serialize worktree collision"),
    )
    .expect("worktree collision request");
    let worktree_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &worktree_collision.to_string_lossy(),
        ],
    );
    assert!(!worktree_rejection.status.success());
    assert!(String::from_utf8_lossy(&worktree_rejection.stdout).contains("reconciliation_required"));
}

fn design_review_recovery_fixture(
    issue: u64,
    implemented: bool,
) -> (
    tempfile::TempDir,
    std::path::PathBuf,
    Store,
    csdlc_v2::IssueRecord,
) {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    fs::create_dir_all(&repo).expect("fixture repo");
    focused_fixture_repo(&repo);
    if issue != 42 && issue != 43 {
        fs::write(
            repo.join(format!("design/issue-{issue}.md")),
            format!("# Approved design for issue {issue}\n"),
        )
        .expect("issue design fixture");
        fs::write(
            repo.join(format!("design/issue-{issue}.mmd")),
            "flowchart LR\n  Create --> Bind\n",
        )
        .expect("issue diagram fixture");
    }
    create_focused_issue(&repo, temp.path(), issue);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "create recovery fixture"]);
    let worktree = temp.path().join(format!("worktrees/issue-{issue}"));
    bind_issue(
        &Store::new(&repo),
        BindRequest {
            issue,
            base_branch: "main".into(),
            branch: format!("issue-{issue}"),
            worktree: worktree.to_string_lossy().into_owned(),
            code_repository: None,
        },
    )
    .expect("bind recovery fixture");
    let store = Store::new(&worktree);
    let mut record = store.load_record(issue).expect("bound recovery record");
    if implemented {
        record = direct_edit(
            &store,
            &record,
            CardKind::Sor,
            SemanticOperation::RecordExecution {
                summary: "implemented recovery fixture".into(),
                changes: vec!["csdlc-v2/tests/gate2.rs".into()],
                artifacts: vec!["focused recovery proof".into()],
            },
            false,
        )
        .expect("record implementation");
        record = direct_edit(
            &store,
            &record,
            CardKind::Spp,
            SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Implemented,
            },
            false,
        )
        .expect("advance implemented recovery fixture");
    }
    (temp, worktree, store, record)
}

fn recovery_request(record: &csdlc_v2::IssueRecord) -> csdlc_v2::RecoverDesignReviewRequest {
    let csdlc_v2::DesignReview::Approved { reviewer, revision } = &record.design_review else {
        panic!("fixture design approval")
    };
    csdlc_v2::RecoverDesignReviewRequest {
        issue: record.issue,
        expected_phase: record.phase,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        previous_reviewer: reviewer.clone(),
        previous_revision: revision.clone(),
        false_reviewer: reviewer.clone(),
        actor: "test:false-design-review-recovery".into(),
        reason: "correct falsely attributed review authority".into(),
        disposition: "false_review_recorded_before_reviewer_returned".into(),
    }
}

fn assert_recovery_result(before: &csdlc_v2::IssueRecord, after: &csdlc_v2::IssueRecord) {
    assert_eq!(after.phase, before.phase);
    assert_eq!(after.generation, before.generation + 1);
    assert!(matches!(
        after.design_review,
        csdlc_v2::DesignReview::Pending
    ));
    assert_eq!(after.branch, before.branch);
    assert_eq!(after.worktree, before.worktree);
    assert_eq!(after.transitions, before.transitions);
    assert_eq!(after.review_assignment, before.review_assignment);
    assert_eq!(after.review, before.review);
    assert_eq!(after.publication, before.publication);
    assert_eq!(after.audit.len(), before.audit.len() + 1);
    assert_eq!(
        after.audit.last().unwrap().operation,
        "recover_design_review"
    );
    assert!(after
        .audit
        .last()
        .unwrap()
        .reason
        .contains("previous_approval"));
    assert!(after
        .audit
        .last()
        .unwrap()
        .reason
        .contains("false_reviewer"));
    assert!(after.audit.last().unwrap().reason.contains("disposition"));
}

#[test]
fn bound_design_review_recovery_clears_false_approval() {
    let (_temp, _worktree, store, record) = design_review_recovery_fixture(42, false);
    let before_cards = store.load_cards(42).expect("cards before recovery");
    let corrected = csdlc_v2::recover_design_review(&store, recovery_request(&record))
        .expect("recover bound false approval");
    assert_recovery_result(&record, &corrected);
    let after_cards = store.load_cards(42).expect("cards after recovery");
    for kind in [
        CardKind::Sip,
        CardKind::Stp,
        CardKind::Spp,
        CardKind::Vpp,
        CardKind::Srp,
        CardKind::Sor,
    ] {
        assert_card_identity_advanced(&before_cards[&kind], &after_cards[&kind]);
        assert_eq!(before_cards[&kind].content, after_cards[&kind].content);
    }
}

#[test]
fn implemented_design_review_recovery_clears_false_approval() {
    let (_temp, _worktree, store, record) = design_review_recovery_fixture(42, true);
    let corrected = csdlc_v2::recover_design_review(&store, recovery_request(&record))
        .expect("recover implemented false approval");
    assert_recovery_result(&record, &corrected);
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
}

#[test]
fn design_review_recovery_rejects_invalid_authority_and_repeat() {
    let (_temp, worktree, store, record) = design_review_recovery_fixture(42, false);
    let pristine = issue_projection_snapshot(&worktree, 42);
    let mut cases = Vec::new();
    let mut value = recovery_request(&record);
    value.expected_generation += 1;
    cases.push(value);
    let mut value = recovery_request(&record);
    value.expected_digest = "0".repeat(64);
    cases.push(value);
    let mut value = recovery_request(&record);
    value.expected_phase = LifecyclePhase::Implemented;
    cases.push(value);
    let mut value = recovery_request(&record);
    value.previous_reviewer = "different-reviewer".into();
    value.false_reviewer = "different-reviewer".into();
    cases.push(value);
    let mut value = recovery_request(&record);
    value.previous_revision = "0".repeat(64);
    cases.push(value);
    let mut value = recovery_request(&record);
    value.false_reviewer = "different-reviewer".into();
    cases.push(value);
    let mut value = recovery_request(&record);
    value.reason.clear();
    cases.push(value);
    let mut value = recovery_request(&record);
    value.disposition.clear();
    cases.push(value);
    for request in cases {
        csdlc_v2::recover_design_review(&store, request).expect_err("invalid recovery");
        assert_eq!(issue_projection_snapshot(&worktree, 42), pristine);
    }
    let mut later = record.clone();
    later.review_assignment = Some(csdlc_v2::ReviewAssignment {
        reviewer: "later-reviewer".into(),
        assigned_by: "test".into(),
        revision: "0".repeat(40),
        scope: vec!["later authority".into()],
    });
    write_consistent_record(&worktree, &mut later);
    let later_projection = issue_projection_snapshot(&worktree, 42);
    csdlc_v2::recover_design_review(&store, recovery_request(&later))
        .expect_err("later authority recovery");
    assert_eq!(issue_projection_snapshot(&worktree, 42), later_projection);
    restore_issue_projection(&worktree, 42, &pristine);
    let mut repeat = recovery_request(&record);
    let corrected =
        csdlc_v2::recover_design_review(&store, repeat.clone()).expect("first recovery");
    repeat.expected_generation = corrected.generation;
    repeat.expected_digest = corrected.digest.clone();
    csdlc_v2::recover_design_review(&store, repeat).expect_err("repeat recovery");
}

#[test]
fn design_review_recovery_matches_issue_275_shape() {
    let (_temp, worktree, store, mut record) = design_review_recovery_fixture(275, true);
    record.audit.push(csdlc_v2::model::AuditEvent {
        sequence: record.audit.len() as u64 + 1,
        generation: record.generation,
        actor: "fresh-session:97817988-0208-4c4f-b721-aa1dc24fdfda".into(),
        reason: "historical false approval preserved for correction provenance".into(),
        operation: "approve_design".into(),
    });
    record.design_review = csdlc_v2::DesignReview::Approved {
        reviewer: "codex:/root/issue-275-implementation".into(),
        revision: "045fdad3f60a30c16c6a5b27f2a2a55f7dc2a38d3bc75b83197a1d576c9c1f66".into(),
    };
    write_consistent_record(&worktree, &mut record);
    let audit = record
        .audit
        .iter()
        .map(|event| serde_json::to_string(event).expect("serialize audit event"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    fs::write(worktree.join(".csdlc/issues/275/audit.jsonl"), audit)
        .expect("write preserved issue 275 audit history");
    record = store
        .load_record(275)
        .expect("current issue 275 corruption shape");
    let mut request = recovery_request(&record);
    request.actor = "codex:issue-275-false-review-recovery".into();
    request.reason = "clear current noncanonical self-review approval while preserving both false approval events".into();
    request.disposition = "current_self_review_and_prior_false_fresh_review_preserved".into();
    let corrected =
        csdlc_v2::recover_design_review(&store, request).expect("recover exact issue 275 shape");
    assert_recovery_result(&record, &corrected);
    assert_eq!(corrected.issue, 275);
    assert_eq!(corrected.phase, LifecyclePhase::Implemented);
    assert!(corrected
        .audit
        .iter()
        .any(|event| event.actor == "fresh-session:97817988-0208-4c4f-b721-aa1dc24fdfda"));
    assert!(corrected
        .audit
        .last()
        .unwrap()
        .reason
        .contains("codex:/root/issue-275-implementation"));
}
