use crate::observability::emit_event;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SCHEMA: &str = "adl.wp08.cloud_control_cloudfront.v1";
const EVENT_SCHEMA: &str = "adl.runtime.cloud_control.event.v1";

#[derive(Debug, Clone)]
pub struct CloudFrontStatusOptions {
    pub out_dir: PathBuf,
    pub run_id: String,
    pub profile: String,
    pub region: String,
    pub expected_account_sha256: String,
    pub distribution_id: Option<String>,
    pub negative_distribution_id: Option<String>,
    pub aws_bin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontStatusSummary {
    pub schema: String,
    pub issue: u32,
    pub status: String,
    pub run_id: String,
    pub aws_profile: String,
    pub aws_region: String,
    pub aws_account_hash: String,
    pub cloudfront: CloudFrontStateSummary,
    pub event_schema: Value,
    pub negative_case_policy: Value,
    pub live_negative_cases: Value,
    pub redaction: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontStateSummary {
    pub distribution_count: usize,
    pub selected_distribution_id_hash: String,
    pub selected_domain_name_hash: String,
    pub selected_status: String,
    pub selected_enabled: bool,
    pub alias_count: usize,
    pub last_modified_time_present: bool,
    pub etag_hash: Option<String>,
}

pub fn prove_cloudfront_status(
    options: CloudFrontStatusOptions,
) -> Result<CloudFrontStatusSummary> {
    fs::create_dir_all(&options.out_dir)
        .with_context(|| format!("failed creating {}", options.out_dir.display()))?;

    let account = aws_output(
        &options.aws_bin,
        &[
            "sts",
            "get-caller-identity",
            "--profile",
            &options.profile,
            "--region",
            &options.region,
            "--query",
            "Account",
            "--output",
            "text",
        ],
    )
    .map_err(|err| anyhow::anyhow!(classify_aws_error(&err)))?;
    let account = account.trim();
    let account_sha = sha256(account);
    if account_sha != options.expected_account_sha256 {
        emit_cloud_control_event(
            "auth_denial",
            "blocked",
            &options.run_id,
            Some("cloud_control_account_hash_mismatch"),
        );
        bail!("AWS profile did not resolve to the approved Agent Logic account hash");
    }
    let account_hash = short_hash(&account_sha);

    emit_cloud_control_event("poll", "started", &options.run_id, None);
    let list_output = aws_output(
        &options.aws_bin,
        &[
            "cloudfront",
            "list-distributions",
            "--profile",
            &options.profile,
            "--output",
            "json",
        ],
    )
    .map_err(|err| anyhow::anyhow!(classify_aws_error(&err)))?;
    let list_json: Value =
        serde_json::from_str(&list_output).context("parse list-distributions")?;
    let items = list_json
        .pointer("/DistributionList/Items")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if items.is_empty() {
        emit_cloud_control_event(
            "unavailable_service",
            "blocked",
            &options.run_id,
            Some("cloudfront_distribution_not_provisioned"),
        );
        bail!("no CloudFront distributions are provisioned in the Agent Logic account");
    }

    let selected = select_distribution(&items, options.distribution_id.as_deref())?;
    let selected_id = selected
        .get("Id")
        .and_then(Value::as_str)
        .context("selected distribution missing Id")?;
    let selected_status = selected
        .get("Status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    emit_cloud_control_event("state_change", selected_status, &options.run_id, None);

    let get_output = aws_output(
        &options.aws_bin,
        &[
            "cloudfront",
            "get-distribution",
            "--profile",
            &options.profile,
            "--id",
            selected_id,
            "--output",
            "json",
        ],
    )
    .map_err(|err| anyhow::anyhow!(classify_aws_error(&err)))?;
    let get_json: Value = serde_json::from_str(&get_output).context("parse get-distribution")?;
    let etag_hash = get_json.get("ETag").and_then(Value::as_str).map(short_hash);

    let negative = run_negative_distribution_case(&options)?;
    let summary = CloudFrontStatusSummary {
        schema: SCHEMA.to_string(),
        issue: 4915,
        status: "passed".to_string(),
        run_id: options.run_id.clone(),
        aws_profile: options.profile.clone(),
        aws_region: options.region.clone(),
        aws_account_hash: account_hash,
        cloudfront: CloudFrontStateSummary {
            distribution_count: items.len(),
            selected_distribution_id_hash: short_hash(selected_id),
            selected_domain_name_hash: short_hash(
                selected
                    .get("DomainName")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
            selected_status: selected_status.to_string(),
            selected_enabled: selected
                .get("Enabled")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            alias_count: selected
                .pointer("/Aliases/Quantity")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize,
            last_modified_time_present: selected.get("LastModifiedTime").is_some(),
            etag_hash,
        },
        event_schema: json!({
            "schema": EVENT_SCHEMA,
            "event_kinds": ["poll", "state_change", "auth_denial", "throttling", "unavailable_service"],
            "redaction": "only account, distribution, domain, and etag hashes are retained"
        }),
        negative_case_policy: json!({
            "missing_profile": "cloud_control_profile_missing",
            "access_denied": "cloud_control_access_denied",
            "throttling": "cloud_control_throttled",
            "nonexistent_distribution": "cloudfront_distribution_not_found",
            "unavailable_service": "cloudfront_unavailable_or_not_provisioned"
        }),
        live_negative_cases: negative,
        redaction: json!({
            "raw_account_id_recorded": false,
            "raw_distribution_id_recorded": false,
            "raw_domain_name_recorded": false,
            "credentials_recorded": false
        }),
    };
    let summary_path = options.out_dir.join("cloudfront_status_summary.json");
    fs::write(
        &summary_path,
        serde_json::to_string_pretty(&summary)? + "\n",
    )
    .with_context(|| format!("failed writing {}", summary_path.display()))?;
    emit_cloud_control_event("poll", "completed", &options.run_id, None);
    Ok(summary)
}

fn select_distribution(items: &[Value], distribution_id: Option<&str>) -> Result<Value> {
    if let Some(id) = distribution_id {
        return items
            .iter()
            .find(|item| item.get("Id").and_then(Value::as_str) == Some(id))
            .cloned()
            .with_context(|| {
                "requested CloudFront distribution was not returned by list-distributions"
            });
    }
    items
        .iter()
        .find(|item| {
            item.get("Enabled")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        })
        .or_else(|| items.first())
        .cloned()
        .context("no CloudFront distributions available")
}

fn run_negative_distribution_case(options: &CloudFrontStatusOptions) -> Result<Value> {
    let Some(id) = options.negative_distribution_id.as_deref() else {
        return Ok(json!({
            "nonexistent_distribution": "not_run",
            "reason": "no negative distribution id supplied"
        }));
    };
    match aws_output(
        &options.aws_bin,
        &[
            "cloudfront",
            "get-distribution",
            "--profile",
            &options.profile,
            "--id",
            id,
            "--output",
            "json",
        ],
    ) {
        Ok(_) => {
            emit_cloud_control_event(
                "unavailable_service",
                "blocked",
                &options.run_id,
                Some("cloudfront_negative_case_unexpected_success"),
            );
            bail!("CloudFront nonexistent-distribution negative case unexpectedly succeeded")
        }
        Err(err) => {
            let class = classify_aws_error(&err);
            if class == "cloudfront_distribution_not_found" {
                emit_cloud_control_event(
                    "unavailable_service",
                    "not_found",
                    &options.run_id,
                    Some(&class),
                );
            }
            Ok(json!({
                "nonexistent_distribution": class,
                "raw_error_recorded": false
            }))
        }
    }
}

fn aws_output(aws_bin: &str, args: &[&str]) -> std::result::Result<String, String> {
    let output = Command::new(aws_bin)
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn classify_aws_error(error: &str) -> String {
    let lower = error.to_ascii_lowercase();
    if lower.contains("nosuchdistribution")
        || lower.contains("the specified distribution does not exist")
    {
        "cloudfront_distribution_not_found".to_string()
    } else if lower.contains("throttl") || lower.contains("too many requests") {
        "cloud_control_throttled".to_string()
    } else if lower.contains("accessdenied")
        || lower.contains("access denied")
        || lower.contains("not authorized")
        || lower.contains("unauthorized")
    {
        "cloud_control_access_denied".to_string()
    } else if lower.contains("profile") && lower.contains("could not be found") {
        "cloud_control_profile_missing".to_string()
    } else {
        "cloudfront_unavailable_or_not_provisioned".to_string()
    }
}

fn emit_cloud_control_event(
    event_kind: &str,
    result: &str,
    run_id: &str,
    failure_class: Option<&str>,
) {
    let failure = failure_class.unwrap_or("none");
    emit_event(
        "csm",
        "cloud_control",
        result,
        &[
            ("schema", EVENT_SCHEMA),
            ("provider", "aws"),
            ("service", "cloudfront"),
            ("event_kind", event_kind),
            ("run_id", run_id),
            ("failure_class", failure),
        ],
    );
}

fn sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn short_hash(value: &str) -> String {
    sha256(value).chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::{classify_aws_error, prove_cloudfront_status, CloudFrontStatusOptions};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn cloudfront_error_classification_covers_negative_cases() {
        assert_eq!(
            classify_aws_error("An error occurred (NoSuchDistribution)"),
            "cloudfront_distribution_not_found"
        );
        assert_eq!(
            classify_aws_error("AccessDenied: not authorized"),
            "cloud_control_access_denied"
        );
        assert_eq!(
            classify_aws_error("Throttling: Rate exceeded"),
            "cloud_control_throttled"
        );
        assert_eq!(
            classify_aws_error("The config profile (missing) could not be found"),
            "cloud_control_profile_missing"
        );
    }

    #[test]
    fn cloudfront_status_writes_redacted_summary_with_fake_aws() {
        let root = temp_dir("cloudfront-status");
        let aws = write_fake_aws(&root, false);
        let account_sha =
            "2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972".to_string();
        let out_dir = root.join("proof");
        let summary = prove_cloudfront_status(CloudFrontStatusOptions {
            out_dir: out_dir.clone(),
            run_id: "fixture-run".to_string(),
            profile: "agent-logic-admin".to_string(),
            region: "us-west-2".to_string(),
            expected_account_sha256: account_sha,
            distribution_id: None,
            negative_distribution_id: Some("E-NOTFOUND".to_string()),
            aws_bin: aws.display().to_string(),
        })
        .expect("fake AWS proof");

        assert_eq!(summary.status, "passed");
        assert_eq!(summary.cloudfront.distribution_count, 1);
        assert_eq!(summary.cloudfront.selected_status, "Deployed");
        assert_eq!(
            summary.live_negative_cases["nonexistent_distribution"],
            "cloudfront_distribution_not_found"
        );
        let text = fs::read_to_string(out_dir.join("cloudfront_status_summary.json")).unwrap();
        assert!(!text.contains("123456789012"));
        assert!(!text.contains("E123ABC"));
        assert!(!text.contains("example.cloudfront.net"));
    }

    #[test]
    fn cloudfront_status_fails_closed_when_negative_case_unexpectedly_succeeds() {
        let root = temp_dir("cloudfront-negative-success");
        let aws = write_fake_aws(&root, true);
        let account_sha =
            "2a33349e7e606a8ad2e30e3c84521f9377450cf09083e162e0a9b1480ce0f972".to_string();
        let out_dir = root.join("proof");
        let error = prove_cloudfront_status(CloudFrontStatusOptions {
            out_dir: out_dir.clone(),
            run_id: "fixture-run".to_string(),
            profile: "agent-logic-admin".to_string(),
            region: "us-west-2".to_string(),
            expected_account_sha256: account_sha,
            distribution_id: None,
            negative_distribution_id: Some("E-NOTFOUND".to_string()),
            aws_bin: aws.display().to_string(),
        })
        .expect_err("unexpectedly successful negative case must fail closed");

        assert!(
            error.to_string().contains("unexpectedly succeeded"),
            "{error}"
        );
        assert!(!out_dir.join("cloudfront_status_summary.json").exists());
    }

    fn temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("adl-{label}-{unique}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_fake_aws(root: &Path, negative_succeeds: bool) -> PathBuf {
        let path = root.join("aws");
        let negative_exit = if negative_succeeds { 0 } else { 255 };
        fs::write(
            &path,
            r#"#!/usr/bin/env bash
		set -euo pipefail
		case "$1 $2" in
		  "sts get-caller-identity")
    printf '%s\n' "123456789012"
    ;;
	  "cloudfront list-distributions")
	    printf '%s\n' '{"DistributionList":{"Quantity":1,"Items":[{"Id":"E123ABC","DomainName":"example.cloudfront.net","Status":"Deployed","Enabled":true,"LastModifiedTime":"2026-07-06T00:00:00Z","Aliases":{"Quantity":2,"Items":["polis.example.com","www.example.com"]}}]}}'
	    ;;
  "cloudfront get-distribution")
    id=""
    while [ "$#" -gt 0 ]; do
      if [ "$1" = "--id" ]; then
        id="${2:-}"
        break
      fi
      shift
	    done
		    if [ "$id" = "E-NOTFOUND" ]; then
		      echo "An error occurred (NoSuchDistribution) when calling the GetDistribution operation" >&2
		      exit __NEGATIVE_EXIT__
		    fi
		    printf '%s\n' '{"ETag":"E-TAG-123","Distribution":{"Id":"E123ABC"}}'
		    ;;
		  *)
    echo "unexpected aws args: $*" >&2
    exit 2
		    ;;
		esac
		"#
            .replace("__NEGATIVE_EXIT__", &negative_exit.to_string()),
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&path, perms).unwrap();
        }
        path
    }
}
