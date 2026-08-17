use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Mutex;

use csdlc_v2::cards::{digest, PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    initialize_native_json, ApproveDesignRequest, BootstrapRequest, DecompositionGraphEdge,
    DecompositionGraphInput, DecompositionGraphNode, DesignReview, DesignReviewRecoveryTruth,
    InitialCardInput, InitializedDecompositionRecoveryReplacement,
    InitializedDecompositionRecoveryRequest, InitializedRecoveryFailurePoint, PlanningProfile,
    PreservedAuthoredArtifact, Store,
};
use sha2::{Digest, Sha256};

static CWD_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn initialized_decomposition_recovery_preserves_history_and_recovers_crashes() {
    let _cwd = CWD_LOCK.lock().expect("cwd lock");
    let original_cwd = std::env::current_dir().expect("original cwd");
    let repo = fixture_repo("happy");
    let store = Store::new(&repo);
    let record = initialize_fixture_issue(&repo);
    let design_before = fs::read(repo.join("design/issue-114.md")).expect("design bytes");
    let diagram_before = fs::read(repo.join("design/issue-114.mmd")).expect("diagram bytes");

    let request = recovery_request(&repo, &record, None);
    let request_path = repo.join("recovery-request.json");
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(&request).expect("serialize recovery request"),
    )
    .expect("write recovery request");

    let output = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "recover-initialized-decomposition",
            "--request",
            &request_path.to_string_lossy(),
        ],
    );
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let recovered = store.load_record(114).expect("recovered record");
    assert_eq!(recovered.phase, csdlc_v2::LifecyclePhase::Initialized);
    assert_eq!(recovered.generation, record.generation + 1);
    assert!(matches!(recovered.design_review, DesignReview::Pending));
    assert!(recovered.branch.is_none());
    assert!(recovered.worktree.is_none());
    assert_eq!(
        fs::read(repo.join("design/issue-114.md")).expect("design after"),
        design_before
    );
    assert_eq!(
        fs::read(repo.join("design/issue-114.mmd")).expect("diagram after"),
        diagram_before
    );
    assert!(!repo.join(".csdlc/issues/.114.recovery-journal").exists());
    let cards = store.load_cards(114).expect("cards");
    for values in cards.values() {
        assert_eq!(
            values.identity.title,
            "[v0.92][WP-18C.04-parent] Durable history coordination"
        );
        assert_eq!(values.identity.slug, "wp18c04-durable-history-coordination");
        assert_eq!(values.identity.generation, recovered.generation);
    }
    let audit = fs::read_to_string(repo.join(".csdlc/issues/114/audit.jsonl")).expect("audit");
    assert!(audit.contains("recover_initialized_decomposition"));
    assert!(audit.contains("operator:planning-1-assignment"));

    let approved = csdlc_v2::approve_design(
        &store,
        ApproveDesignRequest {
            issue: 114,
            expected_generation: recovered.generation,
            expected_digest: recovered.digest.clone(),
            reviewer: "fresh-session:88888888-8888-4888-8888-888888888888".into(),
        },
    )
    .expect("approve recovered design");
    let doctor = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo.to_string_lossy(), "--issue", "114"],
    );
    assert!(
        doctor.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&doctor.stdout),
        String::from_utf8_lossy(&doctor.stderr)
    );
    assert_eq!(approved.generation, recovered.generation + 1);

    std::env::set_current_dir(&repo).expect("fixture cwd for stale retry");
    let repeated = csdlc_v2::recover_initialized_decomposition(&store, request)
        .expect_err("old CAS must be stale after recovery");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after stale retry");
    assert_eq!(repeated.code, csdlc_v2::ErrorCode::StaleGeneration);

    let disconnected_repo = fixture_repo("disconnected-graph");
    let disconnected_store = Store::new(&disconnected_repo);
    let disconnected_record = initialize_fixture_issue(&disconnected_repo);
    let mut disconnected_request = recovery_request(&disconnected_repo, &disconnected_record, None);
    disconnected_request.graph.edges = vec![edge("child-a", "child-b")];
    std::env::set_current_dir(&disconnected_repo).expect("fixture cwd for disconnected graph");
    let disconnected =
        csdlc_v2::recover_initialized_decomposition(&disconnected_store, disconnected_request)
            .expect_err("disconnected parent graph must fail closed");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after disconnected graph");
    assert_eq!(disconnected.code, csdlc_v2::ErrorCode::InvalidInput);

    let duplicate_edge_repo = fixture_repo("duplicate-edge");
    let duplicate_edge_store = Store::new(&duplicate_edge_repo);
    let duplicate_edge_record = initialize_fixture_issue(&duplicate_edge_repo);
    let mut duplicate_edge_request =
        recovery_request(&duplicate_edge_repo, &duplicate_edge_record, None);
    duplicate_edge_request
        .graph
        .edges
        .push(edge("child-a", "child-b"));
    std::env::set_current_dir(&duplicate_edge_repo).expect("fixture cwd for duplicate edge");
    let duplicate_edge =
        csdlc_v2::recover_initialized_decomposition(&duplicate_edge_store, duplicate_edge_request)
            .expect_err("duplicate directed edge must fail closed");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after duplicate edge");
    assert_eq!(duplicate_edge.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(duplicate_edge.message.contains("duplicate directed edge"));

    let wrong_parent_repo = fixture_repo("wrong-parent");
    let wrong_parent_store = Store::new(&wrong_parent_repo);
    let wrong_parent_record = initialize_fixture_issue(&wrong_parent_repo);
    let mut wrong_parent_request = recovery_request(&wrong_parent_repo, &wrong_parent_record, None);
    wrong_parent_request
        .graph
        .nodes
        .iter_mut()
        .find(|node| node.node_id == "parent")
        .expect("parent node")
        .issue = 999;
    std::env::set_current_dir(&wrong_parent_repo).expect("fixture cwd for wrong parent");
    let wrong_parent =
        csdlc_v2::recover_initialized_decomposition(&wrong_parent_store, wrong_parent_request)
            .expect_err("wrong parent owner issue must fail closed");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after wrong parent");
    assert_eq!(wrong_parent.code, csdlc_v2::ErrorCode::InvalidInput);

    let crash_repo = fixture_repo("prepared-crash");
    let crash_store = Store::new(&crash_repo);
    let crash_record = initialize_fixture_issue(&crash_repo);
    let crash_request = recovery_request(
        &crash_repo,
        &crash_record,
        Some(InitializedRecoveryFailurePoint::AfterPreparedManifest),
    );
    std::env::set_current_dir(&crash_repo).expect("fixture cwd for crash");
    let crash = csdlc_v2::recover_initialized_decomposition(&crash_store, crash_request)
        .expect_err("prepared crash");
    assert_eq!(crash.code, csdlc_v2::ErrorCode::InterruptedTransaction);
    let stale_request = recovery_request(&crash_repo, &crash_record, None);
    let stale = csdlc_v2::recover_initialized_decomposition(&crash_store, stale_request)
        .expect_err("recovery roll-forward makes old CAS stale");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after crash");
    assert_eq!(stale.code, csdlc_v2::ErrorCode::StaleGeneration);
    let crash_after = crash_store.load_record(114).expect("rolled forward");
    assert_eq!(crash_after.generation, crash_record.generation + 1);
    assert!(!crash_repo
        .join(".csdlc/issues/.114.recovery-journal")
        .exists());
}

