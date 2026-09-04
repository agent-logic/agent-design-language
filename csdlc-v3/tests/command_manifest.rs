use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    str,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const IMPLEMENTED_REMOTE_BRIDGE_COMMANDS: &[(&str, &str)] = &[
    ("github", "verify_bridge_evidence"),
    ("github-issue", "verify_bridge_evidence"),
    ("github-pr", "verify_bridge_evidence"),
    ("pr-state", "verify_bridge_evidence"),
    ("review", "verify_bridge_evidence"),
    ("publish", "publish"),
    ("finish", "finish"),
    ("clean", "cleanup_preview"),
];

const IMPLEMENTED_REPLACEMENT_VERIFIER_COMMANDS: &[&str] =
    &["cutover", "install", "proof", "shadow", "soak"];

const IMPLEMENTED_LOCAL_COMMANDS: &[&str] = &[
    "issue",
    "bind",
    "edit",
    "validate",
    "doctor",
    "schedule",
    "shepherd",
    "eligibility",
];

const IMPLEMENTED_HELPER_COMMANDS: &[&str] = &["remote", "sprint"];

#[test]
fn help_exposes_one_binary_command_surface() {
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("--help")
        .output()
        .expect("csdlc help should run");
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).expect("help stdout should be utf8");
    assert!(stdout.contains("usage: csdlc <command>"));
    assert!(stdout.contains("foundation --repo-root <path>"));
    assert!(stdout.contains("local --request <path> --registry <path> --registrations <path>"));
    for (command, _) in IMPLEMENTED_REMOTE_BRIDGE_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --repo-root <path> --request <path>")),
            "help should expose implemented remote bridge route {command}"
        );
    }
    for command in IMPLEMENTED_LOCAL_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --request <path>")),
            "help should expose implemented local route {command}"
        );
    }
    for command in IMPLEMENTED_REPLACEMENT_VERIFIER_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --repo-root <path> --request <path>")),
            "help should expose implemented replacement verifier route {command}"
        );
    }
}

#[test]
fn tracked_command_denominators_match_cli_surface_and_cutover_boundary() {
    let root = repo_root();
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/csdlc-v3/v3-command-manifest.json"))
            .expect("command manifest"),
    )
    .expect("command manifest json");
    let denominator: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/csdlc-v3/full-replacement-denominator.json"))
            .expect("full replacement denominator"),
    )
    .expect("full replacement denominator json");

    assert_eq!(manifest["one_binary"], "csdlc");
    assert_eq!(manifest["operational_authority"], false);
    assert_eq!(denominator["cutover_ready"], false);
    assert_eq!(manifest["denominator"]["v2_entrypoints"], 21);
    assert_eq!(manifest["denominator"]["current_v3_commands"], 25);
    assert_eq!(manifest["denominator"]["implemented_commands"], 24);
    assert_eq!(manifest["denominator"]["partial_commands"], 1);
    assert_eq!(manifest["denominator"]["fail_closed_commands"], 0);
    assert_eq!(
        manifest["denominator"]["implemented_replacement_routes"],
        21
    );
    assert_eq!(manifest["denominator"]["partial_replacement_routes"], 0);
    assert_eq!(manifest["denominator"]["fail_closed_replacement_routes"], 0);
    assert_eq!(manifest["denominator"]["remaining_replacement_routes"], 0);
    assert_eq!(
        denominator["required_v2_entrypoints"]
            .as_array()
            .unwrap()
            .len(),
        21
    );

    let commands = manifest["commands"].as_array().expect("manifest commands");
    assert_eq!(commands.len(), 25);
    assert_eq!(status_count(commands, "implemented"), 24);
    assert_eq!(status_count(commands, "partial"), 1);
    assert_eq!(status_count(commands, "fail_closed"), 0);

    for (command, _) in IMPLEMENTED_REMOTE_BRIDGE_COMMANDS {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented");
        assert_eq!(row["authority_status"], "not_live");
    }
    for command in IMPLEMENTED_REPLACEMENT_VERIFIER_COMMANDS {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented");
        assert_eq!(row["authority_status"], "not_live");
    }
    for command in IMPLEMENTED_HELPER_COMMANDS {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented");
        assert!(row["replaces"].as_array().expect("replaces").is_empty());
    }

    let current = denominator["current_v3_commands"]
        .as_array()
        .expect("current v3 commands")
        .iter()
        .map(|value| value.as_str().expect("command").to_owned())
        .collect::<Vec<_>>();
    for command in commands {
        let name = command["command"].as_str().expect("command name");
        assert!(
            current.iter().any(|current| current == name),
            "denominator should include manifest command {name}"
        );
    }
    for name in &current {
        assert!(
            commands
                .iter()
                .any(|command| command["command"].as_str() == Some(name.as_str())),
            "manifest should include denominator command {name}"
        );
    }

    let replacements = denominator["required_v2_entrypoints"]
        .as_array()
        .expect("required v2 entrypoints");
    assert_eq!(
        replacements
            .iter()
            .filter(|row| row["replacement_status"] == "implemented_pre_cutover_bridge")
            .count(),
        IMPLEMENTED_REMOTE_BRIDGE_COMMANDS.len() + IMPLEMENTED_REPLACEMENT_VERIFIER_COMMANDS.len()
    );
    assert_eq!(
        replacements
            .iter()
            .filter(|row| row["replacement_status"] == "fail_closed")
            .count(),
        0
    );
}

