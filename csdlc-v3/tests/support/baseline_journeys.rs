//! PVF: installed deterministic fixture attempts, local Git/CPU/disk, required
//! SIM baseline input. Subsequent remote/proof/terminal scenarios share recorder.
use super::attempt_corpus::{Corpus, FixtureClock};
use super::*;

fn corpus_file(name: &str) -> PathBuf {
    repo_root()
        .join("csdlc-v3/target/sim01-corpus")
        .join(std::process::id().to_string())
        .join(name)
}

// PVF: measurement reliability only; these cases do not prove CLI semantics.
#[test]
fn attempt_recorder_retains_spawn_parse_and_collector_failures() {
    use super::attempt_corpus::{Clock, ClockSample};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    let fixture = operational_fixture("attempt-retention");
    let output = corpus_file("retention-negative.json");
    let index = fixture.root.join("absent-index");
    let binary = Path::new(env!("CARGO_BIN_EXE_csdlc"));
    let mut corpus = Corpus::new(
        &fixture.root,
        binary,
        &repo_root(),
        FixtureClock(0),
        &output,
    );
    assert!(catch_unwind(AssertUnwindSafe(|| {
        corpus.run(
            ("spawn", "missing", "guard"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            &mut Command::new("/nonexistent-sim01-command"),
        );
    }))
    .is_err());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(saved["attempts"][0]["outcome"], "failed");
    assert_eq!(saved["denominators"]["attempted"], 1);
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let result = corpus.run(
            ("parse", "printf", "malformed_output"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            Command::new("/usr/bin/printf").arg("not-json"),
        );
        serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap();
    }))
    .is_err());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(saved["attempts"][1]["outcome"], "failed");
    assert_eq!(saved["attempts"][1]["stdout_text"], "not-json");
    struct InterruptedCollector(bool);
    impl Clock for InterruptedCollector {
        fn sample(&mut self) -> ClockSample {
            assert!(!self.0, "injected collection interruption");
            self.0 = true;
            ClockSample {
                wall_unix_millis: 100,
                monotonic_millis: 0,
            }
        }
    }
    let interrupted = output.with_file_name("retention-censored.json");
    let mut corpus = Corpus::new(
        &fixture.root,
        binary,
        &repo_root(),
        InterruptedCollector(false),
        &interrupted,
    );
    assert!(catch_unwind(AssertUnwindSafe(|| {
        corpus.run(
            ("collector", "printf", "collection_interrupted"),
            &fixture.root,
            (&index, &index),
            &json!({}),
            Command::new("/usr/bin/printf").arg("{}"),
        );
    }))
    .is_err());
    let saved: serde_json::Value =
        serde_json::from_slice(&fs::read(&interrupted).unwrap()).unwrap();
    assert_eq!(saved["denominators"]["censored"], 1);
    assert_eq!(saved["attempts"][0]["outcome"], "censored");
}

