use std::fs;
use std::path::{Path, PathBuf};

use csdlc_v3::adapters::{
    ChildCredentialInjector, CommandInvocation, FakeGitAdapter, FakeProcessAdapter, GitAdapter,
    ProcessAdapter, ProcessOutput, ProcessStatus, StaticCredentialResolver,
};
use csdlc_v3::application::{FoundationState, IssueProjection};
use csdlc_v3::commands::local::{
    authorize_bind, grants_operational_authority, plan_cards, prepare_local_workflow,
    validate_contract, LocalCommand, LocalPreparationRequest, PromptRegistry, WorktreeRegistration,
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

fn read_issue_index(root: &Path, issue: u64) -> serde_json::Value {
    let path = root.join(format!(".csdlc/issues/{issue}/index.json"));
    let bytes = fs::read(&path).expect("real issue index exists");
    serde_json::from_slice(&bytes).expect("real issue index is valid JSON")
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
        title: "[C-SDLC v2] Restore governed draft-to-ready publication reconciliation".into(),
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
        commands: vec![
            LocalCommand::PrepareIssue,
            LocalCommand::BindWorktree,
            LocalCommand::PlanPvf,
            LocalCommand::Doctor,
        ],
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
fn lifecycle_and_durable_storage_canary_uses_real_terminal_issue_and_recovery_provenance() {
    let root = repo_root();
    let index = read_issue_index(&root, 504);
    assert_eq!(index["phase"], "closed_out");

    let store_dir = root.join(format!(
        "csdlc-v3/target/real-issue-canary-504-store-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&store_dir);
    let mut store =
        DurableTransactionStore::create(&store_dir, StateRecord::new(LifecycleState::ClosedOut))
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
