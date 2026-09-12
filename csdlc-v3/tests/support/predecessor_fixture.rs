//! Isolated historical authority inputs reused from terminal_cleanup_cutover_commands.
//! These synthetic prerequisite receipts are not claimed as newly executed proof.
//! The importing installed tests execute cutover and rollback through the binary.
use csdlc_v3::commands::{
    proof::{ShadowCommandSpec, ShadowGeneration, ShadowNormalizationContract},
    terminal::{
        CutoverDecisionRequest, CutoverOperation, TerminalPublicationMode, TerminalRouteRequest,
    },
};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
fn base_request() -> TerminalRouteRequest {
    TerminalRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 630,
        pull_request: Some(641),
        expected_head_sha: Some("0123456789012345678901234567890123456789".into()),
        mode: Some(TerminalPublicationMode::Closing),
        public_adapter_receipt: None,
        terminal_state: None,
        no_pr_closeout: None,
        cleanup: None,
        cutover: None,
        credential_names: Vec::new(),
    }
}
fn native_review_fixture(root: &Path) -> (Vec<u8>, String, String) {
    let revision = git_output(root, &["rev-parse", "HEAD"]);
    let reviewed_revision = format!(
        "git-blake3:{revision}:{}",
        blake3::hash(revision.as_bytes()).to_hex()
    );
    let reviewer = "fresh-session:11111111-2222-4333-8444-555555555555";
    let mut record = serde_json::json!({
        "schema": "csdlc.v3.native_review_receipt.v1",
        "issue": 505,
        "repository": "agent-logic/agent-design-language",
        "phase": "reviewed",
        "generation": 1,
        "digest": "",
        "review_assignment": {
            "reviewer": reviewer,
            "assigned_by": "operator",
            "revision": reviewed_revision,
            "scope": ["src"]
        },
        "review": {
            "reviewer": reviewer,
            "scope": ["src"],
            "reviewed_revision": reviewed_revision,
            "findings": [],
            "completed": true
        }
    });
    let digest = blake3::hash(&serde_json::to_vec(&record).unwrap())
        .to_hex()
        .to_string();
    record["digest"] = serde_json::Value::String(digest.clone());
    (serde_json::to_vec(&record).unwrap(), digest, revision)
}

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

pub fn write_cutover_fixture(root: &Path, _binary_marker: &[u8]) -> String {
    if root.join(".git").exists() {
        fs::remove_dir_all(root.join(".git")).expect("replace git marker");
    }
    fs::create_dir_all(root).expect("fixture root");
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(root.join("fixture-worktrees")).expect("fixture worktree parent");
    fs::create_dir_all(root.join(".adl")).expect("fixture adl parent");
    fs::write(
        root.join(".adl/worktree-policy.json"),
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.worktree_policy.v1",
            "required_parent": root.join("fixture-worktrees")
        }))
        .unwrap(),
    )
    .expect("fixture worktree policy");
    fs::write(root.join("fixture-base"), "native v3 authority\n").unwrap();
    git(root, &["add", "fixture-base", ".adl/worktree-policy.json"]);
    git(root, &["commit", "-m", "authority fixture"]);
    fs::create_dir_all(root.join("build")).expect("build directory");
    fs::copy(env!("CARGO_BIN_EXE_csdlc"), root.join("build/csdlc"))
        .expect("selected real v3 binary");
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(root.join("build/csdlc"))
            .expect("selected binary metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(root.join("build/csdlc"), permissions)
            .expect("selected binary executable");
    }
    let selected_binary = fs::read(root.join("build/csdlc")).expect("selected binary bytes");
    let selected_binary_digest = blake3::hash(&selected_binary).to_hex().to_string();
    write_generation_selector(root, "v2");
    let (review, review_record_digest, revision) = native_review_fixture(root);
    fs::create_dir_all(root.join(".csdlc/evidence/505")).expect("native review parent");
    fs::write(
        root.join(".csdlc/evidence/505/native-authority-review.json"),
        &review,
    )
    .expect("native review record");
    fs::create_dir_all(root.join(".csdlc/issues/505")).expect("review state parent");
    fs::write(root.join(".csdlc/issues/505/index.json"), &review).expect("native review state");
    let review_ref = serde_json::json!({
        "path": ".csdlc/evidence/505/native-authority-review.json",
        "digest": blake3::hash(&review).to_hex().to_string(),
        "revision": revision
    });
    let canary_ref =
        write_proof_command_fixture(root, "v3-only-canary", "v3-only-canary", &revision);
    let rollback_ref =
        write_proof_command_fixture(root, "rollback-readiness", "rollback-readiness", &revision);
    let rollback_digest = rollback_ref["digest"].as_str().unwrap().to_owned();
    let approval = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.cutover_approval.v1",
        "authority_issue": 505,
        "repository": "agent-logic/agent-design-language",
        "decision": "approved",
        "exact_head": revision,
        "selected_binary_digest": selected_binary_digest,
        "selector_metadata_digest": "pre-cutover-selector",
        "rollback_evidence_digest": rollback_digest,
        "review_record_digest": review_record_digest,
        "reviewer_github_login": "reviewer",
        "review_comment_id": 1001,
        "approval_comment_id": 1002,
        "approved_by": "operator"
    }))
    .unwrap();
    fs::create_dir_all(root.join(".csdlc/evidence/505")).expect("approval parent");
    fs::write(
        root.join(".csdlc/evidence/505/cutover-approval.json"),
        &approval,
    )
    .expect("approval evidence");
    let readiness = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.authority_readiness.v1",
        "authority_issue": 505,
        "selected_binary_digest": selected_binary_digest,
        "selected_revision": revision,
        "canary_proof": canary_ref,
        "review_proof": review_ref,
        "rollback_proof": rollback_ref
    }))
    .unwrap();
    fs::write(
        root.join(".csdlc/evidence/505/authority-readiness.json"),
        &readiness,
    )
    .expect("readiness evidence");
    write_generation_selector(root, "v3");
    blake3::hash(&readiness).to_hex().to_string()
}

