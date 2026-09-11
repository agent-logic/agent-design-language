use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use csdlc_v3::commands::{
    local::{required_local_commands, LocalPreparationRequest},
    remote::{
        canonical_authority_selector_digest, GithubMutation, GithubMutationRequest,
        IssueCloseDisposition, IssueCloseStateReason, OperationalRemoteDispatchRequest,
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
    fs::create_dir_all(root.join("csdlc-v3/operator")).expect("selector parent");
    fs::create_dir_all(root.join(".adl")).expect("policy parent");
    fs::write(
        root.join("csdlc-v3/operator/authority-selector.json"),
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

fn run_with_env(
    args: &[&str],
    cwd: &std::path::Path,
    envs: &[(&str, &std::ffi::OsStr)],
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_csdlc"));
    command.args(args).current_dir(cwd);
    for (name, value) in envs {
        command.env(name, value);
    }
    command.output().expect("run csdlc CLI")
}

struct OperationalFixture {
    root: PathBuf,
    request_path: PathBuf,
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
    fs::create_dir_all(root.join(".adl")).unwrap();
    fs::create_dir_all(root.join("worktrees")).unwrap();
    fs::write(
        root.join(".adl/worktree-policy.json"),
        serde_json::to_vec(&json!({
            "schema": "adl.worktree_policy.v1",
            "required_parent": root.join("worktrees")
        }))
        .unwrap(),
    )
    .unwrap();
    let reviewed_head = git(&root, &["rev-parse", "HEAD"]);
    git(
        &root,
        &[
            "commit",
            "--quiet",
            "--allow-empty",
            "-m",
            "V3-F cutover (#591)",
        ],
    );
    let merge_commit = git(&root, &["rev-parse", "HEAD"]);
    let terminal = serde_json::to_vec_pretty(&json!({
        "schema":"csdlc.v3.terminal_receipt.v1", "repository":"agent-logic/agent-design-language",
        "issue":505, "pull_request":591, "head_sha":reviewed_head,
        "disposition":"closed_out", "state_digest":"fixture"
    }))
    .unwrap();
    fs::create_dir_all(root.join(".csdlc/evidence/505")).unwrap();
    fs::write(
        root.join(".csdlc/evidence/505/terminal-receipt.json"),
        &terminal,
    )
    .unwrap();
    let observation = serde_json::to_vec_pretty(&json!({
        "schema":"csdlc.github_pr_state.v1", "repository":"agent-logic/agent-design-language",
        "pull_request":591, "base_ref":"main", "head_sha":reviewed_head,
        "merge_commit_sha":merge_commit, "state":"closed", "merged":true,
        "linked_issue":505, "linkage_source":"github_closing_issues_references",
        "observation_authority":"typed-csdlc-github-pr-authenticated-readback"
    }))
    .unwrap();
    fs::write(
        root.join("csdlc-v3/operator/native-authority-pr-observation.json"),
        &observation,
    )
    .unwrap();
    let receipt_path = root.join("csdlc-v3/operator/native-authority-receipt.json");
    let receipt = serde_json::to_vec_pretty(&json!({
        "schema": "csdlc.v3.native_authority_receipt.v1", "authority_issue": 505,
        "authority_pull_request": 591, "reviewed_head": reviewed_head,
        "merge_commit": merge_commit, "pr_observation_path":"csdlc-v3/operator/native-authority-pr-observation.json",
        "pr_observation_digest":blake3::hash(&observation).to_hex().to_string(), "terminal_receipt_path": ".csdlc/evidence/505/terminal-receipt.json",
        "terminal_receipt_digest": blake3::hash(&terminal).to_hex().to_string(), "source_selector_schema": "csdlc.generation_selector.v2",
        "source_selector_digest": format!("sha256:{}", "3".repeat(64)),
        "operational_authority": "csdlc-v3", "review_authority": "typed-exact-head",
        "approval_authority": "merged-pr-591-closed-issue-505",
        "remote_reconciliation": "canonical-terminal-receipt-and-git-objects"
    }))
    .unwrap();
    fs::write(&receipt_path, &receipt).unwrap();
    let selector_path = root.join("csdlc-v3/operator/authority-selector.json");
    let selector = serde_json::to_vec_pretty(&json!({
        "schema": "csdlc.v3.authority_selector.v1",
        "generation": "v3",
        "operational_authority": "csdlc-v3",
        "authority_issue": 505,
        "authority_pull_request": 591,
        "review_authority": "typed-exact-head",
        "approval_authority": "merged-pr-591-closed-issue-505",
        "receipt_path": "csdlc-v3/operator/native-authority-receipt.json",
        "receipt_digest": blake3::hash(&receipt).to_hex().to_string()
    }))
    .unwrap();
    fs::write(&selector_path, &selector).unwrap();
    git(
        &root,
        &[
            "add",
            ".adl/worktree-policy.json",
            "csdlc-v3/operator/authority-selector.json",
            "csdlc-v3/operator/native-authority-receipt.json",
            "csdlc-v3/operator/native-authority-pr-observation.json",
            ".csdlc/evidence/505/terminal-receipt.json",
        ],
    );
    git(&root, &["commit", "--quiet", "-m", "activate v3"]);
    let exact_head = git(&root, &["rev-parse", "HEAD"]);
    git(
        &root,
        &["update-ref", "refs/remotes/origin/main", &exact_head],
    );
    let request = LocalPreparationRequest {
        issue: 505,
        title: "C-SDLC v3 crash recovery".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: format!("codex/505-{name}"),
        worktree: root
            .join("worktrees/issue-505")
            .to_string_lossy()
            .into_owned(),
        registry_version: "1.0.5".into(),
        expected_lifecycle_digest: None,
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: BTreeMap::new(),
    };
    let request_path = root.join("operational-request.json");
    let registrations_path = root.join("registrations.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    fs::write(&registrations_path, b"[]").unwrap();
    OperationalFixture {
        root,
        request_path,
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
        .arg("--repo-root")
        .arg(&fixture.root);
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
    fs::write(
        &fixture.request_path,
        serde_json::to_vec(&fixture.request).unwrap(),
    )
    .unwrap();
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
    assert!(fixture
        .root
        .join(".git/csdlc-v3/local/transactions/505.json")
        .is_file());

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
            &fs::read(Path::new(&fixture.request.worktree).join(".csdlc/issues/505/index.json"))
                .unwrap()
        )
        .unwrap()["phase"],
        "bound"
    );
    assert!(!fixture.root.join(".csdlc/issues/505").exists());
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/local/transactions/505.json")
        .exists());
}

#[test]
fn bind_recovers_after_branch_creation_before_worktree_registration() {
    let mut fixture = operational_fixture("bind-branch-crash-recovery");
    initialize_operational_fixture(&mut fixture);
    let crashed = run_operational(&fixture, "bind", Some("bind_after_branch_creation"));
    assert_eq!(crashed.status.code(), Some(91));
    assert!(!Path::new(&fixture.request.worktree).exists());

    let recovered = run_operational(&fixture, "bind", None);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    assert_eq!(
        git(
            &fixture.root,
            &["-C", &fixture.request.worktree, "rev-parse", "HEAD"]
        ),
        git(&fixture.root, &["rev-parse", "HEAD"])
    );
    assert!(Path::new(&fixture.request.worktree)
        .join(".csdlc/issues/505/index.json")
        .is_file());
    assert!(!fixture.root.join(".csdlc/issues/505").exists());
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
    assert!(fixture
        .root
        .join(".git/csdlc-v3/local/transactions/505.json")
        .is_file());

    let recovered = run_operational(&fixture, "edit", None);
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    let values: serde_json::Value = serde_json::from_slice(
        &fs::read(
            fixture
                .root
                .join(".git/csdlc-v3/local/issues/505/cards/sip.values.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(values["title"], "Recovered atomic edit");
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/local/transactions/505.json")
        .exists());
}

#[test]
fn v2_selector_keeps_named_local_cli_in_construction_mode() {
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
        registry_version: "1.0.5".into(),
        expected_lifecycle_digest: None,
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: BTreeMap::new(),
    };
    let request_path = fixture.join("local.json");
    let registrations_path = fixture.join("registrations.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    fs::write(
        &registrations_path,
        serde_json::to_vec(&json!([{
            "branch": request.branch,
            "worktree": request.worktree,
            "primary": false
        }]))
        .unwrap(),
    )
    .unwrap();

    let registry = repo_root().join("docs/templates/prompts/current.json");
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
        ],
        &repo_root(),
    );

    assert!(output.status.success(), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["operational_authority"], false);
    assert_eq!(report["writes_v3_state"], false);
}

#[test]
fn remote_operational_dispatch_is_reachable_and_fails_closed_under_v2_selector() {
    let fixture = fixture("remote-v2-fence");
    let remote_selector_digest =
        csdlc_v3::commands::remote::canonical_authority_selector_digest(&fixture).unwrap();
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
fn simple_issue_create_projects_github_style_flags_into_typed_dispatch() {
    let exact_head = "0123456789012345678901234567890123456789";
    let output = run(
        &[
            "github-issue",
            "create",
            "--repo",
            "agent-logic/agent-design-language",
            "--title",
            "A bounded issue",
            "--body",
            "One concrete outcome",
            "--label",
            "type:task",
            "--label",
            "area:tools",
            "--assignee",
            "octocat",
            "--milestone",
            "1",
            "--expected-head",
            exact_head,
        ],
        &repo_root(),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let dispatch: OperationalRemoteDispatchRequest =
        serde_json::from_slice(&output.stdout).expect("typed dispatch JSON");
    assert_eq!(dispatch.exact_review_sha, exact_head);
    let OperationalRemoteOperation::GithubMutation(request) = dispatch.operation else {
        panic!("simple form must produce the shared GitHub mutation operation")
    };
    assert_eq!(request.repository, "agent-logic/agent-design-language");
    assert_eq!(request.issue, 0);
    assert_eq!(request.expected_head_sha, exact_head);
    assert_eq!(request.credential_names, ["GITHUB_TOKEN"]);
    assert_eq!(
        request.mutation,
        GithubMutation::IssueCreate {
            title: "A bounded issue".into(),
            body: "One concrete outcome".into(),
            labels: vec!["type:task".into(), "area:tools".into()],
            assignees: vec!["octocat".into()],
            milestone: Some(1),
        }
    );
}

#[test]
fn simple_issue_create_supports_body_file_and_rejects_ambiguous_or_invalid_input() {
    let fixture = fixture("simple-issue-create-inputs");
    let body_path = fixture.join("body.md");
    fs::write(&body_path, "Body from file\n").unwrap();
    let exact_head = "0123456789012345678901234567890123456789";
    let body_file = run(
        &[
            "github-issue",
            "create",
            "--repo",
            "agent-logic/agent-design-language",
            "--title",
            "File body",
            "--body-file",
            body_path.to_str().unwrap(),
            "--expected-head",
            exact_head,
        ],
        &repo_root(),
    );
    assert!(body_file.status.success(), "{body_file:?}");
    let dispatch: OperationalRemoteDispatchRequest =
        serde_json::from_slice(&body_file.stdout).unwrap();
    let OperationalRemoteOperation::GithubMutation(request) = dispatch.operation else {
        panic!("typed GitHub mutation expected")
    };
    assert!(
        matches!(request.mutation, GithubMutation::IssueCreate { body, .. } if body == "Body from file\n")
    );

    for args in [
        vec![
            "github-issue",
            "create",
            "--repo",
            "owner/repo",
            "--title",
            "x",
            "--body",
            "inline",
            "--body-file",
            body_path.to_str().unwrap(),
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "create",
            "--repo",
            "owner/repo",
            "--title",
            " ",
            "--body",
            "body",
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "create",
            "--repo",
            "owner/repo",
            "--title",
            "x",
            "--body",
            "body",
            "--expected-head",
            "not-a-sha",
        ],
    ] {
        let rejected = run(&args, &repo_root());
        assert!(
            !rejected.status.success(),
            "invalid args accepted: {args:?}"
        );
    }
}

#[test]
fn simple_issue_close_emits_typed_duplicate_close_dispatch() {
    let exact_head = "0123456789012345678901234567890123456789";
    let output = run(
        &[
            "github-issue",
            "close",
            "--repo",
            "agent-logic/agent-design-language",
            "--issue",
            "792",
            "--disposition",
            "duplicate",
            "--duplicate-of",
            "791",
            "--rationale",
            "accidental retry duplicate",
            "--body",
            "Original issue body",
            "--expected-head",
            exact_head,
        ],
        &repo_root(),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let dispatch: OperationalRemoteDispatchRequest =
        serde_json::from_slice(&output.stdout).expect("typed dispatch JSON");
    assert_eq!(dispatch.exact_review_sha, exact_head);
    let OperationalRemoteOperation::GithubMutation(request) = dispatch.operation else {
        panic!("simple close must produce the shared GitHub mutation operation")
    };
    assert_eq!(request.repository, "agent-logic/agent-design-language");
    assert_eq!(request.issue, 792);
    assert_eq!(request.pull_request, None);
    assert_eq!(request.expected_head_sha, exact_head);
    assert_eq!(request.credential_names, ["GITHUB_TOKEN"]);
    assert_eq!(
        request.mutation,
        GithubMutation::IssueClose {
            rationale: "accidental retry duplicate".into(),
            current_body: "Original issue body".into(),
            disposition: IssueCloseDisposition::Duplicate,
            duplicate_of: Some(791),
            github_state_reason: Some(IssueCloseStateReason::NotPlanned),
        }
    );
}

#[test]
fn simple_issue_close_rejects_ambiguous_or_invalid_input() {
    let exact_head = "0123456789012345678901234567890123456789";
    for args in [
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "0",
            "--disposition",
            "no-op",
            "--rationale",
            "no-op",
            "--body",
            "Original",
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "2",
            "--disposition",
            "duplicate",
            "--rationale",
            "missing duplicate owner",
            "--body",
            "Original",
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "2",
            "--disposition",
            "bogus",
            "--rationale",
            "bad disposition",
            "--body",
            "Original",
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "2",
            "--disposition",
            "no-op",
            "--rationale",
            " ",
            "--body",
            "Original",
            "--expected-head",
            exact_head,
        ],
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "2",
            "--disposition",
            "no-op",
            "--rationale",
            "bad head",
            "--body",
            "Original",
            "--expected-head",
            "not-a-sha",
        ],
        vec![
            "github-issue",
            "close",
            "--repo",
            "owner/repo",
            "--issue",
            "2",
            "--disposition",
            "no-op",
            "--rationale",
            "already marked",
            "--body",
            "<!-- csdlc-v3-operation:abc -->",
            "--expected-head",
            exact_head,
        ],
    ] {
        let rejected = run(&args, &repo_root());
        assert!(
            !rejected.status.success(),
            "invalid args accepted: {args:?}"
        );
    }
}

#[test]
fn simple_issue_create_execute_fails_closed_under_v2_authority() {
    let fixture = fixture("simple-issue-create-v2-fence");
    let output = run(
        &[
            "github-issue",
            "create",
            "--repo",
            "agent-logic/agent-design-language",
            "--title",
            "Must not be created",
            "--body",
            "Authority is inactive",
            "--expected-head",
            "0123456789012345678901234567890123456789",
            "--execute",
        ],
        &fixture,
    );
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("canonical_v3_authority_inactive"));
}

#[test]
fn executable_github_routes_reject_wrong_mutation_family_before_dispatch() {
    let fixture = operational_fixture("github-route-family");
    let head = git(&fixture.root, &["rev-parse", "HEAD"]);
    let issue_create = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: canonical_authority_selector_digest(&fixture.root)
            .expect("selector digest"),
        exact_review_sha: head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(GithubMutationRequest {
            repository: "agent-logic/agent-design-language".into(),
            issue: 0,
            pull_request: None,
            cutover_issue: Some(505),
            operator_approval: Some("test route ownership".into()),
            expected_head_sha: head.clone(),
            credential_names: vec!["GITHUB_TOKEN".into()],
            recovery: None,
            mutation: GithubMutation::IssueCreate {
                title: "new issue".into(),
                body: "body".into(),
                labels: vec![],
                assignees: vec![],
                milestone: None,
            },
        }),
    };
    let issue_path = fixture.root.join("issue-create-dispatch.json");
    fs::write(&issue_path, serde_json::to_vec(&issue_create).unwrap()).unwrap();
    let wrong_pr_route = run(
        &[
            "github-pr",
            "--request",
            issue_path.to_str().unwrap(),
            "--execute",
        ],
        &fixture.root,
    );
    assert!(!wrong_pr_route.status.success(), "{wrong_pr_route:?}");
    assert!(
        String::from_utf8_lossy(&wrong_pr_route.stderr)
            .contains("operational_remote_route_mismatch"),
        "{}",
        String::from_utf8_lossy(&wrong_pr_route.stderr)
    );

    let pr_ready = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: canonical_authority_selector_digest(&fixture.root)
            .expect("selector digest"),
        exact_review_sha: head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(GithubMutationRequest {
            repository: "agent-logic/agent-design-language".into(),
            issue: 505,
            pull_request: Some(591),
            cutover_issue: Some(505),
            operator_approval: Some("test route ownership".into()),
            expected_head_sha: head,
            credential_names: vec!["GITHUB_TOKEN".into()],
            recovery: None,
            mutation: GithubMutation::PullRequestReady,
        }),
    };
    let pr_path = fixture.root.join("pr-ready-dispatch.json");
    fs::write(&pr_path, serde_json::to_vec(&pr_ready).unwrap()).unwrap();
    let wrong_issue_route = run(
        &[
            "github-issue",
            "--request",
            pr_path.to_str().unwrap(),
            "--execute",
        ],
        &fixture.root,
    );
    assert!(!wrong_issue_route.status.success(), "{wrong_issue_route:?}");
    assert!(
        String::from_utf8_lossy(&wrong_issue_route.stderr)
            .contains("operational_remote_route_mismatch"),
        "{}",
        String::from_utf8_lossy(&wrong_issue_route.stderr)
    );
}

