use csdlc_v3::conversion::{
    inspect_conversion_operation, restore_conversion_pre_effect, ConversionRequest,
};
use fs2::FileExt;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

const RECORDS: [(u64, &str); 7] = [
    (511, "prepared"),
    (517, "bound_dirty"),
    (497, "implemented"),
    (3, "reviewed"),
    (505, "published"),
    (122, "terminal"),
    (113, "pending_recovery"),
];

#[test]
fn operation_id_dot_segments_and_substituted_topology_bytes_fail_before_effects() {
    for operation_id in [".", ".."] {
        let fixture = Fixture::new();
        let request = fixture.write_value(
            &format!("dot-{}.json", operation_id.len()),
            &fixture.request_value(operation_id, None),
        );
        assert_normal_rejection(&invoke(&request), "non-dot path segment");
        assert!(!fixture
            .git_common
            .join("csdlc-v3/local/conversion-rehearsals")
            .exists());
    }

    let fixture = Fixture::new();
    let substituted = fixture.root.join("substituted.json");
    fs::write(&substituted, b"{}\n").unwrap();
    for field in ["authority_bytes_path", "registry_path"] {
        let mut value = fixture.request_value(&fixture.operation(field), None);
        value[field] = json!(substituted);
        assert_normal_rejection(
            &invoke(&fixture.write_value(&format!("{field}.json"), &value)),
            "canonical registered-worktree path",
        );
    }
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .exists());
}

#[test]
fn source_identity_phase_and_prior_executable_mismatches_fail_before_effects() {
    let cases = [
        ("issue", json!(999), "source index identity mismatch"),
        (
            "repository",
            json!("substituted/repository"),
            "source index identity mismatch",
        ),
        ("phase", json!("reviewed"), "unsupported source role/phase"),
    ];
    for (field, replacement, expected) in cases {
        let fixture = Fixture::new();
        let index = fixture.copied.join("497/index.json");
        let mut value: Value = serde_json::from_slice(&fs::read(&index).unwrap()).unwrap();
        value[field] = replacement;
        fs::write(&index, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
        assert_normal_rejection(
            &invoke(&fixture.write_request(&fixture.operation(field), None)),
            expected,
        );
        assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
    }

    let fixture = Fixture::new();
    let mut value = fixture.request_value(&fixture.operation("prior-digest"), None);
    value["prior_executable_blake3"] = json!("0".repeat(64));
    assert_normal_rejection(
        &invoke(&fixture.write_value("prior-digest.json", &value)),
        "prior executable bytes do not match",
    );
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .exists());

    let fixture = Fixture::new();
    let mut value = fixture.request_value(&fixture.operation("fence-denominator"), None);
    value["writer_fence_issues"] = json!([511, 517, 497, 3, 505, 122, 113]);
    assert_normal_rejection(
        &invoke(&fixture.write_value("fence-denominator.json", &value)),
        "exactly equal the seven-record census plus writer_probe_issue",
    );
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .exists());

    let fixture = Fixture::new();
    let card = fixture.copied.join("511/cards/sip.values.json");
    let mut value: Value = serde_json::from_slice(&fs::read(&card).unwrap()).unwrap();
    value["identity"]["title"] = json!("substituted title");
    fs::write(&card, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    assert_normal_rejection(
        &invoke(&fixture.write_request(&fixture.operation("card-identity"), None)),
        "six-card identity is ambiguous",
    );
    assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
}