#[test]
fn initialized_decomposition_recovery_accepts_preserved_114_gen35_golden_fixture_when_available() {
    let Some(source) = std::env::var_os("ADL_CSDLC_291_GOLDEN_114_ROOT").map(PathBuf::from) else {
        return;
    };
    let _cwd = CWD_LOCK.lock().expect("cwd lock");
    let original_cwd = std::env::current_dir().expect("original cwd");
    let before = tree_digest(&source);
    let repo = fixture_repo("golden-114");
    copy_dir_all(
        &source.join(".csdlc/issues/114"),
        &repo.join(".csdlc/issues/114"),
    );
    copy_dir_all(
        &source.join(".csdlc/prepared/issues/114"),
        &repo.join(".csdlc/prepared/issues/114"),
    );
    if source.join(".csdlc/locks/114.lock").exists() {
        fs::create_dir_all(repo.join(".csdlc/locks")).expect("lock dir");
        fs::copy(
            source.join(".csdlc/locks/114.lock"),
            repo.join(".csdlc/locks/114.lock"),
        )
        .expect("copy lock");
    }
    fs::create_dir_all(repo.join("csdlc-v2/tests")).expect("tests directory");
    fs::write(
        repo.join("csdlc-v2/tests/initialized_decomposition_recovery.rs"),
        "// issue-owned focused denominator\n",
    )
    .expect("focused target fixture");

    let store = Store::new(&repo);
    let record = store.load_record(114).expect("golden record");
    assert_eq!(record.phase, csdlc_v2::LifecyclePhase::Initialized);
    assert_eq!(record.generation, 35);
    assert!(matches!(record.design_review, DesignReview::Pending));
    assert!(record.branch.is_none());
    assert!(record.worktree.is_none());
    assert_eq!(
        record.digest, "3ceb6fa642b537692a097960e4c216f354de777ece387cc82c3b8022e27b2e51",
        "unexpected #114 golden digest; update the retained proof only after operator-approved fixture refresh"
    );

    let mut request = recovery_request(&repo, &record, None);
    request.design_review_recovery = None;
    std::env::set_current_dir(&repo).expect("fixture cwd for golden recovery");
    let recovered =
        csdlc_v2::recover_initialized_decomposition(&store, request).expect("recover golden #114");
    std::env::set_current_dir(&original_cwd).expect("restore cwd after golden recovery");
    assert_eq!(recovered.issue, 114);
    assert_eq!(recovered.generation, 36);
    assert_ne!(recovered.digest, record.digest);
    assert_eq!(tree_digest(&source), before, "live #114 fixture changed");
}

