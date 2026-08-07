use adl_remote_validation::*;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn revision(root: &Path) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .expect("git revision");
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

fn fixture_checkout(label: &str) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let checkout = std::env::temp_dir().join(format!("adl-wp-5823-{label}-{unique}"));
    std::fs::create_dir_all(&checkout).unwrap();
    let git = |args: &[&str]| {
        let status = Command::new("git")
            .args(args)
            .current_dir(&checkout)
            .status()
            .unwrap();
        assert!(status.success());
    };
    git(&["init", "--quiet"]);
    git(&["config", "user.name", "WP-06 Fixture"]);
    git(&["config", "user.email", "wp06@example.invalid"]);
    std::fs::write(checkout.join("tracked.txt"), "clean\n").unwrap();
    git(&["add", "tracked.txt"]);
    git(&["commit", "--quiet", "-m", "fixture"]);
    checkout
}

fn request(adapter: AdapterKind, argv: Vec<String>) -> PortableRequest {
    let profile = CommandProfile {
        argv,
        working_directory: ".".into(),
        environment_allowlist: vec!["PATH".into()],
    };
    PortableRequest {
        schema: REQUEST_SCHEMA.into(),
        request_id: "wp-5823-contract".into(),
        checkout: ".".into(),
        revision: revision(&root()),
        command_profile_digest: command_profile_digest(&profile).unwrap(),
        command_profile: profile,
        adapter,
        requested_platform: "windows".into(),
        resource_budget: ResourceBudget {
            cpu_cores: 2,
            memory_mib: 1024,
            timeout_seconds: 2,
            estimated_max_cost_microusd: None,
        },
        artifact_policy: ArtifactPolicy {
            paths: vec!["tools/remote_validation/Cargo.toml".into()],
            required: true,
            max_total_bytes: 64 * 1024,
        },
        cancellation_file: None,
        fallback: if adapter == AdapterKind::Local {
            FallbackPolicy::Disabled
        } else {
            FallbackPolicy::OfferLocal
        },
    }
}

fn passed_result(request: &PortableRequest) -> PortableResult {
    PortableResult {
        schema: RESULT_SCHEMA.into(),
        request_id: request.request_id.clone(),
        adapter: request.adapter,
        platform: PlatformRecord {
            os: "windows".into(),
            architecture: "x86_64".into(),
            native: false,
            qualification: "fixture".into(),
        },
        revision: request.revision.clone(),
        command_profile_digest: request.command_profile_digest.clone(),
        started_unix_ms: 10,
        finished_unix_ms: 20,
        exit_code: Some(0),
        outcome: RunOutcome::Passed,
        artifact_digests: vec![ArtifactDigest {
            path: "tools/remote_validation/Cargo.toml".into(),
            sha256: "a".repeat(64),
            bytes: 1,
        }],
        redaction_passed: true,
        cleanup: CleanupStatus {
            attempted: true,
            complete: true,
            detail: None,
        },
        fallback: FallbackStatus {
            policy: request.fallback,
            offered: false,
            ran: false,
            local_profile_digest: None,
        },
    }
}

fn local_request(checkout: &Path, argv: Vec<String>) -> PortableRequest {
    let mut value = request(AdapterKind::Local, argv);
    value.revision = revision(checkout);
    value.requested_platform = std::env::consts::OS.into();
    value.artifact_policy.paths = vec!["tracked.txt".into()];
    value
}

#[test]
fn request_round_trip_and_adapter_plan_preserve_provenance() {
    let request = request(AdapterKind::Nessus, vec!["cargo".into(), "check".into()]);
    validate_request(&request).unwrap();
    let encoded = serde_json::to_vec(&request).unwrap();
    let decoded: PortableRequest = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded, request);
    assert_eq!(
        select_adapter(&request, &[AdapterKind::Nessus]).unwrap(),
        AdapterKind::Nessus
    );
    let plan = adapter_plan(&request, AdapterKind::Nessus).unwrap();
    assert_eq!(plan.revision, request.revision);
    assert_eq!(plan.command_profile_digest, request.command_profile_digest);
    assert_eq!(plan.shell_command, "'cargo' 'check'");
    assert_eq!(plan.cpu_cores, 2);
    assert_eq!(plan.memory_mib, 1024);
}

#[test]
fn request_rejects_stale_digest_absolute_paths_and_secret_environment() {
    let mut value = request(AdapterKind::Aws, vec!["cargo".into(), "test".into()]);
    value.command_profile_digest = "0".repeat(64);
    assert!(validate_request(&value)
        .unwrap_err()
        .contains("digest mismatch"));

    let mut value = request(AdapterKind::Aws, vec!["cargo".into(), "test".into()]);
    value.artifact_policy.paths = vec!["/Users/operator/private.log".into()];
    assert!(validate_request(&value)
        .unwrap_err()
        .contains("repository-relative"));

    let mut value = request(AdapterKind::Aws, vec!["cargo".into(), "test".into()]);
    value
        .command_profile
        .environment_allowlist
        .push("AWS_SECRET_ACCESS_KEY".into());
    value.command_profile_digest = command_profile_digest(&value.command_profile).unwrap();
    assert!(validate_request(&value)
        .unwrap_err()
        .contains("secret-bearing"));
}