#[test]
fn converter_holds_exact_native_fence_denominator_and_unrelated_residue_still_rejects() {
    let fixture = Fixture::new();
    let operation = fixture.operation("writer-fence");
    let mut value = fixture.request_value(&operation, None);
    value["writer_fence_probe"] = json!(true);
    let request = fixture.write_value("writer-fence.json", &value);
    let mut child = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["convert", "--request"])
        .arg(&request)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let operation_root = fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&operation);
    let marker = wait_for_probe_marker(&operation_root, "writer-fence-held.json");
    assert_native_writer_locks(&fixture.git_common, false);
    assert!(child.try_wait().unwrap().is_none());
    write_probe_ack(
        &operation_root,
        "writer-fence-probe-complete",
        "during_conversion",
        &marker,
    );

    let post_marker =
        wait_for_probe_marker(&operation_root, "writer-fence-post-activation-held.json");
    assert_native_writer_locks(&fixture.git_common, false);
    let semantic_current = fixture
        .git_common
        .join("csdlc-v3/semantic/issues/511/current.json");
    let before = fs::read(&semantic_current).unwrap();
    std::thread::sleep(Duration::from_millis(50));
    assert!(child.try_wait().unwrap().is_none());
    assert_eq!(fs::read(&semantic_current).unwrap(), before);
    write_probe_ack(
        &operation_root,
        "writer-fence-post-activation-probe-complete",
        "post_activation",
        &post_marker,
    );
    assert_success(&child.wait_with_output().unwrap());
    assert_native_writer_locks(&fixture.git_common, true);

    let fixture = Fixture::new();
    let unrelated = fixture.linked.join(".csdlc/locks");
    fs::create_dir_all(&unrelated).unwrap();
    fs::write(unrelated.join("511.lock"), b"unrelated residue\n").unwrap();
    assert_normal_rejection(
        &invoke(&fixture.write_request(&fixture.operation("unrelated-lock"), None)),
        "legacy state",
    );
}

#[test]
fn stale_writer_fence_probe_acknowledgement_cannot_resume_conversion() {
    let fixture = Fixture::new();
    let operation = fixture.operation("stale-writer-fence-ack");
    let mut value = fixture.request_value(&operation, None);
    value["writer_fence_probe"] = json!(true);
    let request = fixture.write_value("stale-writer-fence-ack.json", &value);
    let child = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["convert", "--request"])
        .arg(&request)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let operation_root = fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&operation);
    let marker = wait_for_probe_marker(&operation_root, "writer-fence-held.json");
    let mut stale = marker["detail"]["request_digest"]
        .as_str()
        .unwrap()
        .to_owned();
    stale.push_str("-stale");
    fs::write(
        operation_root.join("writer-fence-probe-complete"),
        serde_json::to_vec_pretty(&json!({
            "schema":"csdlc.v3.copied_record_writer_fence_probe_ack.v1",
            "operation_id":operation,
            "request_digest":stale,
            "checkpoint":"during_conversion",
        }))
        .unwrap(),
    )
    .unwrap();
    assert_normal_rejection(
        &child.wait_with_output().unwrap(),
        "does not authenticate the current operation/request checkpoint",
    );
    assert!(!operation_root
        .join("writer-fence-post-activation-held.json")
        .exists());
}

#[test]
fn first_journal_entry_survives_abrupt_exit_and_same_request_resumes() {
    let fixture = Fixture::new();
    let operation = fixture.operation("journal-create");
    let request = fixture.write_request(
        &operation,
        Some(json!({
            "point": "operation_journal_creation",
            "boundary": "after",
            "mode": "once"
        })),
    );

    let interrupted = invoke(&request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));
    assert!(
        interrupted.stdout.is_empty(),
        "fault was returned as an ordinary error"
    );

    let journal = fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&operation)
        .join("journal.jsonl");
    let retained = fs::read(&journal).expect("journal entry must survive abrupt exit");
    assert!(String::from_utf8_lossy(&retained).contains("request_digest"));
    for relative in [
        "csdlc-v3",
        "csdlc-v3/local",
        "csdlc-v3/local/conversion-rehearsals",
        &format!("csdlc-v3/local/conversion-rehearsals/{operation}"),
    ] {
        assert!(
            fixture.git_common.join(relative).is_dir(),
            "missing {relative}"
        );
    }
    // This proves first-use layout plus fresh-process recovery. A physical power-loss
    // claim requires a filesystem crash harness; the production code requests that
    // guarantee by fsyncing each new child into its parent.

    let resumed = invoke(&request);
    assert_success(&resumed);
    assert!(fs::read(&journal)
        .expect("journal remains readable")
        .starts_with(&retained));
}