fn fixture_repo(name: &str) -> PathBuf {
    let root = std::env::current_dir()
        .expect("cwd")
        .join("target/csdlc-291-tests")
        .join(format!("{}-{}", std::process::id(), name));
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove stale fixture");
    }
    fs::create_dir_all(root.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(root.join("csdlc-v2/operator")).expect("operator directory");
    fs::create_dir_all(root.join("csdlc-v2/tests")).expect("tests directory");
    fs::create_dir_all(root.join("design")).expect("design directory");
    fs::write(
        root.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        root.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(
        root.join("csdlc-v2/tests/initialized_decomposition_recovery.rs"),
        "// issue-owned focused denominator\n",
    )
    .expect("focused target fixture");
    fs::write(
        root.join("design/issue-114.md"),
        "# Historical durable history design\n",
    )
    .expect("design fixture");
    fs::write(
        root.join("design/issue-114.mmd"),
        "flowchart LR\n  A --> B\n",
    )
    .expect("diagram fixture");
    root
}

fn initialize_fixture_issue(repo: &Path) -> csdlc_v2::IssueRecord {
    let request = BootstrapRequest {
        issue: 114,
        repository: "agent-logic/agent-design-language".into(),
        actor: "fixture-bootstrap".into(),
        design_path: "design/issue-114.md".into(),
        diagram_path: "design/issue-114.mmd".into(),
        design_reviewer: "operator:planning-1-assignment".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "[v0.92][WP-18C.04] Add durable conversation history, continuity, and receipts"
                .into(),
            slug: "v0-92-wp-18c-04-durable-conversation-history".into(),
            version: "v0.92.0".into(),
            goal: "Implement durable history.".into(),
            required_outcome: "Runtime stores durable conversation history.".into(),
            declared_scope: vec!["durable history implementation".into()],
            authority_boundary: vec!["monolithic parent implementation".into()],
            operator_constraints: vec!["do not mutate #112".into()],
            task_boundary: "Implement product behavior.".into(),
            deliverables: vec!["runtime durable history".into()],
            acceptance_criteria: vec![
                "AC-1: history is durable".into(),
                "AC-2: continuity is preserved".into(),
                "AC-3: receipts are retained".into(),
            ],
            dependencies: vec!["#112".into()],
            repo_inputs: vec!["adl-runtime".into()],
            non_goals: vec!["publication".into()],
            plan_summary: "Implement the old monolithic durable-history parent.".into(),
            steps: vec![
                step("S1", "implement stale parent storage", "AC-1"),
                step("S2", "implement stale parent continuity", "AC-2"),
                step("S3", "implement stale parent receipts", "AC-3"),
            ],
            affected_areas: vec!["adl-runtime".into()],
            invariants: vec!["history is durable".into()],
            risks: vec!["stale decomposition".into()],
            planning_profile: PlanningProfile::Small,
            stop_conditions: vec!["review fails".into()],
            validation_lanes: vec![focused_lane()],
            failure_policy: "Fail closed.".into(),
            review_prompts: vec!["Review old parent implementation.".into()],
            review_scope: "old #114 implementation scope".into(),
        },
    };
    initialize_native_json(
        &Store::new(repo),
        &serde_json::to_vec_pretty(&request).expect("serialize bootstrap"),
    )
    .expect("initialize fixture")
}

