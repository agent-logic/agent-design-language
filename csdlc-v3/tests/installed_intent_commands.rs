//! PVF: installed integration proof, deterministic local Git and synthetic transport.
//! Required #869/SIM-07 lane; fixture bootstrap is not issue execution evidence.
use csdlc_v3::storage::{
    semantic::{IssueKey, Observation, SemanticRoot},
    DurableTransactionStore,
};
use fs2::FileExt;
use serde_json::{json, Value};
use std::{
    fs,
    fs::OpenOptions,
    path::Path,
    process::{Command, Output},
};
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use intent_fixture::{git, Fixture};

macro_rules! assert_same_inventory {
    ($before:expr, $after:expr $(, $reason:expr)? $(,)?) => {{
        let before=&$before; let after=$after;
        let changed=before.keys().chain(after.keys()).filter(|path|before.get(*path)!=after.get(*path)).collect::<std::collections::BTreeSet<_>>();
        assert!(changed.is_empty(),"fixture bytes changed at {changed:?} {}",stringify!($($reason)?));
    }};
}

fn success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "installed intent failed: {output:?}"
    );
    let result: Value = serde_json::from_slice(&output.stdout).expect("machine JSON stdout");
    assert!(
        result["envelope"].is_object(),
        "missing common envelope: {result}"
    );
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SIM03_SYNTHETIC_TOKEN"));
    result
}

fn publication_reservation_inventory(
    root: &Path,
) -> std::collections::BTreeMap<std::path::PathBuf, String> {
    intent_fixture::inventory(root)
        .into_iter()
        .filter(|(path, _)| {
            let path = path.to_string_lossy();
            path.starts_with(".git/csdlc-v3/remote/intents")
                || path.starts_with(".git/csdlc-v3/semantic")
        })
        .collect()
}

fn plan() -> Value {
    json!({"schema":"csdlc.v3.intent_plan.v1", "slug":"installed-intent-fixture",
      "cards":{"sip":{},"stp":{},"spp":{"dependencies_inline":"Fixture dependencies ready","repo_inputs_inline":"Tracked fixture inputs","target_files_surfaces_inline":"installed intent commands","deliverables_inline":"Run installed lifecycle commands","validation_plan_inline":"Declared Cargo validator","acceptance_criteria_inline":"Installed command behavior is proven","notes_risks_inline":"Synthetic transport and isolated repository"},"vpp":{},"srp":{},"sor":{}},
      "validators":[{"id":"fixture-proof", "program":"cargo",
        "args":["test", "--manifest-path", "fixture-proof/Cargo.toml", "--offline"], "success_marker":"test result: ok."}],
      "publication":{"base":"main","title":"Installed intent fixture", "body":"Closes #505", "draft":true}})
}

fn prepare(fixture: &mut Fixture) {
    let primary = fixture.root.clone();
    let input = fixture.write_json("plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
}

// #1094 PVF: deterministic installed preparation/proof-boundary regressions;
// local Git/files only, required for declaration admission, not document proof.
#[test]
fn noncargo_preparation_preserves_declarations_but_proof_has_no_effects() {
    let mut fixture = Fixture::new("noncargo-preparation");
    let primary = fixture.root.clone();
    fs::write(
        primary.join("check.py"),
        "raise RuntimeError('must never execute')\n",
    )
    .unwrap();
    git(&primary, &["add", "check.py"]);
    git(&primary, &["commit", "-qm", "tracked declaration input"]);
    let mut input = plan();
    input["validators"] = json!([
        {"id":"document-check","program":"python3","args":["check.py","--self-test"],"success_marker":"pass"},
        {"id":"whitespace","program":"git","args":["diff","--check"],"success_marker":"exit status 0"},
        {"id":"independent-review","program":"manual-review","args":["release-evidence"],"success_marker":"accepted review evidence"}
    ]);
    let path = fixture.write_json("noncargo.json", &input);
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", path.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["validate", "505"]));
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    let before = publication_reservation_inventory(&primary);
    let result = fixture.run(&bound, &["proof", "505"]);
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stdout).contains("intent_validator_execution_unsupported")
    );
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    assert!(!bound.join(".csdlc/v3/issues/505/proof.json").exists());
}

#[test]
fn noncargo_invalid_declarations_fail_before_preparation() {
    let mut fixture = Fixture::new("noncargo-invalid");
    let primary = fixture.root.clone();
    fs::write(primary.join("untracked.py"), "pass\n").unwrap();
    for (program, args) in [
        ("python3", json!(["untracked.py"])),
        ("python3", json!(["../outside.py"])),
        ("python3", json!(["-c", "print('pass')"])),
        ("git", json!(["clean", "-fd"])),
        ("sh", json!(["-c", "true"])),
        ("manual-review", json!(["../unconfined"])),
    ] {
        let mut input = plan();
        input["validators"] =
            json!([{"id":"bad","program":program,"args":args,"success_marker":"pass"}]);
        let path = fixture.write_json("invalid-noncargo.json", &input);
        let before = publication_reservation_inventory(&primary);
        let result = fixture.run(
            &primary,
            &["prepare", "505", "--plan", path.to_str().unwrap()],
        );
        assert!(
            !result.status.success(),
            "unexpected admission: {program} {args}"
        );
        assert_same_inventory!(before, publication_reservation_inventory(&primary));
    }
}

#[test]
fn noncargo_validator_edit_retains_execution_refusal() {
    let mut fixture = Fixture::new("noncargo-edit");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    let changes = fixture.write_json("noncargo-changes.json", &json!({
        "schema":"csdlc.v3.intent_changes.v1",
        "validators":[{"id":"human-review","program":"manual-review","args":["quality-gate"],"success_marker":"accepted evidence"}]
    }));
    success(fixture.run(
        &bound,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    success(fixture.run(&bound, &["validate", "505"]));
    let result = fixture.run(&bound, &["proof", "505"]);
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stdout).contains("intent_validator_execution_unsupported")
    );
}

// #1094 PVF: authentic retained prepared inputs, copied into an isolated repo.
// Admission must preserve the historical source and reject stale replay/proof.
#[test]
fn noncargo_authentic_prepared_record_preserves_source_and_guards() {
    let (mut fixture, _linked, retained, original) =
        converted_authentic_fixture("1094-authentic-noncargo");
    let primary = fixture.root.clone();
    let script = primary.join(".git/installed-candidate/fake-bin/curl");
    let transport = fs::read_to_string(&script).unwrap();
    fs::write(
        &script,
        transport
            .replace("/issues/505", "/issues/511")
            .replace("\"number\":505", "\"number\":511")
            .replace(
                "Installed intent fixture",
                "[v0.92.1][OBS-A] Observatory experience design",
            ),
    )
    .unwrap();
    success(fixture.run(&primary, &["bind", "511"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/511.json")).unwrap(),
    )
    .unwrap();
    let bound = std::path::PathBuf::from(binding["worktree"].as_str().unwrap());
    let changes = fixture.write_json(
        "authentic-noncargo.json",
        &json!({
        "schema":"csdlc.v3.intent_changes.v1", "validators":[{
            "id":"review","program":"manual-review","args":["observatory-design"],
            "success_marker":"accepted independent evidence"}]}),
    );
    let emitted = success(fixture.run(
        &bound,
        &[
            "edit",
            "511",
            "--changes",
            changes.to_str().unwrap(),
            "--emit-request",
        ],
    ));
    let request = fixture.write_json("authentic-edit-request.json", &emitted["request"]);
    success(fixture.run(
        &bound,
        &["edit", "--intent-request", request.to_str().unwrap()],
    ));
    success(fixture.run(&bound, &["validate", "511"]));
    assert_same_inventory!(original, intent_fixture::inventory(&retained));
    let before = publication_reservation_inventory(&primary);
    let stale = fixture.run(
        &bound,
        &["edit", "--intent-request", request.to_str().unwrap()],
    );
    assert!(
        !stale.status.success(),
        "stale declaration edit was admitted"
    );
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    let invalid = fixture.write_json(
        "authentic-invalid.json",
        &json!({
        "schema":"csdlc.v3.intent_changes.v1", "validators":[{
            "id":"review","program":"sh","args":["-c","true"],"success_marker":"pass"}]}),
    );
    let before = publication_reservation_inventory(&primary);
    assert!(!fixture
        .run(
            &bound,
            &["edit", "511", "--changes", invalid.to_str().unwrap()]
        )
        .status
        .success());
    let refused = fixture.run(&bound, &["proof", "511"]);
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stdout).contains("intent_validator_execution_unsupported")
    );
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    assert_same_inventory!(original, intent_fixture::inventory(&retained));
}

fn rehash_native_issue(issue_root: &Path) {
    let index_path = issue_root.join("index.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index.as_object_mut().unwrap().remove("digest");
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serde_json::to_vec(&index).unwrap());
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        for suffix in ["values.json", "md"] {
            hasher.update(&fs::read(issue_root.join(format!("cards/{kind}.{suffix}"))).unwrap());
        }
    }
    let intent_plan = issue_root.join("intent-plan.json");
    if intent_plan.is_file() {
        hasher.update(b"csdlc.v3.intent_plan.v1\0");
        hasher.update(&fs::read(intent_plan).unwrap());
    }
    index["digest"] = hasher.finalize().to_hex().to_string().into();
    fs::write(index_path, serde_json::to_vec(&index).unwrap()).unwrap();
}

fn add_retained_bound_edit_completion(bound: &Path) {
    let issue_root = bound.join(".csdlc/issues/505");
    let index: Value =
        serde_json::from_slice(&fs::read(issue_root.join("index.json")).unwrap()).unwrap();
    let digest = "a".repeat(64);
    let path = bound
        .join(".csdlc/transactions/completed/505")
        .join(format!("edit-{digest}.json"));
    fs::write(
        path,
        serde_json::to_vec(&json!({
            "schema":"csdlc.v3.local_mutation_completion.v1",
            "issue":505,"route":"edit","request_digest":digest,
            "result":{"route":"edit","issue":505,"mutated":true,"phase":"bound",
                "generation":index["generation"],"digest":index["digest"],
                "next_route":"validate","findings":[]}
        }))
        .unwrap(),
    )
    .unwrap();
}

fn add_retained_pull_request_create_completion<F>(primary: &Path, bound: &Path, mutate: F)
where
    F: FnOnce(&mut Value),
{
    use csdlc_v3::commands::remote::{
        canonical_authority_selector_digest, github_mutation_operation_digest,
        github_mutation_operation_marker, GithubMutation, GithubMutationRequest,
    };

    let request = GithubMutationRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: None,
        cutover_issue: None,
        operator_approval: None,
        expected_head_sha: git(bound, &["rev-parse", "HEAD"]),
        credential_names: vec!["GITHUB_TOKEN".into()],
        recovery: None,
        mutation: GithubMutation::PullRequestCreate {
            base: "main".into(),
            head: git(bound, &["symbolic-ref", "--short", "HEAD"]),
            title: "Retained publication".into(),
            body: "Closes #505".into(),
            draft: true,
        },
    };
    let operation = github_mutation_operation_digest(&request);
    let marker = github_mutation_operation_marker(&operation);
    let selector = canonical_authority_selector_digest(primary).unwrap();
    let schema = "csdlc.v3.github_mutation_intent.v1";
    let adapter = "github-api-operational";
    let mut hasher = blake3::Hasher::new();
    for value in [schema, &operation, &marker, &selector, adapter] {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    let intent_digest = hasher.finalize().to_hex().to_string();
    let intents = primary.join(".git/csdlc-v3/remote/intents");
    let mutations = primary.join(".git/csdlc-v3/remote/mutations");
    fs::create_dir_all(&intents).unwrap();
    fs::create_dir_all(&mutations).unwrap();
    fs::write(
        intents.join(format!("{operation}.json")),
        serde_json::to_vec(&json!({
            "schema":schema,"operation_digest":operation,
            "operation_marker":marker,"authority_selector_digest":selector,
            "request":request,"adapter":adapter
        }))
        .unwrap(),
    )
    .unwrap();
    let mut receipt = json!({
        "schema":"csdlc.v3.github_mutation_receipt.v2",
        "repository":"agent-logic/agent-design-language","issue":505,
        "pull_request":638,"expected_head_sha":request.expected_head_sha,
        "operation_digest":operation,"response_digest":"fixture-response",
        "readback_digest":"fixture-readback","intent_digest":intent_digest,
        "reconciliation_digest":"fixture-reconciliation","adapter":adapter,
        "authenticated":true,"idempotent_replay":false
    });
    mutate(&mut receipt);
    fs::write(
        mutations.join(format!("{operation}.json")),
        serde_json::to_vec(&receipt).unwrap(),
    )
    .unwrap();
}

fn observation(fixture: &mut Fixture, cwd: &Path, route: &str) {
    let before = intent_fixture::inventory(&fixture.root);
    let result = success(fixture.run(cwd, &[route, "505"]));
    assert_eq!(result["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&fixture.root),
        "{route} changed fixture bytes"
    );
}

#[test]
fn issue_1029_installed_prepare_reactivates_retained_unbound_native_record() {
    let mut fixture = Fixture::new("legacy-native-preparation");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let native_issue = primary.join(".git/csdlc-v3/local/issues/505");
    let native_before = intent_fixture::inventory(&native_issue);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();

    let status = success(fixture.run(&primary, &["status", "505"]));
    assert_eq!(status["allowed_next"], json!(["prepare"]));
    assert_eq!(
        status["preparation"]["structural_state"],
        "native_record_present_semantic_preparation_required"
    );
    assert_eq!(status["preparation"]["execution_ready"], false);
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&native_issue),
        "legacy status changed retained source"
    );

    let input = fixture.write_json("legacy-plan.json", &plan());
    let prepared = success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert_eq!(prepared["status"], "completed");
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&native_issue),
        "compatibility preparation changed retained source"
    );
    observation(&mut fixture, &primary, "status");
    observation(&mut fixture, &primary, "validate");
}

#[test]
fn issue_1036_installed_prepare_adopts_exact_registered_bound_legacy_record() {
    let mut fixture = Fixture::new("bound-legacy-semantic-adoption");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    let native_issue = bound.join(".csdlc/issues/505");
    assert!(bound.is_dir(), "bound target missing: {}", bound.display());
    assert!(native_issue.join("index.json").is_file());
    assert!(!primary
        .join(".git/csdlc-v3/local/transactions/505.json")
        .exists());
    assert!(!bound.join(".csdlc/transactions/505.json").exists());
    let native_before = intent_fixture::inventory(&native_issue);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    fs::create_dir_all(primary.join(".csdlc/evidence/505")).unwrap();
    fs::write(
        primary.join(".csdlc/evidence/505/retained-proof.json"),
        b"{}\n",
    )
    .unwrap();

    let mut refreshed_plan = plan();
    refreshed_plan["slug"] = json!("refreshed-bound-legacy-plan");
    let input = fixture.write_json("bound-legacy-plan.json", &refreshed_plan);
    let prepared = success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert_eq!(prepared["status"], "completed");
    assert_eq!(prepared["phase"], "bound");
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&native_issue),
        "bound compatibility preparation changed retained source"
    );
    let replayed = success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert_eq!(replayed["status"], "completed");
    assert_same_inventory!(native_before, intent_fixture::inventory(&native_issue));
    observation(&mut fixture, &bound, "status");
    observation(&mut fixture, &bound, "validate");
}

// PVF #1092: bound legacy adoption accepts only an exact completed PR-create
// intent and its authenticated assigned-PR receipt.
#[test]
fn issue_1092_installed_prepare_authenticates_retained_pr_create_receipt() {
    let mut fixture = Fixture::new("bound-legacy-pr-create-completion");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    add_retained_pull_request_create_completion(&primary, &bound, |_| {});
    let remote_before = intent_fixture::inventory(&primary.join(".git/csdlc-v3/remote"));
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    let input = fixture.write_json("bound-pr-create-plan.json", &plan());
    let prepared = success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert_eq!(prepared["phase"], "bound");
    assert_same_inventory!(
        remote_before,
        intent_fixture::inventory(&primary.join(".git/csdlc-v3/remote")),
        "successful adoption changed retained remote records"
    );

    for (name, mutate) in [
        (
            "missing-pr",
            (|receipt: &mut Value| receipt["pull_request"] = Value::Null) as fn(&mut Value),
        ),
        ("zero-pr", |receipt: &mut Value| {
            receipt["pull_request"] = json!(0)
        }),
        ("intent-digest", |receipt: &mut Value| {
            receipt["intent_digest"] = json!("0".repeat(64))
        }),
        ("head", |receipt: &mut Value| {
            receipt["expected_head_sha"] = json!("0".repeat(40))
        }),
        ("repository", |receipt: &mut Value| {
            receipt["repository"] = json!("other/repository")
        }),
        ("issue", |receipt: &mut Value| receipt["issue"] = json!(506)),
    ] {
        let mut fixture = Fixture::new(&format!("bound-legacy-pr-create-{name}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
        add_retained_bound_edit_completion(&bound);
        add_retained_pull_request_create_completion(&primary, &bound, mutate);
        fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
        fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
        let input = fixture.write_json("bound-pr-create-plan.json", &plan());
        let remote_before = intent_fixture::inventory(&primary.join(".git/csdlc-v3/remote"));
        let denied = fixture.run(
            &bound,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(!denied.status.success(), "{name} unexpectedly admitted");
        assert!(
            String::from_utf8_lossy(&denied.stdout).contains("semantic_prepare_refused"),
            "{name}: {denied:?}"
        );
        assert_same_inventory!(
            remote_before,
            intent_fixture::inventory(&primary.join(".git/csdlc-v3/remote")),
            name
        );
    }
}

// PVF #1092: terminal reconciliation from the primary checkout adopts the
// exact head and PR identity from a completed native PR-create receipt. The
// current main checkout must not be mistaken for the historical candidate.
#[test]
fn issue_1092_installed_finish_adopts_settled_legacy_publication_identity() {
    let mut fixture = Fixture::new("legacy-finish-settled-publication-identity");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    fs::write(
        bound.join("historical-candidate.txt"),
        "historical candidate\n",
    )
    .unwrap();
    git(&bound, &["add", "historical-candidate.txt"]);
    git(&bound, &["commit", "--quiet", "-m", "historical candidate"]);
    let historical_head = git(&bound, &["rev-parse", "HEAD"]);
    assert_ne!(historical_head, git(&primary, &["rev-parse", "HEAD"]));
    add_retained_pull_request_create_completion(&primary, &bound, |_| {});
    fixture.enable_pr_transport(&bound);
    fs::write(bound.join("final-candidate.txt"), "final candidate\n").unwrap();
    git(&bound, &["add", "final-candidate.txt"]);
    git(&bound, &["commit", "--quiet", "-m", "final candidate"]);
    let final_head = git(&bound, &["rev-parse", "HEAD"]);
    assert_ne!(historical_head, final_head);

    fs::write(
        primary.join(".git/installed-candidate/remote-pr-638.json"),
        serde_json::to_vec(&json!({
            "number":638,"head":{"sha":final_head},"merged":true,"state":"closed",
            "body":"Closes #505"
        }))
        .unwrap(),
    )
    .unwrap();
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();

    let finished = success(fixture.run(&primary, &["finish", "505"]));
    assert_eq!(finished["status"], "completed");
    assert_eq!(finished["compatibility"], "legacy_merged_publication");
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["pull_request"], 638);
    assert_eq!(receipt["head_sha"], final_head);
    assert_eq!(fixture.remote_effects(), 0);

    let state_path = primary.join(".git/csdlc-v3/local/v3/issues/505/terminal.json");
    fs::remove_file(&state_path).unwrap();
    let repaired = success(fixture.run(&primary, &["finish", "505"]));
    assert_eq!(repaired["status"], "completed");
    assert_eq!(repaired["compatibility"], "legacy_merged_publication");
    assert!(state_path.is_file());
}

#[test]
fn issue_1092_installed_finish_accepts_explicit_authenticated_legacy_target() {
    let mut fixture = Fixture::new("legacy-finish-explicit-authenticated-target");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    fixture.enable_pr_transport(&bound);
    fixture.enable_external_pr_transport();
    let head = git(&bound, &["rev-parse", "HEAD"]);
    fs::write(
        primary.join(".git/installed-candidate/remote-pr-638.json"),
        serde_json::to_vec(&json!({
            "number":638,"head":{"sha":head},"merged":true,"state":"closed",
            "body":"Closes agent-logic/agent-design-language#505"
        }))
        .unwrap(),
    )
    .unwrap();
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();

    let finished = success(fixture.run(
        &primary,
        &[
            "finish",
            "505",
            "--pull-request",
            "638",
            "--publication-repository",
            "agent-logic/codefriend.ai",
        ],
    ));
    assert_eq!(finished["status"], "completed");
    assert_eq!(finished["compatibility"], "legacy_merged_publication");
    let receipt_path = primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json");
    let receipt: Value = serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    assert_eq!(
        receipt["publication_repository"],
        "agent-logic/codefriend.ai"
    );
    let state_path = primary.join(".git/csdlc-v3/local/v3/issues/505/terminal.json");
    fs::remove_file(&state_path).unwrap();
    let repaired = success(fixture.run(&primary, &["finish", "505"]));
    assert_eq!(repaired["status"], "completed");
    assert_eq!(repaired["compatibility"], "legacy_merged_publication");
    assert!(state_path.is_file());
    let repaired_receipt: Value = serde_json::from_slice(&fs::read(receipt_path).unwrap()).unwrap();
    assert_eq!(
        repaired_receipt["publication_repository"],
        "agent-logic/codefriend.ai"
    );
    assert_eq!(fixture.remote_effects(), 0);
}

// PVF #1092: an external closing PR cannot suppress an already retained native
// publication identity for a current semantic issue.
#[test]
fn issue_1092_current_semantic_external_publication_rejects_native_conflict() {
    let (mut fixture, linked) = reviewed_fixture("current-external-publication");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    fixture.enable_external_pr_transport();
    let external_head = "1".repeat(40);
    let external_pr_path = primary.join(".git/installed-candidate/remote-pr-638.json");
    fs::write(
        &external_pr_path,
        serde_json::to_vec(&json!({
            "number":638,"head":{"sha":external_head},"merged":true,"state":"closed",
            "body":"Closes agent-logic/agent-design-language#505"
        }))
        .unwrap(),
    )
    .unwrap();
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(
        primary.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&issue).unwrap(),
    )
    .unwrap();

    let external_args = [
        "finish",
        "505",
        "--pull-request",
        "638",
        "--publication-repository",
        "agent-logic/codefriend.ai",
    ];
    let conflict = fixture.run(&linked, &external_args);
    assert!(!conflict.status.success());
    assert!(String::from_utf8_lossy(&conflict.stdout)
        .contains("intent_finish_publication_repository_conflict"));
    assert!(!primary
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .exists());
    let semantic_root =
        SemanticRoot::from_git_common(primary.join(".git"), "agent-logic/agent-design-language")
            .unwrap();
    let key = IssueKey::new("agent-logic/agent-design-language", 505).unwrap();
    let snapshot = match DurableTransactionStore::observe_issue(&semantic_root, &key).unwrap() {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            snapshot
        }
        other => panic!("{other:?}"),
    };
    assert_eq!(
        snapshot.phase(),
        csdlc_v3::lifecycle::LifecycleState::Published
    );
    assert!(linked.exists());
}