#[test]
fn adapter_selection_and_shell_encoding_fail_closed() {
    let value = request(
        AdapterKind::Aws,
        vec!["cargo".into(), "test; rm -rf nope".into()],
    );
    assert!(select_adapter(&value, &[AdapterKind::Local]).is_err());
    assert!(select_adapter(&value, &[AdapterKind::Aws, AdapterKind::Aws]).is_err());
    let plan = adapter_plan(&value, AdapterKind::Aws).unwrap();
    assert_eq!(plan.shell_command, "'cargo' 'test; rm -rf nope'");
    assert!(adapter_plan(&value, AdapterKind::Nessus).is_err());
}

#[test]
fn fallback_is_same_profile_and_never_hides_bad_remote_proof() {
    let value = request(AdapterKind::Aws, vec!["cargo".into(), "test".into()]);
    let allowed = fallback_decision(
        &value,
        ProviderFailure::Unreachable,
        &value.command_profile_digest,
    );
    assert!(allowed.allowed);
    assert!(!allowed.run_local);
    assert!(
        !fallback_decision(
            &value,
            ProviderFailure::StaleRevision,
            &value.command_profile_digest
        )
        .allowed
    );
    assert!(
        !fallback_decision(
            &value,
            ProviderFailure::CleanupIncomplete,
            &value.command_profile_digest
        )
        .allowed
    );
    assert!(!fallback_decision(&value, ProviderFailure::Capacity, &"f".repeat(64)).allowed);
}

#[test]
fn result_rejects_malformed_provenance_redaction_and_cleanup() {
    let value = request(AdapterKind::Aws, vec!["cargo".into(), "test".into()]);
    let mut result = passed_result(&value);
    validate_result(&value, &result).unwrap();

    result.revision = "0".repeat(40);
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("provenance"));
    result = passed_result(&value);
    result.redaction_passed = false;
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("redaction"));
    result = passed_result(&value);
    result.cleanup.complete = false;
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("cleanup"));
    result = passed_result(&value);
    result.platform.os = "linux".into();
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("platform"));
    result = passed_result(&value);
    result.platform.qualification = "live".into();
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("platform"));
    result = passed_result(&value);
    result.artifact_digests[0].path = "/private/result.log".into();
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("artifact provenance"));
    result = passed_result(&value);
    result.artifact_digests[0].path = "undeclared.txt".into();
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("declared policy"));
    result = passed_result(&value);
    result.artifact_digests[0].bytes = value.artifact_policy.max_total_bytes + 1;
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("declared policy"));
    result = passed_result(&value);
    result.fallback.ran = true;
    assert!(validate_result(&value, &result)
        .unwrap_err()
        .contains("fallback"));
}

#[test]
fn local_native_execution_records_artifact_and_exact_revision() {
    let checkout = fixture_checkout("native");
    let value = local_request(&checkout, vec!["/usr/bin/true".into()]);
    let result = run_local(&value, &checkout).unwrap();
    assert_eq!(result.outcome, RunOutcome::Passed);
    assert!(result.platform.native);
    assert_eq!(result.revision, value.revision);
    assert_eq!(result.artifact_digests.len(), 1);
    validate_result(&value, &result).unwrap();
    std::fs::remove_dir_all(checkout).unwrap();
}

#[test]
fn local_timeout_and_cancellation_kill_and_reap_the_child() {
    let checkout = fixture_checkout("timeout-cancel");
    let mut timed = local_request(&checkout, vec!["/bin/sleep".into(), "5".into()]);
    timed.resource_budget.timeout_seconds = 1;
    let result = run_local(&timed, &checkout).unwrap();
    assert_eq!(result.outcome, RunOutcome::TimedOut);
    assert!(result.cleanup.attempted && result.cleanup.complete);

    let mut cancelled = local_request(&checkout, vec!["/bin/sleep".into(), "5".into()]);
    cancelled.cancellation_file = Some("tracked.txt".into());
    let result = run_local(&cancelled, &checkout).unwrap();
    assert_eq!(result.outcome, RunOutcome::Cancelled);
    assert!(result.cleanup.attempted && result.cleanup.complete);
    std::fs::remove_dir_all(checkout).unwrap();
}

#[test]
fn local_execution_rejects_stale_revision_before_spawning() {
    let checkout = fixture_checkout("stale");
    let mut value = local_request(&checkout, vec!["/usr/bin/true".into()]);
    value.revision = "0".repeat(40);
    assert!(run_local(&value, &checkout)
        .unwrap_err()
        .contains("stale revision"));
    std::fs::remove_dir_all(checkout).unwrap();
}

#[test]
fn local_execution_rejects_dirty_tracked_source() {
    let checkout = fixture_checkout("dirty");
    let clean_revision = revision(&checkout);
    verify_checkout_revision(&checkout, &clean_revision).unwrap();
    std::fs::write(checkout.join("tracked.txt"), "dirty\n").unwrap();
    assert!(verify_checkout_revision(&checkout, &clean_revision)
        .unwrap_err()
        .contains("outside untracked evidence"));
    std::fs::remove_dir_all(checkout).unwrap();
}