fn recovery_request(
    repo: &Path,
    record: &csdlc_v2::IssueRecord,
    fail_at: Option<InitializedRecoveryFailurePoint>,
) -> InitializedDecompositionRecoveryRequest {
    let design = fs::read(repo.join(&record.design_path)).expect("design bytes");
    let diagram = fs::read(repo.join(&record.diagram_path)).expect("diagram bytes");
    InitializedDecompositionRecoveryRequest {
        issue: 114,
        expected_generation: record.generation,
        expected_digest: record.digest.clone(),
        actor: "codex:test".into(),
        reason: "recover initialized decomposition parent truth".into(),
        request_root: repo.to_path_buf(),
        recovery_scope: vec![
            "coordination parent card regeneration".into(),
            "preserve historical design evidence".into(),
        ],
        preserved_design: PreservedAuthoredArtifact {
            path: record.design_path.clone(),
            byte_sha256: sha256_hex(&design),
            authored_digest: digest(&design),
        },
        preserved_diagram: PreservedAuthoredArtifact {
            path: record.diagram_path.clone(),
            byte_sha256: sha256_hex(&diagram),
            authored_digest: digest(&diagram),
        },
        graph: DecompositionGraphInput {
            nodes: vec![
                node("child-a", 276, "durable-store-child"),
                node("child-b", 277, "history-api-child"),
                node("child-c", 278, "integration-proof-child"),
                node("parent", 114, "parent-integration-owner"),
            ],
            edges: vec![
                edge("child-a", "child-b"),
                edge("child-b", "child-c"),
                edge("child-c", "parent"),
            ],
            parent_integration_owner: "parent".into(),
            forbidden_cross_child_trust_redefinition: false,
        },
        design_review_recovery: Some(DesignReviewRecoveryTruth {
            previous_review_state: record.design_review.clone(),
            new_review_state: DesignReview::Pending,
            false_reviewer: "operator:planning-1-assignment".into(),
            disposition: "bootstrap assignment was not design approval".into(),
        }),
        replacements: InitializedDecompositionRecoveryReplacement {
            title: "[v0.92][WP-18C.04-parent] Durable history coordination".into(),
            slug: "wp18c04-durable-history-coordination".into(),
            version: "v0.92.0".into(),
            goal: "Coordinate decomposed durable-history child issues.".into(),
            required_outcome: "Parent records child topology and integration evidence only.".into(),
            declared_scope: vec!["coordination parent cards".into()],
            authority_boundary: vec!["typed initialized recovery only".into()],
            initial_assumptions: vec!["child issues own product changes".into()],
            operator_constraints: vec!["do not mutate #112".into()],
            task_boundary: "Recover parent planning truth without product edits.".into(),
            deliverables: vec![
                "coordination-only card packet".into(),
                "csdlc-v2/tests/initialized_decomposition_recovery.rs".into(),
            ],
            acceptance_criteria: vec![
                "AC-1: parent is coordination-only".into(),
                "AC-2: history is preserved".into(),
                "AC-3: review can proceed".into(),
            ],
            dependencies: vec!["#276".into(), "#277".into(), "#278".into()],
            repo_inputs: vec![
                "design/issue-114.md".into(),
                "csdlc-v2/tests/initialized_decomposition_recovery.rs".into(),
            ],
            non_goals: vec!["product implementation".into()],
            plan_summary: "Recover initialized parent card truth.".into(),
            plan_steps: vec![
                step("S1", "preserve history", "AC-1"),
                step("S2", "validate topology", "AC-2"),
                step("S3", "obtain review", "AC-3"),
            ],
            affected_areas: vec![
                ".csdlc/issues/114".into(),
                "csdlc-v2/tests/initialized_decomposition_recovery.rs".into(),
            ],
            invariants: vec!["branch/worktree remain null".into()],
            risks: vec!["stale card truth".into()],
            stop_conditions: vec!["review fails".into()],
            replan_triggers: vec!["child topology changes".into()],
            validation_summary: "Run focused initialized recovery proof.".into(),
            validation_lanes: vec![focused_lane()],
            failure_policy: "Fail closed on stale CAS or unsafe topology.".into(),
            review_scope: "recovered initialized coordination parent".into(),
            review_prompts: vec!["Review recovered parent truth.".into()],
            residual_risk: Vec::new(),
            sor_summary: "Preparation-only initialized recovery; no execution.".into(),
            sor_artifacts: vec![".csdlc/issues/114".into()],
            sor_validation: Vec::new(),
            sor_follow_ups: vec!["children remain separately owned".into()],
        },
        fail_at,
    }
}