#[test]
fn issue_1036_fast_forward_candidate_head_admits_edit_and_proof() {
    let mut fixture = Fixture::new("bound-legacy-fast-forward-head");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    let plan_path = fixture.write_json("fast-forward-plan.json", &plan());
    success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", plan_path.to_str().unwrap()],
    ));
    success(fixture.run(&bound, &["rebuild", "505"]));
    success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", plan_path.to_str().unwrap()],
    ));

    fs::write(bound.join("implementation.txt"), "candidate B\n").unwrap();
    git(&bound, &["add", "implementation.txt"]);
    git(&bound, &["commit", "--quiet", "-m", "candidate B"]);
    let head_b = git(&bound, &["rev-parse", "HEAD"]);
    let status = success(fixture.run(&bound, &["status", "505"]));
    assert!(status["result"]["findings"]
        .as_array()
        .is_some_and(|findings| findings
            .iter()
            .any(|finding| finding["code"] == "binding_live")));
    assert!(
        status["allowed_next"]
            .as_array()
            .is_some_and(|routes| routes.iter().any(|route| route == "edit")),
        "{status}"
    );

    for (name, approved, revision, expected) in [
        (
            "refused",
            false,
            head_b.as_str(),
            "intent_amendment_policy_rejected",
        ),
        (
            "wrong-revision",
            true,
            "1111111111111111111111111111111111111111",
            "intent_amendment_revision_mismatch",
        ),
    ] {
        let changes = fixture.write_json(
            &format!("{name}-implementation.json"),
            &json!({"schema":"csdlc.v3.intent_changes.v1",
                "amendment":{"class":"implementation","transition_approved":approved,
                    "implementation_revision":revision,"new_commit":true},
                "cards":{"sor":{"summary":"candidate B implemented"}}}),
        );
        let before = intent_fixture::inventory(&fixture.root);
        let rejected = fixture.run(
            &bound,
            &["edit", "505", "--changes", changes.to_str().unwrap()],
        );
        assert!(
            !rejected.status.success(),
            "{name} edit unexpectedly admitted"
        );
        assert!(
            String::from_utf8_lossy(&rejected.stdout).contains(expected),
            "unexpected {name} rejection: {rejected:?}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&fixture.root), name);
    }

    let changes = fixture.write_json(
        "accepted-implementation.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1",
            "amendment":{"class":"implementation","transition_approved":true,
                "implementation_revision":head_b,"new_commit":true},
            "cards":{"sor":{"summary":"candidate B implemented"}}}),
    );
    let edited = success(fixture.run(
        &bound,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(edited["status"], "completed");
    let semantic: Value =
        serde_json::from_slice(&fs::read(bound.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    assert_eq!(semantic["inputs"]["binding"]["head"], head_b);

    fs::write(bound.join("implementation.txt"), "candidate C\n").unwrap();
    git(&bound, &["add", "implementation.txt"]);
    git(&bound, &["commit", "--quiet", "-m", "candidate C"]);
    let head_c = git(&bound, &["rev-parse", "HEAD"]);
    let proof = success(fixture.run(&bound, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    let semantic: Value =
        serde_json::from_slice(&fs::read(bound.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    assert_eq!(semantic["inputs"]["binding"]["head"], head_c);

    git(&bound, &["reset", "--hard", &head_b]);
    fs::write(bound.join("unrelated.txt"), "rewritten candidate\n").unwrap();
    git(&bound, &["add", "unrelated.txt"]);
    git(&bound, &["commit", "--quiet", "-m", "unrelated candidate"]);
    let semantic_root = primary.join(".git/csdlc-v3/semantic/issues/505");
    let semantic_before = intent_fixture::inventory(&semantic_root);
    let projection_root = bound.join(".csdlc/v3/issues/505");
    let projection_before = intent_fixture::inventory(&projection_root);
    let rejected = fixture.run(&bound, &["proof", "505"]);
    assert!(!rejected.status.success(), "rewritten HEAD admitted proof");
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("intent_semantic_binding_stale"));
    assert_same_inventory!(semantic_before, intent_fixture::inventory(&semantic_root));
    assert_same_inventory!(
        projection_before,
        intent_fixture::inventory(&projection_root)
    );
}

#[test]
fn issue_1036_bound_legacy_adoption_rejects_mismatched_primary_binding() {
    let mut fixture = Fixture::new("bound-legacy-wrong-owner");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    let primary_binding = primary.join(".git/csdlc-v3/local/bindings/505.json");
    let mut binding: Value = serde_json::from_slice(&fs::read(&primary_binding).unwrap()).unwrap();
    binding["branch"] = "codex/505-foreign-owner".into();
    fs::write(&primary_binding, serde_json::to_vec(&binding).unwrap()).unwrap();
    let input = fixture.write_json("wrong-owner-plan.json", &plan());
    let before = intent_fixture::inventory(&fixture.root);
    let rejected = fixture.run(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    );
    assert!(!rejected.status.success());
    assert!(
        String::from_utf8_lossy(&rejected.stdout).contains("intent_bound_legacy_binding_mismatch")
    );
    assert!(!primary.join(".git/csdlc-v3/semantic/issues/505").exists());
    assert_same_inventory!(before, intent_fixture::inventory(&fixture.root));
}

#[test]
fn issue_1036_interrupted_bound_legacy_adoption_replays_to_completion() {
    let mut fixture = Fixture::new("bound-legacy-interrupted-adoption");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    let native_issue = bound.join(".csdlc/issues/505");
    let native_before = intent_fixture::inventory(&native_issue);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    let input = fixture.write_json("interrupted-bound-plan.json", &plan());
    let interrupted = fixture.run_with_env(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_prepare_after_activation",
        )],
    );
    assert_eq!(interrupted.status.code(), Some(91));
    assert!(primary.join(".git/csdlc-v3/semantic/issues/505").is_dir());
    let lock_path = primary.join(".git/csdlc-v3/local/locks/505.lock");
    let old_writer_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .unwrap();
    let old_writer_admitted = old_writer_lock.try_lock_exclusive().is_ok();
    if old_writer_admitted {
        fs::write(native_issue.join("index.json"), b"old writer mutation\n").unwrap();
    }
    assert!(!old_writer_admitted, "old writer entered after activation");
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&native_issue),
        "old writer changed retained bytes while adoption awaited recovery"
    );
    let recovered = success(fixture.run(
        &bound,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert_eq!(recovered["status"], "completed");
    let released_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(lock_path)
        .unwrap();
    assert!(released_lock.try_lock_exclusive().is_ok());
    assert_same_inventory!(native_before, intent_fixture::inventory(&native_issue));
    observation(&mut fixture, &bound, "status");
}

#[test]
fn issue_1036_concurrent_rejected_adoption_cannot_release_active_fence() {
    let mut fixture = Fixture::new("bound-legacy-concurrent-adoption");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
    add_retained_bound_edit_completion(&bound);
    let native = bound.join(".csdlc/issues/505");
    let before = intent_fixture::inventory(&native);
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
    let valid = fixture.write_json("concurrent-valid.json", &plan());
    let mut changed = plan();
    changed["cards"]["sip"] = json!({"goal":"different retained truth"});
    let invalid = fixture.write_json("concurrent-invalid.json", &changed);
    let barrier = primary.join(".git/installed-candidate/adoption-continue");
    let mut first = fixture
        .command(
            &bound,
            &["prepare", "505", "--plan", valid.to_str().unwrap()],
        )
        .env("CSDLC_V3_TEST_ADOPTION_BARRIER", &barrier)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while !barrier.with_extension("ready").exists() {
        if first.try_wait().unwrap().is_some() || std::time::Instant::now() >= deadline {
            let _ = first.kill();
            panic!(
                "first adoption did not reach the fenced barrier: {:?}",
                first.wait_with_output()
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let second = fixture.run(
        &bound,
        &["prepare", "505", "--plan", invalid.to_str().unwrap()],
    );
    let native_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .open(primary.join(".git/csdlc-v3/local/locks/505.lock"))
        .unwrap();
    let old_writer_entered = native_lock.try_lock_exclusive().is_ok();
    if old_writer_entered {
        FileExt::unlock(&native_lock).unwrap();
    }
    fs::write(&barrier, b"continue\n").unwrap();
    let accepted = first.wait_with_output().unwrap();
    assert!(
        !old_writer_entered,
        "rejected concurrent caller released the accepted caller's fence"
    );
    assert!(!second.status.success());
    assert!(String::from_utf8_lossy(&second.stdout)
        .contains("bound legacy adoption is already in progress"));
    assert_eq!(success(accepted)["status"], "completed");
    assert!(native_lock.try_lock_exclusive().is_ok());
    FileExt::unlock(&native_lock).unwrap();
    assert_same_inventory!(before, intent_fixture::inventory(&native));
    observation(&mut fixture, &bound, "status");
}

#[test]
fn issue_1036_bound_legacy_adoption_rejects_changed_plan_and_damaged_completion() {
    for case in [
        "changed_plan",
        "damaged_completion",
        "missing_completion_chain",
        "empty_completion_chain",
        "conflicting_non_tip_generation",
    ] {
        let mut fixture = Fixture::new(case);
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let bound = primary.join("worktrees/adl-issue-505-installed-intent-fixture");
        add_retained_bound_edit_completion(&bound);
        fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
        fs::remove_dir_all(bound.join(".csdlc/v3/issues/505")).unwrap();
        let mut candidate = plan();
        let completed = bound.join(".csdlc/transactions/completed/505");
        match case {
            "changed_plan" => {
                candidate["cards"]["sip"] = json!({"goal":"different retained truth"});
            }
            "damaged_completion" => {
                let receipt = fs::read_dir(&completed)
                    .unwrap()
                    .map(Result::unwrap)
                    .map(|entry| entry.path())
                    .max()
                    .expect("bound completion receipt");
                fs::write(receipt, b"{}\n").unwrap();
            }
            "missing_completion_chain" => fs::remove_dir_all(&completed).unwrap(),
            "empty_completion_chain" => {
                fs::remove_dir_all(&completed).unwrap();
                fs::create_dir_all(&completed).unwrap();
            }
            "conflicting_non_tip_generation" => {
                for (request_digest, result_digest) in [("b", "d"), ("c", "e")] {
                    let request_digest = request_digest.repeat(64);
                    fs::write(
                        completed.join(format!("edit-{request_digest}.json")),
                        serde_json::to_vec(&json!({
                            "schema":"csdlc.v3.local_mutation_completion.v1",
                            "issue":505,"route":"edit","request_digest":request_digest,
                            "result":{"route":"edit","issue":505,"mutated":true,
                                "phase":"bound","generation":1,
                                "digest":result_digest.repeat(64),"next_route":"validate",
                                "findings":[]}
                        }))
                        .unwrap(),
                    )
                    .unwrap();
                }
            }
            _ => unreachable!(),
        }
        let input = fixture.write_json(&format!("{case}-plan.json"), &candidate);
        // The persistent empty client-lock inode is coordination state, not
        // retained issue truth; it must survive rejected-attempt cleanup.
        fs::write(
            primary.join(".git/csdlc-v3/local/locks/505.adoption.lock"),
            b"",
        )
        .unwrap();
        let before = intent_fixture::inventory(&fixture.root);
        let rejected = fixture.run(
            &bound,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(!rejected.status.success(), "{case} unexpectedly adopted");
        let output = String::from_utf8_lossy(&rejected.stdout);
        assert!(
            output.contains(if case == "changed_plan" {
                "bound legacy plan differs from retained sip card truth"
            } else {
                "semantic_prepare_refused"
            }),
            "unexpected {case} rejection: {output}"
        );
        assert!(!primary.join(".git/csdlc-v3/semantic/issues/505").exists());
        assert_same_inventory!(before, intent_fixture::inventory(&fixture.root), case);
    }
}

#[test]
fn issue_1029_installed_prepare_rejects_digest_consistent_invalid_legacy_cards_without_semantic_state(
) {
    let mut fixture = Fixture::new("legacy-native-invalid-cards");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let native_issue = primary.join(".git/csdlc-v3/local/issues/505");
    fs::remove_dir_all(primary.join(".git/csdlc-v3/semantic/issues/505")).unwrap();
    fs::write(native_issue.join("cards/sip.values.json"), b"[]\n").unwrap();
    rehash_native_issue(&native_issue);
    let native_before = intent_fixture::inventory(&native_issue);
    let input = fixture.write_json("invalid-legacy-plan.json", &plan());
    let rejected = fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    );
    assert!(
        !rejected.status.success(),
        "invalid legacy cards were accepted"
    );
    assert!(!primary.join(".git/csdlc-v3/semantic/issues/505").exists());
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&native_issue),
        "invalid legacy preparation changed retained source"
    );
}

#[test]
fn installed_prepare_and_bind_from_unrelated_linked_checkout_resolve_primary_state() {
    let mut fixture = Fixture::new("linked-prepared-start");
    let primary = fixture.root.clone();
    let observer = primary
        .parent()
        .expect("fixture root has temp parent")
        .join("observer-worktree");
    if observer.exists() {
        fs::remove_dir_all(&observer).unwrap();
    }
    git(
        &primary,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "fixture-observer",
            observer.to_str().unwrap(),
        ],
    );
    let input = fixture.write_json("plan.json", &plan());
    success(fixture.run(
        &observer,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    assert!(primary
        .join(".git/csdlc-v3/local/issues/505/index.json")
        .is_file());
    assert!(!observer.join(".csdlc/issues/505/index.json").exists());
    success(fixture.run(&observer, &["bind", "505"]));
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/505.json")).unwrap(),
    )
    .unwrap();
    assert!(Path::new(binding["worktree"].as_str().unwrap())
        .join(".git")
        .is_file());
    observation(&mut fixture, &observer, "status");
    observation(&mut fixture, &observer, "validate");
}

#[test]
fn installed_prepare_bind_edit_and_observations_use_canonical_context() {
    let mut fixture = Fixture::new("ordinary-local");
    let primary = fixture.root.clone();
    let input = fixture.write_json("plan.json", &plan());
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &primary,
            &[
                "prepare",
                "505",
                "--plan",
                input.to_str().unwrap(),
                "--preview",
                "plan"
            ]
        )
        .status
        .success());
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "forbidden prepare preview mutated state"
    );
    prepare(&mut fixture);
    observation(&mut fixture, &primary, "status");
    observation(&mut fixture, &primary, "validate");
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    assert!(linked.join(".git").is_file());
    for cwd in [&primary, &linked] {
        observation(&mut fixture, cwd, "status");
        observation(&mut fixture, cwd, "validate");
    }
    let changes = fixture.write_json("changes.json", &json!({"schema":"csdlc.v3.intent_changes.v1", "amendment":{"class":"scope_acceptance","transition_approved":true}, "cards":{"sip":{"title":"Edited through ordinary intent"}}}));
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &linked,
            &[
                "edit",
                "505",
                "--changes",
                changes.to_str().unwrap(),
                "--preview",
                "plan"
            ]
        )
        .status
        .success());
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "forbidden edit preview mutated state"
    );
    let edited = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(edited["envelope"]["issue"]["number"], 505);
    assert_eq!(
        edited["envelope"]["issue_version"]["before"]["basis"],
        "observed_intent_snapshot"
    );
    assert_eq!(
        edited["envelope"]["issue_version"]["before"]["digest"],
        edited["intent_snapshot"]["version"]["digest"]
    );
    assert!(edited["envelope"]["issue_version"]["before"]["digest"]
        .as_str()
        .is_some_and(|digest| digest.len() == 64));
    assert_eq!(
        edited["envelope"]["issue_version"]["after"]["digest"],
        edited["result"]["digest"]
    );
    assert_ne!(
        edited["envelope"]["issue_version"]["after"]["digest"],
        edited["envelope"]["issue_version"]["before"]["digest"]
    );
    success(fixture.run(
        &primary,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert!(
        fs::read_to_string(linked.join(".csdlc/v3/issues/505/cards/sip.md"))
            .unwrap()
            .contains("Edited through ordinary intent")
    );
    observation(&mut fixture, &linked, "validate");
}

#[test]
fn installed_rebuild_diagnoses_and_repairs_six_active_registry_projections() {
    let pvf: Value =
        serde_json::from_slice(include_bytes!("fixtures/projection_rebuild_pvf.json")).unwrap();
    assert_eq!(pvf["issue"], 871);
    assert_eq!(pvf["required_states"].as_array().unwrap().len(), 4);
    assert_eq!(
        pvf["required_amendment_classes"].as_array().unwrap().len(),
        7
    );

    let mut fixture = Fixture::new("projection-rebuild");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::remove_dir_all(linked.join(".csdlc/v3/issues/505/cards")).unwrap();
    let before = intent_fixture::inventory(&fixture.root);
    for route in ["status", "validate"] {
        let result = success(fixture.run(&linked, &[route, "505"]));
        assert_eq!(result["projection"]["observation"]["status"], "missing");
        assert_eq!(
            result["projection"]["observation"]["paths"]
                .as_array()
                .unwrap()
                .len(),
            13
        );
        assert_same_inventory!(before, intent_fixture::inventory(&fixture.root));
    }

    let emitted = success(fixture.run(&linked, &["rebuild", "505", "--emit-request"]));
    assert!(
        emitted["request"]["snapshot"]["semantic_version"]["generation"]
            .as_u64()
            .is_some_and(|generation| generation > 0)
    );
    assert!(emitted["request"]["snapshot"]["semantic_version"]["digest"]
        .as_str()
        .is_some_and(|digest| digest.starts_with("semantic-state-v1:")));
    let semantic_state = primary.join(".git/csdlc-v3/semantic/issues/505/current.json");
    let semantic_before: Value =
        serde_json::from_slice(&fs::read(&semantic_state).unwrap()).unwrap();
    let rebuilt = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(rebuilt["status"], "completed");
    assert_eq!(rebuilt["projection"]["before"]["status"], "missing");
    assert_eq!(rebuilt["projection"]["after"]["status"], "healthy");
    let semantic_after: Value =
        serde_json::from_slice(&fs::read(&semantic_state).unwrap()).unwrap();
    for field in [
        "phase",
        "inputs",
        "input_version",
        "invalidations",
        "pending",
        "completed",
    ] {
        assert_eq!(
            semantic_after["payload"][field], semantic_before["payload"][field],
            "rebuild changed semantic fact {field}"
        );
    }
    let card_root = linked.join(".csdlc/v3/issues/505/cards");
    let legacy_root = linked.join(".csdlc/issues/505/cards");
    assert!(card_root.join("manifest.json").is_file());
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        for suffix in ["md", "values.json"] {
            let name = format!("{kind}.{suffix}");
            let projected = fs::read(card_root.join(&name)).unwrap();
            let legacy = fs::read(legacy_root.join(&name)).unwrap();
            if suffix == "values.json" {
                let values: Value = serde_json::from_slice(&projected).unwrap();
                assert_eq!(values["schema"], "csdlc.v3.card_values.v1");
                assert_eq!(values["issue"], 505);
                assert_eq!(values["card"], kind);
                if kind == "spp" {
                    assert_eq!(values["dependencies_inline"], "Fixture dependencies ready");
                    assert_eq!(
                        values["acceptance_criteria_inline"],
                        "Installed command behavior is proven"
                    );
                }
            } else {
                let rendered = std::str::from_utf8(&projected).unwrap();
                assert!(rendered.contains("Canonical Template Source"));
                assert!(!legacy.is_empty());
            }
        }
    }
    success(fixture.run(&linked, &["validate", "505"]));
    let mut wrong_checkout = emitted.clone();
    wrong_checkout["request"]["snapshot"]["checkout"]["root"] =
        json!(primary.join("wrong-checkout"));
    let wrong_path = fixture.write_json("wrong-rebuild.json", &wrong_checkout);
    let wrong = fixture.run(
        &linked,
        &["rebuild", "--intent-request", wrong_path.to_str().unwrap()],
    );
    assert!(!wrong.status.success());
    assert!(String::from_utf8_lossy(&wrong.stdout).contains("intent_snapshot_stale"));
    let first_projection = intent_fixture::inventory(&card_root);
    let semantic_version = rebuilt["semantic_version"].clone();
    let replay = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_eq!(replay["semantic_version"], semantic_version);
    assert_eq!(intent_fixture::inventory(&card_root), first_projection);

    fs::write(card_root.join("stp.md"), "format drift\n").unwrap();
    let altered = intent_fixture::inventory(&fixture.root);
    for route in ["status", "validate"] {
        let result = success(fixture.run(&linked, &[route, "505"]));
        assert_eq!(result["projection"]["observation"]["status"], "altered");
        assert_same_inventory!(altered, intent_fixture::inventory(&fixture.root));
    }
    success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(intent_fixture::inventory(&card_root), first_projection);

    let old_manifest = fs::read(card_root.join("manifest.json")).unwrap();
    let old_manifest_value: Value = serde_json::from_slice(&old_manifest).unwrap();
    let old_suffix = old_manifest_value["projection_digest"]
        .as_str()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    let stale_path = fixture.write_json("stale-rebuild.json", &emitted);
    let changes = fixture.write_json(
        "projection-change.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","amendment":{"class":"scope_acceptance","transition_approved":true},"cards":{"sip":{"title":"New semantic title"}}}),
    );
    let interrupted_edit = fixture.run_with_env(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_local_amendment_after_commit",
        )],
    );
    assert_eq!(interrupted_edit.status.code(), Some(91));
    fs::write(
        card_root.join(format!(".projection-{old_suffix}.pending")),
        &old_manifest,
    )
    .unwrap();
    fs::write(
        card_root.join(format!(".stp.md-{old_suffix}.next")),
        fs::read(card_root.join("stp.md")).unwrap(),
    )
    .unwrap();
    let stale = fixture.run(
        &linked,
        &["rebuild", "--intent-request", stale_path.to_str().unwrap()],
    );
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stdout).contains("intent_snapshot_stale"));
    let interrupted = intent_fixture::inventory(&fixture.root);
    for route in ["status", "validate"] {
        let result = success(fixture.run(&linked, &[route, "505"]));
        assert_eq!(result["projection"]["observation"]["status"], "interrupted");
        assert_same_inventory!(interrupted, intent_fixture::inventory(&fixture.root));
    }
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(preview["action"], "repair_semantic_projection");
    let token = preview["preview_digest"].as_str().unwrap();
    let interrupted_recovery = fixture.run_with_env(
        &linked,
        &["recover", "505", "--execute", "--preview", token],
        &[("CSDLC_TEST_INTERRUPT_AFTER_STALE_PROJECTION_DATA_SYNC", "1")],
    );
    assert!(!interrupted_recovery.status.success());
    assert!(
        String::from_utf8_lossy(&interrupted_recovery.stdout)
            .contains("injected interruption after stale projection data sync"),
        "{interrupted_recovery:?}"
    );
    assert!(card_root
        .join(format!(".projection-{old_suffix}.pending"))
        .exists());
    assert!(!card_root
        .join(format!(".stp.md-{old_suffix}.next"))
        .exists());
    let retained_after_interrupt = intent_fixture::inventory(&fixture.root);
    let status_after_interrupt = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(
        status_after_interrupt["projection"]["observation"]["status"],
        "interrupted"
    );
    assert_same_inventory!(
        retained_after_interrupt,
        intent_fixture::inventory(&fixture.root)
    );
    let fresh_preview = success(fixture.run(&linked, &["recover", "505"]));
    let fresh_token = fresh_preview["preview_digest"].as_str().unwrap();
    assert_ne!(fresh_token, token);
    let recovered = success(fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", fresh_token],
    ));
    assert_eq!(recovered["action"], "repaired_semantic_projection");
    assert!(!card_root
        .join(format!(".projection-{old_suffix}.pending"))
        .exists());
    assert!(!card_root
        .join(format!(".stp.md-{old_suffix}.next"))
        .exists());

    let manifest = fs::read(card_root.join("manifest.json")).unwrap();
    let manifest_value: Value = serde_json::from_slice(&manifest).unwrap();
    let suffix = manifest_value["projection_digest"]
        .as_str()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    fs::write(
        card_root.join(format!(".projection-{suffix}.pending")),
        manifest,
    )
    .unwrap();
    fs::remove_file(card_root.join("sor.md")).unwrap();
    let interrupted = intent_fixture::inventory(&fixture.root);
    for route in ["status", "validate"] {
        let result = success(fixture.run(&linked, &[route, "505"]));
        assert_eq!(result["projection"]["observation"]["status"], "interrupted");
        assert_same_inventory!(interrupted, intent_fixture::inventory(&fixture.root));
    }
    let recovered = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(recovered["projection"]["after"]["status"], "healthy");
    let healthy = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(healthy["projection"]["observation"]["status"], "healthy");

    fs::write(&semantic_state, b"{\"corrupted\":true}\n").unwrap();
    let corrupt = intent_fixture::inventory(&fixture.root);
    let refused = fixture.run(&linked, &["rebuild", "505"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stdout).contains("intent_semantic_"));
    assert_same_inventory!(corrupt, intent_fixture::inventory(&fixture.root));
}

#[test]
fn installed_rebuild_repairs_projection_before_exact_binding_head_refresh() {
    let mut fixture = Fixture::new("projection-rebuild-before-rebind");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["rebuild", "505"]));

    let manifest = linked.join(".csdlc/v3/issues/505/cards/manifest.json");
    fs::write(&manifest, b"{}\n").unwrap();
    git(&linked, &["add", ".csdlc/v3/issues/505"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Retain interrupted projection bytes",
        ],
    );

    let blocked = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(blocked["projection"]["observation"]["status"], "altered");
    assert_eq!(blocked["projection"]["repair_command"], "rebuild");
    assert_ne!(blocked["allowed_next"], json!(["rebuild"]));

    let rebuilt = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(rebuilt["status"], "completed");
    assert_eq!(rebuilt["projection"]["after"]["status"], "healthy");
    git(&linked, &["add", ".csdlc/v3/issues/505"]);
    git(
        &linked,
        &["commit", "--quiet", "-m", "Repair retained projection"],
    );

    let rebound = success(fixture.run(&linked, &["bind", "505"]));
    assert_eq!(rebound["status"], "completed");
    assert_eq!(rebound["binding_refreshed"], true);
    let rebuilt_after_bind = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(
        rebuilt_after_bind["projection"]["after"]["status"],
        "healthy"
    );
}

#[test]
fn installed_intent_rejects_old_and_unsupported_content_without_preparation_effects() {
    for (label, input) in [
        ("old-schema", json!({"schema":"csdlc.v3.intent_plan.v0"})),
        ("unknown-fields", {
            let mut input = plan();
            input["authority_digest"] = json!("forged");
            input
        }),
    ] {
        let mut fixture = Fixture::new(label);
        let primary = fixture.root.clone();
        let path = fixture.write_json("plan.json", &input);
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(
            &primary,
            &["prepare", "505", "--plan", path.to_str().unwrap()],
        );
        assert!(
            !output.status.success(),
            "unsupported input executed: {output:?}"
        );
        let result: Value = serde_json::from_slice(&output.stdout).expect("typed JSON rejection");
        assert_ne!(result["envelope"]["status"], "completed");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        assert!(!primary
            .join(".git/csdlc-v3/local/issues/505/index.json")
            .exists());
    }
}