#[test]
fn same_operation_rejects_an_altered_source_census_before_effects() {
    let fixture = Fixture::new();
    let operation = fixture.operation("identity-mismatch");
    let request = fixture.write_request(
        &operation,
        Some(json!({
            "point": "operation_journal_creation",
            "boundary": "after",
            "mode": "once"
        })),
    );
    let interrupted = invoke(&request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));

    let index = fixture.copied.join("511/index.json");
    let mut bytes = fs::read(&index).unwrap();
    bytes.extend_from_slice(b"\n");
    fs::write(&index, bytes).unwrap();

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("operation identity mismatch"));
    assert!(!fixture.git_common.join("csdlc-v3/semantic/issues").exists());
}

#[test]
fn linked_worktree_admission_rejects_primary_arbitrary_and_identity_mismatch() {
    let fixture = Fixture::new();

    let mut primary_request = fixture.request_value(&fixture.operation("primary"), None);
    primary_request["linked_worktree"] = json!(fixture.primary);
    primary_request["linked_branch"] = json!("main");
    primary_request["linked_head"] = json!(git_stdout(&["rev-parse", "HEAD"], &fixture.primary));
    let rejected_primary = invoke(&fixture.write_value("primary.json", &primary_request));
    assert_normal_rejection(&rejected_primary, "registered non-primary worktree");

    let arbitrary = fixture.root.join("arbitrary");
    fs::create_dir_all(&arbitrary).unwrap();
    let mut arbitrary_request = fixture.request_value(&fixture.operation("arbitrary"), None);
    arbitrary_request["linked_worktree"] = json!(arbitrary);
    let rejected_arbitrary = invoke(&fixture.write_value("arbitrary.json", &arbitrary_request));
    assert_normal_rejection(&rejected_arbitrary, "git");

    let mut escape_request = fixture.request_value(&fixture.operation("escape"), None);
    escape_request["linked_worktree"] = json!(fixture.linked.join("..").join("linked"));
    let rejected_escape = invoke(&fixture.write_value("escape.json", &escape_request));
    assert_normal_rejection(&rejected_escape, "without traversal components");

    let other = fixture.root.join("other-repository");
    fs::create_dir_all(&other).unwrap();
    git(&["init", "-q", "-b", "main"], &other);
    git(
        &["config", "user.email", "issue872@example.invalid"],
        &other,
    );
    git(&["config", "user.name", "Issue 872 test"], &other);
    fs::write(other.join("seed"), b"other repository\n").unwrap();
    git(&["add", "seed"], &other);
    git(&["commit", "-qm", "seed"], &other);
    let mut common_request = fixture.request_value(&fixture.operation("common"), None);
    common_request["linked_worktree"] = json!(other);
    common_request["linked_branch"] = json!("main");
    common_request["linked_head"] = json!(git_stdout(&["rev-parse", "HEAD"], &other));
    let rejected_common = invoke(&fixture.write_value("common.json", &common_request));
    assert_normal_rejection(&rejected_common, "different Git common directory");

    let mut branch_request = fixture.request_value(&fixture.operation("branch"), None);
    branch_request["linked_branch"] = json!("codex/wrong-branch");
    let rejected_branch = invoke(&fixture.write_value("branch.json", &branch_request));
    assert_normal_rejection(&rejected_branch, "branch mismatch");

    let mut mismatch_request = fixture.request_value(&fixture.operation("mismatch"), None);
    mismatch_request["linked_head"] = json!("0000000000000000000000000000000000000000");
    let rejected_mismatch = invoke(&fixture.write_value("mismatch.json", &mismatch_request));
    assert_normal_rejection(&rejected_mismatch, "HEAD mismatch");

    assert!(!fixture.git_common.join("csdlc-v3/semantic/issues").exists());
}

#[test]
fn incomplete_convertible_record_stops_the_whole_census_before_effects() {
    let fixture = Fixture::new();
    fs::remove_file(fixture.copied.join("497/index.json")).unwrap();
    let request = fixture.write_request(&fixture.operation("incomplete"), None);

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&rejected.stdout);
    assert!(stdout.contains("issue 497"), "{stdout}");
    assert!(
        stdout.contains("unsupported_ambiguous_incomplete_source"),
        "{stdout}"
    );
    assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/projections")
        .exists());
    assert!(!all_files(&fixture.git_common).iter().any(|path| path
        .file_name()
        .is_some_and(|name| name.to_string_lossy().contains("ledger"))));
}