fn write_proof_command_fixture(
    root: &Path,
    manifest_id: &str,
    lane: &str,
    revision: &str,
) -> serde_json::Value {
    let source_ref = format!(".csdlc/evidence/505/readiness-source/{manifest_id}.json");
    let source = serde_json::to_vec(&serde_json::json!({
        "manifest_id": manifest_id,
        "revision": revision,
        "result": "pass"
    }))
    .unwrap();
    fs::create_dir_all(root.join(".csdlc/evidence/505/readiness-source"))
        .expect("proof source parent");
    fs::write(root.join(&source_ref), &source).expect("proof source");
    let digest = blake3::hash(&source).to_hex().to_string();
    let registry_ref = ".csdlc/evidence/505/readiness-source/doctor-current-registry.json";
    fs::write(
        root.join(registry_ref),
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.csdlc.prompt_template_registry.v1",
            "csdlc_prompt_template_set": "1.0.4",
            "semver": "1.0.4",
            "status": "active",
            "templates": {
                "sip": {"path": "fixture/templates/sip.md"},
                "stp": {"path": "fixture/templates/stp.md"},
                "spp": {"path": "fixture/templates/spp.md"},
                "vpp": {"path": "fixture/templates/vpp.md"},
                "srp": {"path": "fixture/templates/srp.md"},
                "sor": {"path": "fixture/templates/sor.md"}
            }
        }))
        .unwrap(),
    )
    .expect("fixture prompt registry");
    let request_ref =
        format!(".csdlc/evidence/505/readiness-source/{manifest_id}-doctor-request.json");
    fs::write(
        root.join(&request_ref),
        serde_json::to_vec(&serde_json::json!({
            "issue": 505,
            "title": "C-SDLC v3 authority transition",
            "repository": "agent-logic/agent-design-language",
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": "fixture-worktrees/issue-505",
            "registry_version": "1.0.4",
            "commands": ["prepare_issue", "bind_worktree", "edit_cards", "plan_pvf", "doctor", "schedule", "shepherd", "eligibility"],
            "card_updates": {}
        }))
        .unwrap(),
    )
    .expect("proof command request");
    let registrations_ref = ".csdlc/evidence/505/readiness-source/doctor-registrations.json";
    fs::write(
        root.join(registrations_ref),
        serde_json::to_vec(&serde_json::json!([{
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": "fixture-worktrees/issue-505",
            "primary": false
        }]))
        .unwrap(),
    )
    .expect("proof command registrations");
    let normalization = ShadowNormalizationContract::DoctorIssuePhaseV1;
    let argv = vec![
        "doctor".into(),
        "--request".into(),
        request_ref.clone(),
        "--registry".into(),
        registry_ref.into(),
        "--registrations".into(),
        registrations_ref.into(),
        "--repo-root".into(),
        ".".into(),
    ];
    let command = ShadowCommandSpec {
        generation: ShadowGeneration::V3,
        binary_ref: "build/csdlc".into(),
        argv,
        request_ref,
        timeout_millis: 10_000,
        side_effect_boundary_refs: vec![source_ref.clone()],
        provider_side_effects: false,
    };
    // Model an immutable pre-cutover receipt for the terminal verifier. The
    // operational proof route now rejects unbound construction fixtures; do
    // not reopen that production mutation path just to manufacture test data.
    // Actual command execution and write placement are covered independently
    // by proof_worktree_binding, using authenticated linked-worktree context.
    let path = format!(".csdlc/evidence/505/v3-proof/{manifest_id}.json");
    let request_digest = blake3::hash(&fs::read(root.join(&command.request_ref)).unwrap())
        .to_hex()
        .to_string();
    let binary_digest = blake3::hash(&fs::read(root.join(&command.binary_ref)).unwrap())
        .to_hex()
        .to_string();
    let receipt = serde_json::json!({
        "schema": "csdlc.v3.proof_receipt.v2", "issue": 505,
        "repository": "agent-logic/agent-design-language", "manifest_id": manifest_id,
        "lane": lane, "deterministic": true,
        "source_evidence_ref": source_ref, "source_evidence_digest": digest,
        "normalization": normalization, "command": command,
        "request_evidence": {"ref": command.request_ref, "digest": request_digest},
        "binary": {"identity": command.binary_ref, "digest": binary_digest},
        "exit": {"code": 0, "success": true},
        "normalized_output": {"command": "doctor", "issue": 505, "phase": "bound"},
        "side_effect_boundary": [{"changed": false}], "provider_side_effects": false
    });
    fs::create_dir_all(root.join(".csdlc/evidence/505/v3-proof")).unwrap();
    let bytes = serde_json::to_vec(&receipt).unwrap();
    fs::write(root.join(&path), &bytes).unwrap();

    serde_json::json!({
        "path": path,
        "digest": blake3::hash(&bytes).to_hex().to_string(),
        "revision": revision
    })
}