#[test]
fn implemented_remote_bridge_routes_execute_typed_evidence_without_live_authority() {
    for (command, operation) in IMPLEMENTED_REMOTE_BRIDGE_COMMANDS {
        let fixture = RemoteFixture::new(operation);
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([
                command,
                "--repo-root",
                fixture.root.to_str().expect("fixture path"),
                "--request",
                fixture.request.to_str().expect("request path"),
            ])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} should run: {error}"));
        assert!(
            output.status.success(),
            "{command} should execute typed bridge evidence: stderr={}",
            str::from_utf8(&output.stderr).unwrap_or("<non-utf8>")
        );
        let stdout = str::from_utf8(&output.stdout).expect("stdout should be utf8");
        assert!(
            stdout.contains("\"schema\":\"csdlc.v3.remote_delivery.v1\""),
            "{command} should emit the remote-delivery schema: {stdout}"
        );
        assert!(
            stdout.contains("\"operational_authority\":false"),
            "{command} must remain non-authoritative before cutover: {stdout}"
        );
        assert!(
            stdout.contains("\"trusted_authority\":false"),
            "{command} must not claim trusted live authority before cutover: {stdout}"
        );
        assert!(
            stdout.contains(&format!("\"operation\":\"{operation}\"")),
            "{command} should preserve the requested operation: {stdout}"
        );
    }
}