#[test]
fn installed_intent_rejects_foreign_repository_and_altered_authority_without_fallback() {
    for label in ["foreign-repository", "altered-authority"] {
        let mut fixture = Fixture::new(label);
        let primary = fixture.root.clone();
        if label == "foreign-repository" {
            prepare(&mut fixture);
            git(
                &primary,
                &[
                    "remote",
                    "set-url",
                    "origin",
                    "https://github.com/other/repository.git",
                ],
            );
        } else {
            let selector = primary.join("csdlc-v3/operator/authority-selector.json");
            let mut value: Value = serde_json::from_slice(&fs::read(&selector).unwrap()).unwrap();
            value["receipt_digest"] = json!("0".repeat(64));
            fs::write(selector, serde_json::to_vec(&value).unwrap()).unwrap();
        }
        let input = fixture.write_json("plan.json", &plan());
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(
            &primary,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(
            !output.status.success(),
            "identity guard allowed execution: {output:?}"
        );
        let value: Value = serde_json::from_slice(&output.stdout).expect("machine-readable denial");
        assert!(
            value["envelope"]["findings"]
                .as_array()
                .is_some_and(|findings| !findings.is_empty()),
            "missing guard finding: {value}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        if label == "foreign-repository" {
            assert_eq!(
                value["envelope"]["reason_code"],
                "intent_issue_repository_mismatch"
            );
        } else {
            assert!(!primary
                .join(".git/csdlc-v3/local/issues/505/index.json")
                .exists());
        }
    }
}

#[test]
fn installed_unknown_intent_never_reports_success_or_creates_state() {
    let mut fixture = Fixture::new("unknown-intent");
    let primary = fixture.root.clone();
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&primary, &["unknown-intent", "505"]);
    assert!(!output.status.success());
    let value: Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable unknown command");
    assert_ne!(value["envelope"]["status"], "completed");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

fn linked_worktree(primary: &Path) -> std::path::PathBuf {
    git(primary, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(std::path::PathBuf::from)
        .find(|path| path != primary)
        .expect("registered linked worktree")
}

#[test]
fn installed_advanced_request_uses_same_writer_and_rejects_stale_snapshot() {
    let mut fixture = Fixture::new("advanced-snapshot");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let changes=fixture.write_json("changes.json",&json!({"schema":"csdlc.v3.intent_changes.v1","amendment":{"class":"scope_acceptance","transition_approved":true},"cards":{"sip":{"title":"Advanced intent execution"}}}));
    let before = intent_fixture::inventory(&primary);
    let emitted = success(fixture.run(
        &linked,
        &[
            "edit",
            "505",
            "--changes",
            changes.to_str().unwrap(),
            "--emit-request",
        ],
    ));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "request generation mutated issue state"
    );
    assert_eq!(emitted["request"]["schema"], "csdlc.v3.intent_request.v1");
    let mut foreign = emitted["request"].clone();
    foreign["snapshot"]["platform"] = json!("synthetic-foreign-platform");
    let foreign_path = fixture.write_json("foreign-platform.json", &foreign);
    let before = intent_fixture::inventory(&primary);
    let denied = fixture.run(
        &linked,
        &["edit", "--intent-request", foreign_path.to_str().unwrap()],
    );
    assert!(
        !denied.status.success(),
        "foreign-platform mutation snapshot admitted"
    );
    let denial: Value = serde_json::from_slice(&denied.stdout).unwrap();
    assert_eq!(
        denial["envelope"]["reason_code"],
        "intent_platform_not_admitted"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let request = fixture.write_json("advanced.json", &emitted["request"]);
    success(fixture.run(
        &linked,
        &["edit", "--intent-request", request.to_str().unwrap()],
    ));
    assert!(
        fs::read_to_string(linked.join(".csdlc/v3/issues/505/cards/sip.md"))
            .unwrap()
            .contains("Advanced intent execution")
    );
    let before = intent_fixture::inventory(&primary);
    let stale = fixture.run(
        &linked,
        &["edit", "--intent-request", request.to_str().unwrap()],
    );
    assert!(
        !stale.status.success(),
        "stale serialized request silently refreshed: {stale:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_recovery_requires_fresh_preview_of_actual_interrupted_transaction() {
    let mut fixture = Fixture::new("recovery-preview");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let changes=fixture.write_json("changes.json",&json!({"schema":"csdlc.v3.intent_changes.v1","amendment":{"class":"scope_acceptance","transition_approved":true},"cards":{"sip":{"title":"Recovered ordinary edit"}}}));
    let crash = fixture.run_with_env(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_local_amendment_after_commit",
        )],
    );
    assert_eq!(
        crash.status.code(),
        Some(91),
        "did not reach actual transaction interruption: {crash:?}"
    );
    let before = intent_fixture::inventory(&primary);
    let missing = fixture.run(&linked, &["recover", "505", "--execute"]);
    assert!(!missing.status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "recovery preview performed repair"
    );
    let digest = preview["preview_digest"]
        .as_str()
        .expect("fresh preview digest");
    let wrong = fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", "stale-preview"],
    );
    assert!(!wrong.status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", digest],
    ));
    observation(&mut fixture, &linked, "validate");
    let before = intent_fixture::inventory(&primary);
    let consumed = fixture.run(
        &linked,
        &["recover", "505", "--execute", "--preview", digest],
    );
    assert!(
        !consumed.status.success(),
        "consumed preview accepted for changed transaction state"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_bind_recovery_handles_absent_target_before_loading_worktree() {
    let mut fixture = Fixture::new("bind-recovery-absent-target");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let crash = fixture.run_with_env(
        &primary,
        &["bind", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_bind_after_reservation",
        )],
    );
    assert_eq!(
        crash.status.code(),
        Some(91),
        "did not reach bind reservation interruption: {crash:?}"
    );
    assert!(
        git(&primary, &["worktree", "list", "--porcelain"])
            .lines()
            .filter(|line| line.starts_with("worktree "))
            .count()
            == 1,
        "bind crash unexpectedly created a target worktree"
    );

    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(preview["action"], "reconcile_native_bind");
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "bind recovery preview mutated state"
    );
    let recovered = success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(recovered["status"], "completed");
    assert_eq!(recovered["semantic_outcome"], "failure");
    assert_eq!(recovered["native_effect_truth"], "not_performed");
    assert_eq!(recovered["performed_mutation"], false);
}

#[test]
fn installed_proof_runs_real_validator_and_rejects_zero_test_success() {
    for zero_tests in [false, true] {
        let mut fixture = Fixture::new(if zero_tests {
            "zero-test-proof"
        } else {
            "real-validator-proof"
        });
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        if zero_tests {
            fs::write(
                linked.join("fixture-proof/src/lib.rs"),
                "pub const NO_TESTS: bool = true;\n",
            )
            .unwrap();
            git(&linked, &["add", "fixture-proof/src/lib.rs"]);
            git(
                &linked,
                &["commit", "--quiet", "-m", "fixture with no tests"],
            );
        }
        let target = linked.join("target/intent-validation");
        assert!(
            !target.exists(),
            "proof target already warm; measurement would be misleading"
        );
        observation(&mut fixture, &linked, "status");
        observation(&mut fixture, &linked, "validate");
        assert!(!target.exists(), "observation ran a validator");
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            target.exists(),
            "proof did not run the declared Cargo validator: {output:?}"
        );
        if zero_tests {
            assert!(
                !output.status.success(),
                "zero executed tests accepted as proof: {output:?}"
            );
            let value: Value =
                serde_json::from_slice(&output.stdout).expect("zero-test rejection result");
            assert_ne!(value["envelope"]["status"], "completed");
        } else {
            let proof = success(output);
            assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
            let repeated = success(fixture.run(&primary, &["proof", "505"]));
            assert_eq!(repeated["proof"]["validators"][0]["tests_passed"], 1);
            fs::write(linked.join("tracked"), "changed after proof\n").unwrap();
            let before = intent_fixture::inventory(&primary);
            let status = success(fixture.run(&linked, &["status", "505"]));
            assert_eq!(
                status["evidence"]["proof_current"], false,
                "dirty source retained current proof recommendation"
            );
            assert_same_inventory!(
                before,
                intent_fixture::inventory(&primary),
                "status repaired or reran stale proof"
            );
        }
    }
}

#[test]
fn installed_prepare_rejects_validators_that_proof_would_not_admit() {
    let mut fixture = Fixture::new("prepare-validator-admission");
    let primary = fixture.root.clone();
    let mut invalid_plan = plan();
    invalid_plan["validators"][0]["args"] = json!([
        "test",
        "--manifest-path",
        "fixture-proof/Cargo.toml",
        "--offline",
        "--color",
        "always"
    ]);
    let input = fixture.write_json("invalid-validator-plan.json", &invalid_plan);
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    );
    assert!(
        !output.status.success(),
        "prepare accepted proof-inadmissible validator: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("intent_validator_argument_not_admitted"),
        "unexpected validator admission refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!primary
        .join(".git/csdlc-v3/local/issues/505/index.json")
        .exists());
}

#[test]
fn installed_edit_can_correct_validators_with_a_single_cargo_filter() {
    let mut fixture = Fixture::new("edit-validator-correction");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        "// implementation in progress\n",
    )
    .unwrap();
    let changes = fixture.write_json(
        "validator-changes.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","validators":[{
            "id":"fixture-proof",
            "program":"cargo",
            "args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline","tracked_fixture_bytes_are_preserved"],
            "success_marker":"test result: ok."
        }]}),
    );
    let edited = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(edited["status"], "completed");
    let repeated = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(repeated["status"], "expected_noop");
    // Declaring a validator does not execute it or admit dirty candidate bytes.
    let mut invalid: Value = serde_json::from_slice(&fs::read(&changes).unwrap()).unwrap();
    invalid["validators"][0]["timeout_seconds"] = json!(0);
    let invalid_path = fixture.write_json("invalid-dirty-validator.json", &invalid);
    let before = intent_fixture::inventory(&primary);
    let rejected = fixture.run(
        &linked,
        &["edit", "505", "--changes", invalid_path.to_str().unwrap()],
    );
    assert!(!rejected.status.success());
    assert!(
        String::from_utf8_lossy(&rejected.stdout).contains("intent_validator_timeout_not_admitted")
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let dirty_proof = fixture.run(&linked, &["proof", "505"]);
    assert!(!dirty_proof.status.success());
    assert!(
        String::from_utf8_lossy(&dirty_proof.stdout).contains("intent_candidate_tracked_changes")
    );
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn tracked_fixture_bytes_are_preserved() {
    assert_eq!(include_str!("../../tracked"), "fixture\n");
}

#[test]
fn unselected_failure_after_validator_edit() {
    panic!("validator edit did not preserve the selected Cargo filter");
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track filtered validator fixture",
        ],
    );
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
}

#[test]
fn installed_recover_reconciles_interrupted_validator_edit() {
    let mut fixture = Fixture::new("recover-validator-edit");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let changes = fixture.write_json(
        "validator-changes.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","validators":[{
            "id":"fixture-proof",
            "program":"cargo",
            "args":["test","--manifest-path","fixture-proof/Cargo.toml","--offline","tracked_fixture_bytes_are_preserved"],
            "success_marker":"test result: ok."
        }]}),
    );
    let crash = fixture.run_with_env(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_local_amendment_after_commit",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(preview["action"], "repair_semantic_projection");
    let recovered = success(fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(recovered["status"], "completed");
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn tracked_fixture_bytes_are_preserved() {
    assert_eq!(include_str!("../../tracked"), "fixture\n");
}

#[test]
fn unselected_failure_after_validator_recovery() {
    panic!("validator recovery did not preserve the selected Cargo filter");
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track recovered validator fixture",
        ],
    );
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
}

#[test]
fn installed_proof_rejects_marker_only_custom_test_harness() {
    let mut fixture = Fixture::new("marker-only-custom-harness");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        "pub const MARKER_ONLY_FIXTURE: bool = true;\n",
    )
    .unwrap();
    fs::create_dir_all(linked.join("fixture-proof/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/tests/marker_only.rs"),
        r#"fn main() {
    println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out");
}
"#,
    )
    .unwrap();
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[test]]\nname = \"marker_only\"\npath = \"tests/marker_only.rs\"\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    git(
        &linked,
        &[
            "add",
            "fixture-proof/Cargo.toml",
            "fixture-proof/src/lib.rs",
            "fixture-proof/tests/marker_only.rs",
        ],
    );
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track marker-only custom harness",
        ],
    );
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "marker-only custom harness established passing proof: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("intent_validator_custom_harness_not_admitted"),
        "unexpected custom-harness refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
    assert!(!linked.join(".csdlc/v3/issues/505/proof.json").exists());
}

#[test]
fn installed_proof_rejects_marker_only_harness_in_nonvirtual_workspace_member() {
    let mut fixture = Fixture::new("marker-only-nonvirtual-workspace");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[workspace]\nmembers = [\"marker-member\"]\ndefault-members = [\"marker-member\"]\n",
    );
    fs::write(&manifest, content).unwrap();
    fs::create_dir_all(linked.join("fixture-proof/marker-member/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/marker-member/Cargo.toml"),
        "[package]\nname = \"marker-member\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[test]]\nname = \"marker_only\"\npath = \"tests/marker_only.rs\"\nharness = false\n",
    )
    .unwrap();
    fs::write(
        linked.join("fixture-proof/marker-member/tests/marker_only.rs"),
        r#"fn main() {
    println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out");
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track marker-only nonvirtual workspace member",
        ],
    );
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "workspace member custom harness established passing proof: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("intent_validator_custom_harness_not_admitted"),
        "unexpected workspace custom-harness refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
    assert!(!linked.join(".csdlc/v3/issues/505/proof.json").exists());
}

#[test]
fn installed_proof_rejects_marker_only_harness_in_selected_sibling_member() {
    let mut fixture = Fixture::new("marker-only-sibling-workspace-member");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[workspace]\nmembers = [\"../marker-member\"]\ndefault-members = [\"../marker-member\"]\n",
    );
    fs::write(&manifest, content).unwrap();
    fs::create_dir_all(linked.join("marker-member/tests")).unwrap();
    fs::write(
        linked.join("marker-member/Cargo.toml"),
        "[package]\nname = \"marker-member\"\nversion = \"0.1.0\"\nedition = \"2021\"\nworkspace = \"../fixture-proof\"\n\n[[test]]\nname = \"marker_only\"\npath = \"tests/marker_only.rs\"\nharness = false\n",
    )
    .unwrap();
    fs::write(
        linked.join("marker-member/tests/marker_only.rs"),
        r#"fn main() {
    println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out");
}
"#,
    )
    .unwrap();
    git(
        &linked,
        &["add", "fixture-proof/Cargo.toml", "marker-member"],
    );
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track marker-only sibling workspace member",
        ],
    );
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "sibling custom harness established passing proof: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("intent_validator_custom_harness_not_admitted"),
        "unexpected sibling custom-harness refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
}

#[test]
fn installed_proof_rejects_marker_only_example_opted_into_default_tests() {
    let mut fixture = Fixture::new("marker-only-opted-in-example");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        "pub const MARKER_ONLY_EXAMPLE_FIXTURE: bool = true;\n",
    )
    .unwrap();
    fs::create_dir_all(linked.join("fixture-proof/examples")).unwrap();
    fs::write(
        linked.join("fixture-proof/examples/marker_only.rs"),
        r#"fn main() {
    println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out");
}
"#,
    )
    .unwrap();
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[example]]\nname = \"marker_only\"\npath = \"examples/marker_only.rs\"\ntest = true\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    git(&linked, &["add", "fixture-proof"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track marker-only opted-in example",
        ],
    );
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "opted-in example custom harness established passing proof: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("intent_validator_custom_harness_not_admitted"),
        "unexpected opted-in example refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
}

#[test]
fn installed_proof_default_selection_ignores_member_and_bench_custom_harnesses() {
    let mut fixture = Fixture::new("unselected-workspace-and-bench-harnesses");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[workspace]\nmembers = [\"unselected-member\"]\n\n[[bench]]\nname = \"unselected_bench\"\npath = \"benches/unselected.rs\"\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    fs::create_dir_all(linked.join("fixture-proof/benches")).unwrap();
    fs::write(
        linked.join("fixture-proof/benches/unselected.rs"),
        "fn main() { panic!(\"unselected custom benchmark executed\"); }\n",
    )
    .unwrap();
    fs::create_dir_all(linked.join("fixture-proof/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/tests/disabled.rs"),
        "fn main() { panic!(\"disabled custom test executed\"); }\n",
    )
    .unwrap();
    content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[test]]\nname = \"disabled\"\npath = \"tests/disabled.rs\"\ntest = false\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    fs::create_dir_all(linked.join("fixture-proof/unselected-member/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/unselected-member/Cargo.toml"),
        "[package]\nname = \"unselected-member\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[[test]]\nname = \"unselected\"\npath = \"tests/unselected.rs\"\nharness = false\n",
    )
    .unwrap();
    fs::write(
        linked.join("fixture-proof/unselected-member/tests/unselected.rs"),
        "fn main() { panic!(\"unselected workspace member executed\"); }\n",
    )
    .unwrap();
    let lock = Command::new("cargo")
        .current_dir(&linked)
        .args([
            "generate-lockfile",
            "--offline",
            "--manifest-path",
            "fixture-proof/Cargo.toml",
        ])
        .output()
        .unwrap();
    assert!(
        lock.status.success(),
        "workspace lock refresh failed: {lock:?}"
    );
    git(&linked, &["add", "fixture-proof"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track unselected workspace and benchmark harnesses",
        ],
    );
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
}

#[test]
fn installed_proof_allows_lib_when_unselected_test_uses_custom_harness() {
    let mut fixture = Fixture::new("unselected-custom-test-harness");
    let primary = fixture.root.clone();
    let mut lib_plan = plan();
    lib_plan["validators"][0]["args"] = json!([
        "test",
        "--manifest-path",
        "fixture-proof/Cargo.toml",
        "--offline",
        "--lib"
    ]);
    let input = fixture.write_json("lib-plan.json", &lib_plan);
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::create_dir_all(linked.join("fixture-proof/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/tests/unselected.rs"),
        "fn main() { panic!(\"unselected custom harness executed\"); }\n",
    )
    .unwrap();
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[test]]\nname = \"unselected\"\npath = \"tests/unselected.rs\"\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    git(&linked, &["add", "fixture-proof"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "Track unselected custom test harness",
        ],
    );
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
}

#[test]
fn installed_proof_rejects_custom_harness_selected_after_standard_test() {
    let mut fixture = Fixture::new("custom-harness-second-selector");
    let primary = fixture.root.clone();
    let mut repeated_test_plan = plan();
    repeated_test_plan["validators"][0]["args"] = json!([
        "test",
        "--manifest-path",
        "fixture-proof/Cargo.toml",
        "--offline",
        "--test",
        "empty_standard",
        "--test",
        "marker_only"
    ]);
    let input = fixture.write_json("repeated-test-plan.json", &repeated_test_plan);
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        "pub const REPEATED_SELECTOR_FIXTURE: bool = true;\n",
    )
    .unwrap();
    fs::create_dir_all(linked.join("fixture-proof/tests")).unwrap();
    fs::write(
        linked.join("fixture-proof/tests/empty_standard.rs"),
        "// The standard harness runs zero tests.\n",
    )
    .unwrap();
    fs::write(
        linked.join("fixture-proof/tests/marker_only.rs"),
        r#"fn main() {
    println!("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out");
}
"#,
    )
    .unwrap();
    let manifest = linked.join("fixture-proof/Cargo.toml");
    let mut content = fs::read_to_string(&manifest).unwrap();
    content.push_str(
        "\n[[test]]\nname = \"empty_standard\"\npath = \"tests/empty_standard.rs\"\n\n[[test]]\nname = \"marker_only\"\npath = \"tests/marker_only.rs\"\nharness = false\n",
    );
    fs::write(&manifest, content).unwrap();
    git(&linked, &["add", "fixture-proof"]);
    git(
        &linked,
        &["commit", "--quiet", "-m", "Track repeated test selectors"],
    );
    let before = intent_fixture::inventory(&primary);
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "second selected custom harness established passing proof: {output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("intent_validator_custom_harness_not_admitted"),
        "unexpected repeated-selector refusal: {output:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
    assert!(!linked.join(".csdlc/v3/issues/505/proof.json").exists());
}