#[test]
fn installed_local_journey_baseline() {
    let mut fixture = operational_fixture("attempt-baseline-local");
    let primary = fixture.root.clone();
    copy_observation_templates(
        &repo_root().join("docs/templates/prompts/1.0.5"),
        &primary.join("docs/templates/prompts/1.0.5"),
    );
    git(&primary, &["add", "docs"]);
    git(
        &primary,
        &["commit", "--quiet", "-m", "fixture prompt templates"],
    );
    git(
        &primary,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
    let installed = primary.join(".git/installed-candidate/csdlc");
    observation::install_candidate(&installed);
    let mut corpus = Corpus::new(
        &primary,
        &installed,
        &repo_root(),
        FixtureClock(0),
        &corpus_file("baseline.json"),
    );
    let mut index = primary.join(".git/csdlc-v3/local/issues/505/index.json");
    for (id, route, scenario, crash) in [
        ("local-01", "issue", "healthy_prepare", None),
        ("local-01-inspect", "doctor", "primary_discovery", None),
        ("local-02", "bind", "healthy_bind", None),
        ("local-03", "edit", "healthy_edit", None),
        (
            "local-04",
            "edit",
            "interrupted_edit",
            Some("after_backup_rename"),
        ),
        ("local-05", "doctor", "pending_observation", None),
        ("local-06", "edit", "explicit_recovery", None),
        ("local-07", "doctor", "nonprimary_discovery", None),
    ] {
        if scenario == "healthy_edit" || scenario == "interrupted_edit" {
            fixture
                .request
                .card_updates
                .insert("sip".into(), json!({"title":scenario}));
        }
        fs::write(
            &fixture.request_path,
            serde_json::to_vec(&fixture.request).unwrap(),
        )
        .unwrap();
        let mut command = Command::new(&installed);
        command
            .current_dir(&fixture.root)
            .arg(route)
            .arg("--request")
            .arg(&fixture.request_path)
            .arg("--registry")
            .arg(repo_root().join("docs/templates/prompts/current.json"))
            .arg("--registrations")
            .arg(&fixture.registrations_path);
        if let Some(point) = crash {
            command.env("CSDLC_V3_TEST_CRASH_POINT", point);
        }
        let request = serde_json::to_value(&fixture.request).unwrap();
        let after_index = if route == "bind" {
            PathBuf::from(&fixture.request.worktree).join(".csdlc/issues/505/index.json")
        } else {
            index.clone()
        };
        let output = corpus.run(
            (id, route, scenario),
            &fixture.root,
            (&index, &after_index),
            &request,
            &mut command,
        );
        if crash.is_some() {
            let expected = output.status.code() == Some(91);
            corpus.classify(
                if expected { "interrupted" } else { "failed" },
                "injected process exit between directory swaps",
                false,
                Some("pending local transaction"),
            );
            assert!(
                expected,
                "unexpected injected crash result; failed attempt retained"
            );
            continue;
        }
        if scenario == "pending_observation" {
            let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert!(!output.status.success());
            assert!(report
                .to_string()
                .contains("local_transaction_recovery_required"));
            assert_eq!(corpus.attempts.last().unwrap()["effects"], json!([]));
            corpus.classify(
                "blocked",
                "local_transaction_recovery_required",
                false,
                Some("pending local transaction"),
            );
            continue;
        }
        if !output.status.success() {
            corpus.classify("failed", "installed journey did not complete", false, None);
            corpus.write(&corpus_file("baseline.json"));
            panic!("{scenario}: {output:?}");
        }
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["schema"], "csdlc.v3.operational_local.v1");
        assert_eq!(report["operational_authority"], true);
        assert_eq!(report["result"]["issue"], 505);
        assert_eq!(report["result"]["route"], route);
        assert_eq!(
            report["result"]["phase"],
            if route == "issue" || scenario == "primary_discovery" {
                "ready"
            } else {
                "bound"
            }
        );
        assert!(report["result"]["digest"]
            .as_str()
            .is_some_and(|digest| !digest.is_empty()));
        if let Some(digest) = report["result"]["digest"].as_str() {
            fixture.request.expected_lifecycle_digest = Some(digest.into());
        }
        if route == "bind" {
            fixture.root = PathBuf::from(&fixture.request.worktree);
            index = fixture.root.join(".csdlc/issues/505/index.json");
        }
        corpus.classify("completed", "installed operation completed", true, None);
    }
    git(
        &primary,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    let inputs = fixture.root.join(".csdlc/evidence/505/journey");
    fs::create_dir_all(&inputs).unwrap();
    observation::install_candidate(&inputs.join("csdlc"));
    fs::write(
        inputs.join("doctor.json"),
        serde_json::to_vec(&fixture.request).unwrap(),
    )
    .unwrap();
    fs::write(
        inputs.join("source.json"),
        b"{\"role\":\"installed-doctor-proof-input\"}",
    )
    .unwrap();
    let evidence_digest = blake3::hash(&fs::read(inputs.join("source.json")).unwrap())
        .to_hex()
        .to_string();
    let bound: serde_json::Value = serde_json::from_slice(&fs::read(&index).unwrap()).unwrap();
    let proof = json!({
        "issue":505,"repository":fixture.request.repository,
        "binding":{"worktree":fixture.root,"branch":fixture.request.branch,
            "exact_head":git(&fixture.root,&["rev-parse","HEAD"]),"git_common_dir":primary.join(".git"),
            "generation":bound["generation"],"lifecycle_digest":bound["digest"]},
        "evidence_root":fixture.root,
        "proof":{"manifest_id":"sim01-installed-doctor","lane":"worktree","deterministic":true,
            "evidence_ref":".csdlc/evidence/505/journey/source.json","evidence_digest":evidence_digest,
            "observed_digest":evidence_digest,"stale":false,"normalization":"doctor_issue_phase_v1",
            "command":{"generation":"v3","binary_ref":".csdlc/evidence/505/journey/csdlc",
                "argv":["doctor","--request",".csdlc/evidence/505/journey/doctor.json","--registry",repo_root().join("docs/templates/prompts/current.json"),"--registrations",fixture.registrations_path],
                "request_ref":".csdlc/evidence/505/journey/doctor.json","timeout_millis":10000,
                "side_effect_boundary_refs":[".csdlc/issues/505"],"provider_side_effects":false}}
    });
    let proof_path = inputs.join("proof.json");
    fs::write(&proof_path, serde_json::to_vec(&proof).unwrap()).unwrap();
    let mut command = Command::new(&installed);
    command
        .current_dir(&fixture.root)
        .args(["proof", "--request"])
        .arg(&proof_path);
    let output = corpus.run(
        ("local-08", "proof", "healthy_bound_proof"),
        &fixture.root,
        (&index, &index),
        &proof,
        &mut command,
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    if report["status"] == "ready" {
        assert_eq!(report["evidence_refs"].as_array().unwrap().len(), 1);
        corpus.record_nested_cli(1);
    }
    let completed = output.status.success() && report["status"] == "ready";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "native bound proof outcome",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "proof: {output:?}");
    remote_terminal_baseline(&fixture, &primary, &installed, &index, &mut corpus);
}

fn remote_terminal_baseline(
    fixture: &OperationalFixture,
    primary: &Path,
    installed: &Path,
    index: &Path,
    corpus: &mut Corpus<FixtureClock>,
) {
    use csdlc_v3::commands::remote::{
        github_mutation_operation_digest, github_mutation_operation_marker,
        typed_review_receipt_payload_digest, GithubMutation, GithubMutationRequest,
        TypedReviewReceipt,
    };
    use std::os::unix::fs::PermissionsExt;
    let head = git(&fixture.root, &["rev-parse", "HEAD"]);
    let inputs = primary.join(".git/journey-remote");
    fs::create_dir_all(&inputs).unwrap();
    let fake_bin = inputs.join("bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let remote_flag = inputs.join("published");
    let token = inputs.join("token");
    fs::write(&token, "SIM01_SECRET_SENTINEL").unwrap();
    let mutation = GithubMutationRequest {
        repository: fixture.request.repository.clone(),
        issue: 505,
        pull_request: None,
        cutover_issue: Some(505),
        operator_approval: Some("isolated baseline fixture".into()),
        expected_head_sha: head.clone(),
        credential_names: vec!["GITHUB_TOKEN".into()],
        recovery: None,
        mutation: GithubMutation::PullRequestCreate {
            base: "main".into(),
            head: fixture.request.branch.clone(),
            title: "Baseline fixture".into(),
            body: "Closes #505".into(),
            draft: true,
        },
    };
    let marker = github_mutation_operation_marker(&github_mutation_operation_digest(&mutation));
    let pr = json!({"number":639,"id":639,"title":"Baseline fixture","body":format!("Closes #505\n\n{marker}"),
        "draft":true,"merged":false,"head":{"sha":head,"ref":fixture.request.branch},"base":{"ref":"main"}});
    let pr_state = inputs.join("pr-state.json");
    fs::write(&pr_state, serde_json::to_vec(&pr).unwrap()).unwrap();
    let quote = |s: &str| format!("'{}'", s.replace('\'', "'\\''"));
    // Fake transport never reaches a network. The fake published flag is an
    // explicit remote effect, separately visible in the attempt byte inventory.
    let script = format!("#!/bin/sh\ncase \"$*\" in *'--config -'*) cat >/dev/null;; esac\ncase \"$*\" in\n *POST*api.github.com/repos/agent-logic/agent-design-language/pulls*) touch {}; printf '%s' {};;\n *api.github.com/repos/agent-logic/agent-design-language/pulls\\?*) if test -f {}; then printf '%s' {}; else printf '[]'; fi;;\n *api.github.com/repos/agent-logic/agent-design-language/pulls/639*) cat {};;\n *api.github.com/repos/agent-logic/agent-design-language/issues/505*) printf '%s' '{{\"number\":505,\"state\":\"closed\"}}';;\n *) exit 9;; esac\n",
        quote(remote_flag.to_str().unwrap()),quote(&pr.to_string()),quote(remote_flag.to_str().unwrap()),quote(&json!([pr]).to_string()),quote(pr_state.to_str().unwrap()));
    let script = script.replacen(
        "#!/bin/sh\n",
        &format!(
            "#!/bin/sh\nprintf 'call\\n' >> {}\n",
            quote(corpus.transport_log.to_str().unwrap())
        ),
        1,
    );
    fs::write(fake_bin.join("curl"), script).unwrap();
    fs::set_permissions(fake_bin.join("curl"), fs::Permissions::from_mode(0o700)).unwrap();
    let path = format!("{}:{}", fake_bin.display(), std::env::var("PATH").unwrap());
    let digest = canonical_authority_selector_digest(&fixture.root).unwrap();
    let mut review_request = json!({"repository":fixture.request.repository,"issue":505,"pull_request":639,
        "actor":"fixture-author","implementer":"fixture-author","reviewer":"fixture-reviewer",
        "review_revision":head,"expected_head_sha":head,"head_sha":head,"mode":"closing",
        "title":"Baseline fixture","body":"Closes #505","review_present":true,"credential_names":["GITHUB_TOKEN"]});
    let invoke = |corpus: &mut Corpus<FixtureClock>,
                  id: &str,
                  route: &str,
                  scenario: &str,
                  request: &serde_json::Value,
                  flag: Option<&str>,
                  cwd: &Path| {
        let request_path = inputs.join("invoke.json");
        fs::write(&request_path, serde_json::to_vec(request).unwrap()).unwrap();
        let mut command = Command::new(installed);
        command
            .current_dir(cwd)
            .args([route, "--request"])
            .arg(&request_path)
            .env("PATH", &path)
            .env("GITHUB_TOKEN", "SIM01_SECRET_SENTINEL")
            .env("ADL_GITHUB_TOKEN_FILE", &token);
        if let Some(flag) = flag {
            command.arg(flag);
        }
        let output = corpus.run(
            (id, route, scenario),
            cwd,
            (index, index),
            request,
            &mut command,
        );
        let report: serde_json::Value =
            serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Null);
        (output, report)
    };
    let review_dispatch = json!({"expected_lifecycle_digest":digest,"exact_review_sha":head,"operation":{"kind":"review","request":review_request}});
    let (output, report) = invoke(
        corpus,
        "remote-01",
        "review",
        "healthy_review_validation",
        &review_dispatch,
        Some("--execute"),
        &fixture.root,
    );
    let completed =
        output.status.success() && report["result"]["outcome"]["result"]["status"] == "ready";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "native review validation of fixture principals; not human review",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "review {output:?}");
    let review = TypedReviewReceipt {
        publication_linkage: None,
        schema: "csdlc.v3.typed_review_receipt.v1".into(),
        repository: fixture.request.repository.clone(),
        issue: 505,
        implementer: "fixture-author".into(),
        reviewer: "fixture-reviewer".into(),
        reviewed_revision: head.clone(),
        expected_head_sha: head.clone(),
        evidence_digest: blake3::hash(&output.stdout).to_hex().to_string(),
    };
    let review_path = fixture.root.join(".csdlc/evidence/505/journey/review.json");
    fs::write(&review_path, serde_json::to_vec(&review).unwrap()).unwrap();
    review_request["typed_review_receipt_path"] = json!(review_path);
    review_request["typed_review_receipt_digest"] =
        json!(typed_review_receipt_payload_digest(&review));
    let publication = json!({"expected_lifecycle_digest":digest,"exact_review_sha":head,"operation":{"kind":"publish","request":review_request}});
    let (output, report) = invoke(
        corpus,
        "remote-02",
        "publish",
        "publication_guard_missing_readback",
        &publication,
        Some("--execute"),
        &fixture.root,
    );
    let denied = report["result"]["outcome"]["result"]["status"] == "blocked";
    corpus.classify(
        if denied { "blocked" } else { "failed" },
        "publication requires authenticated readback before acceptance",
        false,
        Some("readback not yet retained"),
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(denied, "publication guard {output:?}");
    let dispatch = json!({"expected_lifecycle_digest":digest,"exact_review_sha":head,"operation":{"kind":"github_mutation","request":mutation}});
    let (output, report) = invoke(
        corpus,
        "remote-03",
        "github-pr",
        "healthy_fake_pr_create",
        &dispatch,
        Some("--execute"),
        &fixture.root,
    );
    let completed = output.status.success()
        && report["result"]["outcome"]["result"]["receipt"]["pull_request"] == 639;
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "native PR create with fake authenticated reconciliation",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed && remote_flag.exists(), "create {output:?}");
    let (output, report) = invoke(
        corpus,
        "remote-04",
        "pr-state",
        "published_pr_readback",
        &review_request,
        Some("--observe-github"),
        &fixture.root,
    );
    let completed = output.status.success() && report["result"]["status"] == "ready";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "installed authenticated fake PR readback",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "readback {output:?}");
    // Publication consumes authenticated fake readback through the same
    // native observation path as operator publication, without forged files.
    let (output, report) = invoke(
        corpus,
        "remote-05",
        "publish",
        "healthy_publication_validation",
        &review_request,
        Some("--observe-github"),
        &fixture.root,
    );
    let completed = output.status.success() && report["result"]["status"] == "ready";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "publication accepts fixture review and native fake-transport readback",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "publish {output:?}");
    let mut merged_pr = pr.clone();
    merged_pr["draft"] = json!(false);
    merged_pr["merged"] = json!(true);
    fs::write(&pr_state, serde_json::to_vec(&merged_pr).unwrap()).unwrap();
    corpus.attempts.last_mut().unwrap()["external_event_after"] = json!({"kind":"simulated_remote_merge","actor":"fixture","scope":"fake PR639; no real merge command","wait_duration":"not measured"});
    let terminal = json!({"repository":fixture.request.repository,"issue":505,"pull_request":639,"expected_head_sha":head,"mode":"closing",
        "credential_names":["GITHUB_TOKEN"],"terminal_state":{"repository_root":primary,
            "state_path":".git/csdlc-v3/local/v3/issues/505/terminal.json","receipt_path":".git/csdlc-v3/local/evidence/505/terminal-receipt.json"}});
    let (output, report) = invoke(
        corpus,
        "terminal-01",
        "finish",
        "healthy_fake_finish",
        &terminal,
        Some("--observe-github"),
        primary,
    );
    let completed = output.status.success()
        && report["performed_mutation"] == true
        && report["result"]["status"] == "ready";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "native finish persists authenticated fake terminal observation",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "finish {output:?}");
    // Keep dirty in-progress issue evidence intact. The eligible-cleanup control
    // is a separately registered clean checkout at the same terminal head.
    let clean = primary.join("worktrees/cleanup-control");
    git(
        primary,
        &[
            "worktree",
            "add",
            "--detach",
            clean.to_str().unwrap(),
            &head,
        ],
    );
    let receipt_path = primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json");
    let receipt_digest = blake3::hash(&fs::read(&receipt_path).unwrap())
        .to_hex()
        .to_string();
    let mut cleanup = terminal.clone();
    cleanup["terminal_state"] = serde_json::Value::Null;
    cleanup["cleanup"] = json!({"approved_parent":primary.join("worktrees"),"repository_root":primary,"candidate_path":clean,
        "remove":false,"terminal_receipt":true,"terminal_receipt_path":receipt_path,"terminal_receipt_digest":receipt_digest});
    let (output, report) = invoke(
        corpus,
        "terminal-02",
        "clean",
        "eligible_cleanup_preview",
        &cleanup,
        None,
        primary,
    );
    let completed =
        output.status.success() && report["result"]["cleanup"]["decision"] == "removable";
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "clean registered control preview; original dirty issue checkout retained",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "clean preview {output:?}");
    cleanup["cleanup"]["remove"] = json!(true);
    cleanup["cleanup"]["preview_receipt_digest"] =
        report["result"]["cleanup"]["receipt_digest"].clone();
    let (output, report) = invoke(
        corpus,
        "terminal-03",
        "clean",
        "eligible_cleanup_remove",
        &cleanup,
        None,
        primary,
    );
    let completed =
        output.status.success() && report["performed_mutation"] == true && !clean.exists();
    corpus.classify(
        if completed { "completed" } else { "failed" },
        "native removal of the exact previewed clean fixture registration",
        completed,
        None,
    );
    corpus.write(&corpus_file("baseline.json"));
    assert!(completed, "clean remove {output:?}");
}
