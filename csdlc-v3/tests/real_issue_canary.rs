use std::path::{Path, PathBuf};
use std::{fs, process::Command};

use csdlc_v3::adapters::{
    ChildCredentialInjector, CommandInvocation, FakeGitAdapter, FakeProcessAdapter, GitAdapter,
    ProcessAdapter, ProcessOutput, ProcessStatus, StaticCredentialResolver,
};
use csdlc_v3::application::{FoundationState, IssueProjection};
use csdlc_v3::commands::local::{
    authorize_bind, grants_operational_authority, plan_cards, prepare_local_workflow,
    required_local_commands, validate_contract, LocalPreparationRequest, PromptRegistry,
    WorktreeRegistration,
};
use csdlc_v3::commands::remote::{
    github_adapter_receipt_payload_digest, github_readback_receipt_payload_digest,
    typed_review_receipt_payload_digest, GithubAdapterReceipt, GithubReadbackReceipt,
    RemotePublicationMode, RemoteReadbackSource, RemoteRouteReceipts, RemoteRouteRequest,
    TypedReviewReceipt,
};
use csdlc_v3::lifecycle::{
    Capability, CapabilitySet, LifecycleCommand, LifecycleState, ProjectionInvalidation,
    ReviewRecoveryProvenance, TransitionOutcome,
};
use csdlc_v3::repository::RepositoryContext;
use csdlc_v3::storage::{CommitResult, DurableTransactionStore, ProjectionWrite, StateRecord};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn git_common_dir(root: &Path) -> PathBuf {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("rev-parse")
        .arg("--git-common-dir")
        .output()
        .expect("git common dir should be discoverable");
    assert!(output.status.success(), "git common dir failed: {output:?}");
    let path = PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("git common dir should be utf8")
            .trim(),
    );
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

fn read_issue_index(root: &Path, issue: u64) -> serde_json::Value {
    let path = root.join(format!(".csdlc/issues/{issue}/index.json"));
    let bytes = fs::read(&path).expect("real issue index exists");
    serde_json::from_slice(&bytes).expect("real issue index is valid JSON")
}

fn read_card_values(root: &Path, issue: u64, card: &str) -> serde_json::Value {
    let path = root.join(format!(".csdlc/issues/{issue}/cards/{card}.values.json"));
    let bytes = fs::read(&path).expect("real card values exist");
    serde_json::from_slice(&bytes).expect("real card values are valid JSON")
}

fn real_issue_title(root: &Path, issue: u64) -> String {
    read_card_values(root, issue, "sip")["identity"]["title"]
        .as_str()
        .expect("real issue title is rendered in SIP values")
        .to_owned()
}

fn lifecycle_state_from_real_phase(phase: &str) -> LifecycleState {
    match phase {
        "closed_out" => LifecycleState::ClosedOut,
        other => panic!("real issue phase {other:?} is not supported by this canary"),
    }
}

fn prompt_registry(root: &Path) -> PromptRegistry {
    let registry_bytes =
        fs::read(root.join("docs/templates/prompts/current.json")).expect("prompt registry");
    PromptRegistry::from_current_json(&registry_bytes).expect("registry parses")
}

fn v2_entrypoints(root: &Path) -> Vec<String> {
    let mut entrypoints = fs::read_dir(root.join("csdlc-v2/src/bin"))
        .expect("v2 bin directory")
        .map(|entry| {
            let entry = entry.expect("v2 bin entry");
            let path = entry.path();
            assert_eq!(
                path.extension().and_then(|value| value.to_str()),
                Some("rs")
            );
            path.file_stem()
                .and_then(|value| value.to_str())
                .expect("v2 bin stem")
                .to_owned()
        })
        .collect::<Vec<_>>();
    entrypoints.sort();
    entrypoints
}

fn real_issue_request(
    root: &Path,
    issue: u64,
    registry: &PromptRegistry,
) -> LocalPreparationRequest {
    let index = read_issue_index(root, issue);
    LocalPreparationRequest {
        issue,
        title: real_issue_title(root, issue),
        repository: index["repository"]
            .as_str()
            .expect("real issue repository")
            .to_owned(),
        branch: index["branch"]
            .as_str()
            .expect("real issue branch")
            .to_owned(),
        worktree: index["worktree"]
            .as_str()
            .expect("real issue worktree")
            .to_owned(),
        registry_version: registry.version.clone(),
        expected_lifecycle_digest: index["digest"].as_str().map(str::to_owned),
        commands: required_local_commands().to_vec(),
    }
}