#[test]
fn installed_issue_mutations_execute_exact_targets_and_preserve_omitted_metadata() {
    for linked_invocation in [false, true] {
        for operation in [
            json!({"action":"issue_create","title":"New synthetic issue","body":"Synthetic new issue","labels":[],"assignees":[],"milestone":null}),
            json!({"action":"issue_comment","body":"Synthetic comment"}),
            json!({"action":"issue_edit","title":"Edited remote title","body":null}),
            json!({"action":"issue_close","rationale":"Duplicate fixture scope","current_body":"Fixture issue","disposition":"duplicate","duplicate_of":504,"github_state_reason":"not_planned"}),
        ] {
            let name = operation["action"].as_str().unwrap();
            let mut fixture = Fixture::new(&format!("{name}-{linked_invocation}"));
            fixture.enable_issue_transport();
            let primary = fixture.root.clone();
            prepare(&mut fixture);
            success(fixture.run(&primary, &["bind", "505"]));
            let linked = linked_worktree(&primary);
            let cwd = if linked_invocation { &linked } else { &primary };
            let path = fixture.write_json("operation.json", &operation);
            let before = intent_fixture::inventory(&primary);
            let preview = success(fixture.run(
                cwd,
                &["github-issue", "505", "--operation", path.to_str().unwrap()],
            ));
            assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let wrong_family = fixture.run(
                cwd,
                &[
                    "github-pr",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            );
            assert!(
                !wrong_family.status.success(),
                "wrong family accepted: {wrong_family:?}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            success(fixture.run(
                cwd,
                &[
                    "github-issue",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            ));
            assert_eq!(
                fixture.remote_effects(),
                1,
                "operation did not produce exactly one remote effect"
            );
            if name == "issue_edit" {
                let remote = fixture.remote_issue();
                assert_eq!(remote["title"], "Edited remote title");
                assert_eq!(remote["labels"], json!(["retained"]));
                assert_eq!(remote["assignees"], json!(["fixture-assignee"]));
                assert_eq!(remote["milestone"]["number"], 4);
                assert!(remote["body"]
                    .as_str()
                    .unwrap()
                    .starts_with("Fixture issue"));
            }
            if name == "issue_close" {
                let remote = fixture.remote_issue();
                assert_eq!(remote["state"], "closed");
                assert_eq!(remote["state_reason"], "not_planned");
                assert!(remote["body"].as_str().unwrap().contains("504"));
            }
        }
    }
}

#[test]
fn installed_issue_creation_recovers_by_exact_one_shot_request_replay() {
    let mut fixture = Fixture::new("issue-create-reserved-crash-recovery");
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let operation = json!({"action":"issue_create","title":"Recovered synthetic issue",
        "body":"Recovered exact request","labels":[],"assignees":[],"milestone":null});
    let initial = fixture.write_json("create.json", &operation);
    let crash = fixture.run_with_env(
        &linked,
        &[
            "github-issue",
            "505",
            "--operation",
            initial.to_str().unwrap(),
            "--execute",
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_creation_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 0);
    let recovery = fixture.write_json(
        "create-recovery.json",
        &json!({"operation":operation,"recovery":"retry_after_authenticated_absence"}),
    );
    let recovered = success(fixture.run(
        &linked,
        &[
            "github-issue",
            "505",
            "--operation",
            recovery.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(recovered["semantic"]["created_issue"]["issue"], 506);
    assert_eq!(
        recovered["result"]["reconciliation"]["remote_object_id"],
        12345
    );
    assert_eq!(fixture.remote_effects(), 1);
    let before = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(
        &linked,
        &[
            "github-issue",
            "505",
            "--operation",
            recovery.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(replay["status"], "expected_noop");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert_eq!(fixture.remote_effects(), 1);
}

fn external_review(linked: &Path) -> Value {
    use csdlc_v3::commands::remote::{typed_review_receipt_payload_digest, TypedReviewReceipt};
    let head = git(linked, &["rev-parse", "HEAD"]);
    let proof_path = ".csdlc/v3/issues/505/proof.json";
    let proof = fs::read(linked.join(proof_path)).expect("real installed proof receipt");
    let receipt:TypedReviewReceipt=serde_json::from_value(json!({
        "schema":"csdlc.v3.typed_review_receipt.v1","repository":"agent-logic/agent-design-language","issue":505,
        "implementer":"synthetic-fixture-author","reviewer":"synthetic-independent-fixture-reviewer",
        "reviewed_revision":head,"expected_head_sha":head,
        "evidence_digest":blake3::hash(b"Synthetic external fixture review, not review of #869 source").to_hex().to_string(),
        "publication_linkage":{"repository":"agent-logic/agent-design-language","issue":505,"mode":"closing"}
    })).unwrap();
    json!({"receipt_digest":typed_review_receipt_payload_digest(&receipt),"receipt":receipt,"proof_path":proof_path,"proof_digest":blake3::hash(&proof).to_hex().to_string()})
}

#[test]
fn installed_review_consumes_external_exact_head_evidence_and_rejects_wrong_identity() {
    use csdlc_v3::commands::remote::{typed_review_receipt_payload_digest, TypedReviewReceipt};
    for linked_invocation in [false, true] {
        let mut fixture = Fixture::new(&format!("independent-review-{linked_invocation}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        success(fixture.run(&linked, &["proof", "505"]));
        let valid = external_review(&linked);
        let cwd = if linked_invocation { &linked } else { &primary };
        for defect in [
            "same-principal",
            "stale-head",
            "wrong-repository",
            "wrong-proof-digest",
        ] {
            let mut altered = valid.clone();
            match defect {
                "same-principal" => {
                    altered["receipt"]["reviewer"] = altered["receipt"]["implementer"].clone()
                }
                "stale-head" => altered["receipt"]["reviewed_revision"] = json!("0".repeat(40)),
                "wrong-repository" => altered["receipt"]["repository"] = json!("other/repository"),
                "wrong-proof-digest" => altered["proof_digest"] = json!("0".repeat(64)),
                _ => unreachable!(),
            }
            let receipt: TypedReviewReceipt =
                serde_json::from_value(altered["receipt"].clone()).unwrap();
            altered["receipt_digest"] = json!(typed_review_receipt_payload_digest(&receipt));
            let path = fixture.write_json("review.json", &altered);
            let before = intent_fixture::inventory(&primary);
            let output = fixture.run(
                cwd,
                &["review", "505", "--evidence", path.to_str().unwrap()],
            );
            assert!(
                !output.status.success(),
                "invalid review {defect} accepted: {output:?}"
            );
            assert_same_inventory!(
                before,
                intent_fixture::inventory(&primary),
                "rejected review persisted evidence"
            );
        }
        let path = fixture.write_json("review.json", &valid);
        let accepted = success(fixture.run(
            cwd,
            &["review", "505", "--evidence", path.to_str().unwrap()],
        ));
        let reference = accepted["review_receipt_path"]
            .as_str()
            .expect("persisted review reference");
        assert!(linked.join(reference).is_file());
        let before = intent_fixture::inventory(&primary);
        let replay = success(fixture.run(
            cwd,
            &["review", "505", "--evidence", path.to_str().unwrap()],
        ));
        assert_eq!(replay["envelope"]["status"], "expected_noop");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

fn reviewed_fixture(label: &str) -> (Fixture, std::path::PathBuf) {
    let mut fixture = Fixture::new(label);
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["proof", "505"]));
    let review = fixture.write_json("review.json", &external_review(&linked));
    success(fixture.run(
        &linked,
        &["review", "505", "--evidence", review.to_str().unwrap()],
    ));
    fixture.enable_pr_transport(&linked);
    (fixture, linked)
}

#[test]
fn issue_1036_review_amendment_rejects_before_head_refresh_invalidates_evidence() {
    let (mut fixture, linked) = reviewed_fixture("review-head-drift-admission");
    let primary = fixture.root.clone();
    let binding_a: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    let head_a = binding_a["inputs"]["binding"]["head"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(binding_a["phase"], "reviewed");

    fs::write(linked.join("review-drift.txt"), "candidate B\n").unwrap();
    git(&linked, &["add", "review-drift.txt"]);
    git(&linked, &["commit", "--quiet", "-m", "candidate B"]);
    assert_ne!(git(&linked, &["rev-parse", "HEAD"]), head_a);
    let changes = fixture.write_json(
        "stale-review-amendment.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1",
            "amendment":{"class":"review","transition_approved":true,"new_commit":true},
            "cards":{"srp":{"summary":"stale review must not authorize this edit"}}}),
    );
    let before = intent_fixture::inventory(&fixture.root);
    let rejected = fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    );
    assert!(
        !rejected.status.success(),
        "stale review amendment was admitted"
    );
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("intent_amendment_policy_rejected"));
    assert_same_inventory!(before, intent_fixture::inventory(&fixture.root));
    let retained: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    assert_eq!(retained["phase"], "reviewed");
    assert_eq!(retained["inputs"]["binding"]["head"], head_a);
    assert!(primary.join(".git/csdlc-v3/semantic/issues/505").is_dir());
}

#[test]
fn installed_publication_pr_mutations_and_uncertain_ready_retry_use_native_receipts() {
    for ordinary_publication in [false, true] {
        let (mut fixture, linked) =
            reviewed_fixture(&format!("publication-{ordinary_publication}"));
        let primary = fixture.root.clone();
        let cwd = if ordinary_publication {
            &primary
        } else {
            &linked
        };
        for route in ["clean", "finish"] {
            let before = intent_fixture::inventory(&primary);
            let absent = fixture.run(cwd, &[route, "505"]);
            assert!(
                !absent.status.success(),
                "{route} succeeded without terminal target: {absent:?}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
        if ordinary_publication {
            fixture.remote_flag("drop-publication-readback", true);
            let failed_readback = fixture.run(cwd, &["publish", "505"]);
            assert!(!failed_readback.status.success());
            let payload: Value = serde_json::from_slice(&failed_readback.stdout).unwrap();
            assert_eq!(payload["status"], "recovery_required");
            assert!(
                payload["envelope"]["findings"]
                    .as_array()
                    .expect("common result envelope must expose structured findings")
                    .iter()
                    .all(|finding| {
                        let code = finding["code"].as_str().unwrap();
                        !code.contains("recovery_required")
                            && !code.contains("reconciliation_required")
                            && !code.contains("reconciliation_pending")
                    }),
                "fixture must exercise explicit owner status rather than code-substring inference"
            );
            assert_eq!(payload["envelope"]["status"], "recovery_required");
            assert_eq!(payload["envelope"]["process_status"], "failed");
            assert_eq!(
                payload["envelope"]["effects"]["outcome"], "unknown",
                "a dropped authenticated readback must preserve unknown-effect truth"
            );
            assert_eq!(fixture.remote_effects(), 1);
            fixture.remote_flag("drop-publication-readback", false);
            fixture.remote_flag("drop-readback", false);
            let recovery = success(fixture.run(&primary, &["recover", "505"]));
            success(fixture.run(
                &primary,
                &[
                    "recover",
                    "505",
                    "--execute",
                    "--preview",
                    recovery["preview_digest"].as_str().unwrap(),
                ],
            ));
            observation(&mut fixture, cwd, "pr-state");
            assert_eq!(
                fixture.remote_effects(),
                1,
                "authenticated observation duplicated publication effect"
            );
        } else {
            let operation = json!({"action":"pull_request_create","base":"main","head":git(&linked,&["symbolic-ref","--short","HEAD"]),"title":"Installed intent fixture","body":"Closes #505","draft":true});
            let mut invalid = operation.clone();
            invalid["head"] = json!("codex/foreign");
            let path = fixture.write_json("operation.json", &invalid);
            let before = intent_fixture::inventory(&primary);
            assert!(!fixture
                .run(
                    cwd,
                    &[
                        "github-pr",
                        "505",
                        "--operation",
                        path.to_str().unwrap(),
                        "--execute"
                    ]
                )
                .status
                .success());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let path = fixture.write_json("operation.json", &operation);
            success(fixture.run(
                cwd,
                &[
                    "github-pr",
                    "505",
                    "--operation",
                    path.to_str().unwrap(),
                    "--execute",
                ],
            ));
        }
        assert_eq!(fixture.remote_effects(), 1);
        observation(&mut fixture, cwd, "pr-state");
        let published = fixture.remote_pr();
        assert_eq!(published["base"]["ref"], "main");
        assert_eq!(
            published["head"]["sha"],
            git(&linked, &["rev-parse", "HEAD"])
        );
        let mut wrong_head = published.clone();
        wrong_head["head"]["sha"] = json!("0".repeat(40));
        fixture.set_remote_pr(&wrong_head);
        let before = intent_fixture::inventory(&primary);
        assert!(
            !fixture.run(cwd, &["publish", "505"]).status.success(),
            "publication admitted wrong remote HEAD"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        fixture.set_remote_pr(&published);
        let operation=fixture.write_json("operation.json",&json!({"action":"pull_request_update","title":"Updated fixture PR","body":"Closes #505"}));
        success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                operation.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(fixture.remote_effects(), 2);
        assert_eq!(fixture.remote_pr()["title"], "Updated fixture PR");
        let current_remote = fixture.remote_pr();
        for field in ["head", "base"] {
            let mut foreign = current_remote.clone();
            if field == "head" {
                foreign["head"]["sha"] = json!("0".repeat(40));
            } else {
                foreign["base"]["ref"] = json!("foreign-base");
            }
            fixture.set_remote_pr(&foreign);
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture
                    .run(
                        cwd,
                        &[
                            "github-pr",
                            "505",
                            "--operation",
                            operation.to_str().unwrap(),
                            "--execute"
                        ]
                    )
                    .status
                    .success(),
                "explicit PR update admitted foreign {field}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            assert_eq!(fixture.remote_effects(), 2);
        }
        fixture.set_remote_pr(&current_remote);
        let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
        fixture.remote_flag("uncertain-response", true);
        let uncertain = fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        );
        assert!(
            !uncertain.status.success(),
            "uncertain ready outcome reported completed"
        );
        let unknown: Value = serde_json::from_slice(&uncertain.stdout).unwrap();
        assert_eq!(unknown["envelope"]["status"], "recovery_required");
        assert_eq!(unknown["envelope"]["effects"]["outcome"], "unknown");
        assert_eq!(fixture.remote_effects(), 3);
        assert_eq!(fixture.remote_pr()["draft"], false);
        fixture.remote_flag("uncertain-response", false);
        fixture.remote_flag("drop-readback", false);
        if ordinary_publication {
            let before = intent_fixture::inventory(&primary);
            let pending = success(fixture.run(cwd, &["status", "505"]));
            assert_eq!(pending["allowed_next"], json!(["recover"]));
            assert!(!pending["pending_remote"].as_array().unwrap().is_empty());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            let preview = success(fixture.run(cwd, &["recover", "505"]));
            let token = preview["preview_digest"].as_str().unwrap();
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            for extra in [vec!["--execute"], vec!["--execute", "--preview", "stale"]] {
                let mut args = vec!["recover", "505"];
                args.extend(extra);
                assert!(!fixture.run(cwd, &args).status.success());
                assert_same_inventory!(before, intent_fixture::inventory(&primary));
            }
            success(fixture.run(cwd, &["recover", "505", "--execute", "--preview", token]));
            assert_eq!(
                fixture.remote_effects(),
                3,
                "ordinary recovery duplicated observed ready mutation"
            );
        }
        success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(
            fixture.remote_effects(),
            3,
            "reconciliation duplicated ready mutation"
        );
        let before = intent_fixture::inventory(&primary);
        let noop = success(fixture.run(
            cwd,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        ));
        assert_eq!(noop["envelope"]["status"], "expected_noop");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

#[test]
fn installed_merge_finish_and_exact_bound_cleanup_preserve_authority_and_archive_residue() {
    let (mut fixture, linked) = reviewed_fixture("merge-finish-clean");
    let primary = fixture.root.clone();
    success(fixture.run(&primary, &["publish", "505"]));
    let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let stale_primary_issue = primary.join(".csdlc/issues/505");
    fs::create_dir_all(&stale_primary_issue).unwrap();
    fs::copy(
        linked.join(".csdlc/issues/505/index.json"),
        stale_primary_issue.join("index.json"),
    )
    .unwrap();
    let missing_approval = fixture.write_json(
        "merge.json",
        &json!({"action":"pull_request_merge","base":"main","method":"merge"}),
    );
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &primary,
            &[
                "github-pr",
                "505",
                "--operation",
                missing_approval.to_str().unwrap(),
                "--execute"
            ]
        )
        .status
        .success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let merge=fixture.write_json("merge.json",&json!({"action":"pull_request_merge","base":"main","method":"merge","operator_approval":"synthetic operator authorizes only fixture PR639 exact candidate merge"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(fixture.remote_effects(), 3);
    assert_eq!(fixture.remote_pr()["merged"], true);
    assert_eq!(fixture.remote_issue()["state"], "closed");
    fs::remove_dir_all(primary.join(".csdlc/issues")).unwrap();
    let before = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(replay["envelope"]["status"], "expected_noop");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let preview = success(fixture.run(&primary, &["finish", "505", "--preview", "plan"]));
    assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(&linked, &["finish", "505"]));
    let terminal = primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json");
    assert!(terminal.is_file(), "native terminal evidence missing");
    for residue in ["foreign-note.txt", "tracked"] {
        let path = linked.join(residue);
        let previous = fs::read(&path).ok();
        fs::write(&path, b"retained dirty fixture bytes").unwrap();
        let before = intent_fixture::inventory(&primary);
        assert!(
            !fixture.run(&primary, &["clean", "505"]).status.success(),
            "unsafe residue permitted cleanup: {residue}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        if let Some(bytes) = previous {
            fs::write(path, bytes).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }
    let stale_projection = primary
        .parent()
        .expect("fixture root has parent")
        .join("stale-projection-worktree");
    if stale_projection.exists() {
        fs::remove_dir_all(&stale_projection).unwrap();
    }
    git(
        &primary,
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "fixture-stale-projection",
            stale_projection.to_str().unwrap(),
        ],
    );
    let stale_issue_root = stale_projection.join(".csdlc/issues/505");
    fs::create_dir_all(&stale_issue_root).unwrap();
    fs::copy(
        linked.join(".csdlc/issues/505/index.json"),
        stale_issue_root.join("index.json"),
    )
    .unwrap();
    let before = intent_fixture::inventory(&primary);
    let clean = success(fixture.run(&primary, &["clean", "505"]));
    assert_same_inventory!(
        before,
        intent_fixture::inventory(&primary),
        "cleanup preview archived or changed source files"
    );
    let token = clean["preview_token"]
        .as_str()
        .expect("cleanup preview token");
    let card = linked.join(".csdlc/v3/issues/505/cards/sip.md");
    let original = fs::read(&card).unwrap();
    fs::write(
        &card,
        [original.as_slice(), b"\nchanged after preview\n"].concat(),
    )
    .unwrap();
    let changed = intent_fixture::inventory(&primary);
    assert!(
        !fixture
            .run(&primary, &["clean", "505", "--execute", "--preview", token])
            .status
            .success(),
        "stale generated-residue preview admitted"
    );
    assert_same_inventory!(changed, intent_fixture::inventory(&primary));
    fs::write(&card, &original).unwrap();
    let archive_parent = primary.join(".git/csdlc-v3/local/archives");
    fs::write(
        &archive_parent,
        b"fixture blocks archive directory creation",
    )
    .unwrap();
    let originals = intent_fixture::inventory(&linked);
    let archive_failure = fixture.run(&primary, &["clean", "505", "--execute", "--preview", token]);
    assert!(
        !archive_failure.status.success(),
        "archive destination failure reported cleanup success"
    );
    let failed_state = intent_fixture::inventory(&linked);
    for (path, value) in &originals {
        assert_eq!(
            failed_state.get(path),
            Some(value),
            "archive failure changed original {path:?}"
        );
    }
    for added in failed_state
        .keys()
        .filter(|path| !originals.contains_key(*path))
    {
        assert!(
            matches!(
                added.to_str(),
                Some(".csdlc/locks" | ".csdlc/locks/505.lock")
            ),
            "unexpected archive-failure write {added:?}"
        );
    }
    assert!(fs::read(linked.join(".csdlc/locks/505.lock"))
        .unwrap_or_else(|error| panic!("missing cleanup lock after {archive_failure:?}: {error}"))
        .is_empty());
    let failure: Value = serde_json::from_slice(&archive_failure.stdout).unwrap();
    assert_ne!(
        failure["envelope"]["effects"]["outcome"], "none",
        "lock-creating failure reported no effect"
    );
    let originals = failed_state;
    fs::remove_file(&archive_parent).unwrap();
    let interrupted = fixture.interrupt_clean_after_archive(
        &primary,
        &["clean", "505", "--execute", "--preview", token],
    );
    assert!(!interrupted.status.success());
    assert_same_inventory!(
        originals,
        intent_fixture::inventory(&linked),
        "interrupted archive deleted original bytes before durable boundary"
    );
    assert!(fs::read_dir(&archive_parent).unwrap().any(|entry| entry
        .unwrap()
        .path()
        .join("manifest.json")
        .is_file()));

    let retained = intent_fixture::inventory(&primary);
    let continuation = success(fixture.run(&primary, &["clean", "505"]));
    assert_ne!(
        continuation["envelope"]["status"], "expected_noop",
        "registered partial removal misreported as complete"
    );
    assert_same_inventory!(retained, intent_fixture::inventory(&primary));
    let continued_token = continuation["preview_token"]
        .as_str()
        .expect("fresh continuation preview");
    success(fixture.run(
        &linked,
        &["clean", "505", "--execute", "--preview", continued_token],
    ));
    assert!(!linked.exists(), "exact bound worktree not removed");
    assert_eq!(
        git(&primary, &["worktree", "list", "--porcelain"])
            .lines()
            .filter(|line| line.starts_with("worktree "))
            .count(),
        2
    );
    assert!(
        stale_projection
            .join(".csdlc/issues/505/index.json")
            .is_file(),
        "cleanup removed unrelated stale projection"
    );
    assert!(terminal.is_file(), "cleanup lost terminal authority");
    let archived = fs::read_dir(primary.join(".git/csdlc-v3/local/archives"))
        .expect("durable cleanup archive")
        .collect::<Vec<_>>();
    assert!(
        !archived.is_empty(),
        "native cleanup omitted generated residue archive"
    );
    let before = intent_fixture::inventory(&primary);
    let noop = success(fixture.run(&primary, &["clean", "505"]));
    assert_eq!(noop["envelope"]["status"], "expected_noop");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

// PVF #1041: installed deterministic local Git fixture. This proves the
// explicit no-effect reconciliation for a merged terminal issue whose exact
// checkout was removed outside native cleanup; no network or paid runner.
#[test]
fn cleanup_recover_reconciles_explicit_already_absent_without_prior_operation() {
    let (mut fixture, linked) = reviewed_fixture("cleanup-already-absent-reconciliation");
    let primary = fixture.root.clone();
    success(fixture.run(&primary, &["publish", "505"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let merge = fixture.write_json(
        "merge.json",
        &json!({
            "action":"pull_request_merge",
            "base":"main",
            "method":"merge",
            "operator_approval":"synthetic operator authorizes only fixture PR639 exact candidate merge"
        }),
    );
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    success(fixture.run(&linked, &["finish", "505"]));

    let binding_path = primary.join(".git/csdlc-v3/local/bindings/505.json");
    let binding: Value = serde_json::from_slice(&fs::read(&binding_path).unwrap()).unwrap();
    let receipt_path = primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json");
    let receipt = fs::read(&receipt_path).unwrap();
    let disposition = fixture.write_json(
        "absent-cleanup-disposition.json",
        &json!({
            "schema":"csdlc.v3.semantic_cleanup_absence_recovery_disposition.v1",
            "disposition":"reconcile_already_absent_cleanup",
            "repository":"agent-logic/agent-design-language",
            "issue":505,
            "worktree":binding["worktree"],
            "branch":binding["branch"],
            "head":intent_fixture::git(&linked, &["rev-parse", "HEAD"]),
            "terminal_receipt_digest":blake3::hash(&receipt).to_hex().to_string(),
            "operator":"synthetic-fixture-operator",
            "rationale":"The exact terminal checkout was externally removed before native cleanup reserved an operation.",
            "evidence_refs":["fixture:externally-removed-worktree"]
        }),
    );

    let present = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    );
    assert!(
        !present.status.success(),
        "present checkout was reconciled as absent"
    );
    assert!(
        String::from_utf8_lossy(&present.stdout).contains("intent_cleanup_absence_target_present"),
        "unexpected present-target failure: stdout={} stderr={}",
        String::from_utf8_lossy(&present.stdout),
        String::from_utf8_lossy(&present.stderr)
    );

    intent_fixture::git(
        &primary,
        &["worktree", "remove", "--force", linked.to_str().unwrap()],
    );
    assert!(!linked.exists());
    let binding_bytes = fs::read(&binding_path).unwrap();
    let mut stale_binding = binding.clone();
    stale_binding["head"] = json!("0".repeat(40));
    stale_binding["registration"] = json!("0".repeat(64));
    fs::write(&binding_path, serde_json::to_vec(&stale_binding).unwrap()).unwrap();
    let stale = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    );
    assert!(
        !stale.status.success()
            && String::from_utf8_lossy(&stale.stdout).contains("intent_cleanup_binding_invalid"),
        "stale local binding was accepted: {stale:?}"
    );
    fs::write(&binding_path, binding_bytes).unwrap();
    let preview = success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    assert_eq!(preview["status"], "ready");
    assert_eq!(preview["performed_mutation"], false);
    let token = preview["preview_token"].as_str().expect("preview token");
    let interrupted = fixture.run_with_env(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
            "--execute",
            "--preview",
            token,
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "cleanup_absence_after_reservation",
        )],
    );
    assert_eq!(interrupted.status.code(), Some(91));
    fs::create_dir_all(&linked).unwrap();
    let overlapped = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    );
    assert!(
        !overlapped.status.success()
            && String::from_utf8_lossy(&overlapped.stdout)
                .contains("intent_cleanup_absence_target_present"),
        "restored checkout was reconciled as absent: {overlapped:?}"
    );
    fs::remove_dir(&linked).unwrap();
    let resumed = success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
    ));
    let resumed_token = resumed["preview_token"].as_str().expect("resumed token");
    let reconciled = success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
            "--execute",
            "--preview",
            resumed_token,
        ],
    ));
    assert_eq!(reconciled["status"], "expected_noop");
    assert_eq!(reconciled["performed_mutation"], false);
    assert_eq!(
        reconciled["semantic"]["effect_truth"], "not_performed",
        "reconciliation fabricated a native cleanup effect"
    );

    let root =
        SemanticRoot::from_git_common(primary.join(".git"), "agent-logic/agent-design-language")
            .unwrap();
    let key = IssueKey::new("agent-logic/agent-design-language", 505).unwrap();
    let snapshot = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            snapshot
        }
        other => panic!("{other:?}"),
    };
    assert!(snapshot.pending().is_none());
    assert_eq!(
        snapshot.completed().last().unwrap().truth(),
        csdlc_v3::storage::semantic::protocol::EffectTruth::NotPerformed
    );
    assert!(
        !intent_fixture::git(&primary, &["worktree", "list", "--porcelain"])
            .contains(linked.to_str().unwrap()),
        "externally removed checkout remained registered"
    );
}

#[test]
fn installed_generated_request_roundtrips_exact_stdout_without_manual_extraction() {
    let mut fixture = Fixture::new("generated-wrapper");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let emitted = fixture.run(&primary, &["bind", "505", "--emit-request"]);
    let bytes = emitted.stdout.clone();
    success(emitted);
    let path = primary.join(".git/installed-candidate/generated-request.json");
    fs::write(&path, &bytes).unwrap();
    success(fixture.run(
        &primary,
        &["bind", "--intent-request", path.to_str().unwrap()],
    ));
    assert!(linked_worktree(&primary)
        .join(".csdlc/issues/505/index.json")
        .is_file());
}

#[test]
fn installed_issue_edit_metadata_operations_are_complete_and_explicit() {
    for (name, fields, expected_labels, expected_assignees, expected_milestone) in [
        (
            "add",
            json!({"labels":{"operation":"add","names":["new"]}}),
            json!(["retained", "new"]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "remove",
            json!({"labels":{"operation":"remove","names":["retained"]}}),
            json!([]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "replace",
            json!({"labels":{"operation":"replace","names":["replacement"]}}),
            json!(["replacement"]),
            json!(["fixture-assignee"]),
            json!({"number":4}),
        ),
        (
            "assignees",
            json!({"assignees":["new-assignee"]}),
            json!(["retained"]),
            json!(["new-assignee"]),
            json!({"number":4}),
        ),
        (
            "milestone-set",
            json!({"milestone":{"operation":"set","number":7}}),
            json!(["retained"]),
            json!(["fixture-assignee"]),
            json!({"number":7}),
        ),
        (
            "milestone-clear",
            json!({"milestone":{"operation":"clear"}}),
            json!(["retained"]),
            json!(["fixture-assignee"]),
            Value::Null,
        ),
    ] {
        let mut fixture = Fixture::new(&format!("metadata-{name}"));
        fixture.enable_issue_transport();
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        let mut operation = json!({"action":"issue_edit","title":null,"body":null});
        operation
            .as_object_mut()
            .unwrap()
            .extend(fields.as_object().unwrap().clone());
        let path = fixture.write_json("operation.json", &operation);
        success(fixture.run(
            &linked,
            &[
                "github-issue",
                "505",
                "--operation",
                path.to_str().unwrap(),
                "--execute",
            ],
        ));
        let remote = fixture.remote_issue();
        assert_eq!(remote["labels"], expected_labels);
        assert_eq!(remote["assignees"], expected_assignees);
        assert_eq!(remote["milestone"], expected_milestone);
        assert_eq!(remote["title"], "Installed intent fixture");
        assert_eq!(fixture.remote_effects(), 1);
        for field in ["labels", "assignees", "milestone"] {
            let mut invalid = json!({"action":"issue_edit","title":"invalid metadata","body":null});
            invalid[field] = Value::Null;
            let path = fixture.write_json("operation.json", &invalid);
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture
                    .run(
                        &linked,
                        &[
                            "github-issue",
                            "505",
                            "--operation",
                            path.to_str().unwrap(),
                            "--execute"
                        ]
                    )
                    .status
                    .success(),
                "null {field} admitted as omitted metadata"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
    }
}

#[test]
fn installed_status_preserves_native_recommendations_and_explicit_human_decisions() {
    let mut fixture = Fixture::new("status-decisions");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let before = intent_fixture::inventory(&primary);
    let unknown = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(unknown["operator_decisions"]["required"], true);
    assert_eq!(unknown["shepherd"]["routing"]["state"], "operator_required");
    assert_eq!(unknown["evidence"]["validators_run"], false);
    assert!(unknown["eligibility"].is_object());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    for (dependencies, retry, schedule, shepherd) in [
        (true, false, "ready", "waiting"),
        (false, false, "blocked", "waiting"),
        (true, true, "ready", "retryable"),
    ] {
        let decisions = fixture.write_json(
            "decisions.json",
            &json!({
                "design_ready":true,"dependencies_ready":dependencies,"budget_available":true,
                "retryable_failure":retry
            }),
        );
        let before = intent_fixture::inventory(&primary);
        let output = success(fixture.run(
            &primary,
            &["status", "505", "--decisions", decisions.to_str().unwrap()],
        ));
        assert_eq!(output["operator_decisions"]["required"], false);
        assert_eq!(output["scheduling"]["routing"]["state"], schedule);
        assert_eq!(output["shepherd"]["routing"]["state"], shepherd);
        assert_eq!(output["evidence"]["validators_run"], false);
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

#[test]
fn installed_no_pr_finish_requires_closed_authenticated_issue_and_retains_disposition() {
    let mut fixture = Fixture::new("no-pr-disposition");
    fixture.enable_issue_transport();
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let disposition = fixture.write_json(
        "disposition.json",
        &json!({
            "disposition":"retired_without_execution", "operator":"synthetic-fixture-operator",
            "rationale":"Synthetic administrative retirement, not implementation delivery",
            "evidence_refs":["fixture:operator-retirement-decision"]
        }),
    );
    let args = [
        "finish",
        "505",
        "--disposition",
        disposition.to_str().unwrap(),
    ];
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture.run(&primary, &args).status.success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let mut remote = fixture.remote_issue();
    remote["state"] = json!("closed");
    remote["updated_at"] = json!("2026-09-11T12:00:00Z");
    remote["closed_at"] = json!("2026-09-11T12:00:00Z");
    fs::write(
        primary.join(".git/installed-candidate/remote-issue.json"),
        serde_json::to_vec(&remote).unwrap(),
    )
    .unwrap();
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
            "--preview",
            "plan",
        ],
    ));
    assert_eq!(preview["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(&primary, &args));
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert!(receipt["pull_request"].is_null());
    assert_eq!(
        receipt["no_pr_closeout"]["disposition"],
        "retired_without_execution"
    );
    assert_eq!(
        receipt["no_pr_closeout"]["expected_issue_closed_at"],
        remote["closed_at"]
    );
    let clean = success(fixture.run(&primary, &["clean", "505"]));
    let token = clean["preview_token"].as_str().unwrap();
    success(fixture.run(&primary, &["clean", "505", "--execute", "--preview", token]));
    assert!(!linked.exists());
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn installed_remote_recover_retries_once_only_after_authenticated_absence() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-absence");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    let ready = fixture.write_json("operation.json", &json!({"action":"pull_request_ready"}));
    fixture.remote_flag("reject-ready-before-effect", true);
    assert!(!fixture
        .run(
            &linked,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute"
            ]
        )
        .status
        .success());
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["draft"], true);
    fixture.remote_flag("reject-ready-before-effect", false);
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 2);
    assert_eq!(fixture.remote_pr()["draft"], false);
    let before = intent_fixture::inventory(&primary);
    let settled = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(settled["envelope"]["effects"]["outcome"], "none");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert_eq!(fixture.remote_effects(), 2);
}

#[test]
fn installed_remote_recover_retries_ready_once_after_crash_before_dispatch() {
    let (mut fixture, linked) = reviewed_fixture("remote-ready-reserved-crash");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    let crash = fixture.run_with_env(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["draft"], true);

    let preview = success(fixture.run(&primary, &["recover", "505"]));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 2);
    assert_eq!(fixture.remote_pr()["draft"], false);
    success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(
        fixture.remote_effects(),
        2,
        "ready recovery replayed dispatch"
    );
}

#[test]
fn installed_remote_recover_retries_once_after_crash_before_dispatch() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-reserved-crash");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 0);
    assert!(!primary
        .join(".git/installed-candidate/remote-pr.json")
        .exists());
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    fixture.remote_flag("wrong-branch-head", true);
    let rejected = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!rejected.status.success());
    assert_eq!(fixture.remote_effects(), 0);
    fixture.remote_flag("wrong-branch-head", false);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["number"], 639);
    let settled = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(settled["envelope"]["effects"]["outcome"], "none");
    assert_eq!(fixture.remote_effects(), 1);
}

