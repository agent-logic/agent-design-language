//! PVF #869: required deterministic installed predecessor-disposition proof.
//! Local CPU/Git/disk; synthetic authenticated transport, no live activation.
//! Historical authority inputs are fixtures, not newly produced proof receipts.
#![cfg(unix)]
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
#[path = "support/predecessor_fixture.rs"]
mod predecessor_fixture;
use csdlc_v3::commands::{
    remote::canonical_authority_selector_digest,
    terminal::{CutoverOperation, TerminalRouteRequest},
};
use serde_json::{json, Value};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

static TERMINAL_FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn parsed(output: &Output) -> Value {
    let result: Value = serde_json::from_slice(&output.stdout).expect("one JSON result");
    assert!(!String::from_utf8_lossy(&output.stdout).contains("SIM03_SYNTHETIC_TOKEN"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
    result
}

fn retain_generic_case(fixture: &mut intent_fixture::Fixture, label: &str) {
    fixture.attempts.last_mut().unwrap()["case_id"] = json!(label);
    let corpus = json!({"schema":"csdlc.v3.installed_predecessor_attempts.v1","issue":869,
        "source_head":intent_fixture::git(&intent_fixture::source_root(), &["rev-parse","HEAD"]),
        "installed_binary_blake3":blake3::hash(&fs::read(&fixture.binary).unwrap()).to_hex().to_string(),
        "attempts":fixture.attempts});
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/sim03-predecessor-corpus")
        .join(format!(
            "{}.json",
            fixture.root.file_name().unwrap().to_str().unwrap()
        ));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        serde_json::to_string_pretty(&corpus)
            .unwrap()
            .replace(fixture.root.to_str().unwrap(), "$FIXTURE_ROOT"),
    )
    .unwrap();
}

#[test]
fn installed_legacy_generic_github_is_retired_before_mutation() {
    let mut fixture = intent_fixture::Fixture::new("predecessor-github");
    fixture.enable_issue_transport();
    let root = fixture.root.clone();
    let head = intent_fixture::git(&root, &["rev-parse", "HEAD"]);
    let request = json!({
        "expected_lifecycle_digest":canonical_authority_selector_digest(&root).unwrap(),
        "exact_review_sha":head,
        "operation":{"kind":"github_mutation","request":{
            "repository":"agent-logic/agent-design-language","issue":505,
            "expected_head_sha":head,"credential_names":["GITHUB_TOKEN"],
            "mutation":{"action":"issue_comment","body":"Synthetic installed generic route proof"}
        }}
    });
    let path = fixture.write_json("generic-current.json", &request);
    let before = intent_fixture::inventory(&root);
    let denied = fixture.run(
        &root,
        &["github", "--request", path.to_str().unwrap(), "--execute"],
    );
    retain_generic_case(&mut fixture, "legacy-generic-github-retired");
    assert!(!denied.status.success());
    let denied = parsed(&denied);
    assert_eq!(denied["envelope"]["reason_code"], "legacy_writer_retired");
    assert_eq!(denied["envelope"]["effects"]["outcome"], "none");
    assert_eq!(denied["performed_mutation"], false);
    assert_eq!(before, intent_fixture::inventory(&root));
    assert_eq!(fixture.remote_effects(), 0);
}

struct TerminalFixture {
    root: PathBuf,
    binary: PathBuf,
    transport: PathBuf,
    attempts: Vec<Value>,
}
impl TerminalFixture {
    fn new() -> (Self, String) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sim03-predecessor")
            .join(format!(
                "{}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                TERMINAL_FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        assert!(!root.exists());
        let readiness = predecessor_fixture::write_cutover_fixture(&root, b"installed candidate");
        let binary = root.join(".git/installed-candidate/csdlc");
        fs::create_dir_all(binary.parent().unwrap()).unwrap();
        fs::copy(env!("CARGO_BIN_EXE_csdlc"), &binary).unwrap();
        fs::set_permissions(&binary, fs::Permissions::from_mode(0o700)).unwrap();
        let transport = root.join(".git/synthetic-transport");
        fs::create_dir_all(&transport).unwrap();
        let data: Value = serde_json::from_slice(
            &fs::read(root.join(".csdlc/evidence/505/authority-readiness.json")).unwrap(),
        )
        .unwrap();
        fs::write(
            transport.join("pr.json"),
            serde_json::to_vec(
                &json!({"number":591,"merged":true,"head":{"sha":data["selected_revision"]}}),
            )
            .unwrap(),
        )
        .unwrap();
        fs::write(
            transport.join("issue.json"),
            br#"{"number":505,"title":"Authority transition","body":"Fixture authority issue","state":"open","labels":[],"assignees":[],"milestone":null}"#,
        )
        .unwrap();
        let quote = |p: &Path| format!("'{}'", p.to_str().unwrap().replace('\'', "'\\''"));
        let script = format!("#!/bin/sh\nurl=\nfor arg do case \"$arg\" in https://api.github.com/*) url=$arg;; esac; done\ncat >/dev/null\ncase \"$url\" in\n *'/pulls/591'*) cat {};;\n *'/issues/505'*) cat {};;\n *) exit 22;;\nesac\n", quote(&transport.join("pr.json")), quote(&transport.join("issue.json")));
        fs::write(transport.join("curl"), script).unwrap();
        fs::set_permissions(transport.join("curl"), fs::Permissions::from_mode(0o700)).unwrap();
        (
            Self {
                root,
                binary,
                transport,
                attempts: vec![],
            },
            readiness,
        )
    }
    fn run(
        &mut self,
        label: &str,
        route: &str,
        request: &TerminalRouteRequest,
        observe: bool,
        expected_success: bool,
    ) -> Value {
        let path = self.root.join(".git/installed-candidate/request.json");
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        let before = intent_fixture::inventory(&self.root);
        let start = Instant::now();
        let mut command = Command::new(&self.binary);
        command
            .current_dir(&self.root)
            .env_clear()
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.transport.display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("HOME", &self.transport)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GITHUB_TOKEN", "SIM03_SYNTHETIC_TOKEN")
            .env("ADL_GITHUB_TOKEN_FILE", self.transport.join("token"))
            .args([route, "--request"])
            .arg(&path);
        if observe {
            command.arg("--observe-github");
        }
        let output = command.output().unwrap();
        let report = parsed(&output);
        self.attempts.push(json!({"case_id":label,"route":route,"topology":"isolated_primary","elapsed_millis":start.elapsed().as_millis(),"exit_code":output.status.code(),"result":report}));
        let corpus = json!({"schema":"csdlc.v3.installed_predecessor_attempts.v1","issue":869,
            "source_head":intent_fixture::git(&intent_fixture::source_root(), &["rev-parse","HEAD"]),
            "installed_binary_blake3":blake3::hash(&fs::read(&self.binary).unwrap()).to_hex().to_string(),
            "fixture_prerequisites":"Synthetic historical authority inputs; not a proof-generation journey or live authorization",
            "attempts":self.attempts});
        let log = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sim03-predecessor-corpus")
            .join(format!(
                "{}.json",
                self.root.file_name().unwrap().to_str().unwrap()
            ));
        fs::create_dir_all(log.parent().unwrap()).unwrap();
        fs::write(
            log,
            serde_json::to_string_pretty(&corpus)
                .unwrap()
                .replace(self.root.to_str().unwrap(), "$FIXTURE_ROOT"),
        )
        .unwrap();
        assert_eq!(
            output.status.success(),
            expected_success,
            "{label}: {output:?}"
        );
        if !expected_success {
            assert_eq!(
                before,
                intent_fixture::inventory(&self.root),
                "rejected {label} changed fixture"
            );
        }
        report
    }

    fn run_args(&self, args: &[&str]) -> Output {
        self.run_args_with_env(args, &[])
    }

    fn run_args_with_env(&self, args: &[&str], env: &[(&str, &str)]) -> Output {
        Command::new(&self.binary)
            .current_dir(&self.root)
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.transport.display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("HOME", &self.transport)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GITHUB_TOKEN", "SIM03_SYNTHETIC_TOKEN")
            .env("ADL_GITHUB_TOKEN_FILE", self.transport.join("token"))
            .envs(env.iter().copied())
            .args(args)
            .output()
            .unwrap()
    }
}

#[test]
fn installed_cutover_after_effect_recovery_reports_semantic_attachment() {
    let (f, digest) = TerminalFixture::new();
    prepare_semantic_authority_issue(&f);
    let mut apply = predecessor_fixture::cutover_request(&f.root, digest, CutoverOperation::Apply);
    apply.pull_request = Some(591);
    apply.credential_names = vec!["GITHUB_TOKEN".into()];
    let operation = f
        .root
        .join(".git/installed-candidate/cutover-after-effect-operation.json");
    fs::write(&operation, serde_json::to_vec(&apply).unwrap()).unwrap();
    let crash = f.run_args_with_env(
        &[
            "cutover",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_administrative_after_effect",
        )],
    );
    assert_eq!(crash.status.code(), Some(91), "{crash:?}");
    assert!(f.root.join(".adl/bin/csdlc").is_file());

    let before_preview = intent_fixture::inventory(&f.root);
    let preview = f.run_args(&["recover", "505"]);
    assert!(preview.status.success(), "{preview:?}");
    assert_eq!(parsed(&preview)["read_only"], true);
    assert_eq!(before_preview, intent_fixture::inventory(&f.root));

    let completed = f.run_args(&["recover", "505", "--execute"]);
    assert!(completed.status.success(), "{completed:?}");
    let completed = parsed(&completed);
    assert_eq!(completed["status"], "completed");
    assert_eq!(completed["read_only"], false);
    assert_eq!(completed["performed_mutation"], true);
    assert_eq!(completed["native_invocation_performed"], false);
    assert_ne!(before_preview, intent_fixture::inventory(&f.root));
}

fn prepare_semantic_authority_issue(f: &TerminalFixture) {
    fs::write(f.transport.join("token"), "SIM03_SYNTHETIC_TOKEN").unwrap();
    fs::set_permissions(f.transport.join("token"), fs::Permissions::from_mode(0o600)).unwrap();
    intent_fixture::git(
        &f.root,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    intent_fixture::copy_tree(
        &intent_fixture::source_root().join("docs/templates/prompts/1.0.5"),
        &f.root.join("docs/templates/prompts/1.0.5"),
    );
    fs::create_dir_all(f.root.join("docs/templates/prompts")).unwrap();
    fs::copy(
        intent_fixture::source_root().join("docs/templates/prompts/current.json"),
        f.root.join("docs/templates/prompts/current.json"),
    )
    .unwrap();
    let evidence = f.root.join(".csdlc/evidence/505");
    let stash = f.root.join(".git/administrative-evidence-stash");
    fs::create_dir_all(&stash).unwrap();
    for entry in fs::read_dir(&evidence).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name() != "terminal-receipt.json" {
            fs::rename(entry.path(), stash.join(entry.file_name())).unwrap();
        }
    }
    let review = fs::read(f.root.join(".csdlc/issues/505/index.json")).unwrap();
    fs::remove_dir_all(f.root.join(".csdlc/issues/505")).unwrap();
    let plan = f.root.join(".git/installed-candidate/authority-plan.json");
    fs::write(&plan, serde_json::to_vec(&json!({
        "schema":"csdlc.v3.intent_plan.v1","slug":"authority-administration",
        "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Authority evidence ready","repo_inputs_inline":"Isolated authority fixture","target_files_surfaces_inline":"cutover rollback","deliverables_inline":"Record administrative effects","validation_plan_inline":"Installed one-shot recovery","acceptance_criteria_inline":"Exact replay does not repeat effects","notes_risks_inline":"No live authority"},"vpp":{},"srp":{},"sor":{}},
        "validators":[],"publication":{"base":"main","title":"Authority administration","body":"Closes #505","draft":true}
    })).unwrap()).unwrap();
    let output = f.run_args(&["prepare", "505", "--plan", plan.to_str().unwrap()]);
    assert!(output.status.success(), "semantic prepare: {output:?}");
    fs::create_dir_all(f.root.join(".csdlc/issues/505")).unwrap();
    fs::write(f.root.join(".csdlc/issues/505/index.json"), review).unwrap();
    fs::write(
        f.transport.join("issue.json"),
        br#"{"number":505,"title":"Authority transition","body":"Fixture authority issue","state":"closed","labels":[],"assignees":[],"milestone":null}"#,
    )
    .unwrap();
    for entry in fs::read_dir(&stash).unwrap() {
        let entry = entry.unwrap();
        fs::rename(entry.path(), evidence.join(entry.file_name())).unwrap();
    }
}

