use adl::agent_comms::{
    AcipAddressKindV1, AcipAddressV1, AcipAttachmentRefV1, AcipIntentV1, AcipMessageEnvelopeV1,
    AcipPayloadRefV1, AcipRouteClassV1, AcipTraceRequirementV1, AcipVisibilityV1,
};
use adl::runtime_aws_signal::{
    publish_acip_sns_projection_signal, AcipSnsProjectionRequest, PublishDisposition,
};
use anyhow::{bail, Context, Result};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut run_id = "wp08-4685-acip-sns".to_string();
    let mut projection_level = "content_summary".to_string();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(
                    args.next().context("--out requires a value")?,
                ))
            }
            "--run-id" => run_id = args.next().context("--run-id requires a value")?,
            "--projection-level" => {
                projection_level = args.next().context("--projection-level requires a value")?
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: run_wp08_acip_sns_live_proof --out <dir> [--run-id <id>] [--projection-level delivery_metadata|content_summary]"
                );
                return Ok(());
            }
            other => bail!("unknown argument: {other}"),
        }
    }

    let out_dir = out_dir.context("--out is required")?;
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed creating {}", out_dir.display()))?;

    let message = sample_acip_message();
    let request = AcipSnsProjectionRequest {
        runtime_id: "wp08-acip-sns-4685",
        agent_id: "wp08-runtime-signal",
        cycle_id: Some("cycle-wp08-acip-sns-0001"),
        message: &message,
        route_class: AcipRouteClassV1::CrossBoundaryDeferred,
        projection_level: projection_level.as_str(),
        message_ref: "runtime/wp08/acip/messages/msg-wp08-acip-sns-0001.json",
        trace_ref: Some("runtime/wp08/acip/traces/public-summary.json"),
    };

    let outcome = publish_acip_sns_projection_signal(&out_dir, &request);
    let status = match outcome.disposition {
        PublishDisposition::PublishedLive => "passed",
        PublishDisposition::PublishedMock => "failed_mock_not_live",
        PublishDisposition::Skipped => "failed_skipped",
        PublishDisposition::Blocked => "failed_blocked",
    };

    let topic_arn = env::var("ADL_AWS_SNS_TOPIC_ARN").unwrap_or_default();
    let account_hash = env::var("ADL_AWS_ACCOUNT_HASH").unwrap_or_else(|_| "unknown".to_string());
    let account_sha256 =
        env::var("ADL_AWS_ACCOUNT_SHA256").unwrap_or_else(|_| "unknown".to_string());
    let summary = json!({
        "schema": "adl.wp08.acip_sns_live_proof.v1",
        "issue": 4685,
        "status": status,
        "run_id": run_id,
        "aws_profile": env::var("ADL_AWS_PROFILE").or_else(|_| env::var("AWS_PROFILE")).unwrap_or_default(),
        "aws_region": env::var("ADL_AWS_REGION").unwrap_or_default(),
        "aws_account_hash": account_hash,
        "aws_account_sha256": account_sha256,
        "sns": {
            "topic_arn_hash": short_hash(&topic_arn),
            "topic_name": topic_arn.rsplit(':').next().unwrap_or("unknown"),
            "message_id": outcome.provider_message_id,
        },
        "acip_projection": {
            "schema_version": "adl.runtime.aws_signal.v1",
            "signal_kind": "acip_projection",
            "runtime_id": request.runtime_id,
            "cycle_id": request.cycle_id.unwrap_or("not_applicable"),
            "route_class": "cross_boundary_deferred",
            "projection_level": request.projection_level,
            "message_ref": request.message_ref,
            "trace_ref": request.trace_ref,
            "correlation_id": "wp08-acip-sns-correlation-0001",
            "content_sha256_recorded": request.projection_level == "content_summary",
        },
        "negative_case_policy": {
            "missing_profile": "aws_acip_sns_profile_missing",
            "missing_topic": "aws_acip_sns_topic_missing",
            "malformed_or_denied_projection": "projection_denied",
            "sns_unavailable_or_access_denied": "aws_acip_sns_publish_failed",
        },
        "redaction": {
            "raw_account_id_recorded": false,
            "raw_topic_arn_recorded": false,
            "credentials_recorded": false,
            "raw_message_content_recorded": false,
        }
    });
    let summary_path = out_dir.join("acip_sns_summary.json");
    fs::write(&summary_path, serde_json::to_string_pretty(&summary)?)
        .with_context(|| format!("failed writing {}", summary_path.display()))?;

    if !matches!(outcome.disposition, PublishDisposition::PublishedLive) {
        bail!(
            "ACIP SNS live proof did not publish live: status={status} failure_class={:?}",
            outcome.failure_class
        );
    }

    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}

fn sample_acip_message() -> AcipMessageEnvelopeV1 {
    AcipMessageEnvelopeV1 {
        schema_version: "acip.message.v1".to_string(),
        message_id: "msg-wp08-acip-sns-0001".to_string(),
        conversation_id: "conv-wp08-acip-sns-0001".to_string(),
        timestamp_utc: "2026-07-06T16:00:00Z".to_string(),
        monotonic_order: 1,
        sender: AcipAddressV1 {
            kind: AcipAddressKindV1::Agent,
            id: "wp08-runtime-signal".to_string(),
        },
        recipient: AcipAddressV1 {
            kind: AcipAddressKindV1::Group,
            id: "approval-gated-sns-subscribers".to_string(),
        },
        intent: AcipIntentV1::Delegation,
        visibility: AcipVisibilityV1::Shared,
        trace_requirement: AcipTraceRequirementV1::Summary,
        content: "private runtime coordination content intentionally excluded from live summary"
            .to_string(),
        payload_refs: vec![AcipPayloadRefV1 {
            payload_kind: "runtime_signal".to_string(),
            payload_ref: "runtime/wp08/acip/signal.json".to_string(),
            media_type: "application/json".to_string(),
            content_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_string(),
            byte_length: 128,
            inline_summary: Some("bounded ACIP SNS signal proof".to_string()),
        }],
        artifact_refs: vec!["runtime/wp08/acip/signal.json".to_string()],
        attachments: Vec::<AcipAttachmentRefV1>::new(),
        authority_scope: None,
        correlation_id: Some("wp08-acip-sns-correlation-0001".to_string()),
        prior_message_id: None,
    }
}

fn short_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
        .chars()
        .take(16)
        .collect()
}