#[test]
fn installed_remote_recover_attaches_retained_receipt_after_native_dispatch_crash() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-native-receipt-crash");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[("CSDLC_V3_TEST_CRASH_POINT", "semantic_remote_after_native")],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["number"], 639);
    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(preview["action"], "reconcile_retained_remote_effect");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 1, "recovery replayed publication");
    let settled = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(settled["envelope"]["effects"]["outcome"], "none");
}

// PVF #1092: an exact authenticated-absence receipt retires only the stale
// historical publication reservation. The later merged closing PR must still
// pass ordinary terminal authentication before finish succeeds.
#[test]
fn issue_1092_installed_recover_retires_absent_historical_publication() {
    let (mut fixture, linked) = reviewed_fixture("stale-publication-later-closing-pr");
    let primary = fixture.root.clone();
    let historical_head = git(&linked, &["rev-parse", "HEAD"]);
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 0);

    fs::write(
        linked.join("later-closing-pr.txt"),
        "later reviewed delivery\n",
    )
    .unwrap();
    git(&linked, &["add", "later-closing-pr.txt"]);
    git(
        &linked,
        &["commit", "--quiet", "-m", "later closing delivery"],
    );
    let closing_head = git(&linked, &["rev-parse", "HEAD"]);
    assert_ne!(historical_head, closing_head);

    let intents = primary.join(".git/csdlc-v3/remote/intents");
    let intent_path = fs::read_dir(&intents)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
        .expect("retained publication intent");
    let operation = intent_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let intent: Value = serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
    assert_eq!(intent["request"]["expected_head_sha"], historical_head);
    let mut hasher = blake3::Hasher::new();
    for value in [
        intent["schema"].as_str().unwrap(),
        &operation,
        intent["operation_marker"].as_str().unwrap(),
        intent["authority_selector_digest"].as_str().unwrap(),
        intent["adapter"].as_str().unwrap(),
    ] {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    let intent_digest = hasher.finalize().to_hex().to_string();
    let recoveries = primary.join(".git/csdlc-v3/remote/recoveries");
    fs::create_dir_all(&recoveries).unwrap();
    let recovery_path = recoveries.join(format!("{operation}.json"));
    let recovery = json!({
        "schema":"csdlc.v3.github_mutation_recovery.v1",
        "operation_digest":operation,"intent_digest":intent_digest,
        "recovery":"retry_after_authenticated_absence",
        "repository":"agent-logic/agent-design-language","issue":505,
        "pull_request":null,"expected_head_sha":historical_head
    });
    fs::write(&recovery_path, serde_json::to_vec(&recovery).unwrap()).unwrap();
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    let recovered = success(fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(
        recovered["result"]["recovery"],
        "authenticated_absent_historical_publication"
    );
    assert_eq!(fixture.remote_effects(), 0);

    fs::write(
        primary.join(".git/installed-candidate/remote-pr-640.json"),
        serde_json::to_vec(&json!({
            "number":640,"head":{"sha":closing_head},"merged":true,"state":"closed",
            "body":"Closes #505"
        }))
        .unwrap(),
    )
    .unwrap();
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    let finished = success(fixture.run(&linked, &["finish", "505", "--pull-request", "640"]));
    assert_eq!(finished["status"], "completed");
    assert_eq!(fixture.remote_effects(), 0);
    let replay = success(fixture.run(&linked, &["finish", "505"]));
    assert_eq!(replay["status"], "expected_noop");
}

#[test]
fn installed_remote_recover_pr_create_waits_for_branch_and_reuses_definite_rejection_once() {
    let (mut fixture, linked) = reviewed_fixture("pr-create-recovery-branch-later");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 0);

    fixture.remote_flag("remote-head-present", false);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let missing = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stdout).contains("github_pr_head_branch_missing"));
    assert_eq!(fixture.remote_effects(), 0);
    let recoveries = primary.join(".git/csdlc-v3/remote/recoveries");
    assert!(
        !recoveries.exists() || fs::read_dir(&recoveries).unwrap().next().is_none(),
        "missing branch consumed recovery"
    );

    // Model the #1028 compatibility shape: branch observation races with an
    // authenticated create rejection, so the recovery receipt is durable but
    // the semantic operation retains the definite rejection.
    fixture.remote_flag("remote-head-present", true);
    fixture.remote_flag("reject-pr-create", true);
    let rejected_preview = success(fixture.run(&primary, &["recover", "505"]));
    let rejected = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            rejected_preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!rejected.status.success());
    assert_eq!(fixture.remote_effects(), 0);
    assert_eq!(fs::read_dir(&recoveries).unwrap().count(), 1);

    fixture.remote_flag("reject-pr-create", false);
    let converging = success(fixture.run(&primary, &["recover", "505"]));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            converging["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(
        fixture.remote_pr()["head"]["sha"],
        git(&linked, &["rev-parse", "HEAD"])
    );
    success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(fixture.remote_effects(), 1, "replay duplicated PR creation");
    assert_eq!(fs::read_dir(&recoveries).unwrap().count(), 1);
}

#[test]
fn installed_remote_recover_rejected_reuse_crash_never_dispatches_twice() {
    let (mut fixture, linked) = reviewed_fixture("pr-create-rejected-reuse-crash");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));

    fixture.remote_flag("reject-pr-create", true);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let rejected = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!rejected.status.success());
    assert_eq!(fixture.remote_effects(), 0);

    fixture.remote_flag("reject-pr-create", false);
    fixture.remote_flag("drop-publication-readback", true);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let crashed = fixture.run_with_env(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_recovery_after_native",
        )],
    );
    assert_eq!(crashed.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);

    fixture.remote_flag("drop-publication-readback", false);
    fixture.remote_flag("drop-readback", false);
    fs::remove_file(primary.join(".git/installed-candidate/remote-pr.json")).unwrap();
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let restart = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!restart.status.success());
    assert_eq!(
        fixture.remote_effects(),
        1,
        "old definite rejection authorized a second PR create after ambiguous dispatch"
    );
}

#[test]
fn installed_remote_recover_pr_create_rejects_missing_or_wrong_head_before_reservation() {
    for (name, flag, code) in [
        (
            "pr-create-initial-missing-head",
            "remote-head-present",
            "github_pr_head_branch_missing",
        ),
        (
            "pr-create-initial-wrong-head",
            "remote-head-wrong",
            "github_pr_head_branch_mismatch",
        ),
    ] {
        let (mut fixture, linked) = reviewed_fixture(name);
        fixture.remote_flag(flag, flag == "remote-head-wrong");
        let before = publication_reservation_inventory(&fixture.root);
        let rejected = fixture.run(&linked, &["publish", "505"]);
        assert!(!rejected.status.success());
        assert!(String::from_utf8_lossy(&rejected.stdout).contains(code));
        assert_eq!(fixture.remote_effects(), 0);
        assert_eq!(
            before,
            publication_reservation_inventory(&fixture.root),
            "{name} changed native intent or semantic reservation state"
        );
    }
}

#[test]
fn installed_remote_recover_pr_create_never_reuses_ambiguous_dispatch_or_wrong_branch() {
    let (mut fixture, linked) = reviewed_fixture("pr-create-recovery-ambiguous");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    fixture.remote_flag("drop-publication-readback", true);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let uncertain_output = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!uncertain_output.status.success());
    let uncertain: Value = serde_json::from_slice(&uncertain_output.stdout).unwrap();
    assert_eq!(uncertain["status"], "recovery_required");
    assert_eq!(fixture.remote_effects(), 1);
    fs::remove_file(primary.join(".git/installed-candidate/remote-pr.json")).unwrap();
    fixture.remote_flag("drop-publication-readback", false);
    fixture.remote_flag("drop-readback", false);
    let exhausted = success(fixture.run(&primary, &["recover", "505"]));
    let settled = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            exhausted["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!settled.status.success());
    assert_eq!(
        fixture.remote_effects(),
        1,
        "ambiguous dispatch reused recovery"
    );

    let (mut wrong, wrong_linked) = reviewed_fixture("pr-create-recovery-wrong-head");
    wrong.remote_flag("remote-head-wrong", true);
    let before = publication_reservation_inventory(&wrong.root);
    let rejected = wrong.run(&wrong_linked, &["publish", "505"]);
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("github_pr_head_branch_mismatch"));
    assert_eq!(wrong.remote_effects(), 0);
    assert_eq!(before, publication_reservation_inventory(&wrong.root));
}

#[test]
fn installed_remote_recover_uses_retained_origin_after_bound_head_advances() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-advanced-bound-head");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[("CSDLC_V3_TEST_CRASH_POINT", "semantic_remote_after_native")],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);

    fs::write(
        linked.join("advanced-head.txt"),
        "unrelated later revision\n",
    )
    .unwrap();
    git(&linked, &["add", "advanced-head.txt"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "advance bound head after publication",
        ],
    );

    let before = intent_fixture::inventory(&primary);
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(preview["status"], "recovery_required");
    assert_eq!(preview["action"], "reconcile_retained_remote_effect");
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(fixture.remote_effects(), 1, "recovery replayed publication");
}

#[test]
fn installed_remote_recover_never_retries_when_native_receipt_is_already_durable() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-receipt-drift");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[("CSDLC_V3_TEST_CRASH_POINT", "semantic_remote_after_native")],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);
    fs::remove_file(primary.join(".git/installed-candidate/remote-pr.json")).unwrap();
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let failed = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!failed.status.success());
    assert_eq!(
        fixture.remote_effects(),
        1,
        "durable receipt permitted retry"
    );
}

#[test]
fn installed_remote_recover_rejects_tampered_native_receipt_target_identity() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-receipt-target-tamper");
    let primary = fixture.root.clone();
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[("CSDLC_V3_TEST_CRASH_POINT", "semantic_remote_after_native")],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);
    let mutations = primary.join(".git/csdlc-v3/remote/mutations");
    let receipt_path = fs::read_dir(&mutations)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
        .expect("durable native receipt");
    let mut receipt: Value = serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["issue"] = json!(999);
    fs::write(&receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    let failed = fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!failed.status.success());
    assert!(String::from_utf8_lossy(&failed.stdout).contains("semantic_outcome_identity_mismatch"));
    assert_eq!(fixture.remote_effects(), 1);
}