#[test]
fn invalid_generation_in_a_late_record_stops_before_any_semantic_effect() {
    let fixture = Fixture::new();
    let index_path = fixture.copied.join("122/index.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index["generation"] = json!(0);
    fs::write(&index_path, serde_json::to_vec_pretty(&index).unwrap()).unwrap();
    let request = fixture.write_request(&fixture.operation("late-generation"), None);

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&rejected.stdout);
    assert!(stdout.contains("issue 122"), "{stdout}");
    assert!(stdout.contains("invalid numeric generation"), "{stdout}");
    assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/projections")
        .exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .exists());
}

#[test]
fn converted_snapshot_is_acknowledged_before_installed_observation() {
    let fixture = Fixture::new();
    let operation = fixture.operation("projection-ack");
    let request = fixture.write_request(&operation, None);
    assert_success(&invoke(&request));

    let operation_root = fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&operation);
    let receipt: Value =
        serde_json::from_slice(&fs::read(operation_root.join("receipts/505.json")).unwrap())
            .unwrap();
    assert_eq!(
        receipt["detail"]["semantic_equivalence"]["source"]["publication"]["pull_request"],
        591
    );
    assert!(receipt["detail"]["semantic_equivalence"]["source"]["review"].is_object());
    assert!(!receipt["detail"]["semantic_equivalence"]["destination"]
        .to_string()
        .contains("/usr/bin/true"));
    let request_value: Value = serde_json::from_slice(&fs::read(&request).unwrap()).unwrap();
    assert_eq!(
        blake3::hash(&fs::read(operation_root.join("executable-slot/active")).unwrap())
            .to_hex()
            .to_string(),
        request_value["prior_executable_blake3"].as_str().unwrap()
    );

    let observed = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["status", "--git-common"])
        .arg(fs::canonicalize(&fixture.git_common).unwrap())
        .args([
            "--repository",
            "agent-logic/agent-design-language",
            "--issue",
            "511",
        ])
        .output()
        .expect("installed observation process must start");
    assert!(
        observed.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&observed.stdout),
        String::from_utf8_lossy(&observed.stderr)
    );
    let status: Value = serde_json::from_slice(&observed.stdout).unwrap();
    assert_eq!(status.get("status").and_then(Value::as_str), Some("passed"));
    assert_eq!(
        status.get("projection_required").and_then(Value::as_bool),
        Some(false)
    );

    let current: Value = serde_json::from_slice(
        &fs::read(
            fixture
                .git_common
                .join("csdlc-v3/semantic/issues/511/current.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(current.get("generation").and_then(Value::as_u64), Some(2));
    let commits = fixture
        .git_common
        .join("csdlc-v3/semantic/issues/511/commits");
    assert!(all_files(&commits).iter().any(|path| {
        fs::read(path).is_ok_and(|bytes| {
            String::from_utf8_lossy(&bytes)
                .contains("\"acknowledged_card_projection\":\"card-projection-v1:")
        })
    }));
}

#[test]
fn production_evidence_and_restore_are_derived_from_durable_operation_state() {
    let pre_effect = Fixture::new();
    let pre_request = pre_effect.write_request(
        &pre_effect.operation("pre-effect-restore"),
        Some(json!({
            "point": "whole_census_staging_complete",
            "boundary": "after",
            "mode": "once"
        })),
    );
    let interrupted = invoke(&pre_request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));
    let pre: ConversionRequest = serde_json::from_slice(&fs::read(&pre_request).unwrap()).unwrap();
    let evidence = inspect_conversion_operation(&pre).unwrap();
    assert_eq!(evidence.outcome, "interrupted");
    assert_eq!(
        evidence.abrupt_fault_point.as_deref(),
        Some("whole_census_staging_complete")
    );
    assert_eq!(evidence.abrupt_fault_boundary.as_deref(), Some("after"));
    assert_eq!(evidence.semantic_effect_count, 0);
    assert_eq!(evidence.remote_effect_count, 0);
    let evidence_cli = invoke_route("operation-evidence", &pre_request);
    assert!(evidence_cli.status.success());
    let evidence_json: Value = serde_json::from_slice(&evidence_cli.stdout).unwrap();
    assert_eq!(evidence_json["request_digest"], evidence.request_digest);
    assert_eq!(evidence_json["outcome"], "interrupted");
    let restored = restore_conversion_pre_effect(&pre).unwrap();
    assert!(restored.allowed);
    assert_eq!(restored.status, "restored_pre_effect");
    assert_eq!(restored.source_hashes_before, restored.source_hashes_after);
    assert_eq!(restored.source_record_count, RECORDS.len());
    assert!(restored
        .receipt_path
        .as_ref()
        .is_some_and(|path| path.is_file()));
    let restored_cli = invoke_route("restore-pre-effect", &pre_request);
    assert!(restored_cli.status.success());
    let restored_json: Value = serde_json::from_slice(&restored_cli.stdout).unwrap();
    assert_eq!(restored_json["status"], "restored_pre_effect");
    assert_eq!(restored_json["allowed"], true);

    let post_effect = Fixture::new();
    let post_request = post_effect.write_request(
        &post_effect.operation("post-effect-refusal"),
        Some(json!({
            "point": "semantic_state_activation",
            "boundary": "after",
            "mode": "once"
        })),
    );
    let interrupted = invoke(&post_request);
    assert!(!interrupted.status.success());
    let post: ConversionRequest =
        serde_json::from_slice(&fs::read(&post_request).unwrap()).unwrap();
    let evidence = inspect_conversion_operation(&post).unwrap();
    assert!(evidence.semantic_effect_count >= 1);
    let refused = restore_conversion_pre_effect(&post).unwrap();
    assert!(!refused.allowed);
    assert_eq!(refused.status, "refused_post_effect");
    assert!(refused.effect_count >= 1);
    assert!(refused.receipt_path.is_none());
    let refused_cli = invoke_route("restore-pre-effect", &post_request);
    assert_eq!(refused_cli.status.code(), Some(2));
    let refused_json: Value = serde_json::from_slice(&refused_cli.stdout).unwrap();
    assert_eq!(refused_json["status"], "refused_post_effect");
    assert_eq!(refused_json["allowed"], false);
}

#[test]
fn remote_uncertainty_and_single_reconciliation_are_journal_derived() {
    let fixture = Fixture::new();
    let request = fixture.write_request(
        &fixture.operation("remote-reconcile"),
        Some(json!({
            "point": "fake_remote_request_dispatch",
            "boundary": "after",
            "mode": "once"
        })),
    );
    let interrupted = invoke(&request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));
    let request_value: ConversionRequest =
        serde_json::from_slice(&fs::read(&request).unwrap()).unwrap();
    let uncertain = inspect_conversion_operation(&request_value).unwrap();
    assert_eq!(uncertain.remote_state, "uncertain");
    assert_eq!(uncertain.remote_effect_count, 1);
    assert_eq!(uncertain.remote_reconcile_count, 0);
    assert_eq!(
        uncertain.remote_operation_identity.as_deref(),
        Some(request_value.operation_id.as_str())
    );

    assert_success(&invoke(&request));
    let reconciled = inspect_conversion_operation(&request_value).unwrap();
    assert_eq!(reconciled.outcome, "completed");
    assert_eq!(reconciled.remote_state, "reconciled");
    assert_eq!(reconciled.remote_effect_count, 1);
    assert_eq!(reconciled.remote_reconcile_count, 1);
}

