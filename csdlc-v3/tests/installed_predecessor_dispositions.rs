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
    time::{Instant, SystemTime, UNIX_EPOCH},
};

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
fn installed_generic_github_executes_and_reconciles_one_mutation() {
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
    let mut stale = request.clone();
    stale["expected_lifecycle_digest"] = json!("0".repeat(64));
    let path = fixture.write_json("generic-stale.json", &stale);
    let before = intent_fixture::inventory(&root);
    let denied = fixture.run(
        &root,
        &["github", "--request", path.to_str().unwrap(), "--execute"],
    );
    retain_generic_case(&mut fixture, "generic-github-stale-selector");
    assert!(!denied.status.success());
    assert_ne!(
        parsed(&denied)["envelope"]["effects"]["outcome"],
        "performed"
    );
    assert_eq!(before, intent_fixture::inventory(&root));
    assert_eq!(fixture.remote_effects(), 0);
    let path = fixture.write_json("generic-current.json", &request);
    let executed = fixture.run(
        &root,
        &["github", "--request", path.to_str().unwrap(), "--execute"],
    );
    retain_generic_case(&mut fixture, "generic-github-exact-mutation");
    assert!(executed.status.success(), "{executed:?}");
    assert_eq!(
        parsed(&executed)["envelope"]["effects"]["outcome"],
        "performed"
    );
    assert_eq!(fixture.remote_effects(), 1);
    let before = intent_fixture::inventory(&root);
    let replay = fixture.run(
        &root,
        &["github", "--request", path.to_str().unwrap(), "--execute"],
    );
    retain_generic_case(&mut fixture, "generic-github-settled-replay");
    assert!(replay.status.success(), "{replay:?}");
    assert_eq!(parsed(&replay)["envelope"]["effects"]["outcome"], "none");
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(before, intent_fixture::inventory(&root));
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
                "{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
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
            br#"{"number":505,"state":"closed"}"#,
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
}

#[test]
fn installed_cutover_and_rollback_preserve_guarded_dispositions() {
    let (mut f, digest) = TerminalFixture::new();
    let mut apply =
        predecessor_fixture::cutover_request(&f.root, digest.clone(), CutoverOperation::Apply);
    apply.pull_request = Some(591);
    apply.credential_names = vec!["GITHUB_TOKEN".into()];
    let mut denied = apply.clone();
    denied.cutover.as_mut().unwrap().approval.clear();
    f.run("cutover-missing-approval", "cutover", &denied, true, false);
    let applied = f.run("cutover-authenticated-apply", "cutover", &apply, true, true);
    assert_eq!(applied["result"]["status"], "ready");
    assert_eq!(
        fs::read(f.root.join(".adl/bin/csdlc")).unwrap(),
        fs::read(&f.binary).unwrap()
    );
    assert!(f.root.join(".git/csdlc-v3/cutover-receipt.json").is_file());
    f.run("rollback-wrong-operation", "rollback", &apply, false, false);
    predecessor_fixture::publish_v2_rollback(&f.root);
    let rollback =
        predecessor_fixture::cutover_request(&f.root, digest, CutoverOperation::Rollback);
    let rolled_back = f.run(
        "rollback-tracked-suspension",
        "rollback",
        &rollback,
        false,
        true,
    );
    assert_eq!(rolled_back["result"]["status"], "ready");
    assert!(!f.root.join(".adl/bin/csdlc").exists());
    let selector: Value = serde_json::from_slice(
        &fs::read(f.root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["generation"], "rollback");
    f.run(
        "rollback-settled-replay",
        "rollback",
        &rollback,
        false,
        true,
    );
}