#[test]
fn foundation_and_local_commands_accept_real_issue_596_without_v3_authority() {
    let root = repo_root();
    let context = RepositoryContext::discover(&root).expect("repository context");
    let foundation = FoundationState::load(&context).expect("foundation state loads");
    assert_eq!(foundation.operational_authority(), "csdlc-v2");
    assert_eq!(foundation.issue_start_minutes_max(), 3);

    let projection = IssueProjection::load(&context, 596).expect("real issue projection loads");
    assert_eq!(projection.issue, 596);
    assert_eq!(projection.schema, "csdlc.issue.index.v1");
    assert_eq!(projection.card_count, 6);
    assert!(
        !projection.phase.trim().is_empty(),
        "real issue projection carries lifecycle phase"
    );
    assert!(projection
        .cards
        .iter()
        .any(|card| card.key == "vpp" && card.value.contains("kind=vpp")));

    let registry = prompt_registry(&root);
    let request = real_issue_request(&root, 596, &registry);
    let request_json = serde_json::to_vec(&request).expect("typed request serializes");
    let parsed =
        LocalPreparationRequest::from_json(&request_json).expect("typed real issue request parses");
    validate_contract(&parsed).expect("real issue local command contract validates");

    let cards = plan_cards(596, &parsed.registry_version, &registry).expect("cards planned");
    assert_eq!(cards.card_kinds, ["sip", "stp", "spp", "vpp", "srp", "sor"]);

    let registrations = [WorktreeRegistration {
        branch: parsed.branch.clone(),
        worktree: parsed.worktree.clone(),
        primary: false,
    }];
    let bind = authorize_bind(&parsed, &registrations).expect("real issue topology binds");
    assert_eq!(bind.issue, 596);
    assert_eq!(bind.branch, parsed.branch);
    assert_eq!(bind.worktree, parsed.worktree);

    let plan = prepare_local_workflow(&parsed, &registry, &registrations)
        .expect("real issue reaches non-authoritative local plan");
    assert_eq!(plan.issue, 596);
    assert!(plan
        .commands
        .iter()
        .all(|command| !grants_operational_authority(*command)));
}

#[test]
fn eligibility_cli_consumes_real_bound_issue_state() {
    let root = repo_root();
    let registry = prompt_registry(&root);
    let request = real_issue_request(&root, 5853, &registry);
    let dir = root.join("csdlc-v3/target/real-issue-canary-5853");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("real issue canary fixture dir");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request).expect("real request serializes"),
    )
    .expect("write real request fixture");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&[WorktreeRegistration {
            branch: request.branch.clone(),
            worktree: request.worktree.clone(),
            primary: false,
        }])
        .expect("real registration serializes"),
    )
    .expect("write real registration fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("eligibility")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(root.join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--repo-root")
        .arg(&root)
        .output()
        .expect("run eligibility canary against real bound issue");
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty(), "{output:?}");
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("eligibility emits machine JSON");
    assert_eq!(value["command"], "eligibility");
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["route_status"]["code"], "ready_to_execute");
    assert_eq!(value["route_status"]["issue_start_minutes_max"], 3);
    assert_eq!(value["route_result"]["kind"], "eligibility");
    assert_eq!(value["route_result"]["ready_to_execute"], true);
    assert_eq!(value["route_result"]["issue_start_minutes_max"], 3);
    assert_eq!(value["result"]["lifecycle_state"]["issue"], 5853);
    assert_eq!(value["result"]["lifecycle_state"]["ready_to_execute"], true);
}