fn invoke(request: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["convert", "--request"])
        .arg(request)
        .output()
        .expect("conversion process must start")
}

fn invoke_route(route: &str, request: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args([route, "--request"])
        .arg(request)
        .output()
        .expect("conversion evidence process must start")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        value.get("status").and_then(Value::as_str),
        Some("completed")
    );
}

fn wait_for_probe_marker(operation_root: &Path, name: &str) -> Value {
    let marker = operation_root.join(name);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !marker.is_file() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(marker.is_file(), "converter never exposed {name}");
    serde_json::from_slice(&fs::read(marker).unwrap()).unwrap()
}

fn write_probe_ack(operation_root: &Path, name: &str, checkpoint: &str, marker: &Value) {
    fs::write(
        operation_root.join(name),
        serde_json::to_vec_pretty(&json!({
            "schema":"csdlc.v3.copied_record_writer_fence_probe_ack.v1",
            "operation_id":marker["operation_id"],
            "request_digest":marker["detail"]["request_digest"],
            "checkpoint":checkpoint,
        }))
        .unwrap(),
    )
    .unwrap();
}

fn assert_native_writer_locks(git_common: &Path, available: bool) {
    for issue in [511_u64, 517, 497, 3, 505, 122, 113, 868] {
        let path = git_common
            .join("csdlc-v3/local/locks")
            .join(format!("{issue}.lock"));
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        assert_eq!(
            file.try_lock_exclusive().is_ok(),
            available,
            "native writer lock {issue} availability mismatch"
        );
    }
}