#[test]
fn installed_remote_recover_reconciles_pending_merge_without_retry_flag() {
    let (mut fixture, linked) = reviewed_fixture("remote-recover-merge-receipt-crash");
    let primary = fixture.root.clone();
    success(fixture.run(&primary, &["publish", "505"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let merge = fixture.write_json(
        "merge-recovery.json",
        &json!({"action":"pull_request_merge","base":"main","method":"merge",
            "operator_approval":"synthetic operator authorizes fixture PR639 exact merge recovery"}),
    );
    let crash = fixture.run_with_env(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
        &[("CSDLC_V3_TEST_CRASH_POINT", "semantic_remote_after_native")],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 3);
    assert_eq!(fixture.remote_pr()["merged"], true);
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    assert_eq!(preview["action"], "reconcile_retained_remote_effect");
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(
        fixture.remote_effects(),
        3,
        "merge recovery replayed effect"
    );
}

#[test]
fn installed_proof_refuses_ignored_configuration_outside_declared_caches() {
    let mut fixture = Fixture::new("ignored-configuration");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let ignore = linked.join(".gitignore");
    let mut rules = fs::read_to_string(&ignore).unwrap();
    rules.push_str("\n/undeclared-config.json\n");
    fs::write(ignore, rules).unwrap();
    git(&linked, &["add", ".gitignore"]);
    git(
        &linked,
        &["commit", "-m", "Declare ignored fixture configuration"],
    );
    fs::write(
        linked.join("undeclared-config.json"),
        b"{\"undeclared_input\":true}",
    )
    .unwrap();
    let before = intent_fixture::inventory(&primary);
    let refused = fixture.run(&linked, &["proof", "505"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stdout).contains("intent_candidate_ignored_source"));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert!(!linked.join("target/intent-validation").exists());
}

#[test]
fn installed_proof_retains_actual_validator_outcome_when_source_changes_during_execution() {
    let mut fixture = Fixture::new("validator-mutates-source");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn actual_test_changes_declared_source() {
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../tracked");
    assert_eq!(std::fs::read_to_string(&source).unwrap(), "fixture\n");
    std::fs::write(source, "changed by actual validator\n").unwrap();
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &["commit", "-m", "Install source-mutating fixture validator"],
    );
    let failed = fixture.run(&linked, &["proof", "505"]);
    assert!(!failed.status.success());
    let result: Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(result["proof"]["status"], "failed");
    assert_eq!(result["proof"]["inputs_unchanged"], false);
    assert_eq!(result["proof"]["validators"][0]["tests_passed"], 1);
    assert_eq!(
        fs::read_to_string(linked.join("tracked")).unwrap(),
        "changed by actual validator\n"
    );
    assert!(result["proof"]["input_revalidation"].is_object());
    assert_eq!(result["evidence_persisted"], true);
    let retained: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap())
            .unwrap();
    assert_eq!(retained["validators"], result["proof"]["validators"]);
    assert_eq!(retained["status"], "failed");
}

#[test]
fn installed_bind_and_validate_refuse_malformed_bound_state_without_repair() {
    let mut fixture = Fixture::new("malformed-bound-observation");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join(".csdlc/issues/505/index.json"),
        b"{malformed native state",
    )
    .unwrap();
    for cwd in [&primary, &linked] {
        for route in ["bind", "validate"] {
            let before = intent_fixture::inventory(&primary);
            assert!(!fixture.run(cwd, &[route, "505"]).status.success());
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
    }
}

#[test]
fn installed_publication_plan_tampering_invalidates_ordinary_and_emitted_requests() {
    for (field, value) in [
        ("title", json!("Unreviewed title")),
        ("body", json!("Unreviewed body\n\nCloses #505")),
        ("base", json!("unreviewed-base")),
        ("draft", json!(false)),
    ] {
        let (mut fixture, linked) = reviewed_fixture(&format!("plan-tampering-{field}"));
        let primary = fixture.root.clone();
        let emitted = fixture.run(&linked, &["publish", "505", "--emit-request"]);
        assert!(emitted.status.success());
        let request = fixture.write_json(
            "emitted-publish.json",
            &serde_json::from_slice::<Value>(&emitted.stdout).unwrap(),
        );
        let index = fs::read(linked.join(".csdlc/issues/505/index.json")).unwrap();
        let semantic = primary.join(".git/csdlc-v3/semantic/issues/505");
        let pointer: Value =
            serde_json::from_slice(&fs::read(semantic.join("current.json")).unwrap()).unwrap();
        let generation = pointer["generation"].as_u64().unwrap();
        let digest = pointer["digest"]
            .as_str()
            .unwrap()
            .rsplit(':')
            .next()
            .unwrap();
        let commit = semantic.join(format!("commits/{generation}-{digest}.json"));
        let mut changed: Value = serde_json::from_slice(&fs::read(&commit).unwrap()).unwrap();
        changed["inputs"]["intent_plan"]["publication"][field] = value;
        fs::write(commit, serde_json::to_vec(&changed).unwrap()).unwrap();
        for args in [
            vec!["publish", "505"],
            vec!["publish", "--intent-request", request.to_str().unwrap()],
        ] {
            let before = intent_fixture::inventory(&primary);
            assert!(
                !fixture.run(&primary, &args).status.success(),
                "unreviewed publication {field} admitted"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
            assert_eq!(fixture.remote_effects(), 0);
            assert_eq!(
                fs::read(linked.join(".csdlc/issues/505/index.json")).unwrap(),
                index
            );
        }
    }
}

#[test]
fn installed_proof_refuses_manifest_and_source_hidden_inside_excluded_evidence() {
    for hidden_manifest in [true, false] {
        let mut fixture = Fixture::new(if hidden_manifest {
            "excluded-manifest"
        } else {
            "excluded-source"
        });
        let primary = fixture.root.clone();
        let mut requested = plan();
        if hidden_manifest {
            requested["validators"][0]["args"][2] = json!(".csdlc/probe/Cargo.toml");
            fs::create_dir_all(primary.join(".csdlc/probe")).unwrap();
            fs::write(
                primary.join(".csdlc/probe/Cargo.toml"),
                "[package]\nname=\"fixture-proof\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[lib]\npath=\"lib.rs\"\n",
            )
            .unwrap();
            fs::write(
                primary.join(".csdlc/probe/lib.rs"),
                "#[test] fn hidden_test() { assert_eq!(2 + 2, 4); }\n",
            )
            .unwrap();
        }
        let plan_file = fixture.write_json("plan.json", &requested);
        let before_prepare = intent_fixture::inventory(&primary);
        let prepared = fixture.run(
            &primary,
            &["prepare", "505", "--plan", plan_file.to_str().unwrap()],
        );
        if hidden_manifest {
            assert!(
                !prepared.status.success(),
                "excluded validator manifest should be rejected during prepare"
            );
            assert!(
                String::from_utf8_lossy(&prepared.stdout)
                    .contains("intent_validator_input_not_tracked"),
                "unexpected prepare refusal for excluded manifest: {prepared:?}"
            );
            assert_same_inventory!(before_prepare, intent_fixture::inventory(&primary));
            continue;
        }
        success(prepared);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
        fs::write(
            linked.join(".csdlc/probe/lib.rs"),
            "#[test] fn hidden_test() { assert_eq!(2 + 2, 4); }\n",
        )
        .unwrap();
        let manifest = if hidden_manifest {
            linked.join(".csdlc/probe/Cargo.toml")
        } else {
            linked.join("fixture-proof/Cargo.toml")
        };
        let libpath = if hidden_manifest {
            "lib.rs"
        } else {
            "../.csdlc/probe/lib.rs"
        };
        fs::write(manifest, format!("[package]\nname=\"fixture-proof\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[lib]\npath=\"{libpath}\"\n")).unwrap();
        if !hidden_manifest {
            git(&linked, &["add", "fixture-proof/Cargo.toml"]);
            git(
                &linked,
                &[
                    "commit",
                    "-m",
                    "Track manifest pointing to excluded fixture source",
                ],
            );
        }
        let before = intent_fixture::inventory(&primary);
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            !output.status.success(),
            "excluded validator input yielded accepted proof"
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("intent_validator_input_not_tracked"),
            "unexpected validator admission refusal: {output:?}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        assert!(!linked.join("target/intent-validation").exists());
    }
}

#[test]
fn installed_proof_retains_failure_for_compiler_discovered_excluded_input() {
    let mut fixture = Fixture::new("compiler-excluded-input");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
    fs::write(
        linked.join(".csdlc/probe/input.txt"),
        "undeclared compile input\n",
    )
    .unwrap();
    fs::write(linked.join("fixture-proof/src/lib.rs"), "#[test]\nfn reads_excluded_compile_input() { assert_eq!(include_str!(\"../../.csdlc/probe/input.txt\"), \"undeclared compile input\\n\"); }\n").unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &[
            "commit",
            "-m",
            "Track compiler-discovered fixture dependency",
        ],
    );
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(
        !output.status.success(),
        "compiler-discovered excluded input yielded current proof"
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["proof"]["status"], "failed");
    assert_eq!(
        value["proof"]["input_revalidation"]["finding"],
        "intent_validator_compiler_input_not_tracked"
    );
    assert_eq!(value["proof"]["validators"][0]["tests_passed"], 1);
    assert_eq!(value["evidence_persisted"], true);
    let before = intent_fixture::inventory(&primary);
    let status = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(status["evidence"]["proof_current"], false);
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
}

#[test]
fn installed_proof_retains_useful_bounded_failure_diagnostics_without_sensitive_lines() {
    let mut fixture = Fixture::new("validator-diagnostics");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join("fixture-proof/src/lib.rs"),
        r#"#[test]
fn diagnostic_failure() {
    eprintln!("authorization: synthetic-private-diagnostic");
    panic!("fixture assertion explains the failure");
}
"#,
    )
    .unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &["commit", "-m", "Track bounded failure diagnostic fixture"],
    );
    let output = fixture.run(&linked, &["proof", "505"]);
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    let validator = &value["proof"]["validators"][0];
    assert_eq!(validator["passed"], false);
    assert_eq!(validator["tests_failed"], 1);
    let diagnostic = &validator["stdout_evidence"];
    assert_eq!(diagnostic["lossless"], false);
    assert!(diagnostic["excerpt"]
        .as_str()
        .unwrap()
        .contains("fixture assertion explains the failure"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("synthetic-private-diagnostic"));
    assert!(diagnostic["redacted_lines"].as_u64().unwrap() > 0);
    assert_eq!(validator["stdout_digest"].as_str().unwrap().len(), 64);
}

#[test]
fn installed_proof_accounts_for_build_scripts_and_nested_automatic_targets() {
    for (name, relative, include_path, build_script) in [
        (
            "build-script",
            "fixture-proof/build.rs",
            "../.csdlc/probe/hidden.rs",
            true,
        ),
        (
            "nested-test",
            "fixture-proof/tests/fixture_nested/main.rs",
            "../../../.csdlc/probe/hidden.rs",
            false,
        ),
        (
            "nested-bin",
            "fixture-proof/src/bin/fixture_nested/main.rs",
            "../../../../.csdlc/probe/hidden.rs",
            false,
        ),
    ] {
        let mut fixture = Fixture::new(&format!("compiler-{name}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        fs::create_dir_all(linked.join(".csdlc/probe")).unwrap();
        fs::write(
            linked.join(".csdlc/probe/hidden.rs"),
            "const EXCLUDED_INPUT: u64 = 4;\n",
        )
        .unwrap();
        let source = linked.join(relative);
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        let body = if build_script {
            format!(
                "include!(\"{include_path}\");\nfn main() {{ assert_eq!(EXCLUDED_INPUT, 4); }}\n"
            )
        } else {
            format!("include!(\"{include_path}\");\nfn main() {{}}\n#[test] fn nested_reads_excluded_input() {{ assert_eq!(EXCLUDED_INPUT, 4); }}\n")
        };
        fs::write(source, body).unwrap();
        git(&linked, &["add", relative]);
        git(
            &linked,
            &[
                "commit",
                "-m",
                "Track compiler target with excluded include",
            ],
        );
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(
            !output.status.success(),
            "{name} excluded dependency yielded accepted proof"
        );
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["proof"]["status"], "failed", "{name}: {value}");
        assert_eq!(
            value["proof"]["input_revalidation"]["finding"],
            "intent_validator_compiler_input_not_tracked",
            "{name}: {value}"
        );
        assert!(
            value["proof"]["validators"][0]["tests_passed"]
                .as_u64()
                .unwrap()
                >= 1,
            "normal tracked library must actually execute"
        );
        assert_eq!(value["evidence_persisted"], true);
        let retained: Value = serde_json::from_slice(
            &fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(retained["validators"], value["proof"]["validators"]);
        assert_eq!(retained["status"], "failed");
    }
}

#[test]
fn installed_proof_refuses_external_workspace_inheritance_and_patch_inputs() {
    for patched in [false, true] {
        let mut fixture = Fixture::new(if patched {
            "external-patch"
        } else {
            "external-inherited"
        });
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        let external = linked.parent().unwrap().join("external-helper");
        fs::create_dir_all(external.join("src")).unwrap();
        fs::write(
            external.join("Cargo.toml"),
            "[package]\nname='external-helper'\nversion='0.1.0'\nedition='2021'\n",
        )
        .unwrap();
        fs::write(external.join("src/lib.rs"), "pub fn value() -> u8 { 1 }\n").unwrap();
        let workspace = if patched {
            "[workspace]\nmembers=['fixture-proof']\n[patch.crates-io]\nexternal-helper={path='../external-helper'}\n"
        } else {
            "[workspace]\nmembers=['fixture-proof']\n[workspace.dependencies]\nexternal-helper={path='../external-helper'}\n"
        };
        fs::write(linked.join("Cargo.toml"), workspace).unwrap();
        let manifest = linked.join("fixture-proof/Cargo.toml");
        let mut content = fs::read_to_string(&manifest).unwrap();
        content.push_str(if patched {
            "\n[dependencies]\nexternal-helper='0.1.0'\n"
        } else {
            "\n[dependencies]\nexternal-helper={workspace=true}\n"
        });
        fs::write(manifest, content).unwrap();
        git(&linked, &["add", "Cargo.toml", "fixture-proof/Cargo.toml"]);
        git(
            &linked,
            &["commit", "-m", "Declare external workspace validator input"],
        );
        let output = fixture.run(&linked, &["proof", "505"]);
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            value
                .to_string()
                .contains("intent_validator_input_outside_repository"),
            "{value}"
        );
        assert!(
            !linked.join("target/intent-validation").exists(),
            "admission must reject before executing validator"
        );
    }
}

// PVF: #1048 deterministic installed tooling regression; synthetic GitHub,
// local CPU/Git only, required issue proof. No live cloud or GitHub writes.
#[test]
fn publication_amendment_rejects_malformed_preparation_before_state_or_dispatch() {
    for (field, value) in [
        ("body", "Summary. Closes #505"),
        ("body", "Closes #506"),
        ("body", "Closes #505\nCloses #506"),
        ("base", "--bad"),
        ("base", "main..bad"),
        ("title", " "),
    ] {
        let mut fixture = Fixture::new("publication-invalid-prepare");
        let primary = fixture.root.clone();
        let mut input = plan();
        input["publication"][field] = json!(value);
        let input = fixture.write_json("invalid-plan.json", &input);
        let before = publication_reservation_inventory(&primary);
        let output = fixture.run(
            &primary,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(if field == "body" {
                "intent_publication_body_invalid"
            } else {
                "intent_publication_metadata_invalid"
            })
        );
        assert_same_inventory!(before, publication_reservation_inventory(&primary));
        assert_eq!(fixture.remote_effects(), 0);
    }
}

#[test]
fn publication_amendment_prepared_and_bound_preserve_identity_and_are_idempotent() {
    let mut fixture = Fixture::new("publication-ready-bound");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    for bound in [false, true] {
        if bound {
            success(fixture.run(&primary, &["bind", "505"]));
        }
        let cwd = if bound {
            linked_worktree(&primary)
        } else {
            primary.clone()
        };
        let mut publication = plan()["publication"].clone();
        publication["title"] = json!(if bound {
            "Corrected bound title"
        } else {
            "Corrected prepared title"
        });
        let changes = fixture.write_json(
            "publication-change.json",
            &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
        );
        success(fixture.run(
            &cwd,
            &["edit", "505", "--changes", changes.to_str().unwrap()],
        ));
        let before = publication_reservation_inventory(&primary);
        let replay = success(fixture.run(
            &cwd,
            &["edit", "505", "--changes", changes.to_str().unwrap()],
        ));
        assert_eq!(replay["status"], "expected_noop");
        assert_same_inventory!(before, publication_reservation_inventory(&primary));
        assert_eq!(fixture.remote_effects(), 0);
    }
}

#[test]
fn publication_metadata_preserves_proof_review_before_single_dispatch() {
    let (mut fixture, linked) = reviewed_fixture("publication-reviewed-amendment");
    let primary = fixture.root.clone();
    let state_path = linked.join(".csdlc/v3/issues/505/state.json");
    let before: Value = serde_json::from_slice(&fs::read(&state_path).unwrap()).unwrap();
    let proof_before = fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap();
    let review_before = external_review(&linked);
    let mut publication = plan()["publication"].clone();
    publication["body"] = json!("Corrected metadata.\n\nCloses #505");
    let changes = fixture.write_json(
        "publication-change.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    let after: Value = serde_json::from_slice(&fs::read(&state_path).unwrap()).unwrap();
    assert_eq!(before["inputs"]["binding"], after["inputs"]["binding"]);
    assert_ne!(before["input_version"], after["input_version"]);
    // No proof or review command between the metadata edit and publication.
    assert_eq!(
        fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap(),
        proof_before
    );
    assert_eq!(external_review(&linked), review_before);
    assert_eq!(fixture.remote_effects(), 0);
    let published = success(fixture.run(&linked, &["publish", "505"]));
    assert_eq!(published["status"], "completed");
    assert_eq!(fixture.remote_effects(), 1);
    // Even after another proof downgrades phase, published topology is immutable.
    success(fixture.run(&linked, &["proof", "505"]));
    publication["base"] = json!("other-base");
    let bad = fixture.write_json(
        "bad-topology.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    let before = publication_reservation_inventory(&primary);
    assert!(!fixture
        .run(
            &linked,
            &["edit", "505", "--changes", bad.to_str().unwrap()]
        )
        .status
        .success());
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    assert_eq!(fixture.remote_effects(), 1);
}

#[test]
fn publication_continues_after_committed_metadata_edit_without_recovery() {
    let (mut fixture, linked) = reviewed_fixture("publication-recovery");
    let mut publication = plan()["publication"].clone();
    publication["title"] = json!("Recovered metadata");
    let changes = fixture.write_json(
        "publication-change.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    let crash = fixture.run_with_env(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_local_amendment_after_commit",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    // The authoritative amendment already committed. Stale views are not a
    // reason to repeat proof/review or run an operator recovery ceremony.
    let proof = fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap();
    success(fixture.run(&linked, &["publish", "505"]));
    assert_eq!(
        fs::read(linked.join(".csdlc/v3/issues/505/proof.json")).unwrap(),
        proof
    );
    assert_eq!(fixture.remote_effects(), 1);
}

// PVF #1048: deterministic installed compatibility and negative-effect regression.
// Local Git / synthetic transport only; required issue proof, no live writes.
#[test]
fn issue_1048_review_legacy_retention_publishes_with_existing_receipt() {
    let (mut fixture, linked) = reviewed_fixture("1048-legacy-review");
    let review = external_review(&linked);
    let head = git(&linked, &["rev-parse", "HEAD"]);
    let current = linked.join(csdlc_v3::commands::remote::intent::proof_review_path(
        505,
        &head,
        review["proof_digest"].as_str().unwrap(),
    ));
    let legacy = linked.join(csdlc_v3::commands::remote::intent::review_path(505, &head));
    // Model an existing installation's immutable per-HEAD review layout.
    let combined: Value = serde_json::from_slice(&fs::read(&current).unwrap()).unwrap();
    let packet = &combined["external_review"];
    fs::write(&legacy, serde_json::to_vec(&packet["receipt"]).unwrap()).unwrap();
    fs::write(
        legacy.with_extension("external.json"),
        serde_json::to_vec(packet).unwrap(),
    )
    .unwrap();
    fs::remove_file(&current).unwrap();
    assert!(!current.exists());
    let published = success(fixture.run(&linked, &["publish", "505"]));
    assert_eq!(published["status"], "completed");
    assert_eq!(fixture.remote_effects(), 1);
    assert!(legacy.is_file());
    assert!(
        !current.exists(),
        "publication must not rewrite retained review layout"
    );
}

#[test]
fn issue_1048_review_pending_publication_denies_topology_correction() {
    let (mut fixture, linked) = reviewed_fixture("1048-pending-publication");
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 0);
    let unchanged = fixture.write_json(
        "pending-unchanged-publication.json",
        &json!({
            "schema":"csdlc.v3.intent_changes.v1", "publication":plan()["publication"]
        }),
    );
    let unchanged_before = publication_reservation_inventory(&fixture.root);
    let unchanged_result = fixture.run(
        &linked,
        &["edit", "505", "--changes", unchanged.to_str().unwrap()],
    );
    assert!(
        !unchanged_result.status.success(),
        "pending no-op was accepted: {unchanged_result:?}"
    );
    assert!(String::from_utf8_lossy(&unchanged_result.stdout).contains("PendingOperation"));
    assert_same_inventory!(
        unchanged_before,
        publication_reservation_inventory(&fixture.root)
    );
    let mut publication = plan()["publication"].clone();
    publication["base"] = json!("corrected-base");
    publication["draft"] = json!(false);
    let changes = fixture.write_json(
        "pending-publication-change.json",
        &json!({
        "schema":"csdlc.v3.intent_changes.v1", "publication":publication}),
    );
    let before = publication_reservation_inventory(&fixture.root);
    let denied = fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    );
    assert!(!denied.status.success());
    assert!(
        String::from_utf8_lossy(&denied.stdout).contains("PendingOperation"),
        "{denied:?}"
    );
    assert_same_inventory!(before, publication_reservation_inventory(&fixture.root));
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn publication_amendment_rejects_invalid_edits_without_reservation() {
    let mut fixture = Fixture::new("publication-edit-invalid");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let branch = git(&linked, &["symbolic-ref", "--short", "HEAD"]);
    let before = publication_reservation_inventory(&primary);
    for (field, value) in [
        ("body", "Summary. Closes #505"),
        ("body", "Closes #506"),
        ("base", branch.as_str()),
        ("title", "bad\ntitle"),
    ] {
        let mut publication = plan()["publication"].clone();
        publication[field] = json!(value);
        let changes = fixture.write_json(
            "invalid-edit.json",
            &json!({"schema":"csdlc.v3.intent_changes.v1", "publication":publication}),
        );
        let output = fixture.run(
            &linked,
            &["edit", "505", "--changes", changes.to_str().unwrap()],
        );
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(if field == "body" {
                "intent_publication_body_invalid"
            } else {
                "intent_publication_metadata_invalid"
            })
        );
        assert_same_inventory!(before.clone(), publication_reservation_inventory(&primary));
    }
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn publication_amendment_prepared_recovery_keeps_planned_branch_identity() {
    let mut fixture = Fixture::new("publication-prepared-recovery");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    let mut publication = plan()["publication"].clone();
    publication["title"] = json!("Prepared recovery");
    let changes = fixture.write_json(
        "publication-change.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1", "publication":publication}),
    );
    let crash = fixture.run_with_env(
        &primary,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_local_amendment_after_commit",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    let preview = success(fixture.run(&primary, &["recover", "505"]));
    success(fixture.run(
        &primary,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let state: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    assert_eq!(
        state["inputs"]["intent_plan"]["publication"]["title"],
        "Prepared recovery"
    );
    assert_eq!(fixture.remote_effects(), 0);
}

// PVF: #1046 installed owner contract, deterministic Git/filesystem and synthetic
// transport; small local CPU/disk, required tooling gate, no live service proof.
#[test]
fn issue_1046_installed_publication_prepare_and_amendment_guards() {
    let mut fixture = Fixture::new("publication-plan-amendment");
    let primary = fixture.root.clone();
    for body in ["Description. Closes #505", "Closes #505\nFixes #506"] {
        let mut invalid = plan();
        invalid["publication"]["body"] = body.into();
        let input = fixture.write_json("invalid-publication.json", &invalid);
        let before = intent_fixture::inventory(&primary);
        let result = fixture.run(
            &primary,
            &["prepare", "505", "--plan", input.to_str().unwrap()],
        );
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stdout).contains("intent_publication_body_invalid"));
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let mut publication = plan()["publication"].clone();
    publication["body"] = "Closes #505\n\nRepaired body".into();
    let changes = fixture.write_json(
        "publication-changes.json",
        &json!({
            "schema":"csdlc.v3.intent_changes.v1", "publication":publication
        }),
    );
    let emitted = success(fixture.run(
        &linked,
        &[
            "edit",
            "505",
            "--changes",
            changes.to_str().unwrap(),
            "--emit-request",
        ],
    ));
    let stale = fixture.write_json("stale-publication.json", &emitted["request"]);
    let edited = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(edited["publication_amended"], true);
    let root =
        SemanticRoot::from_git_common(primary.join(".git"), "agent-logic/agent-design-language")
            .unwrap();
    let key = IssueKey::new("agent-logic/agent-design-language", 505).unwrap();
    let Observation::Current(snapshot) =
        DurableTransactionStore::observe_issue(&root, &key).unwrap()
    else {
        panic!("missing semantic state")
    };
    assert_eq!(
        snapshot.inputs().publication().body,
        "Closes #505\n\nRepaired body"
    );
    let before = intent_fixture::inventory(&primary);
    assert!(!fixture
        .run(
            &linked,
            &["edit", "--intent-request", stale.to_str().unwrap()]
        )
        .status
        .success());
    assert_same_inventory!(before, intent_fixture::inventory(&primary));

    let repeated = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(repeated["status"], "expected_noop");
    for extra in [
        json!({"validators":[]}),
        json!({"cards":{"sor":{"summary":"mixed"}}}),
    ] {
        let mut mixed = json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication});
        mixed
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        let input = fixture.write_json("mixed-publication.json", &mixed);
        let before = intent_fixture::inventory(&primary);
        let result = fixture.run(
            &linked,
            &["edit", "505", "--changes", input.to_str().unwrap()],
        );
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stdout)
            .contains("intent_changes_single_surface_required"));
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
}

// PVF: deterministic installed local tooling; no remote effects, required #1048 regression.
#[test]
fn publication_amendment_saved_request_cannot_overwrite_newer_correction() {
    for bound in [false, true] {
        let mut fixture = Fixture::new("publication-stale-version");
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        let cwd = if bound {
            success(fixture.run(&primary, &["bind", "505"]));
            linked_worktree(&primary)
        } else {
            primary.clone()
        };
        let mut publication = plan()["publication"].clone();
        publication["title"] = json!("Saved correction A");
        let changes = fixture.write_json(
            "correction-a.json",
            &json!({
                "schema":"csdlc.v3.intent_changes.v1", "publication":publication
            }),
        );
        let emitted = success(fixture.run(
            &cwd,
            &[
                "edit",
                "505",
                "--changes",
                changes.to_str().unwrap(),
                "--emit-request",
            ],
        ));
        publication["title"] = json!("Current correction B");
        let newer = fixture.write_json(
            "correction-b.json",
            &json!({
                "schema":"csdlc.v3.intent_changes.v1", "publication":publication
            }),
        );
        success(fixture.run(&cwd, &["edit", "505", "--changes", newer.to_str().unwrap()]));
        let current = success(fixture.run(
            &cwd,
            &[
                "edit",
                "505",
                "--changes",
                newer.to_str().unwrap(),
                "--emit-request",
            ],
        ));
        // Reject stale A, even if its content is changed to the current value,
        // and reject a missing semantic version rather than inventing one.
        for mode in ["stale", "stale-noop", "missing-version"] {
            let mut saved = emitted["request"].clone();
            if mode == "stale-noop" {
                saved["content"]["publication"] = publication.clone();
            }
            if mode == "missing-version" {
                saved = current["request"].clone();
                saved["snapshot"]
                    .as_object_mut()
                    .unwrap()
                    .remove("semantic_version");
            }
            let saved = fixture.write_json(&format!("{mode}.json"), &saved);
            let before = intent_fixture::inventory(&primary);
            let output = fixture.run(&cwd, &["edit", "--intent-request", saved.to_str().unwrap()]);
            assert!(!output.status.success(), "{mode}: {output:?}");
            assert!(
                String::from_utf8_lossy(&output.stdout)
                    .contains("intent_publication_stale_semantic_version")
                    || String::from_utf8_lossy(&output.stdout).contains("intent_snapshot_stale"),
                "{mode}: {output:?}"
            );
            assert_same_inventory!(before, intent_fixture::inventory(&primary));
        }
        let root = SemanticRoot::from_git_common(
            primary.join(".git"),
            "agent-logic/agent-design-language",
        )
        .unwrap();
        let key = IssueKey::new("agent-logic/agent-design-language", 505).unwrap();
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
            Observation::Current(s) | Observation::ProjectionRepairRequired(s) => s,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            snapshot.inputs().publication().title,
            "Current correction B"
        );
        assert_eq!(fixture.remote_effects(), 0);
    }
}

// PVF #1068: deterministic installed terminal qualification, same-host local Git
// and synthetic authenticated transport; required repair gate, no live writes.
#[test]
fn external_pr_finish_authenticates_without_fabricating_publication_history() {
    let (mut fixture, linked) = reviewed_fixture("external-pr-finish");
    let primary = fixture.root.clone();
    let head = git(&linked, &["rev-parse", "HEAD"]);
    let pr = json!({"number":639,"head":{"sha":head},"merged":true,"state":"closed","body":"Closes #505"});
    fixture.set_remote_pr(&pr);
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    for (field, bad, reason) in [
        (
            "head",
            json!({"sha":"0000000000000000000000000000000000000000"}),
            "head_mismatch",
        ),
        ("merged", json!(false), "pull_request_not_merged"),
        ("body", json!("Closes #506"), "closing_linkage_missing"),
        ("number", json!(640), "github_observation_pr_mismatch"),
    ] {
        let mut invalid = pr.clone();
        invalid[field] = bad;
        fixture.set_remote_pr(&invalid);
        let before = intent_fixture::inventory(&primary);
        let denied = fixture.run(&linked, &["finish", "505", "--pull-request", "639"]);
        assert!(!denied.status.success());
        assert!(
            String::from_utf8_lossy(&denied.stdout).contains(reason),
            "{denied:?}"
        );
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
    fixture.set_remote_pr(&pr);
    issue["state"] = json!("open");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    let before = intent_fixture::inventory(&primary);
    let denied = fixture.run(&linked, &["finish", "505", "--pull-request", "639"]);
    assert!(String::from_utf8_lossy(&denied.stdout).contains("closing_issue_still_open"));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    let before = intent_fixture::inventory(&primary);
    success(fixture.run(
        &linked,
        &[
            "finish",
            "505",
            "--pull-request",
            "639",
            "--preview",
            "plan",
        ],
    ));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    success(fixture.run(&linked, &["finish", "505", "--pull-request", "639"]));
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["pull_request"], 639);
    assert_eq!(receipt["head_sha"], head);
    assert_eq!(fixture.remote_effects(), 0);
    let before = intent_fixture::inventory(&primary);
    success(fixture.run(&linked, &["finish", "505"]));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let denied = fixture.run(&linked, &["finish", "505", "--pull-request", "640"]);
    assert!(String::from_utf8_lossy(&denied.stdout).contains("intent_finish_pull_request_conflict"));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let native_intents = primary.join(".git/csdlc-v3/remote/intents");
    assert!(!native_intents.exists() || fs::read_dir(native_intents).unwrap().next().is_none());
}