#[test]
fn installed_cutover_and_rollback_preserve_guarded_dispositions() {
    let (mut f, digest) = TerminalFixture::new();
    prepare_semantic_authority_issue(&f);
    let mut apply =
        predecessor_fixture::cutover_request(&f.root, digest.clone(), CutoverOperation::Apply);
    apply.pull_request = Some(591);
    apply.credential_names = vec!["GITHUB_TOKEN".into()];
    let mut denied = apply.clone();
    denied.cutover.as_mut().unwrap().approval.clear();
    f.run("cutover-missing-approval", "cutover", &denied, true, false);
    let operation = f
        .root
        .join(".git/installed-candidate/cutover-operation.json");
    fs::write(&operation, serde_json::to_vec(&apply).unwrap()).unwrap();
    let applied_output = f.run_args(&[
        "cutover",
        "505",
        "--operation",
        operation.to_str().unwrap(),
        "--execute",
    ]);
    assert!(applied_output.status.success(), "{applied_output:?}");
    let applied = parsed(&applied_output);
    assert_eq!(applied["status"], "completed");
    assert_eq!(applied["semantic"]["effect_truth"], "performed");
    assert_eq!(
        fs::read(f.root.join(".adl/bin/csdlc")).unwrap(),
        fs::read(&f.binary).unwrap()
    );
    assert!(f.root.join(".git/csdlc-v3/cutover-receipt.json").is_file());
    let replay = f.run_args(&[
        "cutover",
        "505",
        "--operation",
        operation.to_str().unwrap(),
        "--execute",
    ]);
    assert!(replay.status.success(), "{replay:?}");
    assert_eq!(parsed(&replay)["performed_mutation"], false);
    let rollback =
        predecessor_fixture::cutover_request(&f.root, digest, CutoverOperation::Rollback);
    let rollback_operation = f
        .root
        .join(".git/installed-candidate/rollback-operation.json");
    fs::write(&rollback_operation, serde_json::to_vec(&rollback).unwrap()).unwrap();
    let reserved = f.run_args(&[
        "rollback",
        "505",
        "--operation",
        rollback_operation.to_str().unwrap(),
        "--execute",
    ]);
    assert!(
        !reserved.status.success(),
        "rollback must wait for tracked selector revert"
    );
    assert_eq!(parsed(&reserved)["status"], "recovery_required");
    predecessor_fixture::publish_v2_rollback(&f.root);
    let before_read_only_recovery = intent_fixture::inventory(&f.root);
    let status = f.run_args(&["status", "505"]);
    assert!(status.status.success(), "{status:?}");
    let status = parsed(&status);
    assert_eq!(status["status"], "recovery_required");
    assert_eq!(status["read_only"], true);
    assert_eq!(status["performed_mutation"], false);
    assert_eq!(status["allowed_next"], json!(["recover"]));
    assert_eq!(
        before_read_only_recovery,
        intent_fixture::inventory(&f.root)
    );
    let observed_only = f.run_args(&["recover", "505"]);
    assert!(observed_only.status.success(), "{observed_only:?}");
    let observed_only = parsed(&observed_only);
    assert_eq!(observed_only["read_only"], true);
    assert_eq!(observed_only["performed_mutation"], false);
    assert_eq!(
        before_read_only_recovery,
        intent_fixture::inventory(&f.root),
        "read-only recovery must not attach or execute the retained rollback"
    );
    assert!(f.root.join(".adl/bin/csdlc").is_file());
    let rolled_back = f.run_args(&["recover", "505", "--execute"]);
    assert!(rolled_back.status.success(), "{rolled_back:?}");
    assert_eq!(
        parsed(&rolled_back)["semantic"]["effect_truth"],
        "performed"
    );
    assert!(!f.root.join(".adl/bin/csdlc").exists());
    let selector: Value = serde_json::from_slice(
        &fs::read(f.root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["generation"], "rollback");
    let completed_replay = f.run_args(&[
        "rollback",
        "505",
        "--operation",
        rollback_operation.to_str().unwrap(),
        "--execute",
    ]);
    assert!(completed_replay.status.success(), "{completed_replay:?}");
    let completed_replay = parsed(&completed_replay);
    assert_eq!(completed_replay["status"], "expected_noop");
    assert_eq!(completed_replay["read_only"], true);
    assert_eq!(completed_replay["performed_mutation"], false);
}