#[test]
fn remote_bridge_aliases_reject_wrong_operation_shape() {
    let fixture = RemoteFixture::new("verify_bridge_evidence");
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "finish",
            "--repo-root",
            fixture.root.to_str().expect("fixture path"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc finish should run");
    assert!(
        !output.status.success(),
        "finish should reject a generic bridge-verification request"
    );
    let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
    assert!(
        stderr.contains("route_operation_mismatch"),
        "wrong operation should fail explicitly: {stderr}"
    );
}

#[test]
fn generic_bridge_verification_rejects_identity_mismatched_evidence() {
    let fixture = RemoteFixture::new("verify_bridge_evidence");
    fixture.overwrite_pvf_issue(701);
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "github",
            "--repo-root",
            fixture.root.to_str().expect("fixture path"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc github should run");
    assert!(
        !output.status.success(),
        "generic bridge verification must reject mismatched evidence identity"
    );
    let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
    assert!(
        stderr.contains("EvidenceIdentityMismatch"),
        "identity mismatch should fail explicitly: {stderr}"
    );
}

#[test]
fn publish_alias_does_not_require_terminal_pr_or_closed_issue() {
    let fixture = RemoteFixture::new_publish_minimal();
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "publish",
            "--repo-root",
            fixture.root.to_str().expect("fixture path"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc publish should run");
    assert!(
        output.status.success(),
        "publish should derive publication evidence before terminal state: stderr={}",
        str::from_utf8(&output.stderr).unwrap_or("<non-utf8>")
    );
    let stdout = str::from_utf8(&output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("\"status\":\"publication_derived\""));
    assert!(stdout.contains("\"mutation_allowed\":false"));
}

#[test]
fn finish_alias_fails_closed_when_remote_truth_is_not_terminal() {
    let fixture = RemoteFixture::new_with_state("finish", false, Some(700), true);
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "finish",
            "--repo-root",
            fixture.root.to_str().expect("fixture path"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc finish should run");
    assert!(
        !output.status.success(),
        "finish must not exit successfully when operator action is required"
    );
    let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
    assert!(
        stderr.contains("OperatorRequired"),
        "nonterminal finish should surface operator-required truth: {stderr}"
    );
}

#[test]
fn clean_alias_is_preview_only_before_cutover() {
    let fixture = RemoteFixture::new_clean_minimal(false);
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "clean",
            "--repo-root",
            fixture.root.to_str().expect("fixture path"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc clean should run");
    assert!(
        !output.status.success(),
        "clean preview alias must reject removal-eligible cleanup input before cutover"
    );
    let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
    assert!(
        stderr.contains("cleanup_preview_requires_preview_candidate"),
        "clean preview mismatch should be explicit: {stderr}"
    );
}

#[test]
fn replacement_verifier_routes_execute_without_live_authority() {
    for command in IMPLEMENTED_REPLACEMENT_VERIFIER_COMMANDS {
        let fixture = ReplacementFixture::new(command);
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe reserved route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: implemented_pre_cutover_verifier"),
            "{command} help should be truthful: {help_stdout}"
        );
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([
                command,
                "--repo-root",
                fixture.root.to_str().expect("fixture root"),
                "--request",
                fixture.request.to_str().expect("request path"),
            ])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} should run: {error}"));
        assert!(
            output.status.success(),
            "{command} should execute the typed pre-cutover verifier: stderr={}",
            str::from_utf8(&output.stderr).unwrap_or("<non-utf8>")
        );
        let stdout = str::from_utf8(&output.stdout).expect("stdout should be utf8");
        assert!(
            stdout.contains("\"schema\":\"csdlc.v3.replacement_verifier.v1\""),
            "{command} should emit replacement verifier schema: {stdout}"
        );
        assert!(
            stdout.contains("\"implemented\":true"),
            "{command} should be executable, not a placeholder: {stdout}"
        );
        assert!(
            stdout.contains("\"mutation_allowed\":false"),
            "{command} must not mutate before cutover: {stdout}"
        );
        assert!(
            stdout.contains("\"operational_authority\":false"),
            "{command} must not claim live authority: {stdout}"
        );
        assert!(
            stdout.contains("operator_cutover_approval_missing"),
            "{command} should preserve the #505 approval gate: {stdout}"
        );
    }
}

#[test]
fn replacement_verifier_rejects_wrong_command_and_bad_evidence() {
    let fixture = ReplacementFixture::new_with_command("cutover", "proof");
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "cutover",
            "--repo-root",
            fixture.root.to_str().expect("fixture root"),
            "--request",
            fixture.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc cutover should run");
    assert!(!output.status.success());
    let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
    assert!(stderr.contains("command_mismatch"), "{stderr}");

    let bad = ReplacementFixture::new_with_digest("proof", "wrong-digest");
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args([
            "proof",
            "--repo-root",
            bad.root.to_str().expect("fixture root"),
            "--request",
            bad.request.to_str().expect("request path"),
        ])
        .output()
        .expect("csdlc proof should run");
    assert!(
        output.status.success(),
        "bad evidence is a verifier report, not a process crash"
    );
    let stdout = str::from_utf8(&output.stdout).expect("stdout should be utf8");
    assert!(
        stdout.contains("\"status\":\"blocked_by_evidence\""),
        "{stdout}"
    );
    assert!(stdout.contains("evidence_digest_mismatch"), "{stdout}");
}

#[test]
fn implemented_remote_bridge_routes_expose_non_authoritative_help() {
    for (command, _) in IMPLEMENTED_REMOTE_BRIDGE_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented bridge route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: implemented"),
            "{command} help should be truthful: {help_stdout}"
        );
        assert!(
            help_stdout.contains("structured pre-cutover bridge evidence only"),
            "{command} help should describe its pre-cutover transport boundary: {help_stdout}"
        );
        assert!(
            help_stdout.contains("C-SDLC v3 is not live authority before #505 cutover"),
            "{command} help should preserve authority boundary: {help_stdout}"
        );
    }
}

#[test]
fn implemented_local_routes_expose_non_authoritative_help() {
    for command in IMPLEMENTED_LOCAL_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented local route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: implemented"),
            "{command} help should be truthful: {help_stdout}"
        );
        assert!(
            help_stdout.contains("C-SDLC v3 is not live authority before #505 cutover"),
            "{command} help should preserve authority boundary: {help_stdout}"
        );
    }
}

struct RemoteFixture {
    root: PathBuf,
    request: PathBuf,
}