// PVF: deterministic operational CLI contract with a local fake curl; no network mutation.
#[cfg(unix)]
#[test]
fn executable_github_pr_ready_uses_graphql_and_authenticated_readback() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = operational_fixture("github-pr-ready-positive");
    let head = git(&fixture.root, &["rev-parse", "HEAD"]);
    let request = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: canonical_authority_selector_digest(&fixture.root)
            .expect("selector digest"),
        exact_review_sha: head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(GithubMutationRequest {
            repository: "agent-logic/agent-design-language".into(),
            issue: 824,
            pull_request: Some(822),
            cutover_issue: Some(505),
            operator_approval: Some("offline operational ready fixture".into()),
            expected_head_sha: head.clone(),
            credential_names: vec!["GITHUB_TOKEN".into()],
            recovery: None,
            mutation: GithubMutation::PullRequestReady,
        }),
    };
    let request_path = fixture.root.join("pr-ready-dispatch.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    let token_path = fixture.root.join("github.token");
    fs::write(&token_path, "fixture-token\n").unwrap();
    let fake_bin = fixture.root.join("fake-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let ready_marker = fixture.root.join("ready-state");
    let curl_path = fake_bin.join("curl");
    let script = format!(
        r#"#!/bin/sh
case "$*" in
  *api.github.com/graphql*)
    : > {marker}
    printf '%s\n' '{{"data":{{"markPullRequestReadyForReview":{{"pullRequest":{{"number":822,"headRefOid":"{head}","isDraft":false}}}}}}}}'
    ;;
  *repos/agent-logic/agent-design-language/pulls/822*)
    if [ -f {marker} ]; then draft=false; else draft=true; fi
    printf '{{"number":822,"node_id":"PR_kwDO-ready-822","head":{{"sha":"{head}"}},"draft":%s}}\n' "$draft"
    ;;
  *) exit 2 ;;
