// Fixture constructors adapted from frozen tools/remote_validation/tests/contract.rs.
use adl_remote_validation::*;

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
        revision: "248f00e359e412f6bc0061ac88be9ff374e3a212".into(),
        source_ref: if adapter == AdapterKind::Local {
            None
        } else {
            Some("refs/heads/codex/wp-5823-fixture".into())
        },
        command_profile_digest: command_profile_digest(&profile).unwrap(),
        command_profile: profile,
        adapter,
        requested_platform: "windows".into(),
        resource_budget: ResourceBudget {
            cpu_cores: 2,
            memory_mib: 1024,
            timeout_seconds: 2,
            estimated_max_cost_microusd: (adapter == AdapterKind::Aws).then_some(200_000),
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
        resource_budget: request.resource_budget.clone(),
        artifact_policy: request.artifact_policy.clone(),
        cancellation_file: request.cancellation_file.clone(),
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

fn main() {
    let request = request(AdapterKind::Aws, vec!["cargo".into(), "check".into()]);
    let plan: AdapterPlan = adapter_plan(&request, AdapterKind::Aws).unwrap();
    assert_eq!(plan.revision, request.revision);
    assert_eq!(plan.source_ref, request.source_ref);
    assert_eq!(plan.resource_budget, request.resource_budget);
    assert_eq!(plan.cancellation_file, request.cancellation_file);
    assert!(adapter_plan(&request, AdapterKind::Local).is_err());
    let result: PortableResult = passed_result(&request);
    validate_result(&request, &result).unwrap();
    let mut wrong = result.clone();
    wrong.revision = "0".repeat(40);
    assert!(validate_result(&request, &wrong).is_err());
    println!("artifact consumer: six API symbols, provenance, budget, cancellation and rejection checks passed");
}