struct ReplacementFixture {
    root: PathBuf,
    request: PathBuf,
}

impl ReplacementFixture {
    fn new(command: &str) -> Self {
        Self::new_with_command(command, command)
    }

    fn new_with_command(route: &str, command: &str) -> Self {
        let root = fixture_root("replacement-verifier");
        fs::create_dir_all(&root).expect("fixture root");
        let evidence_path = root.join("evidence.json");
        write_json(
            &evidence_path,
            r#"{"schema":"csdlc.v3.fixture_evidence.v1","ok":true}"#,
        );
        let digest = blake3::hash(&fs::read(&evidence_path).expect("evidence bytes"))
            .to_hex()
            .to_string();
        let request = root.join(format!("{route}-request.json"));
        write_replacement_request(&request, command, &digest);
        Self { root, request }
    }

    fn new_with_digest(command: &str, digest: &str) -> Self {
        let root = fixture_root("replacement-verifier-bad-digest");
        fs::create_dir_all(&root).expect("fixture root");
        write_json(
            &root.join("evidence.json"),
            r#"{"schema":"csdlc.v3.fixture_evidence.v1","ok":true}"#,
        );
        let request = root.join(format!("{command}-request.json"));
        write_replacement_request(&request, command, digest);
        Self { root, request }
    }
}

fn fixture_root(kind: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(10_000);
    let id = NEXT.fetch_add(1, Ordering::SeqCst);
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    repo_root()
        .join("csdlc-v3")
        .join("target")
        .join(kind)
        .join(format!("{}-{nonce}-{id}", std::process::id()))
}

fn write_replacement_request(path: &Path, command: &str, digest: &str) {
    write_json(
        path,
        &format!(
            r#"{{
  "schema": "csdlc.v3.replacement_verifier_request.v1",
  "command": "{command}",
  "issue": 505,
  "authority_issue": 505,
  "repository": "agent-logic/agent-design-language",
  "operator_cutover_approved": false,
  "evidence": [
    {{
      "path": "evidence.json",
      "digest": "{digest}"
    }}
  ],
  "blockers": []
}}"#
        ),
    );
}

impl RemoteFixture {
    fn new(operation: &str) -> Self {
        Self::new_with_state(operation, true, Some(700), false)
    }