fn focused_lane() -> ValidationLane {
    ValidationLane {
        lane: "initialized-decomposition-recovery-focused".into(),
        proof_role: "exact focused initialized recovery proof".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into(), "AC-3".into()],
        deterministic: true,
        resource_profile: ResourceProfile::Medium,
        budget_seconds: 600,
        budget_tokens: 6_000,
        argv: vec![
            "cargo".into(),
            "test".into(),
            "--locked".into(),
            "--manifest-path".into(),
            "csdlc-v2/Cargo.toml".into(),
            "--test".into(),
            "initialized_decomposition_recovery".into(),
        ],
        parallel_group: "csdlc-v2".into(),
        defer_reason: None,
    }
}

fn node(id: &str, issue: u64, role: &str) -> DecompositionGraphNode {
    DecompositionGraphNode {
        node_id: id.into(),
        issue,
        role: role.into(),
        repository: "agent-logic/agent-design-language".into(),
        in_scope: true,
    }
}

fn edge(from: &str, to: &str) -> DecompositionGraphEdge {
    DecompositionGraphEdge {
        from: from.into(),
        to: to.into(),
        relation: "must_precede".into(),
    }
}

fn step(id: &str, action: &str, ac: &str) -> PlanStep {
    PlanStep {
        id: id.into(),
        action: action.into(),
        acceptance_ids: vec![ac.into()],
        status: StepStatus::Pending,
    }
}

fn command(root: &Path, program: &str, args: &[&str]) -> Output {
    Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"))
}

fn copy_dir_all(source: &Path, dest: &Path) {
    fs::create_dir_all(dest).expect("create copied directory");
    for entry in fs::read_dir(source).expect("read source directory") {
        let entry = entry.expect("source entry");
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        let metadata = entry.metadata().expect("entry metadata");
        if metadata.is_dir() {
            copy_dir_all(&source_path, &dest_path);
        } else if metadata.is_file() {
            fs::copy(&source_path, &dest_path).expect("copy file");
        } else {
            panic!(
                "unsupported golden fixture entry: {}",
                source_path.display()
            );
        }
    }
}

fn tree_digest(root: &Path) -> String {
    let mut files = Vec::new();
    collect_files(root, root, &mut files);
    let mut hasher = Sha256::new();
    for (relative, digest) in files {
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(digest.as_bytes());
        hasher.update([0]);
    }
    let digest = hasher.finalize();
    hex(digest.as_ref())
}

fn collect_files(base: &Path, path: &Path, files: &mut Vec<(String, String)>) {
    let metadata = fs::symlink_metadata(path).expect("tree metadata");
    if metadata.file_type().is_symlink() {
        panic!("golden fixture contains symlink: {}", path.display());
    }
    if metadata.is_file() {
        let relative = path
            .strip_prefix(base)
            .expect("relative path")
            .to_string_lossy()
            .into_owned();
        files.push((relative, sha256_hex(&fs::read(path).expect("tree file"))));
        files.sort();
        return;
    }
    if metadata.is_dir() {
        for entry in fs::read_dir(path).expect("tree directory") {
            collect_files(base, &entry.expect("tree entry").path(), files);
        }
        files.sort();
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex(digest.as_ref())
}

fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(64);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut out, "{byte:02x}").expect("format SHA-256");
    }
    out
}
