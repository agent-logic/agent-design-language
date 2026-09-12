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

#[path = "support/attempt_corpus.rs"]
mod attempt_corpus;
#[path = "support/baseline_journeys.rs"]
mod baseline_journeys;
#[path = "support/observation.rs"]
mod observation;
use observation::inventory as observation_inventory;

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

fn copy_observation_templates(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_observation_templates(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn installed_diagnostics_preserve_healthy_missing_corrupt_and_pending_state() {
    for linked in [false, true] {
        for state in ["healthy", "missing", "corrupt", "pending"] {
            let mut fixture = operational_fixture(&format!("observation-{linked}-{state}"));
            let primary = fixture.root.clone();
            copy_observation_templates(
                &repo_root().join("docs/templates/prompts/1.0.5"),
                &primary.join("docs/templates/prompts/1.0.5"),
            );
            fs::create_dir_all(primary.join("docs/csdlc-v3")).unwrap();
            for name in [
                "CONTRACT.md",
                "predecessor-coverage.json",
                "proportional-lifecycle.json",
            ] {
                fs::copy(
                    repo_root().join("docs/csdlc-v3").join(name),
                    primary.join("docs/csdlc-v3").join(name),
                )
                .unwrap();
            }
            git(&primary, &["add", "docs"]);
            git(
                &primary,
                &["commit", "--quiet", "-m", "isolated observation templates"],
            );
            let head = git(&primary, &["rev-parse", "HEAD"]);
            git(&primary, &["update-ref", "refs/remotes/origin/main", &head]);
            if linked || state != "missing" {
                initialize_operational_fixture(&mut fixture);
            }
            if linked {
                let bound = run_operational(&fixture, "bind", None);
                assert!(bound.status.success(), "{bound:?}");
                let result: serde_json::Value = serde_json::from_slice(&bound.stdout).unwrap();
                fixture.request.expected_lifecycle_digest =
                    Some(result["result"]["digest"].as_str().unwrap().to_owned());
                fixture.root = PathBuf::from(&fixture.request.worktree);
            }
            let storage = if linked {
                fixture.root.join(".csdlc")
            } else {
                fixture.root.join(".git/csdlc-v3/local")
            };
            fixture.request.schedule_readiness = Some(
                serde_json::from_value(json!({
                    "phase_ready":true,"cards_ready":true,"design_ready":true,
                    "dependencies_ready":true,"paths_clear":true,"budget_available":true
                }))
                .unwrap(),
            );
            fixture.request.shepherd_routing = Some(
                serde_json::from_value(json!({
                    "validation":"passed"
                }))
                .unwrap(),
            );
            if state == "pending" {
                fixture
                    .request
                    .card_updates
                    .insert("sip".into(), json!({"title":"Pending edit"}));
            }
            fs::write(
                &fixture.request_path,
                serde_json::to_vec(&fixture.request).unwrap(),
            )
            .unwrap();
            if state == "pending" {
                let crashed = run_operational(&fixture, "edit", Some("after_backup_rename"));
                assert_eq!(crashed.status.code(), Some(91), "{crashed:?}");
            } else if state == "corrupt" {
                fs::write(storage.join("issues/505/index.json"), b"corrupt-index").unwrap();
            } else if linked && state == "missing" {
                fs::remove_dir_all(storage.join("issues/505")).unwrap();
            }
            // Remove an existing fixture lock so an otherwise byte-identical
            // diagnostic cannot hide creation behind setup's earlier mutation.
            if storage.join("locks").exists() {
                fs::remove_dir_all(storage.join("locks")).unwrap();
            }
            let installed = primary.join("candidate/bin/csdlc");
            observation::install_candidate(&installed);
            let before = observation_inventory(&primary);
            let registrations = git(&primary, &["worktree", "list", "--porcelain"]);
            for route in ["doctor", "eligibility", "validate", "schedule", "shepherd"] {
                let output = Command::new(&installed)
                    .current_dir(&fixture.root)
                    .arg(route)
                    .arg("--request")
                    .arg(&fixture.request_path)
                    .arg("--registry")
                    .arg(repo_root().join("docs/templates/prompts/current.json"))
                    .arg("--registrations")
                    .arg(&fixture.registrations_path)
                    // Native v3 does not implement the legacy observability
                    // overrides. They must not redirect machine JSON or cause
                    // an implicit log artifact during observation.
                    .env("ADL_OBSERVABILITY_STDERR", "0")
                    .env("ADL_OBSERVABILITY_LOG", primary.join("legacy-compat.log"))
                    .output()
                    .unwrap();
                let after = observation_inventory(&primary);
                assert!(!primary.join("legacy-compat.log").exists());
                let changed: std::collections::BTreeSet<_> = before
                    .keys()
                    .chain(after.keys())
                    .filter(|path| before.get(*path) != after.get(*path))
                    .collect();
                assert!(
                    changed.is_empty(),
                    "{route} changed {state} lifecycle/Git storage; linked={linked}: {changed:?}"
                );
                assert_eq!(
                    git(&primary, &["worktree", "list", "--porcelain"]),
                    registrations
                );
                if state == "healthy" {
                    assert!(output.status.success(), "{route}: {output:?}");
                    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                    assert_eq!(report["operational_authority"], true);
                    assert_eq!(report["writes_v3_state"], false);
                } else {
                    assert!(
                        !output.status.success(),
                        "{route} accepted {state}: {output:?}"
                    );
                    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                    assert_eq!(report["writes_v3_state"], false);
                    let failed = report["findings"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|finding| finding["status"] == "failed");
                    assert_eq!(report["status"], if failed { "failed" } else { "blocked" });
                    assert!(!String::from_utf8_lossy(&output.stderr).contains("\"findings\""));
                    assert!(
                        !output.stderr.is_empty(),
                        "legacy quiet override changed native error channel"
                    );
                    if state == "pending" {
                        assert!(
                            String::from_utf8_lossy(&output.stdout).contains("recovery_required"),
                            "pending transaction needs explicit recovery finding: {output:?}"
                        );
                    }
                }
            }
            // Helpers have separate semantics: foundation observes repository
            // contracts, local is a non-authoritative plan, sprint consumes
            // explicit issue readbacks. None acquires issue repair authority.
            let helper_dir = fixture.root.join(".csdlc/evidence/observation-helpers");
            fs::create_dir_all(&helper_dir).unwrap();
            let local_request = helper_dir.join("local.json");
            let local_plan = fixture.request.clone();
            fs::write(&local_request, serde_json::to_vec(&local_plan).unwrap()).unwrap();
            let helper_registrations = helper_dir.join("registrations.json");
            let observed: Vec<_> = git(&primary, &["worktree", "list", "--porcelain"])
                .split("\n\n")
                .filter_map(|block| {
                    let path = block
                        .lines()
                        .find_map(|line| line.strip_prefix("worktree "))?;
                    let branch = block
                        .lines()
                        .find_map(|line| line.strip_prefix("branch refs/heads/"))?;
                    Some(
                        json!({"branch":branch,"worktree":path,"primary":Path::new(path)==primary}),
                    )
                })
                .collect();
            fs::write(
                &helper_registrations,
                serde_json::to_vec(&observed).unwrap(),
            )
            .unwrap();
            let umbrella = helper_dir.join("umbrella.json");
            let child = helper_dir.join("child.json");
            fs::write(&umbrella, serde_json::to_vec(&json!({"issue":{
                "repository":"agent-logic/agent-design-language","number":866,"title":"fixture sprint",
                "state":"open","body":"- Membership version: `1`\n## Child issues\n- #867\n"
            }})).unwrap()).unwrap();
            fs::write(&child, serde_json::to_vec(&json!({"issue":{
                "repository":"agent-logic/agent-design-language","number":867,"title":"fixture child",
                "state":"open","body":"fixture"
            }})).unwrap()).unwrap();
            let sprint_request = helper_dir.join("sprint.json");
            fs::write(&sprint_request, serde_json::to_vec(&json!({
                "repository":"agent-logic/agent-design-language","version":"v0.92.2",
                "sprints":[{"sprint":1,"umbrella_issue":866,"title":"fixture sprint",
                    "execution_mode":"sequential","serial_gates":["accepted predecessor"],
                    "umbrella_readback_ref":".csdlc/evidence/observation-helpers/umbrella.json",
                    "child_readback_refs":{"867":".csdlc/evidence/observation-helpers/child.json"}}]
            })).unwrap()).unwrap();
            let contract_path = fixture.root.join("docs/csdlc-v3/CONTRACT.md");
            let contract = fs::read(&contract_path).unwrap();
            if state == "missing" {
                fs::remove_file(&contract_path).unwrap();
                fs::remove_file(&child).unwrap();
            } else if state == "corrupt" {
                fs::write(&contract_path, "invalid contract").unwrap();
                fs::write(&child, "invalid-json").unwrap();
            }
            for helper in ["foundation", "local", "sprint"] {
                let before_helper = observation_inventory(&primary);
                let mut cmd = Command::new(&installed);
                cmd.current_dir(&fixture.root).arg(helper);
                if helper == "foundation" || helper == "sprint" {
                    cmd.arg("--repo-root").arg(&fixture.root);
                }
                if helper == "local" {
                    cmd.arg("--request")
                        .arg(&local_request)
                        .arg("--registry")
                        .arg(repo_root().join("docs/templates/prompts/current.json"))
                        .arg("--registrations")
                        .arg(&helper_registrations);
                } else if helper == "sprint" {
                    cmd.arg("--request").arg(&sprint_request);
                }
                let output = cmd.output().unwrap();
                assert_eq!(
                    observation_inventory(&primary),
                    before_helper,
                    "{helper} wrote inspected storage"
                );
                // The retained local construction helper requires a registered
                // target even for its preview. Preserve that baseline refusal
                // on primary; never fabricate a future registration to pass.
                let expected_success = if helper == "local" {
                    linked
                } else {
                    !matches!(state, "missing" | "corrupt")
                };
                assert_eq!(
                    output.status.success(),
                    expected_success,
                    "{helper}, {state}: {output:?}"
                );
                if output.status.success() && helper != "foundation" {
                    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                    assert_eq!(report["read_only"], true);
                    assert_eq!(report["operational_authority"], false);
                }
            }
            fs::write(contract_path, contract).unwrap();
            if state == "pending" {
                if linked {
                    let mut different = fixture.request.clone();
                    different.title = "not the original interrupted request".into();
                    fs::write(
                        &fixture.request_path,
                        serde_json::to_vec(&different).unwrap(),
                    )
                    .unwrap();
                    let before_retry = observation_inventory(&primary);
                    let rejected = run_operational(&fixture, "edit", None);
                    assert!(
                        !rejected.status.success(),
                        "different request recovered pending edit"
                    );
                    assert_eq!(observation_inventory(&primary), before_retry);
                    fs::write(
                        &fixture.request_path,
                        serde_json::to_vec(&fixture.request).unwrap(),
                    )
                    .unwrap();
                }
                // The recovery operation advertised by diagnostics must work
                // in the actual linked checkout as well as the primary.
                let recovered = run_operational(&fixture, "edit", None);
                assert!(
                    recovered.status.success(),
                    "advertised recovery failed: {recovered:?}"
                );
                assert!(!storage.join("transactions/505.json").exists());
                let values: serde_json::Value = serde_json::from_slice(
                    &fs::read(storage.join("issues/505/cards/sip.values.json")).unwrap(),
                )
                .unwrap();
                assert_eq!(values["title"], "Pending edit");
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn installed_pr_state_observes_fake_remote_without_replaying_pending_mutation() {
    use csdlc_v3::commands::remote::{typed_review_receipt_payload_digest, TypedReviewReceipt};
    use std::os::unix::fs::PermissionsExt;
    for linked in [false, true] {
        for state in ["healthy", "missing", "corrupt", "pending"] {
            let mut fixture = operational_fixture(&format!("remote-observation-{linked}-{state}"));
            let primary = fixture.root.clone();
            initialize_operational_fixture(&mut fixture);
            if linked {
                let bound = run_operational(&fixture, "bind", None);
                assert!(bound.status.success(), "{bound:?}");
                fixture.root = PathBuf::from(&fixture.request.worktree);
            }
            let head = git(&fixture.root, &["rev-parse", "HEAD"]);
            let inputs = fixture.root.join(".csdlc/evidence/505/remote-observation");
            fs::create_dir_all(&inputs).unwrap();
            let review = TypedReviewReceipt {
                schema: "csdlc.v3.typed_review_receipt.v1".into(),
                repository: "agent-logic/agent-design-language".into(),
                issue: 505,
                implementer: "fixture-author".into(),
                reviewer: "fixture-reviewer".into(),
                reviewed_revision: head.clone(),
                expected_head_sha: head.clone(),
                evidence_digest: "fixture-review-evidence".into(),
            };
            let review_path = inputs.join("review.json");
            fs::write(&review_path, serde_json::to_vec(&review).unwrap()).unwrap();
            let request_path = inputs.join("pr-state.json");
            fs::write(&request_path,serde_json::to_vec(&json!({
                "repository":"agent-logic/agent-design-language","issue":505,"pull_request":639,
                "actor":"fixture-author","implementer":"fixture-author","reviewer":"fixture-reviewer",
                "review_revision":head,"expected_head_sha":head,"head_sha":head,
                "mode":"closing","title":"fixture PR","body":"Closes #505","review_present":true,
                "typed_review_receipt_path":review_path,"typed_review_receipt_digest":typed_review_receipt_payload_digest(&review),
                "credential_names":["GITHUB_TOKEN"]
            })).unwrap()).unwrap();
            let installed = inputs.join("candidate/csdlc");
            observation::install_candidate(&installed);
            let fake_bin = inputs.join("fake-bin");
            fs::create_dir_all(&fake_bin).unwrap();
            let curl = fake_bin.join("curl");
            let response = json!({"number":639,"node_id":"PR_fixture_639","head":{"sha":head},
                "draft":true,"title":"fixture PR","body":"Closes #505"})
            .to_string();
            // Any write attempt fails visibly. No URL can escape this fake
            // transport, and the diagnostic's fixture token is never real.
            let read_response = match state {
                "missing" => "exit 22".to_owned(),
                "corrupt" => "printf '%s' 'not-json-SIM01_SECRET_SENTINEL'".to_owned(),
                _ => format!("printf '%s' '{response}'"),
            };
            fs::write(&curl, format!("#!/bin/sh\ncase \"$*\" in *'--config -'*) cat >/dev/null;; esac\ncase \"$*\" in\n *graphql*|*POST*|*PATCH*|*PUT*|*DELETE*) exit 7;;\n *api.github.com/repos/agent-logic/agent-design-language/pulls/639*) {read_response};;\n *) exit 9;;\nesac\n")).unwrap();
            fs::set_permissions(&curl, fs::Permissions::from_mode(0o700)).unwrap();
            let path = format!(
                "{}:{}",
                fake_bin.display(),
                std::env::var("PATH").unwrap_or_default()
            );
            let token = inputs.join("token");
            fs::write(&token, "SIM01_SECRET_SENTINEL").unwrap();
            let command = |route: &str, request: &Path, flag: &str| {
                let mut cmd = Command::new(&installed);
                cmd.current_dir(&fixture.root)
                    .arg(route)
                    .arg("--request")
                    .arg(request)
                    .arg(flag)
                    .env("GITHUB_TOKEN", "SIM01_SECRET_SENTINEL")
                    .env("ADL_GITHUB_TOKEN_FILE", &token)
                    .env("PATH", &path);
                cmd.output().unwrap()
            };
            if state == "pending" {
                let mutation_path = inputs.join("pending-ready.json");
                let mutation = OperationalRemoteDispatchRequest {
                    expected_lifecycle_digest: canonical_authority_selector_digest(&fixture.root)
                        .unwrap(),
                    exact_review_sha: head.clone(),
                    operation: OperationalRemoteOperation::GithubMutation(GithubMutationRequest {
                        repository: "agent-logic/agent-design-language".into(),
                        issue: 505,
                        pull_request: Some(639),
                        cutover_issue: Some(505),
                        operator_approval: Some("isolated fake pending fixture".into()),
                        expected_head_sha: head.clone(),
                        credential_names: vec!["GITHUB_TOKEN".into()],
                        recovery: None,
                        mutation: GithubMutation::PullRequestReady,
                    }),
                };
                fs::write(&mutation_path, serde_json::to_vec(&mutation).unwrap()).unwrap();
                let failed = command("github-pr", &mutation_path, "--execute");
                assert!(!failed.status.success());
                let failure: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
                assert_eq!(failure["envelope"]["status"], "recovery_required");
                assert_eq!(failure["envelope"]["effects"]["outcome"], "unknown");
                assert!(
                    String::from_utf8_lossy(&failed.stdout).contains("uncertain"),
                    "{failed:?}"
                );
            }
            let before = observation_inventory(&primary);
            let output = command("pr-state", &request_path, "--observe-github");
            let after = observation_inventory(&primary);
            let changed: std::collections::BTreeSet<_> = before
                .keys()
                .chain(after.keys())
                .filter(|path| before.get(*path) != after.get(*path))
                .collect();
            assert!(
                changed.is_empty(),
                "pr-state changed pending/evidence storage: {changed:?}"
            );
            assert!(!String::from_utf8_lossy(&output.stdout).contains("SIM01_SECRET_SENTINEL"));
            assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM01_SECRET_SENTINEL"));
            if state == "healthy" {
                assert!(output.status.success(), "{output:?}");
                let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                assert_eq!(report["read_only"], true);
                assert_eq!(report["result"]["status"], "ready");
            } else if state == "pending" {
                let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                assert_ne!(
                    report["result"]["status"], "ready",
                    "pending remote intent was hidden by ready observation"
                );
                // Explicit retry observes that the fake remote has become ready;
                // the mutator, not pr-state, may now retain reconciliation.
                let settled_response = response.replace("\"draft\":true", "\"draft\":false");
                fs::write(&curl, format!("#!/bin/sh\ncase \"$*\" in *'--config -'*) cat >/dev/null;; esac\ncase \"$*\" in *graphql*|*POST*|*PATCH*|*PUT*|*DELETE*) exit 7;;\n *api.github.com/repos/agent-logic/agent-design-language/pulls/639*) printf '%s' '{settled_response}';;\n *) exit 9;; esac\n")).unwrap();
                let settled = command("github-pr", &inputs.join("pending-ready.json"), "--execute");
                assert!(settled.status.success(), "{settled:?}");
                let before = observation_inventory(&primary);
                let observed = command("pr-state", &request_path, "--observe-github");
                assert!(observed.status.success(), "{observed:?}");
                let report: serde_json::Value = serde_json::from_slice(&observed.stdout).unwrap();
                assert_eq!(report["result"]["status"], "ready");
                assert_eq!(before, observation_inventory(&primary));
                let receipt_path = fs::read_dir(primary.join(".git/csdlc-v3/remote/mutations"))
                    .unwrap()
                    .next()
                    .unwrap()
                    .unwrap()
                    .path();
                let mut receipt: serde_json::Value =
                    serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
                receipt["pull_request"] = json!(640);
                fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
                let before = observation_inventory(&primary);
                let observed = command("pr-state", &request_path, "--observe-github");
                let report: serde_json::Value = serde_json::from_slice(&observed.stdout).unwrap();
                assert_ne!(report["result"]["status"], "ready");
                assert_eq!(before, observation_inventory(&primary));
            } else {
                assert!(!output.status.success(), "{output:?}");
                let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                assert_eq!(report["writes_v3_state"], false);
                assert!(!String::from_utf8_lossy(&output.stderr).contains("\"code\""));
            }
        }
    }
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
fn v2_selector_denies_operational_local_but_allows_explicit_construction_inspection() {
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

    assert!(!output.status.success(), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["envelope"]["status"], "blocked");
    assert_eq!(
        report["envelope"]["reason_code"],
        "operational_context_required"
    );
    assert!(!fixture.join(".csdlc").exists());
    let historical = run(
        &[
            "local",
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
    assert!(historical.status.success(), "{historical:?}");
    let historical: serde_json::Value = serde_json::from_slice(&historical.stdout).unwrap();
    assert_eq!(historical["operational_authority"], false);
    assert_eq!(historical["writes_v3_state"], false);
    assert_eq!(historical["envelope"]["effects"]["outcome"], "none");
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
    let rollback_json: serde_json::Value = serde_json::from_slice(&rollback.stdout).unwrap();
    assert_eq!(rollback_json["envelope"]["status"], "blocked");
    assert_eq!(rollback_json["envelope"]["effects"]["outcome"], "none");
    assert_eq!(rollback_json["command"], "rollback");
    assert_eq!(rollback_json["result"]["status"], "blocked");
    assert_eq!(rollback_json["performed_mutation"], false);
}

// PVF: required deterministic local CLI routing; small CPU, no network dispatch.
#[test]
fn executable_merge_is_owned_only_by_github_pr() {
    let fixture = operational_fixture("merge-route-family");
    let head = git(&fixture.root, &["rev-parse", "HEAD"]);
    let dispatch = serde_json::json!({
        "expected_lifecycle_digest": canonical_authority_selector_digest(&fixture.root).unwrap(),
        "exact_review_sha": head,
        "operation": {"kind":"github_mutation","request":{
            "repository":"agent-logic/agent-design-language","issue":505,"pull_request":591,
            "expected_head_sha":head,"credential_names":["GITHUB_TOKEN"],
            "mutation":{"action":"pull_request_merge","base":"main","method":"merge","review_receipt_path":"missing-review.json","review_receipt_digest":"missing"}
        }}
    });
    let path = fixture.root.join("merge-dispatch.json");
    fs::write(&path, serde_json::to_vec(&dispatch).unwrap()).unwrap();
    for route in ["github", "github-issue", "publish", "review"] {
        let output = run(
            &[route, "--request", path.to_str().unwrap(), "--execute"],
            &fixture.root,
        );
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("operational_remote_route_mismatch"),
            "{route}: {output:?}"
        );
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["envelope"]["process_status"], "failed");
    }
    let output = run(
        &[
            "github-pr",
            "--request",
            path.to_str().unwrap(),
            "--execute",
        ],
        &fixture.root,
    );
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("operational_remote_route_mismatch"));
}

// PVF: required SIM-02/SIM-07 installed result routing; deterministic local
// CPU/Git fixtures; all effects observed, no network or live owner activation.
#[test]
fn installed_shepherd_result_states_preserve_owner_routing() {
    let fixture = operational_fixture("sim02-shepherd-envelope");
    copy_observation_templates(
        &repo_root().join("docs/templates/prompts/1.0.5"),
        &fixture.root.join("docs/templates/prompts/1.0.5"),
    );
    let installed = fixture.root.join(".git/installed-candidate/csdlc");
    observation::install_candidate(&installed);
    let invoke = |route: &str| {
        Command::new(&installed)
            .current_dir(&fixture.root)
            .args([route, "--request"])
            .arg(&fixture.request_path)
            .arg("--registry")
            .arg(repo_root().join("docs/templates/prompts/current.json"))
            .arg("--registrations")
            .arg(&fixture.registrations_path)
            .output()
            .unwrap()
    };
    let prepared = invoke("issue");
    assert!(prepared.status.success(), "{prepared:?}");
    let prepared: serde_json::Value = serde_json::from_slice(&prepared.stdout).unwrap();
    let mut request = serde_json::to_value(&fixture.request).unwrap();
    request["expected_lifecycle_digest"] = prepared["result"]["digest"].clone();
    let mut reports = Vec::new();
    for (field, owner_state, common) in [
        ("operator_decision_needed", "operator_required", "blocked"),
        ("repair_needed", "repair_required", "blocked"),
        ("retryable_failure", "retryable", "failed"),
        ("dependency_wait", "waiting", "deferred"),
        ("none", "ready", "ready"),
    ] {
        let mut routing = json!({"validation":"passed"});
        if field != "none" {
            routing[field] = json!(true);
        }
        request["shepherd_routing"] = routing;
        fs::write(&fixture.request_path, serde_json::to_vec(&request).unwrap()).unwrap();
        let before = observation_inventory(&fixture.root);
        let output = invoke("shepherd");
        assert!(output.status.success(), "{output:?}");
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["result"]["routing"]["state"], owner_state);
        assert_eq!(report["envelope"]["status"], common);
        assert_eq!(report["envelope"]["effects"]["outcome"], "none");
        assert_eq!(
            report["envelope"]["wait_owner"],
            if common == "ready" {
                "none"
            } else {
                "operator"
            }
        );
        assert_eq!(before, observation_inventory(&fixture.root));
        reports.push(report);
    }
    let evidence = repo_root().join("csdlc-v3/target/sim02-envelope-samples");
    fs::create_dir_all(&evidence).unwrap();
    fs::write(
        evidence.join("shepherd.json"),
        serde_json::to_vec_pretty(&reports).unwrap(),
    )
    .unwrap();
}

// PVF: required installed heterogeneous helper-envelope contract; deterministic
// local JSON/filesystem observations, no network or operational authority.
#[test]
fn installed_helper_envelopes_normalize_release_and_sprint_reports() {
    let fixture = operational_fixture("sim02-helper-envelope");
    let installed = fixture.root.join(".git/installed-candidate/csdlc");
    observation::install_candidate(&installed);
    fs::write(
        fixture.root.join("invalid-readback.json"),
        b"{\"issue\":{}}",
    )
    .unwrap();
    let mut reports = Vec::new();
    for (name, route, request, status) in [
        (
            "release",
            "release-preflight",
            json!({"repository":"agent-logic/agent-design-language","version":"v0.92.2","candidate_sha":"0".repeat(40),"notes_path":"missing.md","notes_digest":"0".repeat(64),"gate_path":"missing.json","gate_digest":"0".repeat(64)}),
            "failed",
        ),
        (
            "sprint-empty-completion-mapping",
            "sprint",
            json!({"repository":"agent-logic/agent-design-language","version":"v0.92.2","sprints":[]}),
            "completed",
        ),
        (
            "sprint-invalid",
            "sprint",
            json!({"repository":"agent-logic/agent-design-language","version":"v0.92.2","sprints":[{"sprint":1,"umbrella_issue":866,"title":"Missing readback fixture","execution_mode":"sequential","serial_gates":[],"umbrella_readback_ref":"invalid-readback.json","child_readback_refs":{}}]}),
            "failed",
        ),
    ] {
        let path = fixture.root.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_vec(&request).unwrap()).unwrap();
        let before = observation_inventory(&fixture.root);
        let mut command = Command::new(&installed);
        command.current_dir(&fixture.root).arg(route);
        if route == "sprint" {
            command.arg("--repo-root").arg(&fixture.root);
        }
        command.arg("--request").arg(&path);
        let output = command.output().unwrap();
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["envelope"]["status"], status, "{output:?}");
        assert_eq!(report["envelope"]["effects"]["outcome"], "none");
        assert_eq!(
            report["envelope"]["authority_status"],
            if route == "sprint" {
                "non_operational"
            } else {
                "not_established"
            }
        );
        assert!(report["envelope"]["findings"]
            .as_array()
            .unwrap()
            .iter()
            .all(serde_json::Value::is_object));
        if route == "release-preflight" {
            assert!(!output.status.success());
            assert_eq!(report["findings"], json!(["unsupported_release_identity"]));
            assert_eq!(
                report["envelope"]["findings"],
                json!([{"code":"unsupported_release_identity","message":"unsupported_release_identity"}])
            );
            assert_eq!(
                report["envelope"]["reason_code"],
                "unsupported_release_identity"
            );
        } else {
            assert!(
                output.status.success(),
                "sprint classification preserves legacy exit semantics"
            );
            assert_eq!(report["envelope"]["process_status"], "succeeded");
            if name == "sprint-invalid" {
                assert_eq!(report["status"], "invalid");
                assert!(report["envelope"]["findings"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|finding| finding["code"] == "serial_gates_missing"));
                assert_eq!(report["envelope"]["reason_code"], "serial_gates_missing");
            } else {
                assert_eq!(report["status"], "complete_not_cutover_authority");
            }
        }
        assert_eq!(before, observation_inventory(&fixture.root));
        reports.push(json!({"label":name,"envelope":report["envelope"]}));
    }
    let path = repo_root().join("csdlc-v3/target/sim02-envelope-samples/helpers.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec_pretty(&reports).unwrap()).unwrap();
}
