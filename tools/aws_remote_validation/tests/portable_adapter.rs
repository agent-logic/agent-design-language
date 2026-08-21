#[path = "../src/aws_remote_validation.rs"]
mod aws_remote_validation;

use aws_remote_validation::portable_remote_validation::{
    command_profile_digest, AdapterKind, ArtifactPolicy, CommandProfile, FallbackPolicy,
    PortableRequest, ResourceBudget, REQUEST_SCHEMA,
};
use aws_remote_validation::{
    apply_portable_aws_request, portable_aws_adapter_plan, AwsRemoteValidationConfig,
};
use std::path::PathBuf;

fn request(adapter: AdapterKind) -> PortableRequest {
    let profile = CommandProfile {
        argv: vec!["cargo".into(), "test".into(), "--locked".into()],
        working_directory: ".".into(),
        environment_allowlist: vec!["PATH".into()],
    };
    PortableRequest {
        schema: REQUEST_SCHEMA.into(),
        request_id: "wp-5823-aws-adapter".into(),
        checkout: ".".into(),
        revision: "a".repeat(40),
        source_ref: Some("refs/heads/codex/wp-5823-fixture".into()),
        command_profile_digest: command_profile_digest(&profile).unwrap(),
        command_profile: profile,
        adapter,
        requested_platform: "linux".into(),
        resource_budget: ResourceBudget {
            cpu_cores: 8,
            memory_mib: 32768,
            timeout_seconds: 900,
            estimated_max_cost_microusd: Some(150_000),
        },
        artifact_policy: ArtifactPolicy {
            paths: vec!["artifacts/summary.json".into()],
            required: true,
            max_total_bytes: 1_048_576,
        },
        cancellation_file: None,
        fallback: FallbackPolicy::OfferLocal,
    }
}

fn config() -> AwsRemoteValidationConfig {
    AwsRemoteValidationConfig {
        issue: Some(5823),
        run_id: "portable-fixture".into(),
        region: "us-west-2".into(),
        profile: Some("agent-logic-admin".into()),
        repo_url: "https://github.com/agent-logic/agent-design-language.git".into(),
        git_ref: "old-ref".into(),
        cache_bucket: None,
        cache_prefix: None,
        sccache_tarball_url: None,
        nextest_tarball_url: None,
        ssh_key_name: None,
        ssh_private_key_path: None,
        ssh_user: None,
        ssh_allowed_cidr: None,
        cache_volume_id: None,
        cache_volume_name: None,
        cache_volume_size_gib: None,
        cache_volume_type: None,
        cache_volume_iops: None,
        cache_volume_throughput_mbps: None,
        cache_volume_device_name: None,
        cache_volume_mount_path: None,
        command: "old-command".into(),
        out_path: PathBuf::from("summary.json"),
        artifact_dir: PathBuf::from("artifacts"),
        ami_id: "ami-fixture".into(),
        subnet_id: "subnet-fixture".into(),
        security_group_id: "sg-fixture".into(),
        instance_profile_name: "profile-fixture".into(),
        instance_types: vec!["m7a.2xlarge".into()],
        on_demand_only: false,
        allow_on_demand_fallback: false,
        budget_name: None,
        expected_max_cost_usd: Some(1.0),
        estimated_hourly_cost_usd: Some(0.1),
        cancellation_file: None,
        total_run_timeout_seconds: Some(3600),
        poll_interval_seconds: 1,
        ssm_ready_timeout_seconds: 60,
        command_timeout_seconds: None,
        termination_timeout_seconds: 60,
    }
}

#[test]
fn portable_request_maps_exactly_to_aws_adapter_inputs() {
    let request = request(AdapterKind::Aws);
    let plan = portable_aws_adapter_plan(&request).unwrap();
    assert_eq!(plan.revision, "a".repeat(40));
    assert_eq!(
        plan.source_ref.as_deref(),
        Some("refs/heads/codex/wp-5823-fixture")
    );
    assert_eq!(plan.working_directory, ".");
    assert_eq!(plan.environment_allowlist, vec!["PATH"]);
    assert_eq!(
        plan.shell_command,
        "cd -- '.' && env -i PATH=\"${PATH-}\" 'cargo' 'test' '--locked'"
    );
    assert_eq!(plan.resource_budget, request.resource_budget);
    assert_eq!(plan.artifact_policy, request.artifact_policy);
    assert_eq!(plan.cancellation_file, request.cancellation_file);

    let mut config = config();
    apply_portable_aws_request(&mut config, &request).unwrap();
    assert_eq!(config.git_ref, "refs/heads/codex/wp-5823-fixture");
    assert_eq!(
        config.command,
        "cd -- '.' && env -i PATH=\"${PATH-}\" 'cargo' 'test' '--locked'"
    );
    assert_eq!(config.command_timeout_seconds, Some(900));
    assert_eq!(config.expected_max_cost_usd, Some(0.15));
    assert_eq!(config.total_run_timeout_seconds, Some(900));
}

#[test]
fn cumulative_provider_cost_is_rejected_before_launch() {
    let mut config = config();
    config.expected_max_cost_usd = Some(0.20);
    config.estimated_hourly_cost_usd = Some(1.0);
    config.total_run_timeout_seconds = Some(900);
    assert!(config
        .validate()
        .unwrap_err()
        .to_string()
        .contains("projected cumulative provider cost exceeds"));
}

#[test]
fn declared_cost_ceiling_requires_rate_and_total_timeout() {
    let mut missing_rate = config();
    missing_rate.expected_max_cost_usd = Some(0.20);
    missing_rate.estimated_hourly_cost_usd = None;
    missing_rate.total_run_timeout_seconds = Some(900);
    assert!(missing_rate
        .validate()
        .unwrap_err()
        .to_string()
        .contains("estimated hourly cost is required"));

    let mut missing_timeout = config();
    missing_timeout.expected_max_cost_usd = Some(0.20);
    missing_timeout.estimated_hourly_cost_usd = Some(0.10);
    missing_timeout.total_run_timeout_seconds = None;
    assert!(missing_timeout
        .validate()
        .unwrap_err()
        .to_string()
        .contains("total run timeout is required"));
}

#[test]
fn provider_work_requires_an_explicit_cost_ceiling() {
    let mut config = config();
    config.expected_max_cost_usd = None;
    assert!(config
        .validate()
        .unwrap_err()
        .to_string()
        .contains("expected maximum cost is required"));
}

#[test]
fn aws_adapter_rejects_other_provider_requests() {
    let request = request(AdapterKind::Nessus);
    assert!(portable_aws_adapter_plan(&request).is_err());
}