// PVF #1090: deterministic installed terminal reconciliation and replay for a retained
// merged checkpoint followed by the actual exact-head closing PR.
#[test]
fn issue_1090_installed_finish_replays_checkpoint_then_closing_pr() {
    use csdlc_v3::commands::remote::{
        canonical_authority_selector_digest, github_mutation_operation_digest,
        github_mutation_operation_marker, GithubMutation, GithubMutationRequest,
    };

    fn stable_digest(values: &[&str]) -> String {
        let mut hasher = blake3::Hasher::new();
        for value in values {
            hasher.update(value.as_bytes());
            hasher.update(b"\0");
        }
        hasher.finalize().to_hex().to_string()
    }

    let mut fixture = Fixture::new("issue-1083-checkpoint-then-close");
    let primary = fixture.root.clone();
    let input = fixture.write_json("checkpoint-plan.json", &plan());
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["proof", "505"]));
    let review = fixture.write_json("checkpoint-review.json", &external_review(&linked));
    success(fixture.run(
        &linked,
        &["review", "505", "--evidence", review.to_str().unwrap()],
    ));
    fixture.enable_pr_transport(&linked);
    success(fixture.run(&linked, &["publish", "505"]));

    let head = git(&linked, &["rev-parse", "HEAD"]);
    let second_request = GithubMutationRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: Some(638),
        cutover_issue: None,
        operator_approval: None,
        expected_head_sha: head.clone(),
        credential_names: vec!["GITHUB_TOKEN".into()],
        recovery: None,
        mutation: GithubMutation::PullRequestReady,
    };
    let second_operation = github_mutation_operation_digest(&second_request);
    let second_marker = github_mutation_operation_marker(&second_operation);
    let selector = canonical_authority_selector_digest(&primary).unwrap();
    let schema = "csdlc.v3.github_mutation_intent.v1";
    let adapter = "github-api-operational";
    let second_intent_digest = stable_digest(&[
        schema,
        &second_operation,
        &second_marker,
        &selector,
        adapter,
    ]);
    let intents = primary.join(".git/csdlc-v3/remote/intents");
    let mutations = primary.join(".git/csdlc-v3/remote/mutations");
    fs::create_dir_all(&intents).unwrap();
    fs::create_dir_all(&mutations).unwrap();
    fs::write(
        intents.join(format!("{second_operation}.json")),
        serde_json::to_vec(&json!({
            "schema":schema,"operation_digest":second_operation,
            "operation_marker":second_marker,"authority_selector_digest":selector,
            "request":second_request,"adapter":adapter
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        mutations.join(format!("{second_operation}.json")),
        serde_json::to_vec(&json!({
            "schema":"csdlc.v3.github_mutation_receipt.v2",
            "repository":"agent-logic/agent-design-language","issue":505,
            "pull_request":638,"expected_head_sha":head,
            "operation_digest":second_operation,"response_digest":"fixture-response",
            "readback_digest":"fixture-readback","intent_digest":second_intent_digest,
            "reconciliation_digest":"fixture-reconciliation","adapter":adapter,
            "authenticated":true,"idempotent_replay":false
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(
        primary.join(".git/installed-candidate/remote-pr-638.json"),
        serde_json::to_vec(&json!({
            "number":638,"head":{"sha":head},"merged":true,"state":"closed",
            "body":"Earlier checkpoint delivery.\n\nPart of #505"
        }))
        .unwrap(),
    )
    .unwrap();
    fixture.set_remote_pr(&json!({
        "number":639,"head":{"sha":head},"merged":true,"state":"closed",
        "body":"Checkpoint delivery.\n\nPart of #505"
    }));
    fs::write(
        primary.join(".git/installed-candidate/remote-pr-640.json"),
        serde_json::to_vec(&json!({
            "number":640,"head":{"sha":head},"merged":true,"state":"closed",
            "body":"Closes #505"
        }))
        .unwrap(),
    )
    .unwrap();
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();
    fixture.remote_flag("wrong-branch-head", true);

    let effects = fixture.remote_effects();
    fixture.set_remote_pr(&json!({
        "number":639,"head":{"sha":head},"merged":true,"state":"closed",
        "body":"Closes #505"
    }));
    let denied = fixture.run(&linked, &["finish", "505", "--pull-request", "640"]);
    assert!(!denied.status.success());
    assert!(
        String::from_utf8_lossy(&denied.stdout).contains("historical_pull_request_not_checkpoint")
    );
    assert_eq!(fixture.remote_effects(), effects);
    assert!(!primary
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .exists());
    fixture.set_remote_pr(&json!({
        "number":639,"head":{"sha":head},"merged":true,"state":"closed",
        "body":"Checkpoint delivery.\n\nPart of #505"
    }));
    let finished = success(fixture.run(&linked, &["finish", "505", "--pull-request", "640"]));
    assert_eq!(finished["status"], "completed");
    assert_eq!(
        fixture.remote_effects(),
        effects,
        "finish must be read-only remotely"
    );
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["pull_request"], 640);
    assert_eq!(receipt["head_sha"], head);
    let before_replay = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(&linked, &["finish", "505"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_same_inventory!(before_replay, intent_fixture::inventory(&primary));
}

// PVF #1090: deterministic installed reconciliation and replay for an obsolete absent
// create candidate followed by an independently authenticated exact-head PR.
#[test]
fn issue_1090_installed_finish_replays_authenticated_absent_stale_create() {
    use csdlc_v3::commands::remote::{
        canonical_authority_selector_digest, github_mutation_operation_digest,
        github_mutation_operation_marker, GithubMutation, GithubMutationRequest,
    };

    fn stable_digest(values: &[&str]) -> String {
        let mut hasher = blake3::Hasher::new();
        for value in values {
            hasher.update(value.as_bytes());
            hasher.update(b"\0");
        }
        hasher.finalize().to_hex().to_string()
    }

    let (mut fixture, linked) = reviewed_fixture("issue-1083-stale-create-final-pr");
    let primary = fixture.root.clone();
    let head = git(&linked, &["rev-parse", "HEAD"]);
    let branch = git(&linked, &["symbolic-ref", "--short", "HEAD"]);
    let stale_head = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    assert_ne!(head, stale_head);
    let request = GithubMutationRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: None,
        cutover_issue: None,
        operator_approval: None,
        expected_head_sha: stale_head.into(),
        credential_names: vec!["GITHUB_TOKEN".into()],
        recovery: None,
        mutation: GithubMutation::PullRequestCreate {
            base: "main".into(),
            head: branch,
            title: "Obsolete publication candidate".into(),
            body: "Closes #505".into(),
            draft: true,
        },
    };
    let operation = github_mutation_operation_digest(&request);
    let marker = github_mutation_operation_marker(&operation);
    let selector = canonical_authority_selector_digest(&primary).unwrap();
    let schema = "csdlc.v3.github_mutation_intent.v1";
    let adapter = "github-api-operational";
    let intent_digest = stable_digest(&[schema, &operation, &marker, &selector, adapter]);
    let intents = primary.join(".git/csdlc-v3/remote/intents");
    let recoveries = primary.join(".git/csdlc-v3/remote/recoveries");
    fs::create_dir_all(&intents).unwrap();
    fs::create_dir_all(&recoveries).unwrap();
    fs::write(
        intents.join(format!("{operation}.json")),
        serde_json::to_vec(&json!({
            "schema":schema,"operation_digest":operation,
            "operation_marker":marker,"authority_selector_digest":selector,
            "request":request,"adapter":adapter
        }))
        .unwrap(),
    )
    .unwrap();
    let recovery_path = recoveries.join(format!("{operation}.json"));
    let mut recovery = json!({
        "schema":"csdlc.v3.github_mutation_recovery.v1",
        "operation_digest":operation,"intent_digest":"0".repeat(64),
        "recovery":"retry_after_authenticated_absence",
        "repository":"agent-logic/agent-design-language","issue":505,
        "pull_request":null,"expected_head_sha":stale_head
    });
    fs::write(&recovery_path, serde_json::to_vec(&recovery).unwrap()).unwrap();

    fs::write(
        primary.join(".git/installed-candidate/remote-pr-640.json"),
        serde_json::to_vec(&json!({
            "number":640,"head":{"sha":head},"merged":true,"state":"closed",
            "body":"Closes #505"
        }))
        .unwrap(),
    )
    .unwrap();
    let issue_path = primary.join(".git/installed-candidate/remote-issue.json");
    let mut issue = fixture.remote_issue();
    issue["state"] = json!("closed");
    fs::write(&issue_path, serde_json::to_vec(&issue).unwrap()).unwrap();

    let denied = fixture.run(&linked, &["finish", "505", "--pull-request", "640"]);
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stdout).contains("intent_publication_uncertain_head"));
    assert!(!primary
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .exists());
    recovery["intent_digest"] = json!(intent_digest);
    fs::write(&recovery_path, serde_json::to_vec(&recovery).unwrap()).unwrap();

    let finished = success(fixture.run(&linked, &["finish", "505", "--pull-request", "640"]));
    assert_eq!(finished["status"], "completed");
    let receipt: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["pull_request"], 640);
    assert_eq!(receipt["head_sha"], head);
    assert_eq!(fixture.remote_effects(), 0);
    let before_replay = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(&linked, &["finish", "505"]));
    assert_eq!(replay["status"], "expected_noop");
    assert_same_inventory!(before_replay, intent_fixture::inventory(&primary));
}

#[test]
fn external_pr_finish_rejects_conflicting_native_target_and_mixed_inputs() {
    let (mut fixture, linked) = reviewed_fixture("external-pr-conflict");
    let primary = fixture.root.clone();
    success(fixture.run(&primary, &["publish", "505"]));
    let disposition = fixture.write_json(
        "unused-disposition.json",
        &json!({"disposition":"retired_without_execution"}),
    );
    for args in [
        vec!["finish", "505", "--pull-request", "640"],
        vec!["finish", "505", "--pull-request", "0"],
        vec![
            "finish",
            "505",
            "--pull-request",
            "639",
            "--disposition",
            disposition.to_str().unwrap(),
        ],
        vec![
            "finish",
            "505",
            "--disposition",
            disposition.to_str().unwrap(),
            "--pull-request",
            "639",
        ],
        vec!["clean", "505", "--pull-request", "639"],
    ] {
        let before = intent_fixture::inventory(&primary);
        let denied = fixture.run(&linked, &args);
        assert!(!denied.status.success(), "{denied:?}");
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
    }
    assert_eq!(fixture.remote_effects(), 1);
}

// PVF #1064: deterministic installed integration, local Git and fake transport;
// small CPU/disk, required simplification gate. No live external calls.
#[test]
fn compact_review_retains_judgment_and_derives_receipts() {
    let mut fixture = Fixture::new("compact-review");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["proof", "505"]));
    let judgment = json!({
        "schema":"csdlc.v3.review_judgment.v1",
        "implementer":"fixture-author",
        "reviewer":"fixture-independent-reviewer",
        "reviewed_revision":git(&linked, &["rev-parse","HEAD"]),
        "verdict":"pass",
        "evidence":"Synthetic fixture judgment: inspected exact candidate and regression evidence; no findings."
    });
    for defect in ["same-principal", "stale-head", "failed", "empty"] {
        let mut bad = judgment.clone();
        match defect {
            "same-principal" => bad["reviewer"] = bad["implementer"].clone(),
            "stale-head" => bad["reviewed_revision"] = json!("0".repeat(40)),
            "failed" => bad["verdict"] = json!("fail"),
            "empty" => bad["evidence"] = json!(" "),
            _ => unreachable!(),
        }
        let input = fixture.write_json(&format!("{defect}.json"), &bad);
        let before = publication_reservation_inventory(&primary);
        assert!(!fixture
            .run(
                &linked,
                &["review", "505", "--evidence", input.to_str().unwrap()]
            )
            .status
            .success());
        assert_same_inventory!(before, publication_reservation_inventory(&primary));
    }
    let input = fixture.write_json("judgment.json", &judgment);
    let result = success(fixture.run(
        &linked,
        &["review", "505", "--evidence", input.to_str().unwrap()],
    ));
    let retained =
        fs::read_to_string(linked.join(result["review_receipt_path"].as_str().unwrap())).unwrap();
    assert!(retained.contains(judgment["evidence"].as_str().unwrap()));
    assert!(!linked
        .join(result["review_receipt_path"].as_str().unwrap())
        .with_extension("external.json")
        .exists());
    fixture.enable_pr_transport(&linked);
    success(fixture.run(&linked, &["publish", "505"]));
    assert_eq!(fixture.remote_effects(), 1);
}

#[test]
fn local_amendment_ignores_stale_view_without_native_edit_receipts() {
    let mut fixture = Fixture::new("local-amendment-single-owner");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let native_before = intent_fixture::inventory(&linked.join(".csdlc/issues/505"));
    let receipts_before = intent_fixture::inventory(&linked.join(".csdlc/transactions"));
    fs::write(
        linked.join(".csdlc/v3/issues/505/cards/sip.md"),
        "stale view",
    )
    .unwrap();
    let input = fixture.write_json(
        "amend.json",
        &json!({
            "schema":"csdlc.v3.intent_changes.v1",
            "cards":{"sip":{"title":"Single semantic authority"}},
            "amendment":{"class":"scope_acceptance","transition_approved":true}
        }),
    );
    let result = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", input.to_str().unwrap()],
    ));
    assert_eq!(result["local_transaction"], true);
    assert_same_inventory!(
        native_before,
        intent_fixture::inventory(&linked.join(".csdlc/issues/505"))
    );
    assert_same_inventory!(
        receipts_before,
        intent_fixture::inventory(&linked.join(".csdlc/transactions"))
    );
    let rendered = fs::read_to_string(linked.join(".csdlc/v3/issues/505/cards/sip.md")).unwrap();
    assert!(rendered.contains("Single semantic authority"));
    success(fixture.run(&linked, &["bind", "505"]));
    success(fixture.run(&linked, &["proof", "505"]));
}

// PVF #1064: evidence reuse excludes presentation only. Scope and validation
// changes still block publication even at the same source revision.
#[test]
fn candidate_changes_do_not_reuse_metadata_preserved_review() {
    for change_kind in ["scope", "validators"] {
        let (mut fixture, linked) = reviewed_fixture(&format!("evidence-scope-{change_kind}"));
        let changes = if change_kind == "scope" {
            json!({"schema":"csdlc.v3.intent_changes.v1",
                "amendment":{"class":"scope_acceptance","transition_approved":true},
                "cards":{"sip":{"title":"Changed acceptance scope"}}})
        } else {
            let mut validators = plan()["validators"].clone();
            let timeout = validators[0]["timeout_seconds"].as_u64().unwrap_or(300);
            validators[0]["timeout_seconds"] = json!(timeout - 1);
            json!({"schema":"csdlc.v3.intent_changes.v1","validators":validators})
        };
        let input = fixture.write_json("candidate-change.json", &changes);
        success(fixture.run(
            &linked,
            &["edit", "505", "--changes", input.to_str().unwrap()],
        ));
        let result = fixture.run(&linked, &["publish", "505"]);
        assert!(
            !result.status.success(),
            "changed {change_kind} reused old evidence"
        );
        assert!(
            String::from_utf8_lossy(&result.stdout).contains("proof_not_current"),
            "{result:?}"
        );
        assert_eq!(fixture.remote_effects(), 0);
    }
}

// PVF #1064: same deterministic delivery scenario for baseline and candidate.
// This scenario is synthetic; authentic-record conversion has a separate census.
fn complete_simplification_journey(baseline: Option<&Path>) -> Value {
    let mut fixture = Fixture::new(if baseline.is_some() {
        "1064-baseline"
    } else {
        "1064-candidate"
    });
    if let Some(binary) = baseline {
        fixture.select_baseline_binary(binary, "e9d2429bd20eedf33b5efacf1d0c0c5128993e3e");
    }
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    fs::write(
        linked.join(".csdlc/v3/issues/505/cards/sip.md"),
        "stale generated view",
    )
    .unwrap();
    let edit = fixture.write_json(
        "scope.json",
        &json!({
            "schema":"csdlc.v3.intent_changes.v1",
            "amendment":{"class":"scope_acceptance","transition_approved":true},
            "cards":{"sip":{"title":"Qualified complete delivery"}}
        }),
    );
    let edited = fixture.run(
        &linked,
        &["edit", "505", "--changes", edit.to_str().unwrap()],
    );
    if baseline.is_some() {
        assert!(!edited.status.success());
        success(fixture.run(&linked, &["rebuild", "505"]));
        success(fixture.run(
            &linked,
            &["edit", "505", "--changes", edit.to_str().unwrap()],
        ));
        success(fixture.run(&linked, &["bind", "505"]));
    } else {
        success(edited);
    }
    success(fixture.run(&linked, &["proof", "505"]));
    let judgment = json!({"schema":"csdlc.v3.review_judgment.v1",
        "implementer":"synthetic-fixture-author","reviewer":"synthetic-independent-fixture-reviewer",
        "reviewed_revision":git(&linked, &["rev-parse","HEAD"]),"verdict":"pass",
        "evidence":"Synthetic exact-candidate fixture judgment; no production approval."});
    let review = fixture.write_json(
        "review.json",
        &if baseline.is_some() {
            external_review(&linked)
        } else {
            judgment
        },
    );
    success(fixture.run(
        &linked,
        &["review", "505", "--evidence", review.to_str().unwrap()],
    ));
    let mut publication = plan()["publication"].clone();
    publication["body"] = json!("Corrected presentation.\n\nCloses #505");
    let correction = fixture.write_json(
        "metadata.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    success(fixture.run(
        &linked,
        &["edit", "505", "--changes", correction.to_str().unwrap()],
    ));
    if baseline.is_some() {
        success(fixture.run(&linked, &["proof", "505"]));
        let renewed = fixture.write_json("renewed.json", &external_review(&linked));
        success(fixture.run(
            &linked,
            &["review", "505", "--evidence", renewed.to_str().unwrap()],
        ));
    }
    fixture.enable_pr_transport(&linked);
    success(fixture.run(&linked, &["publish", "505"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let merge = fixture.write_json(
        "merge.json",
        &json!({"action":"pull_request_merge","base":"main","method":"merge",
        "operator_approval":"Synthetic operator approval for isolated PR639 only"}),
    );
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    success(fixture.run(&linked, &["finish", "505"]));
    let clean = success(fixture.run(&primary, &["clean", "505"]));
    success(fixture.run(
        &primary,
        &[
            "clean",
            "505",
            "--execute",
            "--preview",
            clean["preview_token"].as_str().unwrap(),
        ],
    ));
    assert!(!linked.exists());
    assert_eq!(fixture.remote_effects(), 3);
    assert!(primary
        .join(".git/csdlc-v3/local/evidence/505/terminal-receipt.json")
        .is_file());
    let reviews = fixture
        .attempts
        .iter()
        .filter(|v| v["argv"][0] == "review")
        .count();
    let failures = fixture
        .attempts
        .iter()
        .filter(|v| v["exit_code"] != 0)
        .count();
    json!({"attempts":fixture.attempts.len(),"reviews":reviews,"failures":failures,
        "elapsed_millis":fixture.attempts.iter().map(|v| v["elapsed_millis"].as_u64().unwrap()).sum::<u64>(),
        "remote_effects":fixture.remote_effects(),"completed":true,"trace":fixture.attempts})
}

#[test]
fn simplified_complete_delivery_journey() {
    let result = complete_simplification_journey(None);
    assert_eq!(result["reviews"], 1);
    assert_eq!(result["failures"], 0);
}

#[test]
#[ignore = "requires an explicitly built isolated baseline executable"]
fn matched_baseline_and_candidate_complete_delivery() {
    let path = std::env::var_os("CSDLC_1064_BASELINE_BINARY").expect("baseline binary required");
    let baseline = complete_simplification_journey(Some(Path::new(&path)));
    let candidate = complete_simplification_journey(None);
    assert!(candidate["attempts"].as_u64().unwrap() < baseline["attempts"].as_u64().unwrap());
    assert_eq!(baseline["reviews"], 2);
    assert_eq!(candidate["reviews"], 1);
    let output =
        std::env::var_os("CSDLC_1064_COMPARISON_OUTPUT").expect("comparison output required");
    fs::write(
        output,
        serde_json::to_vec_pretty(&json!({"baseline":baseline,"candidate":candidate})).unwrap(),
    )
    .unwrap();
}

// PVF #1064: deterministic installed recovery, owned staging only, no external effects.
#[test]
fn ordinary_edit_resumes_incomplete_current_render_before_advancing() {
    let mut fixture = Fixture::new("1064-incomplete-render-edit");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let cards = linked.join(".csdlc/v3/issues/505/cards");
    let manifest = fs::read(cards.join("manifest.json")).unwrap();
    let value: Value = serde_json::from_slice(&manifest).unwrap();
    let suffix = value["projection_digest"]
        .as_str()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    fs::write(
        cards.join(format!(".projection-{suffix}.pending")),
        &manifest,
    )
    .unwrap();
    fs::write(
        cards.join(format!(".sip.md-{suffix}.next")),
        fs::read(cards.join("sip.md")).unwrap(),
    )
    .unwrap();
    // Simulate a crash before replacing the committed manifest.
    fs::write(cards.join("manifest.json"), "previous manifest").unwrap();
    let changes = fixture.write_json(
        "next-edit.json",
        &json!({
            "schema":"csdlc.v3.intent_changes.v1",
            "amendment":{"class":"scope_acceptance","transition_approved":true},
            "cards":{"sip":{"title":"Next edit after incomplete render"}}
        }),
    );
    let result = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(result["projection"]["after"]["status"], "healthy");
    assert!(!cards.join(format!(".projection-{suffix}.pending")).exists());
    assert!(!cards.join(format!(".sip.md-{suffix}.next")).exists());
    let repeated = success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    assert_eq!(repeated["semantic_version"], result["semantic_version"]);
    assert_eq!(fixture.remote_effects(), 0);
}

// PVF #1064: deterministic delivery correction; no renewed proof or review.
#[test]
fn publication_metadata_correction_after_ready_preserves_merge_ready() {
    let (mut fixture, linked) = reviewed_fixture("1064-ready-metadata");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    fixture.enable_pr_transport(&linked);
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    let mut publication = plan()["publication"].clone();
    publication["body"] = json!("Corrected ready PR description.\n\nCloses #505");
    let changes = fixture.write_json(
        "metadata.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    success(fixture.run(
        &linked,
        &["edit", "505", "--changes", changes.to_str().unwrap()],
    ));
    success(fixture.run(&linked, &["publish", "505"]));
    let state: Value =
        serde_json::from_slice(&fs::read(linked.join(".csdlc/v3/issues/505/state.json")).unwrap())
            .unwrap();
    assert_eq!(state["phase"], "merge_ready");
    let remote: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/installed-candidate/remote-pr.json")).unwrap(),
    )
    .unwrap();
    assert!(remote["body"].as_str().unwrap().starts_with(&format!(
        "{}\n\n<!-- csdlc-v3-operation:",
        publication["body"].as_str().unwrap()
    )));
    assert_eq!(remote["draft"], false);
    assert_eq!(fixture.remote_effects(), 3);
}

// PVF #1064: authentic copied prepared record, deterministic isolated continuation.
fn converted_authentic_fixture(
    label: &str,
) -> (
    Fixture,
    std::path::PathBuf,
    std::path::PathBuf,
    std::collections::BTreeMap<std::path::PathBuf, String>,
) {
    let fixture = Fixture::new(label);
    let primary = fixture.root.clone();
    let source =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/issue872-copied-records");
    let copied = primary.join(".csdlc/copied-records");
    intent_fixture::copy_tree(&source, &copied);
    let before = intent_fixture::inventory(&copied);
    let linked = primary.join("worktrees/conversion-target");
    git(
        &primary,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "codex/copied-continuation",
            linked.to_str().unwrap(),
        ],
    );
    let records: Vec<Value> = [
        (511, "prepared"),
        (517, "bound_dirty"),
        (497, "implemented"),
        (3, "reviewed"),
        (505, "published"),
        (122, "terminal"),
        (113, "pending_recovery"),
    ]
    .into_iter()
    .map(|(issue, role)| json!({"issue":issue,"role":role,"source":copied.join(issue.to_string())}))
    .collect();
    let conversion = json!({
        "schema":"csdlc.v3.copied_record_conversion.v1", "repository":"agent-logic/agent-design-language", "operation_id":"issue1064-authentic-continuation",
        "authority_bytes_path":linked.join("csdlc-v3/operator/authority-selector.json"),
        "prior_executable_path":env!("CARGO_BIN_EXE_csdlc"),
        "prior_executable_blake3":blake3::hash(&fs::read(env!("CARGO_BIN_EXE_csdlc")).unwrap()).to_hex().to_string(),
        "writer_fence_issues":[511,517,497,3,505,122,113,868],"writer_probe_issue":868,
        "git_common":primary.join(".git"),"linked_branch":"codex/copied-continuation","linked_head":git(&linked,&["rev-parse","HEAD"]),"linked_worktree":linked,
        "registry_path":linked.join("docs/templates/prompts/current.json"),"records":records
    });
    let conversion_path = fixture.write_json("conversion.json", &conversion);
    let converted = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["convert", "--request", conversion_path.to_str().unwrap()])
        .current_dir(&primary)
        .output()
        .unwrap();
    assert!(
        converted.status.success(),
        "conversion failed: {converted:?}"
    );
    assert_same_inventory!(before, intent_fixture::inventory(&copied));
    (fixture, linked, copied, before)
}