pub fn cutover_request(
    root: &Path,
    readiness_digest: String,
    operation: CutoverOperation,
) -> TerminalRouteRequest {
    let readiness: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/505/authority-readiness.json")).unwrap(),
    )
    .unwrap();
    let revision = readiness["selected_revision"].as_str().unwrap();
    let mut request = base_request();
    request.issue = 505;
    request.cutover = Some(CutoverDecisionRequest {
        operator: "operator".into(),
        approval: ".csdlc/evidence/505/cutover-approval.json".into(),
        selected_binary_provenance: format!("git:{revision}"),
        rollback_evidence: ".csdlc/evidence/505/v3-proof/rollback-readiness.json".into(),
        undo_boundary: "fail-closed before irreversible mutation".into(),
        operation,
        execute: true,
        repository_root: Some(root.to_path_buf()),
        selected_binary_path: Some(PathBuf::from("build/csdlc")),
        authority_selector_path: Some(PathBuf::from("csdlc-v3/operator/authority-selector.json")),
        install_destination_path: Some(PathBuf::from(".adl/bin/csdlc")),
        rollback_receipt_path: Some(PathBuf::from(".csdlc/evidence/505/cutover-receipt.json")),
        readiness_evidence_path: Some(PathBuf::from(
            ".csdlc/evidence/505/authority-readiness.json",
        )),
        readiness_evidence_digest: Some(readiness_digest),
    });
    request
}