#[test]
fn full_replacement_denominator_blocks_cutover_until_every_v2_entrypoint_is_replaced() {
    let root = repo_root();
    let denominator_path = root.join("docs/csdlc-v3/full-replacement-denominator.json");
    let denominator: serde_json::Value =
        serde_json::from_slice(&fs::read(&denominator_path).expect("replacement denominator"))
            .expect("replacement denominator parses");
    assert_eq!(
        denominator["schema"],
        "csdlc.v3.full_replacement_denominator.v1"
    );
    assert_eq!(denominator["authority_issue"], 505);
    assert_eq!(denominator["status"], "incomplete");
    assert_eq!(denominator["cutover_ready"], false);

    let manifest_entrypoints = denominator["required_v2_entrypoints"]
        .as_array()
        .expect("entrypoint denominator")
        .iter()
        .map(|entry| {
            entry["entrypoint"]
                .as_str()
                .expect("entrypoint name")
                .to_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(manifest_entrypoints, v2_entrypoints(&root));
    assert_eq!(manifest_entrypoints.len(), 21);

    let current_v3_commands = denominator["current_v3_commands"]
        .as_array()
        .expect("current v3 commands")
        .iter()
        .map(|value| value.as_str().expect("v3 command"))
        .collect::<Vec<_>>();
    assert_eq!(current_v3_commands, ["foundation", "local"]);
    assert!(denominator["required_v2_entrypoints"]
        .as_array()
        .expect("entrypoint denominator")
        .iter()
        .any(|entry| entry["replacement_status"] == "missing"));
    assert!(denominator["non_claims"]
        .as_array()
        .expect("non-claims")
        .iter()
        .any(|claim| claim
            .as_str()
            .expect("non-claim")
            .contains("does not make v3 operational authority")));
}

#[test]
fn lifecycle_and_durable_storage_canary_derives_terminal_state_from_real_issue_4646() {
    let root = repo_root();
    let index = read_issue_index(&root, 4646);
    let real_phase = index["phase"].as_str().expect("real issue phase");
    assert_eq!(real_phase, "closed_out");
    let real_terminal_state = lifecycle_state_from_real_phase(real_phase);

    let store_dir = root.join(format!(
        "csdlc-v3/target/real-issue-canary-4646-store-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&store_dir);
    let mut store =
        DurableTransactionStore::create(&store_dir, StateRecord::new(real_terminal_state))
            .expect("durable store creates under repo target");
    let committed = store.committed().clone();
    let transaction = store
        .begin(
            LifecycleCommand::Cleanup,
            &CapabilitySet::new([Capability::TerminalReceipt]),
            committed.generation,
            committed.digest.clone(),
            "real issue canary terminal cleanup eligibility",
        )
        .expect("cleanup eligibility begins from real closed-out issue phase");
    let result = store
        .commit(transaction, ProjectionWrite::Success)
        .expect("cleanup eligibility commits durably");
    let CommitResult::Committed(next) = result else {
        panic!("projection repair was not expected");
    };
    assert_eq!(next.state, LifecycleState::ClosedOut);
    assert!(next
        .invalidated_projections
        .contains(&ProjectionInvalidation::CleanupEligibility));

    drop(store);
    let reopened = DurableTransactionStore::open(&store_dir)
        .expect("durable store reopens after committed projection success");
    assert_eq!(reopened.committed().generation, next.generation);
    assert_eq!(reopened.committed().digest, next.digest);

    let recovery = ReviewRecoveryProvenance::new(
        "worker-6",
        "real issue canary stale-review recovery",
        index["review"]["reviewed_revision"]
            .as_str()
            .expect("real review revision"),
    )
    .expect("structured recovery provenance");
    assert!(recovery
        .audit_provenance()
        .contains("stale_review_revision=git-blake3:"));
    let decision = csdlc_v3::lifecycle::decide(
        LifecycleState::Ready,
        LifecycleCommand::Publish,
        &CapabilitySet::new([Capability::PublicationLinkage]),
    );
    assert!(matches!(
        decision.outcome,
        TransitionOutcome::Rejected {
            reason: csdlc_v3::lifecycle::RejectReason::InvalidState
        }
    ));
}

#[test]
fn v3_h3_real_issue_canary_reaches_open_pr_publication_readiness_without_v3_authority() {
    let root = repo_root();
    let index = read_issue_index(&root, 629);
    assert_eq!(index["phase"], "published");
    assert_eq!(index["publication"]["pull_request"], 641);
    assert_eq!(index["publication"]["linkage_mode"], "closing");
    assert_eq!(index["publication"]["base"], "main");

    let revision = index["branch"]
        .as_str()
        .expect("real #629 branch is recorded");
    assert_eq!(revision, "codex/629-v3-h3-github-publication-exec");
    let head = Command::new("git")
        .arg("-C")
        .arg(&root)
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("git rev-parse runs for real issue canary");
    assert!(head.status.success(), "git rev-parse failed: {head:?}");
    let head = String::from_utf8(head.stdout)
        .expect("git head is utf8")
        .trim()
        .to_owned();

    let mut request = RemoteRouteRequest {
        repository: index["repository"]
            .as_str()
            .expect("real #629 repository")
            .to_owned(),
        issue: 629,
        pull_request: Some(641),
        actor: Some("worker-6".into()),
        implementer: Some("worker-6".into()),
        reviewer: Some("reviewer-629".into()),
        review_revision: Some(head.clone()),
        expected_head_sha: Some(head.clone()),
        head_sha: Some(head),
        mode: Some(RemotePublicationMode::Closing),
        body: Some("Closes #629\n\nPart of #625".into()),
        review_present: true,
        typed_review_receipt_path: None,
        typed_review_receipt_digest: None,
        readback_source: Some(RemoteReadbackSource::Github),
        readback_receipt_path: None,
        readback_receipt_digest: None,
        adapter_receipt_path: None,
        adapter_receipt_digest: None,
        closes_issue: Some(629),
        part_of_issue: None,
        credential_names: vec!["GITHUB_TOKEN".into()],
    };
    let readback = GithubReadbackReceipt {
        schema: "csdlc.v3.github_readback_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request.expect("pull request"),
        head_sha: request.head_sha.clone().expect("head sha"),
        closes_issue: request.closes_issue,
        part_of_issue: request.part_of_issue,
        source: RemoteReadbackSource::Github,
        observed_by: "real-issue-canary-v3-github-adapter".into(),
    };
    let adapter = GithubAdapterReceipt {
        schema: "csdlc.v3.github_adapter_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request.expect("pull request"),
        head_sha: request.head_sha.clone().expect("head sha"),
        readback_receipt_digest: github_readback_receipt_payload_digest(&readback),
        credential_names: request.credential_names.clone(),
        adapter: "github".into(),
        authenticated: true,
    };
    let receipts = RemoteRouteReceipts {
        typed_review: Some(TypedReviewReceipt {
            schema: "csdlc.v3.typed_review_receipt.v1".into(),
            repository: request.repository.clone(),
            issue: request.issue,
            implementer: request.implementer.clone().expect("implementer"),
            reviewer: request.reviewer.clone().expect("reviewer"),
            reviewed_revision: request.review_revision.clone().expect("review revision"),
            expected_head_sha: request.expected_head_sha.clone().expect("expected head"),
            evidence_digest: "real-issue-canary-typed-review".into(),
        }),
        github_readback: Some(readback),
        adapter: Some(adapter),
    };
    request.typed_review_receipt_digest = receipts
        .typed_review
        .as_ref()
        .map(typed_review_receipt_payload_digest);
    request.readback_receipt_digest = receipts
        .github_readback
        .as_ref()
        .map(github_readback_receipt_payload_digest);
    request.adapter_receipt_digest = receipts
        .adapter
        .as_ref()
        .map(github_adapter_receipt_payload_digest);

    let receipt_dir = git_common_dir(&root).join("csdlc-v3/test-fixtures/629/real-issue-canary");
    let _ = fs::remove_dir_all(&receipt_dir);
    fs::create_dir_all(&receipt_dir).expect("receipt fixture dir");
    let typed_path = receipt_dir.join("typed-review-receipt.json");
    let readback_path = receipt_dir.join("github-readback-receipt.json");
    let adapter_path = receipt_dir.join("github-adapter-receipt.json");
    let request_path = receipt_dir.join("request.json");
    fs::write(
        &typed_path,
        serde_json::to_vec_pretty(receipts.typed_review.as_ref().expect("typed receipt"))
            .expect("typed receipt json"),
    )
    .expect("write typed receipt");
    fs::write(
        &readback_path,
        serde_json::to_vec_pretty(receipts.github_readback.as_ref().expect("readback receipt"))
            .expect("readback receipt json"),
    )
    .expect("write readback receipt");
    fs::write(
        &adapter_path,
        serde_json::to_vec_pretty(receipts.adapter.as_ref().expect("adapter receipt"))
            .expect("adapter receipt json"),
    )
    .expect("write adapter receipt");
    request.typed_review_receipt_path = Some(typed_path.to_string_lossy().into());
    request.readback_receipt_path = Some(readback_path.to_string_lossy().into());
    request.adapter_receipt_path = Some(adapter_path.to_string_lossy().into());
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(&request).expect("request json"),
    )
    .expect("write request");

    for route in ["publish", "pr-state", "github-pr"] {
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .current_dir(&root)
            .arg(route)
            .arg("--request")
            .arg(&request_path)
            .output()
            .unwrap_or_else(|error| panic!("run {route}: {error}"));
        assert!(output.status.success(), "{route} failed: {output:?}");
        assert!(output.stderr.is_empty(), "{route} stderr should be empty");
        let value: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("machine-readable JSON");
        assert_eq!(value["schema"], "csdlc.v3.remote_publication.v1");
        assert_eq!(value["command"], route);
        assert_eq!(value["read_only"], true);
        assert_eq!(value["operational_authority"], false);
        assert_eq!(value["cutover_issue"], 505);
        assert_eq!(value["result"]["status"], "blocked");
        assert_eq!(value["result"]["issue"], 629);
        assert_eq!(
            value["result"]["repository"],
            "agent-logic/agent-design-language"
        );
        let findings = value["result"]["findings"]
            .as_array()
            .expect("findings array");
        let expected_code = if route == "publish" {
            "production_review_receipt_not_implemented"
        } else {
            "production_github_adapter_not_implemented"
        };
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == expected_code),
            "{route} must fail closed on synthetic receipts: {findings:?}"
        );
    }
}

#[derive(Default)]
struct CapturedCredential {
    pairs: Vec<(String, String)>,
}

impl ChildCredentialInjector for CapturedCredential {
    fn inject_child_credential(&mut self, name: &str, value: &str) {
        self.pairs.push((name.to_owned(), value.to_owned()));
    }
}

#[test]
fn adapters_canary_builds_real_issue_observation_commands_without_secret_leakage() {
    let root = repo_root();
    let registry = prompt_registry(&root);
    let request = real_issue_request(&root, 596, &registry);
    let invocation = CommandInvocation::new(
        "git",
        [
            "-C".to_owned(),
            root.to_string_lossy().to_string(),
            "rev-parse".to_owned(),
            "--abbrev-ref".to_owned(),
            "HEAD".to_owned(),
        ],
    )
    .expect("structured git argv accepted");
    assert_eq!(invocation.argv().last().map(String::as_str), Some("HEAD"));
    assert!(CommandInvocation::new("bash", ["-lc", "echo no"]).is_err());
    assert!(CommandInvocation::new(
        "git",
        ["-c", "http.extraHeader=Authorization: bearer secret"]
    )
    .is_err());

    let secured = CommandInvocation::new("csdlc", ["foundation", "--repo-root", "."])
        .expect("structured csdlc argv accepted")
        .with_child_credential("ADL_CANARY_TOKEN")
        .expect("safe child credential name");
    let resolver = StaticCredentialResolver::new("ADL_CANARY_TOKEN", "not-a-real-secret");
    let mut captured = CapturedCredential::default();
    secured
        .inject_child_credential_for_process(&resolver, &mut captured)
        .expect("credential injected only through child injector");
    assert_eq!(captured.pairs.len(), 1);
    assert_eq!(captured.pairs[0].0, "ADL_CANARY_TOKEN");
    assert!(!format!("{secured:?}").contains("not-a-real-secret"));

    let mut process = FakeProcessAdapter::new(ProcessOutput {
        status: ProcessStatus::Exit(0),
        stdout: "{}".into(),
        stderr: String::new(),
        truncated: false,
    });
    let output = process.run(secured);
    assert_eq!(output.status, ProcessStatus::Exit(0));
    assert_eq!(process.invocations().len(), 1);

    let mut git = FakeGitAdapter::default();
    let observed = git.observe_branch(invocation);
    assert_eq!(observed.branch, "HEAD");
    assert!(!observed.authorizes_lifecycle);
    assert_eq!(request.issue, 596);
}