    fn new_publish_minimal() -> Self {
        let fixture = Self::new_with_state("publish", false, Some(700), true);
        write_json(
            &fixture.request,
            r#"{
  "repository": "agent-logic/agent-design-language",
  "issue": 700,
  "pull_request": 701,
  "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "mode": "closing",
  "operation": "publish",
  "pvf_evidence_ref": "pvf.json",
  "typed_review_ref": "review.json",
  "publication_intent_ref": "publication.json"
}"#,
        );
        fixture
    }

    fn new_with_state(
        operation: &str,
        merged: bool,
        closes_issue: Option<u64>,
        issue_open: bool,
    ) -> Self {
        Self::new_with_cleanup_state(operation, merged, closes_issue, issue_open, true)
    }

    fn new_with_cleanup_preview(operation: &str, preview: bool) -> Self {
        Self::new_with_cleanup_state(operation, true, Some(700), false, preview)
    }

    fn new_clean_minimal(preview: bool) -> Self {
        let fixture = Self::new_with_cleanup_preview("cleanup_preview", preview);
        write_json(
            &fixture.request,
            r#"{
  "repository": "agent-logic/agent-design-language",
  "issue": 700,
  "pull_request": 701,
  "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "mode": "closing",
  "operation": "cleanup_preview",
  "cleanup_inspection_ref": "cleanup.json"
}"#,
        );
        fixture
    }

    fn new_with_cleanup_state(
        operation: &str,
        merged: bool,
        closes_issue: Option<u64>,
        issue_open: bool,
        cleanup_preview: bool,
    ) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let id = NEXT.fetch_add(1, Ordering::SeqCst);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = repo_root()
            .join("csdlc-v3")
            .join("target")
            .join("remote-bridge-command-fixtures")
            .join(format!("{}-{nonce}-{id}", std::process::id()));
        fs::create_dir_all(&root).expect("fixture root");
        let repo = root.join("repo");
        let parent = root.join("worktrees");
        let worktree = parent.join("issue-700");
        let git_common = repo.join(".git").join("worktrees").join("issue-700");
        fs::create_dir_all(&git_common).expect("git common dir");
        fs::create_dir_all(&worktree).expect("worktree dir");
        fs::write(
            worktree.join(".git"),
            format!("gitdir: {}\n", git_common.display()),
        )
        .expect("worktree gitdir file");
        fs::write(
            git_common.join("gitdir"),
            worktree.join(".git").display().to_string(),
        )
        .expect("common gitdir backref");

        let head = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        write_json(
            &root.join("pvf.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.pvf_result.v1",
  "issue": 700,
  "revision": "{head}",
  "evidence_digest": "pvf-digest"
}}"#
            ),
        );
        write_json(
            &root.join("review.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.accepted_review.v1",
  "issue": 700,
  "reviewed_revision": "{head}",
  "scope_paths": ["csdlc-v3/src/commands/remote/mod.rs"],
  "implementer": "worker-6",
  "reviewer": "independent-reviewer",
  "findings": [{{"id": "clean", "disposition": "resolved"}}],
  "target": {{
    "repository": "agent-logic/agent-design-language",
    "issue": 700,
    "mode": "closing"
  }},
  "typed_review_evidence_digest": "typed-review-digest"
}}"#
            ),
        );
        write_json(
            &root.join("publication.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.publication_intent.v1",
  "repository": "agent-logic/agent-design-language",
  "issue": 700,
  "pull_request": 701,
  "mode": "closing",
  "publisher": "worker-6-publisher",
  "body": "Closes #700",
  "head_sha": "{head}"
}}"#
            ),
        );
        write_json(
            &root.join("pr.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.pr_readback.v1",
  "repository": "agent-logic/agent-design-language",
  "number": 701,
  "head_sha": "{head}",
  "merged": {merged},
  "closes_issue": {closes_issue_json},
  "part_of_issue": null
}}"#,
                closes_issue_json = closes_issue
                    .map(|issue| issue.to_string())
                    .unwrap_or_else(|| "null".to_owned())
            ),
        );
        write_json(
            &root.join("issue.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.issue_readback.v1",
  "repository": "agent-logic/agent-design-language",
  "issue": 700,
  "open": {issue_open}
}}"#
            ),
        );
        write_json(
            &root.join("cleanup.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.cleanup_inspection.v1",
  "preview": {cleanup_preview},
  "preview_receipt": {preview_receipt},
  "committed_closed_out": true,
  "terminal_receipt": true,
  "approved_worktree_parent": "{parent}",
  "registration": {{
    "repository_root": "{repo}",
    "worktree_path": "{worktree}",
    "git_common_dir": "{git_common}"
  }},
  "candidate_path": "{worktree}",
  "preview_identity_digest": null,
  "dirty": false,
  "live": false
}}"#,
                cleanup_preview = cleanup_preview,
                preview_receipt = !cleanup_preview,
                parent = parent.display(),
                repo = repo.display(),
                worktree = worktree.display(),
                git_common = git_common.display()
            ),
        );
        write_json(
            &root.join("request.json"),
            &format!(
                r#"{{
  "repository": "agent-logic/agent-design-language",
  "issue": 700,
  "pull_request": 701,
  "head_sha": "{head}",
  "mode": "closing",
  "operation": "{operation}",
  "pvf_evidence_ref": "pvf.json",
  "typed_review_ref": "review.json",
  "publication_intent_ref": "publication.json",
  "pr_readback_ref": "pr.json",
  "issue_readback_ref": "issue.json",
  "cleanup_inspection_ref": "cleanup.json"
}}"#
            ),
        );
        Self {
            root: root.clone(),
            request: root.join("request.json"),
        }
    }

    fn overwrite_pvf_issue(&self, issue: u64) {
        write_json(
            &self.root.join("pvf.json"),
            &format!(
                r#"{{
  "schema": "csdlc.v3.pvf_result.v1",
  "issue": {issue},
  "revision": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "evidence_digest": "pvf-digest"
}}"#
            ),
        );
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}

fn write_json(path: &Path, json: &str) {
    fs::write(path, json).unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

fn command_row<'a>(commands: &'a [serde_json::Value], command: &str) -> &'a serde_json::Value {
    commands
        .iter()
        .find(|row| row["command"] == command)
        .unwrap_or_else(|| panic!("missing manifest command {command}"))
}

fn status_count(commands: &[serde_json::Value], status: &str) -> usize {
    commands
        .iter()
        .filter(|row| row["implementation_status"] == status)
        .count()
}