esac
"#,
        marker = ready_marker.display(),
        head = head
    );
    fs::write(&curl_path, script).unwrap();
    fs::set_permissions(&curl_path, fs::Permissions::from_mode(0o700)).unwrap();
    let path = format!(
        "{}:{}",
        fake_bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let output = run_with_env(
        &[
            "github-pr",
            "--request",
            request_path.to_str().unwrap(),
            "--execute",
        ],
        &fixture.root,
        &[
            ("ADL_GITHUB_TOKEN_FILE", token_path.as_os_str()),
            ("PATH", std::ffi::OsStr::new(&path)),
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        result["result"]["outcome"]["result"]["receipt"]["pull_request"].as_u64(),
        Some(822)
    );
    assert_eq!(
        result["result"]["outcome"]["result"]["receipt"]["expected_head_sha"].as_str(),
        Some(head.as_str())
    );
    assert!(ready_marker.exists(), "GraphQL mutation was not invoked");
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
    let proof_json: serde_json::Value = serde_json::from_slice(&proof.stdout).unwrap();
    assert_eq!(proof_json["status"], "blocked");
    assert_eq!(proof_json["findings"][0]["code"], "proof_binding_missing");

    let rollback_path = fixture.join("rollback.json");
    let rollback_request = TerminalRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: None,
        expected_head_sha: None,
        mode: None,
        public_adapter_receipt: None,
        terminal_state: None,
        no_pr_closeout: None,
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
