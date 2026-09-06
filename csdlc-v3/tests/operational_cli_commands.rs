use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use csdlc_v3::commands::{
    local::{required_local_commands, LocalPreparationRequest, OperationalLocalContext},
    remote::{
        canonical_authority_selector_digest, OperationalRemoteDispatchRequest,
        OperationalRemoteOperation, RemoteRouteRequest,
    },
    terminal::{CutoverDecisionRequest, CutoverOperation, TerminalRouteRequest},
};
use serde_json::json;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn fixture(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/operational-cli-tests")
        .join(format!("{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join(".git")).expect("git marker");
    fs::create_dir_all(root.join("csdlc-v2/operator")).expect("selector parent");
    fs::create_dir_all(root.join(".adl")).expect("policy parent");
    fs::write(
        root.join("csdlc-v2/operator/generation-selector.json"),
        br#"{"schema":"csdlc.generation_selector.v1","default_generation":"v2","opted_in_issues":[]}"#,
    )
    .expect("selector");
    fs::write(
        root.join(".adl/worktree-policy.json"),
        format!(
            "{{\"schema\":\"adl.worktree_policy.v1\",\"required_parent\":{}}}",
            serde_json::to_string(&root.join("worktrees").to_string_lossy()).unwrap()
        ),
    )
    .expect("policy");
    root
}

fn run(args: &[&str], cwd: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run csdlc CLI")
}

struct OperationalFixture {
    root: PathBuf,
    request_path: PathBuf,
    context_path: PathBuf,
    registrations_path: PathBuf,
    request: LocalPreparationRequest,
}

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_AUTHOR_NAME", "C-SDLC operational test")
        .env("GIT_AUTHOR_EMAIL", "csdlc@example.invalid")
        .env("GIT_COMMITTER_NAME", "C-SDLC operational test")
        .env("GIT_COMMITTER_EMAIL", "csdlc@example.invalid")
        .output()
        .expect("run fixture git");
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn operational_fixture(name: &str) -> OperationalFixture {
    let root = fixture(name);
    fs::remove_dir_all(root.join(".git")).expect("replace git marker");
    git(&root, &["init", "--quiet"]);
    fs::write(root.join("tracked"), "fixture\n").unwrap();
    git(&root, &["add", "tracked"]);
    git(&root, &["commit", "--quiet", "-m", "fixture"]);
    let exact_head = git(&root, &["rev-parse", "HEAD"]);
    let approval_path = root.join(".csdlc/evidence/505/cutover-approval.json");
    fs::create_dir_all(approval_path.parent().unwrap()).unwrap();
    let approval = serde_json::to_vec_pretty(&json!({
        "schema": "csdlc.v3.cutover_approval.v1",
        "authority_issue": 505,
        "repository": "agent-logic/agent-design-language",
        "decision": "approved",
        "exact_head": exact_head,
        "selector_metadata_digest": "pre-cutover-selector"
    }))
    .unwrap();
    fs::write(&approval_path, &approval).unwrap();
    let approval_digest = blake3::hash(&approval).to_hex().to_string();
    let selector_path = root.join("csdlc-v2/operator/generation-selector.json");
    let selector = serde_json::to_vec_pretty(&json!({
        "schema": "csdlc.generation_selector.v2",
        "default_generation": "v3",
        "operational_authority": "csdlc-v3",
        "authority_issue": 505,
        "exact_review_sha": exact_head,
        "readiness_evidence_digest": "fixture-readiness",
        "approval_evidence_digest": approval_digest
    }))
    .unwrap();
    fs::write(&selector_path, &selector).unwrap();
    let selector_digest = blake3::hash(&selector).to_hex().to_string();
    fs::create_dir_all(root.join("worktrees")).unwrap();
    let request = LocalPreparationRequest {
        issue: 505,
        title: "C-SDLC v3 crash recovery".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: format!("codex/505-{name}"),
        worktree: root
            .join("worktrees/issue-505")
            .to_string_lossy()
            .into_owned(),
        registry_version: "1.0.3".into(),
        expected_lifecycle_digest: None,
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: BTreeMap::new(),
    };
    let context = OperationalLocalContext {
        repository_root: root.clone(),
        state_root: root.join(".csdlc"),
        allowed_worktree_parent: root.join("worktrees"),
        expected_authority_selector_digest: selector_digest,
        cutover_approval_path: ".csdlc/evidence/505/cutover-approval.json".into(),
        expected_cutover_approval_digest: approval_digest,
        expected_head_sha: exact_head,
        expected_lifecycle_digest: None,
    };
    let request_path = root.join("operational-request.json");
    let context_path = root.join("operational-context.json");
    let registrations_path = root.join("registrations.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    fs::write(&context_path, serde_json::to_vec(&context).unwrap()).unwrap();
    fs::write(&registrations_path, b"[]").unwrap();
    OperationalFixture {
        root,
        request_path,
        context_path,
        registrations_path,
        request,
    }
}

fn run_operational(
    fixture: &OperationalFixture,
    route: &str,
    crash: Option<&str>,
) -> std::process::Output {
    let registry = repo_root().join("docs/templates/prompts/current.json");
    let mut command = Command::new(env!("CARGO_BIN_EXE_csdlc"));
    command
        .current_dir(repo_root())
        .arg(route)
        .arg("--request")
        .arg(&fixture.request_path)
        .arg("--registry")
        .arg(registry)
        .arg("--registrations")
        .arg(&fixture.registrations_path)
        .arg("--operational-context")
        .arg(&fixture.context_path);
    if let Some(point) = crash {
        command.env("CSDLC_V3_TEST_CRASH_POINT", point);
    }
    command.output().expect("run operational CLI")
}

fn initialize_operational_fixture(fixture: &mut OperationalFixture) -> String {
    let output = run_operational(fixture, "issue", None);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let digest = value["result"]["digest"].as_str().unwrap().to_owned();
    fixture.request.expected_lifecycle_digest = Some(digest.clone());
    let mut context: OperationalLocalContext =
        serde_json::from_slice(&fs::read(&fixture.context_path).unwrap()).unwrap();
    context.expected_lifecycle_digest = Some(digest.clone());
    fs::write(
        &fixture.request_path,
        serde_json::to_vec(&fixture.request).unwrap(),
    )
    .unwrap();
    fs::write(&fixture.context_path, serde_json::to_vec(&context).unwrap()).unwrap();
    digest
}

#[test]
fn bind_recovers_after_process_exit_following_git_side_effect() {
    let mut fixture = operational_fixture("bind-crash-recovery");
    initialize_operational_fixture(&mut fixture);
    let crashed = run_operational(&fixture, "bind", Some("bind_after_git"));
    assert_eq!(
        crashed.status.code(),
        Some(91),
        "{}",
        String::from_utf8_lossy(&crashed.stderr)
    );
    assert!(Path::new(&fixture.request.worktree).is_dir());
    assert!(fixture.root.join(".csdlc/transactions/505.json").is_file());

    let recovered = run_operational(&fixture, "bind", None);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&recovered.stdout).unwrap();
    assert_eq!(value["result"]["phase"], "bound");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(
            &fs::read(fixture.root.join(".csdlc/issues/505/index.json")).unwrap()
        )
        .unwrap()["phase"],
        "bound"
    );
    assert!(!fixture.root.join(".csdlc/transactions/505.json").exists());
}

#[test]
fn edit_recovers_after_process_exit_between_directory_swaps() {
    let mut fixture = operational_fixture("edit-crash-recovery");
    initialize_operational_fixture(&mut fixture);
    fixture
        .request
        .card_updates
        .insert("sip".into(), json!({"title": "Recovered atomic edit"}));
    fs::write(
        &fixture.request_path,
        serde_json::to_vec(&fixture.request).unwrap(),
    )
    .unwrap();
    let crashed = run_operational(&fixture, "edit", Some("after_backup_rename"));
    assert_eq!(
        crashed.status.code(),
        Some(91),
        "{}",
        String::from_utf8_lossy(&crashed.stderr)
    );
    assert!(!fixture.root.join(".csdlc/issues/505").exists());
    assert!(fixture.root.join(".csdlc/transactions/505.json").is_file());

    let recovered = run_operational(&fixture, "edit", None);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    let values: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.root.join(".csdlc/issues/505/cards/sip.values.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(values["title"], "Recovered atomic edit");
    assert!(!fixture.root.join(".csdlc/transactions/505.json").exists());
}

#[test]
fn local_operational_route_is_reachable_and_fails_closed_under_v2_selector() {
    let fixture = fixture("local-v2-fence");
    let request = LocalPreparationRequest {
        issue: 505,
        title: "C-SDLC v3 authority transition".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: "codex/505-v3-f-authority-transition-decision-exec".into(),
        worktree: fixture
            .join("worktrees/issue-505")
            .to_string_lossy()
            .into_owned(),
        registry_version: "1.0.3".into(),
        expected_lifecycle_digest: None,
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: BTreeMap::new(),
    };
    let request_path = fixture.join("local.json");
    let registrations_path = fixture.join("registrations.json");
    let context_path = fixture.join("operational-context.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    fs::write(&registrations_path, b"[]").unwrap();

    let registry = repo_root().join("docs/templates/prompts/current.json");
    let state_root = fixture.join(".csdlc/v3");
    let local_selector_digest = blake3::hash(
        &fs::read(fixture.join("csdlc-v2/operator/generation-selector.json"))
            .expect("selector bytes"),
    )
    .to_hex()
    .to_string();
    let context = OperationalLocalContext {
        repository_root: fixture.clone(),
        state_root: state_root.clone(),
        allowed_worktree_parent: fixture.join("worktrees"),
        expected_authority_selector_digest: local_selector_digest,
        cutover_approval_path: ".csdlc/evidence/505/cutover-approval.json".into(),
        expected_cutover_approval_digest: "0".repeat(64),
        expected_head_sha: "0123456789012345678901234567890123456789".into(),
        expected_lifecycle_digest: None,
    };
    fs::write(&context_path, serde_json::to_vec(&context).unwrap()).unwrap();
    let output = run(
        &[
            "issue",
            "--request",
            request_path.to_str().unwrap(),
            "--registry",
            registry.to_str().unwrap(),
            "--registrations",
            registrations_path.to_str().unwrap(),
            "--repo-root",
            fixture.to_str().unwrap(),
            "--v3-state-root",
            state_root.to_str().unwrap(),
            "--operational-context",
            context_path.to_str().unwrap(),
        ],
        &repo_root(),
    );

    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("canonical_v3_authority_inactive"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!state_root.exists());
}

#[test]
fn remote_operational_dispatch_is_reachable_and_fails_closed_under_v2_selector() {
    let fixture = fixture("remote-v2-fence");
    let remote_selector_digest =
        canonical_authority_selector_digest(&fixture).expect("selector digest");
    let remote = RemoteRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: Some(591),
        actor: Some("worker-8".into()),
        implementer: Some("worker-8".into()),
        reviewer: Some("independent-reviewer".into()),
        review_revision: Some("0123456789012345678901234567890123456789".into()),
        expected_head_sha: Some("0123456789012345678901234567890123456789".into()),
        head_sha: Some("0123456789012345678901234567890123456789".into()),
        mode: None,
        title: None,
        body: None,
        review_present: false,
        typed_review_receipt_path: None,
        typed_review_receipt_digest: None,
        readback_source: None,
        readback_receipt_path: None,
        readback_receipt_digest: None,
        adapter_receipt_path: None,
        adapter_receipt_digest: None,
        closes_issue: None,
        closing_issues: vec![],
        part_of_issue: None,
        credential_names: vec![],
    };
    let dispatch = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: remote_selector_digest,
        exact_review_sha: "0123456789012345678901234567890123456789".into(),
        operation: OperationalRemoteOperation::Review(remote),
    };
    let request_path = fixture.join("remote.json");
    fs::write(&request_path, serde_json::to_vec(&dispatch).unwrap()).unwrap();

    let output = run(
        &[
            "review",
            "--request",
            request_path.to_str().unwrap(),
            "--execute",
        ],
        &fixture,
    );
    assert!(!output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("canonical_v3_authority_inactive"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn proof_and_rollback_commands_reach_real_handlers() {
    let fixture = fixture("proof-rollback");
    let proof_path = fixture.join("proof.json");
    fs::write(
        &proof_path,
        serde_json::to_vec(&json!({
            "issue": 505,
            "repository": "agent-logic/agent-design-language",
            "cutover_issue": 505,
            "operator_approval": null,
            "evidence_root": null,
            "proof": null,
            "shadow": null,
            "soak": null,
            "install": null
        }))
        .unwrap(),
    )
    .unwrap();
    let proof = run(
        &["proof", "--request", proof_path.to_str().unwrap()],
        &fixture,
    );
    assert!(!proof.status.success(), "blocked proof must return nonzero");
    let proof_json: serde_json::Value = serde_json::from_slice(
        String::from_utf8_lossy(&proof.stderr)
            .strip_prefix("csdlc: ")
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    assert_eq!(proof_json["status"], "blocked");
    assert_eq!(proof_json["findings"][0]["code"], "proof_manifest_missing");

    let rollback_path = fixture.join("rollback.json");
    let rollback_request = TerminalRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: None,
        expected_head_sha: None,
        mode: None,
        public_adapter_receipt: None,
        terminal_state: None,
        cleanup: None,
        cutover: Some(CutoverDecisionRequest {
            operator: "".into(),
            approval: "".into(),
            selected_binary_provenance: "".into(),
            rollback_evidence: "".into(),
            undo_boundary: "".into(),
            operation: CutoverOperation::Rollback,
            execute: true,
            repository_root: Some(fixture.clone()),
            selected_binary_path: None,
            authority_selector_path: None,
            install_destination_path: None,
            rollback_receipt_path: None,
            readiness_evidence_path: None,
            readiness_evidence_digest: None,
        }),
        credential_names: vec![],
    };
    fs::write(
        &rollback_path,
        serde_json::to_vec(&rollback_request).unwrap(),
    )
    .unwrap();
    let rollback = run(
        &["rollback", "--request", rollback_path.to_str().unwrap()],
        &fixture,
    );
    assert!(
        !rollback.status.success(),
        "blocked rollback must return nonzero"
    );
    let rollback_json: serde_json::Value = serde_json::from_slice(
        String::from_utf8_lossy(&rollback.stderr)
            .strip_prefix("csdlc: ")
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    assert_eq!(rollback_json["command"], "rollback");
    assert_eq!(rollback_json["result"]["status"], "blocked");
    assert_eq!(rollback_json["performed_mutation"], false);
}