fn write_generation_selector(repository_root: &Path, generation: &str) {
    let path = repository_root.join("csdlc-v3/operator/authority-selector.json");
    fs::create_dir_all(path.parent().unwrap()).expect("selector parent");
    let selector = if generation == "v3" {
        let reviewed_head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        git(
            repository_root,
            &["commit", "--allow-empty", "-m", "V3-F cutover (#591)"],
        );
        let merge_commit = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        let terminal = serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.v3.terminal_receipt.v1", "repository":"agent-logic/agent-design-language",
            "issue":505, "pull_request":591, "head_sha":reviewed_head,
            "disposition":"closed_out", "state_digest":"fixture"
        })).unwrap();
        fs::create_dir_all(repository_root.join(".csdlc/evidence/505")).unwrap();
        fs::write(
            repository_root.join(".csdlc/evidence/505/terminal-receipt.json"),
            &terminal,
        )
        .unwrap();
        let observation = serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.github_pr_state.v1", "repository":"agent-logic/agent-design-language",
            "pull_request":591, "base_ref":"main", "head_sha":reviewed_head,
            "merge_commit_sha":merge_commit, "state":"closed", "merged":true,
            "linked_issue":505, "linkage_source":"github_closing_issues_references",
            "observation_authority":"typed-csdlc-github-pr-authenticated-readback"
        }))
        .unwrap();
        fs::write(
            repository_root.join("csdlc-v3/operator/native-authority-pr-observation.json"),
            &observation,
        )
        .unwrap();
        let receipt = serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "csdlc.v3.native_authority_receipt.v1", "authority_issue": 505,
            "authority_pull_request": 591, "reviewed_head": reviewed_head,
            "merge_commit": merge_commit, "pr_observation_path":"csdlc-v3/operator/native-authority-pr-observation.json",
            "pr_observation_digest":blake3::hash(&observation).to_hex().to_string(), "terminal_receipt_path": ".csdlc/evidence/505/terminal-receipt.json",
            "terminal_receipt_digest": blake3::hash(&terminal).to_hex().to_string(), "source_selector_schema": "csdlc.generation_selector.v2",
            "source_selector_digest": format!("sha256:{}", "3".repeat(64)),
            "operational_authority": "csdlc-v3", "review_authority": "typed-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "remote_reconciliation": "canonical-terminal-receipt-and-git-objects"
        })).unwrap();
        fs::write(
            repository_root.join("csdlc-v3/operator/native-authority-receipt.json"),
            &receipt,
        )
        .unwrap();
        serde_json::json!({
            "schema": "csdlc.v3.authority_selector.v1",
            "generation": "v3",
            "operational_authority": "csdlc-v3",
            "authority_issue": 505,
            "authority_pull_request": 591,
            "review_authority": "typed-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "receipt_path": "csdlc-v3/operator/native-authority-receipt.json",
            "receipt_digest": blake3::hash(&receipt).to_hex().to_string()
        })
    } else {
        serde_json::json!({
            "schema": "csdlc.v3.authority_selector.v1",
            "generation": "rollback",
            "operational_authority": "suspended",
            "authority_issue": 505,
            "authority_pull_request": 591
        })
    };
    fs::write(path, serde_json::to_vec_pretty(&selector).unwrap()).expect("generation selector");
    if generation == "v3" {
        git(
            repository_root,
            &[
                "add",
                "csdlc-v3/operator/authority-selector.json",
                "csdlc-v3/operator/native-authority-receipt.json",
                "csdlc-v3/operator/native-authority-pr-observation.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        git(
            repository_root,
            &["commit", "-m", "activate fixture selector"],
        );
        let head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        git(
            repository_root,
            &["update-ref", "refs/remotes/origin/main", &head],
        );
    }
}

pub fn publish_v2_rollback(repository_root: &Path) {
    write_generation_selector(repository_root, "v2");
    git(
        repository_root,
        &["add", "csdlc-v3/operator/authority-selector.json"],
    );
    git(
        repository_root,
        &["commit", "-m", "revert tracked selector to v2"],
    );
    let head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
    git(
        repository_root,
        &["update-ref", "refs/remotes/origin/main", &head],
    );
}

fn git(root: &Path, args: &[&str]) {
    let _ = git_output(root, args);
}
fn git_stdout(root: &Path, args: &[&str]) -> String {
    git_output(root, args)
}