#[test]
fn authentic_prepared_record_admits_public_continuation() {
    let (mut fixture, _converted_worktree, copied, before) =
        converted_authentic_fixture("1064-authentic-prepared");
    let primary = fixture.root.clone();
    let script = primary.join(".git/installed-candidate/fake-bin/curl");
    let transport = fs::read_to_string(&script).unwrap();
    fs::write(
        &script,
        transport
            .replace("/issues/505", "/issues/511")
            .replace("\"number\":505", "\"number\":511")
            .replace(
                "Installed intent fixture",
                "[v0.92.1][OBS-A] Observatory experience design",
            ),
    )
    .unwrap();
    let observational_before = intent_fixture::inventory(&primary);
    let status = success(fixture.run(&primary, &["status", "511"]));
    assert_eq!(status["phase"], "ready");
    assert_same_inventory!(observational_before, intent_fixture::inventory(&primary));
    let mut invalid_bind = success(fixture.run(&primary, &["bind", "511", "--emit-request"]));
    invalid_bind["request"]["content"] = json!({"unexpected":"content"});
    let invalid_path = fixture.write_json("invalid-bind.json", &invalid_bind);
    let before_invalid = intent_fixture::inventory(&primary);
    let rejected = fixture.run(
        &primary,
        &["bind", "--intent-request", invalid_path.to_str().unwrap()],
    );
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("intent_unexpected_content"));
    assert_same_inventory!(before_invalid, intent_fixture::inventory(&primary));
    let bound = success(fixture.run(&primary, &["bind", "511"]));
    assert_eq!(bound["status"], "completed");
    let binding: Value = serde_json::from_slice(
        &fs::read(primary.join(".git/csdlc-v3/local/bindings/511.json")).unwrap(),
    )
    .unwrap();
    let worktree = std::path::PathBuf::from(binding["worktree"].as_str().unwrap());
    let validators = fixture.write_json(
        "validators.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","validators":plan()["validators"]}),
    );
    success(fixture.run(
        &worktree,
        &["edit", "511", "--changes", validators.to_str().unwrap()],
    ));
    let mut publication = plan()["publication"].clone();
    publication["title"] = json!("[v0.92.1][OBS-A] Observatory experience design");
    publication["body"] = json!("Closes #511");
    let metadata = fixture.write_json(
        "publication.json",
        &json!({"schema":"csdlc.v3.intent_changes.v1","publication":publication}),
    );
    success(fixture.run(
        &worktree,
        &["edit", "511", "--changes", metadata.to_str().unwrap()],
    ));
    success(fixture.run(&worktree, &["proof", "511"]));
    let review = fixture.write_json("review.json", &json!({
        "schema":"csdlc.v3.review_judgment.v1", "implementer":"copied-record-fixture-author", "reviewer":"independent-copied-record-fixture-reviewer",
        "reviewed_revision":git(&worktree,&["rev-parse","HEAD"]),"verdict":"pass",
        "evidence":"Synthetic current-candidate judgment after authentic historical record conversion; no production approval."
    }));
    success(fixture.run(
        &worktree,
        &["review", "511", "--evidence", review.to_str().unwrap()],
    ));
    // The transport fixture defaults to issue 505. Retarget only fake transport
    // endpoints/readbacks; never rewrite the copied records or authority receipt.
    let retarget_transport = || {
        for relative in [
            "fake-bin/curl",
            "remote-issue.json",
            "merge-before.json",
            "merge-after.json",
        ] {
            let path = primary.join(".git/installed-candidate").join(relative);
            if path.is_file() {
                let bytes = fs::read_to_string(&path).unwrap();
                fs::write(
                    path,
                    bytes
                        .replace("/issues/505", "/issues/511")
                        .replace("\"number\":505", "\"number\":511")
                        .replace("Closes #505", "Closes #511"),
                )
                .unwrap();
            }
        }
    };
    fixture.enable_pr_transport(&worktree);
    retarget_transport();
    success(fixture.run(&worktree, &["publish", "511"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "511",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&worktree);
    retarget_transport();
    let merge = fixture.write_json("merge.json", &json!({"action":"pull_request_merge","base":"main","method":"merge","operator_approval":"Synthetic approval for isolated PR639 in copied-record continuation only"}));
    success(fixture.run(
        &primary,
        &[
            "github-pr",
            "511",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    success(fixture.run(&worktree, &["finish", "511"]));
    let cleanup = success(fixture.run(&primary, &["clean", "511"]));
    success(fixture.run(
        &primary,
        &[
            "clean",
            "511",
            "--execute",
            "--preview",
            cleanup["preview_token"].as_str().unwrap(),
        ],
    ));
    assert!(!worktree.exists());
    assert!(primary
        .join(".git/csdlc-v3/local/evidence/511/terminal-receipt.json")
        .is_file());
    assert_eq!(fixture.remote_effects(), 3);
    assert_same_inventory!(before, intent_fixture::inventory(&copied));
}

// PVF #1064: deterministic installed non-local transition recovery, synthetic transport.
#[test]
fn non_local_transitions_resume_interrupted_current_projection() {
    for command in ["proof", "review", "publish"] {
        let mut fixture = Fixture::new(&format!("1064-interrupted-{command}"));
        let primary = fixture.root.clone();
        prepare(&mut fixture);
        success(fixture.run(&primary, &["bind", "505"]));
        let linked = linked_worktree(&primary);
        if command != "proof" {
            success(fixture.run(&linked, &["proof", "505"]));
        }
        let evidence = if command != "proof" {
            Some(fixture.write_json("review.json", &external_review(&linked)))
        } else {
            None
        };
        if command == "publish" {
            success(fixture.run(
                &linked,
                &[
                    "review",
                    "505",
                    "--evidence",
                    evidence.as_ref().unwrap().to_str().unwrap(),
                ],
            ));
            fixture.enable_pr_transport(&linked);
        }
        let cards = linked.join(".csdlc/v3/issues/505/cards");
        let manifest = fs::read(cards.join("manifest.json")).unwrap();
        let value: Value = serde_json::from_slice(&manifest).unwrap();
        let suffix = value["projection_digest"]
            .as_str()
            .unwrap()
            .rsplit(':')
            .next()
            .unwrap();
        let pending = cards.join(format!(".projection-{suffix}.pending"));
        let staged = cards.join(format!(".sip.md-{suffix}.next"));
        fs::write(&pending, &manifest).unwrap();
        fs::write(&staged, fs::read(cards.join("sip.md")).unwrap()).unwrap();
        fs::write(cards.join("manifest.json"), "previous manifest").unwrap();
        let before = intent_fixture::inventory(&primary);
        success(fixture.run(&linked, &["status", "505"]));
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        let args = if command == "review" {
            vec![
                command,
                "505",
                "--evidence",
                evidence.as_ref().unwrap().to_str().unwrap(),
            ]
        } else {
            vec![command, "505"]
        };
        success(fixture.run(&linked, &args));
        assert!(
            !pending.exists() && !staged.exists(),
            "{command} stranded staging"
        );
        let rebuilt = success(fixture.run(&linked, &["rebuild", "505"]));
        assert_eq!(rebuilt["projection"]["after"]["status"], "healthy");
        assert_eq!(fixture.remote_effects(), usize::from(command == "publish"));
    }
}

// PVF #1064: all converted roles are exercised directly; source copies stay immutable.
#[test]
fn converted_later_phases_accept_applicable_corrections_without_native_index() {
    // Exercise the internal owner in a repository-installed test process: the
    // retired CLI writer cannot reach proof authorization.
    if let Some(request_path) = std::env::var_os("CSDLC_TEST_CONVERTED_PROOF_REQUEST") {
        let request = serde_json::from_slice(&fs::read(request_path).unwrap()).unwrap();
        let report = csdlc_v3::commands::proof::classify_route(
            "proof",
            request,
            Some(&std::env::current_dir().unwrap()),
        );
        assert_eq!(
            report.findings[0].code, "proof_lifecycle_stale",
            "{report:?}"
        );
        assert!(!report.performed_mutation);
        return;
    }

    let (mut fixture, linked, copied, source_before) =
        converted_authentic_fixture("1064-later-phase-continuation");
    let primary = fixture.root.clone();
    for issue in [517_u64, 497, 3, 505, 122, 113] {
        let id = issue.to_string();
        assert!(!linked
            .join(format!(".csdlc/issues/{issue}/index.json"))
            .exists());
        let root = SemanticRoot::from_git_common(
            primary.join(".git"),
            "agent-logic/agent-design-language",
        )
        .unwrap();
        let key = IssueKey::new("agent-logic/agent-design-language", issue).unwrap();
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key).unwrap() {
            Observation::Current(value) | Observation::ProjectionRepairRequired(value) => value,
            other => panic!("unexpected conversion state: {other:?}"),
        };
        // The unchanged converter emits the historical marker. Continuation must
        // repair these existing records, not merely newly hashed conversions.
        assert_eq!(
            snapshot.inputs().binding().unwrap().registration,
            "git-worktree-list"
        );
        if issue == 517 {
            let sibling = primary.join("worktrees/sibling");
            git(
                &primary,
                &[
                    "worktree",
                    "add",
                    "-q",
                    "-b",
                    "codex/sibling",
                    sibling.to_str().unwrap(),
                ],
            );
            fs::copy(
                std::env::current_exe().unwrap(),
                primary.join(".git/installed-candidate/proof-owner-test"),
            )
            .unwrap();
            for (label, worktree, branch, generation) in [
                (
                    "sibling",
                    &sibling,
                    "codex/sibling",
                    snapshot.version().generation(),
                ),
                (
                    "stale",
                    &linked,
                    "codex/copied-continuation",
                    snapshot.version().generation() + 1,
                ),
            ] {
                let request = fixture.write_json(&format!("reject-{label}.json"), &json!({
                    "issue":issue,"repository":"agent-logic/agent-design-language",
                    "binding":{"worktree":worktree,"branch":branch,"exact_head":git(worktree,&["rev-parse","HEAD"]),
                        "git_common_dir":primary.join(".git"),"generation":generation,"lifecycle_digest":snapshot.version().digest().as_str()},
                    "evidence_root":worktree
                }));
                let before = intent_fixture::inventory(&primary);
                let rejected = Command::new(
                    primary.join(".git/installed-candidate/proof-owner-test"),
                )
                .current_dir(worktree)
                .args([
                    "--exact",
                    "converted_later_phases_accept_applicable_corrections_without_native_index",
                    "--nocapture",
                ])
                .env("CSDLC_TEST_CONVERTED_PROOF_REQUEST", &request)
                .output()
                .unwrap();
                assert!(
                    rejected.status.success(),
                    "internal proof guard failed: {rejected:?}"
                );
                assert_same_inventory!(before, intent_fixture::inventory(&primary));
            }
        }
        let before = intent_fixture::inventory(&primary);
        let status = success(fixture.run(&primary, &["status", &id]));
        let validated = success(fixture.run(&primary, &["validate", &id]));
        assert_eq!(validated["read_only"], true);
        assert_same_inventory!(before, intent_fixture::inventory(&primary));
        if issue == 122 {
            assert!(!status["allowed_next"]
                .as_array()
                .unwrap()
                .contains(&json!("proof")));
            continue;
        }
        if issue == 113 {
            git(
                &linked,
                &[
                    "commit",
                    "--quiet",
                    "--allow-empty",
                    "-m",
                    "converted implementation advances",
                ],
            );
            success(fixture.run(&linked, &["bind", &id]));
            git(
                &linked,
                &[
                    "commit",
                    "--quiet",
                    "--allow-empty",
                    "-m",
                    "converted implementation advances again",
                ],
            );
        }
        let validators = fixture.write_json(
            &format!("validators-{issue}.json"),
            &json!({"schema":"csdlc.v3.intent_changes.v1","validators":plan()["validators"]}),
        );
        success(fixture.run(
            &linked,
            &["edit", &id, "--changes", validators.to_str().unwrap()],
        ));
        success(fixture.run(&linked, &["validate", &id]));
        success(fixture.run(&linked, &["proof", &id]));
        let judgment = fixture.write_json(&format!("review-{issue}.json"), &json!({
            "schema":"csdlc.v3.review_judgment.v1","implementer":"converted-fixture-author",
            "reviewer":"independent-converted-fixture-reviewer","reviewed_revision":git(&linked,&["rev-parse","HEAD"]),
            "verdict":"pass","evidence":"Synthetic current candidate judgment after real fixture proof; not production approval."
        }));
        success(fixture.run(
            &linked,
            &["review", &id, "--evidence", judgment.to_str().unwrap()],
        ));

        assert!(!linked
            .join(format!(".csdlc/issues/{issue}/index.json"))
            .exists());
    }
    assert_same_inventory!(source_before, intent_fixture::inventory(&copied));
    assert_eq!(fixture.remote_effects(), 0);
}

#[test]
fn completed_proof_replay_leaves_interrupted_projection_read_only() {
    let mut fixture = Fixture::new("1064-replay-interrupted-proof");
    let primary = fixture.root.clone();
    prepare(&mut fixture);
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    success(fixture.run(&linked, &["proof", "505"]));
    let cards = linked.join(".csdlc/v3/issues/505/cards");
    let manifest = fs::read(cards.join("manifest.json")).unwrap();
    let value: Value = serde_json::from_slice(&manifest).unwrap();
    let suffix = value["projection_digest"]
        .as_str()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    fs::write(
        cards.join(format!(".projection-{suffix}.pending")),
        &manifest,
    )
    .unwrap();
    fs::write(
        cards.join(format!(".sip.md-{suffix}.next")),
        fs::read(cards.join("sip.md")).unwrap(),
    )
    .unwrap();
    fs::write(cards.join("manifest.json"), "previous manifest").unwrap();
    let before = intent_fixture::inventory(&primary);
    let replay = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(replay["read_only"], true);
    assert_eq!(replay["performed_mutation"], false);
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    let rebuilt = success(fixture.run(&linked, &["rebuild", "505"]));
    assert_eq!(rebuilt["projection"]["after"]["status"], "healthy");
}

// PVF #1098: required deterministic local installed-owner regressions; small
// CPU/Git/filesystem fixtures and synthetic GitHub transport, no live effects.
#[test]
fn issue_1098_cleanup_refuses_distinct_staged_evidence() {
    let (mut fixture, linked) = reviewed_fixture("cleanup-staged-evidence");
    let primary = fixture.root.clone();
    success(fixture.run(&linked, &["publish", "505"]));
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let merge = fixture.write_json("merge.json", &json!({"action":"pull_request_merge", "base":"main", "method":"merge", "operator_approval":"synthetic operator authorizes fixture PR639 exact candidate merge"}));
    success(fixture.run(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    success(fixture.run(&linked, &["finish", "505"]));
    let relative = ".csdlc/evidence/505/staged-proof.txt";
    let file = linked.join(relative);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "unique staged proof\n").unwrap();
    git(&linked, &["add", "-f", relative]);
    fs::write(&file, "working copy proof\n").unwrap();
    let before = intent_fixture::inventory(&primary);
    let refused = fixture.run(&primary, &["clean", "505"]);
    assert!(
        !refused.status.success(),
        "staged proof admitted for destruction"
    );
    assert!(String::from_utf8_lossy(&refused.stdout).contains("cleanup_archive_staged_changes"));
    assert_same_inventory!(before, intent_fixture::inventory(&primary));
    assert_eq!(
        git(&linked, &["show", ":.csdlc/evidence/505/staged-proof.txt"]),
        "unique staged proof"
    );
    assert_eq!(fs::read_to_string(file).unwrap(), "working copy proof\n");
    assert!(linked.exists());
}

fn successful_retry_recovery_fixture(label: &str) -> (Fixture, std::path::PathBuf) {
    let (mut fixture, linked) = reviewed_fixture(label);
    let crash = fixture.run_with_env(
        &linked,
        &["publish", "505"],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    let crash = fixture.run_with_env(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_recovery_after_native",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    assert_eq!(fixture.remote_effects(), 1);
    assert_eq!(fixture.remote_pr()["number"], 639);
    assert!(
        fs::read_dir(fixture.root.join(".git/csdlc-v3/remote/recoveries"))
            .unwrap()
            .next()
            .is_some()
    );
    fs::write(linked.join("later.txt"), "later candidate\n").unwrap();
    git(&linked, &["add", "later.txt"]);
    git(&linked, &["commit", "--quiet", "-m", "later candidate"]);
    (fixture, linked)
}

fn check_retry_not_absent(case: &str) {
    let (mut fixture, linked) = successful_retry_recovery_fixture(case);
    if case != "retained-receipt" {
        let receipts = fixture.root.join(".git/csdlc-v3/remote/mutations");
        for entry in fs::read_dir(receipts).unwrap() {
            fs::remove_file(entry.unwrap().path()).unwrap();
        }
    }
    if case == "unavailable-readback" {
        fixture.remote_flag("drop-readback", true);
    }
    if case == "changed-pr" {
        let mut pr = fixture.remote_pr();
        pr["title"] = json!("Updated title after successful retry");
        fixture.set_remote_pr(&pr);
    }
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    let output = fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    let recovered: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_ne!(
        recovered["result"]["recovery"], "authenticated_absent_historical_publication",
        "{recovered}"
    );
    if case == "retained-receipt" {
        // An idempotent readback performs no new mutation, but the retained
        // publication must attach as a success with its authenticated PR.
        assert!(output.status.success(), "{recovered}");
        assert_eq!(recovered["semantic"]["outcome"], "success", "{recovered}");
        assert_eq!(recovered["result"]["receipt"]["pull_request"], 639);
        assert_eq!(recovered["result"]["receipt"]["authenticated"], true);
    }
    assert_eq!(
        fixture.remote_effects(),
        1,
        "recovery repeated the remote effect"
    );
    assert_eq!(fixture.remote_pr()["number"], 639);
    if case != "retained-receipt" {
        let pending = success(fixture.run(&linked, &["recover", "505"]));
        assert_eq!(pending["status"], "recovery_required", "{pending}");
    }
}

#[test]
fn issue_1098_recovery_preserves_successful_retry_receipt() {
    check_retry_not_absent("retained-receipt");
}

#[test]
fn issue_1098_recovery_preserves_pending_success_without_receipt() {
    check_retry_not_absent("missing-receipt");
}

#[test]
fn issue_1098_recovery_keeps_unavailable_readback_pending() {
    check_retry_not_absent("unavailable-readback");
}

#[test]
fn issue_1098_recovery_does_not_treat_changed_pr_as_absent() {
    check_retry_not_absent("changed-pr");
}

// #1135 PVF: installed tooling regression, genuine offline Cargo compilation;
// local Git/files/processes only. Source ownership and later status, not model proof.
#[test]
fn installed_proof_matches_same_named_binary_and_library_with_cargo_aggregate() {
    let mut fixture = Fixture::new("compiler-artifact-owner");
    let primary = fixture.root.clone();
    fs::create_dir_all(primary.join("other-package/src")).unwrap();
    fs::write(primary.join("other-package/Cargo.toml"),
        "[package]\nname='other-package'\nversion='0.1.0'\nedition='2021'\n[lib]\nname='shared_name'\n").unwrap();
    fs::write(
        primary.join("other-package/src/lib.rs"),
        "pub fn answer()->u8 { 42 }\n#[test] fn library_test(){assert_eq!(answer(),42); }\n",
    )
    .unwrap();
    let manifest = primary.join("fixture-proof/Cargo.toml");
    let mut body = fs::read_to_string(&manifest).unwrap();
    body.push_str("\n[[bin]]\nname='shared-name'\npath='src/main.rs'\n[dependencies]\nother-package={path='../other-package'}\n");
    fs::write(&manifest, body).unwrap();
    fs::write(
        primary.join("fixture-proof/src/main.rs"),
        "fn main(){assert_eq!(shared_name::answer(),42); }\n",
    )
    .unwrap();
    // Cargo aggregates these build triggers into debug/shared-name.d; rustc's
    // genuine source record must remain the authority for compiler inputs.
    fs::write(
        primary.join("fixture-proof/build.rs"),
        "fn main(){println!(\"cargo:rerun-if-changed=src\"); }\n",
    )
    .unwrap();
    for manifest in ["fixture-proof/Cargo.toml", "other-package/Cargo.toml"] {
        let output = Command::new("cargo")
            .current_dir(&primary)
            .args([
                "generate-lockfile",
                "--offline",
                "--manifest-path",
                manifest,
            ])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    git(&primary, &["add", "fixture-proof", "other-package"]);
    git(
        &primary,
        &["commit", "-qm", "Track real same-name Cargo targets"],
    );
    let mut input = plan();
    input["validators"].as_array_mut().unwrap().push(json!({
        "id":"other-package", "program":"cargo", "args":["test","--offline","--manifest-path","other-package/Cargo.toml"],
        "success_marker":"test result: ok.", "timeout_seconds":60
    }));
    let input = fixture.write_json("ownership-plan.json", &input);
    success(fixture.run(
        &primary,
        &["prepare", "505", "--plan", input.to_str().unwrap()],
    ));
    success(fixture.run(&primary, &["bind", "505"]));
    let linked = linked_worktree(&primary);
    let proof = success(fixture.run(&linked, &["proof", "505"]));
    assert_eq!(proof["proof"]["status"], "passed");
    assert_eq!(proof["proof"]["validators"][0]["tests_passed"], 1);
    assert_eq!(proof["proof"]["validators"][1]["tests_passed"], 1);
    let output = Command::new("cargo")
        .current_dir(&linked)
        .env("CARGO_TARGET_DIR", linked.join("target/intent-validation"))
        .args([
            "build",
            "--offline",
            "--manifest-path",
            "fixture-proof/Cargo.toml",
            "--bin",
            "shared-name",
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let aggregate = linked.join("target/intent-validation/debug/shared-name.d");
    let aggregate_bytes = fs::read(&aggregate).unwrap();
    assert!(
        String::from_utf8_lossy(&aggregate_bytes).contains("fixture-proof/src "),
        "Cargo aggregate includes directory trigger"
    );
    let status = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(status["evidence"]["proof_current"], true);
    assert_eq!(
        fs::read(aggregate).unwrap(),
        aggregate_bytes,
        "native verification must not rewrite Cargo evidence"
    );
}

// PVF #1129: required deterministic installed lifecycle regression; local Git,
// synthetic transport and filesystem only. No live GitHub effects or provider calls.
#[test]
fn issue1129_ready_merge_refusal_preserves_pending_edit_then_recover_finishes() {
    for pending_edit in [false, true] {
        let (mut fixture, linked) = reviewed_fixture(&format!("issue1129-{pending_edit}"));
        let primary = fixture.root.clone();
        success(fixture.run(&linked, &["publish", "505"]));
        if pending_edit {
            let edit = fixture.write_json(
                "edit.json",
                &json!({
                    "action":"issue_edit", "title":"Updated issue title", "body":null
                }),
            );
            fixture.remote_flag("drop-issue-edit-readback", true);
            let failed = fixture.run(
                &linked,
                &[
                    "github-issue",
                    "505",
                    "--operation",
                    edit.to_str().unwrap(),
                    "--execute",
                ],
            );
            assert!(!failed.status.success(), "{failed:?}");
            assert_eq!(fixture.remote_effects(), 2);
            fixture.remote_flag("drop-issue-readback", false);
            fixture.remote_flag("drop-issue-edit-readback", false);
        }
        let before = publication_reservation_inventory(&primary);
        fixture.remote_flag("merge-on-pr-read", true);
        let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
        let refused = fixture.run(
            &linked,
            &[
                "github-pr",
                "505",
                "--operation",
                ready.to_str().unwrap(),
                "--execute",
            ],
        );
        assert!(!refused.status.success(), "{refused:?}");
        assert!(
            String::from_utf8_lossy(&refused.stdout)
                .contains("intent_publication_remote_identity_mismatch"),
            "{refused:?}"
        );
        assert_same_inventory!(before, publication_reservation_inventory(&primary));
        assert_eq!(fixture.remote_pr()["merged"], true);
        let effects = fixture.remote_effects();
        let preview = success(fixture.run(&primary, &["recover", "505"]));
        if pending_edit {
            assert_eq!(preview["action"], "reconcile_retained_remote_effect");
            let stale = fixture.run(
                &primary,
                &["recover", "505", "--execute", "--preview", "stale"],
            );
            assert!(!stale.status.success());
            let recovered = success(fixture.run(
                &primary,
                &[
                    "recover",
                    "505",
                    "--execute",
                    "--preview",
                    preview["preview_digest"].as_str().unwrap(),
                ],
            ));
            assert_eq!(recovered["semantic"]["outcome"], "success");
            assert_eq!(recovered["performed_mutation"], false);
        }
        success(fixture.run(&linked, &["finish", "505"]));
        assert_eq!(
            fixture.remote_effects(),
            effects,
            "recovery or finish replayed a remote write"
        );
    }
}

// PVF #1142: deterministic installed-owner integration, synthetic transport,
// local-only resources; required focused recovery regression (no live GitHub).
fn completed_publication_transport_fixture(
    name: &str,
) -> (Fixture, std::path::PathBuf, std::path::PathBuf) {
    let (mut fixture, linked) = reviewed_fixture(name);
    success(fixture.run(&linked, &["publish", "505"]));
    let operation = fixture.write_json(
        "update.json",
        &json!({
            "action":"pull_request_update", "title":"Recovered title", "body":"Closes #505"
        }),
    );
    let crash = fixture.run_with_env(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
        &[(
            "CSDLC_V3_TEST_CRASH_POINT",
            "semantic_remote_after_reservation",
        )],
    );
    assert_eq!(crash.status.code(), Some(91));
    let mut remote = fixture.remote_pr();
    remote["title"] = json!("Recovered title");
    let pending = success(fixture.run(&linked, &["status", "505"]));
    let digest = pending["pending_remote"][0]["operation_digest"]
        .as_str()
        .unwrap();
    remote["body"] = json!(format!(
        "Closes #505\n\n<!-- csdlc-v3-operation:{digest} -->"
    ));
    fixture.set_remote_pr(&remote);
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    let completed = success(fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(completed["semantic"]["outcome"], "success");
    (fixture, linked, operation)
}

#[test]
fn completed_publication_transport_present_reconciles_without_semantic_rewrite() {
    let (mut fixture, linked, operation) =
        completed_publication_transport_fixture("completed-publication-transport");
    let primary = fixture.root.clone();
    let before = publication_reservation_inventory(&primary);
    let effects = fixture.remote_effects();
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    assert_eq!(preview["pending"]["pending"].as_array().unwrap().len(), 1);
    let recovered = success(fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    ));
    assert_eq!(recovered["result"]["receipt"]["authenticated"], true);
    assert_eq!(recovered["performed_mutation"], false);
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    assert_eq!(fixture.remote_effects(), effects);
    let status = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(status["pending_remote"], json!([]));
    let replay = success(fixture.run(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            operation.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(replay["envelope"]["status"], "expected_noop");
    assert_eq!(fixture.remote_effects(), effects);
    let ready = fixture.write_json("ready.json", &json!({"action":"pull_request_ready"}));
    success(fixture.run(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            ready.to_str().unwrap(),
            "--execute",
        ],
    ));
    fixture.enable_merge_transport(&linked);
    let merge = fixture.write_json("merge.json", &json!({"action":"pull_request_merge", "base":"main", "method":"merge", "operator_approval":"Synthetic fixture approval"}));
    success(fixture.run(
        &linked,
        &[
            "github-pr",
            "505",
            "--operation",
            merge.to_str().unwrap(),
            "--execute",
        ],
    ));
    assert_eq!(fixture.remote_pr()["merged"], true);
    assert_eq!(fixture.remote_effects(), effects + 2);
}

fn assert_completed_publication_transport_refuses(change: &str) {
    let (mut fixture, linked, _) = completed_publication_transport_fixture(change);
    let primary = fixture.root.clone();
    let mut remote = fixture.remote_pr();
    match change {
        "changed-head" => remote["head"]["sha"] = json!("0".repeat(40)),
        "changed-title" => remote["title"] = json!("Later author edit"),
        "changed-body" => remote["body"] = json!("Later author body"),
        "unavailable" => fixture.remote_flag("drop-readback", true),
        "ambiguous" => remote = json!([remote.clone(), remote]),
        "missing-target" => {
            fs::remove_file(primary.join(".git/installed-candidate/remote-pr.json")).unwrap();
        }
        _ => unreachable!(),
    }
    if change != "missing-target" {
        fixture.set_remote_pr(&remote);
    }
    let before = publication_reservation_inventory(&primary);
    let effects = fixture.remote_effects();
    let preview = success(fixture.run(&linked, &["recover", "505"]));
    let refused = fixture.run(
        &linked,
        &[
            "recover",
            "505",
            "--execute",
            "--preview",
            preview["preview_digest"].as_str().unwrap(),
        ],
    );
    assert!(!refused.status.success(), "{change}: {refused:?}");
    assert_same_inventory!(before, publication_reservation_inventory(&primary));
    assert_eq!(fixture.remote_effects(), effects);
    let status = success(fixture.run(&linked, &["status", "505"]));
    assert_eq!(status["pending_remote"].as_array().unwrap().len(), 1);
}

#[test]
fn completed_publication_transport_changed_head_fails_closed() {
    assert_completed_publication_transport_refuses("changed-head");
}
#[test]
fn completed_publication_transport_changed_title_fails_closed() {
    assert_completed_publication_transport_refuses("changed-title");
}
#[test]
fn completed_publication_transport_changed_body_fails_closed() {
    assert_completed_publication_transport_refuses("changed-body");
}
#[test]
fn completed_publication_transport_unavailable_fails_closed() {
    assert_completed_publication_transport_refuses("unavailable");
}
#[test]
fn completed_publication_transport_missing_target_fails_closed() {
    assert_completed_publication_transport_refuses("missing-target");
}

#[test]
fn completed_publication_transport_ambiguous_fails_closed() {
    assert_completed_publication_transport_refuses("ambiguous");
}