fn assert_normal_rejection(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(expected), "{stdout}");
}

struct Fixture {
    root: PathBuf,
    primary: PathBuf,
    linked: PathBuf,
    git_common: PathBuf,
    copied: PathBuf,
    linked_head: String,
}

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "csdlc-v3-conversion-admission-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        let primary = root.join("primary");
        let linked = root.join("linked");
        fs::create_dir_all(&primary).unwrap();
        git(&["init", "-q", "-b", "main"], &primary);
        git(
            &["config", "user.email", "issue872@example.invalid"],
            &primary,
        );
        git(&["config", "user.name", "Issue 872 test"], &primary);
        fs::write(primary.join("seed"), b"isolated fixture\n").unwrap();
        git(&["add", "seed"], &primary);
        git(&["commit", "-qm", "seed"], &primary);
        git(&["branch", "codex/fixture-linked"], &primary);
        git(
            &[
                "worktree",
                "add",
                "-q",
                linked.to_str().unwrap(),
                "codex/fixture-linked",
            ],
            &primary,
        );
        let linked_head = git_stdout(&["rev-parse", "HEAD"], &linked);
        let git_common = primary.join(".git");

        let source =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/issue872-copied-records");
        let copied = primary.join("copied-records");
        for (issue, _) in RECORDS {
            let source_record = source.join(issue.to_string());
            copy_tree(&source_record, &copied.join(issue.to_string()));
        }
        let registry = linked.join("docs/templates/prompts");
        fs::create_dir_all(&registry).unwrap();
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/templates/prompts/current.json"),
            registry.join("current.json"),
        )
        .unwrap();
        let authority = linked.join("csdlc-v3/operator/authority-selector.json");
        fs::create_dir_all(authority.parent().unwrap()).unwrap();
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../csdlc-v3/operator/authority-selector.json"),
            &authority,
        )
        .unwrap();
        Self {
            root,
            primary,
            linked,
            git_common,
            copied,
            linked_head,
        }
    }

    fn operation(&self, suffix: &str) -> String {
        format!("issue872-{suffix}-{}", std::process::id())
    }

    fn request_value(&self, operation_id: &str, fault: Option<Value>) -> Value {
        let records = RECORDS
            .iter()
            .map(|(issue, role)| {
                json!({"issue": issue, "role": role, "source": self.copied.join(issue.to_string())})
            })
            .collect::<Vec<_>>();
        let mut request = json!({
            "schema": "csdlc.v3.copied_record_conversion.v1",
            "repository": "agent-logic/agent-design-language",
            "operation_id": operation_id,
            "authority_bytes_path": self.linked.join("csdlc-v3/operator/authority-selector.json"),
            "prior_executable_path": env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"),
            "prior_executable_blake3": blake3::hash(&fs::read(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal")).unwrap()).to_hex().to_string(),
            "writer_fence_issues": [511, 517, 497, 3, 505, 122, 113, 868],
            "writer_probe_issue": 868,
            "git_common": self.git_common,
            "linked_branch": "codex/fixture-linked",
            "linked_head": self.linked_head,
            "linked_worktree": self.linked,
            "records": records,
            "registry_path": self.linked.join("docs/templates/prompts/current.json")
        });
        if let Some(fault) = fault {
            request["fault_injection"] = fault;
        }
        request
    }

    fn write_request(&self, operation_id: &str, fault: Option<Value>) -> PathBuf {
        self.write_value(
            &format!("{operation_id}.json"),
            &self.request_value(operation_id, fault),
        )
    }

    fn write_value(&self, name: &str, value: &Value) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn git(args: &[&str], cwd: &Path) {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(args: &[&str], cwd: &Path) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).unwrap();
        }
    }
}

fn all_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(all_files(&path));
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    files
}
