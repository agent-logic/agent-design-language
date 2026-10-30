//! CSM Polis durable storage proof support.
//!
//! This module intentionally drives the live S3 proof through the AWS CLI. The
//! runtime command owns the proof semantics and retained evidence, while the AWS
//! CLI remains the operator-approved transport for the Agent Logic account.

use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const PROOF_SCHEMA: &str = "adl.csm.polis_durable_storage_proof.v1";
const PAYLOAD_SCHEMA: &str = "adl.csm.polis_state_storage_payload.v1";
const TAXONOMY_SCHEMA: &str = "adl.csm.polis_artifact_durability_taxonomy.v1";

#[derive(Debug, Clone)]
pub struct PolisStorageProofOptions {
    pub out_dir: PathBuf,
    pub bucket: String,
    pub prefix: String,
    pub profile: String,
    pub region: String,
    pub expected_account_sha256: String,
    pub run_id: String,
    pub aws_bin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolisStorageProofResult {
    pub schema: String,
    pub issue: u64,
    pub status: String,
    pub run_id: String,
    pub checked_at_utc: String,
    pub aws_profile: String,
    pub aws_region: String,
    pub aws_account_hash: String,
    pub aws_account_matches_expected: bool,
    pub bucket_name: String,
    pub bucket_name_hash: String,
    pub object: StoredObjectProof,
    pub restored_artifact: RestoredArtifactProof,
    pub negative_cases: NegativeCases,
    pub durability_contract: DurabilityContract,
    pub retained_artifacts: Vec<String>,
    pub non_claims: Vec<String>,
    pub redaction: RedactionProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObjectProof {
    pub key: String,
    pub version_id: Option<String>,
    pub payload_sha256: String,
    pub payload_bytes: u64,
    pub metadata_sha256_matches: bool,
    pub server_side_encryption: Option<String>,
    pub object_lock_mode: Option<String>,
    pub object_lock_retain_until_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoredArtifactProof {
    pub restore_ref: String,
    pub restored_sha256: String,
    pub checksum_matches: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegativeCases {
    pub missing_object: NegativeCaseProof,
    pub corrupted_restore: NegativeCaseProof,
    pub unsigned_access_denial: NegativeCaseProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegativeCaseProof {
    pub status: String,
    pub expected_failure: String,
    pub observed_failure_class: String,
    pub raw_error_retained: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurabilityContract {
    pub target_class: String,
    pub backend: String,
    pub artifact_taxonomy_ref: String,
    pub selected_backend_assumptions: Vec<String>,
    pub local_proof_scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedactionProof {
    pub raw_account_id_retained: bool,
    pub full_account_digest_retained: bool,
    pub aws_credentials_retained: bool,
    pub raw_aws_errors_retained: bool,
}

pub fn prove_polis_storage(options: PolisStorageProofOptions) -> Result<PolisStorageProofResult> {
    validate_nonempty(&options.bucket, "bucket")?;
    validate_nonempty(&options.profile, "profile")?;
    validate_nonempty(&options.region, "region")?;
    validate_sha256(&options.expected_account_sha256, "expected-account-sha256")?;
    let run_id = validate_path_segment(&options.run_id, "run-id")?;
    let prefix = normalize_prefix(&options.prefix)?;

    fs::create_dir_all(&options.out_dir)
        .with_context(|| format!("create proof output dir {}", options.out_dir.display()))?;
    let restore_dir = options.out_dir.join("restore");
    fs::create_dir_all(&restore_dir)
        .with_context(|| format!("create restore dir {}", restore_dir.display()))?;

    let account = signed_aws_text(
        &options,
        &[
            "sts",
            "get-caller-identity",
            "--query",
            "Account",
            "--output",
            "text",
        ],
    )
    .context("resolve AWS account for Agent Logic profile")?;
    let account = account.trim();
    let account_sha256 = sha256_hex(account.as_bytes());
    if account_sha256 != options.expected_account_sha256 {
        bail!("AWS profile account hash does not match expected Agent Logic account hash");
    }
    let account_hash = account_sha256[..16].to_string();

    let key = format!("{prefix}polis-state/{run_id}/snapshot.json");
    let payload_path = options.out_dir.join("polis_state_snapshot.json");
    let payload = build_payload(&run_id);
    let payload_bytes = serde_json::to_vec_pretty(&payload)?;
    fs::write(&payload_path, &payload_bytes)
        .with_context(|| format!("write payload {}", payload_path.display()))?;
    let payload_sha256 = sha256_hex(&payload_bytes);
    let retain_until =
        (Utc::now() + Duration::days(365)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    signed_aws(
        &options,
        &[
            "s3api",
            "put-object",
            "--bucket",
            &options.bucket,
            "--key",
            &key,
            "--body",
            path_str(&payload_path)?,
            "--metadata",
            &format!(
                "sha256={payload_sha256},artifact-kind=polis-snapshot,durability-class=s3-standard-vendor-11-nines-non-12-nines-claim,issue=4913"
            ),
            "--object-lock-mode",
            "GOVERNANCE",
            "--object-lock-retain-until-date",
            &retain_until,
            "--output",
            "json",
        ],
    )
    .context("put Polis state proof object")?;

    let head_value = signed_aws_json(
        &options,
        &[
            "s3api",
            "head-object",
            "--bucket",
            &options.bucket,
            "--key",
            &key,
            "--output",
            "json",
        ],
    )
    .context("head Polis state proof object")?;
    let metadata_sha256 = head_value
        .pointer("/Metadata/sha256")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if metadata_sha256 != payload_sha256 {
        bail!("S3 object metadata sha256 mismatch");
    }
    let payload_len = payload_bytes.len() as u64;
    let content_length = head_value
        .get("ContentLength")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    if content_length != payload_len {
        bail!("S3 object content length mismatch");
    }
    let version_id = string_field(&head_value, "VersionId")
        .filter(|value| !value.trim().is_empty())
        .context("S3 object version id missing")?;
    let server_side_encryption = string_field(&head_value, "ServerSideEncryption")
        .filter(|value| matches!(value.as_str(), "AES256" | "aws:kms"))
        .context("S3 object server-side encryption missing or unsupported")?;
    let object_lock_mode = string_field(&head_value, "ObjectLockMode")
        .filter(|value| value == "GOVERNANCE")
        .context("S3 object governance lock missing")?;
    let object_lock_retain_until_date = string_field(&head_value, "ObjectLockRetainUntilDate")
        .context("S3 object lock retain-until date missing")?;
    enforce_minimum_retention(&object_lock_retain_until_date)?;

    let restore_path = restore_dir.join("polis_state_snapshot.restored.json");
    signed_aws(
        &options,
        &[
            "s3api",
            "get-object",
            "--bucket",
            &options.bucket,
            "--key",
            &key,
            path_str(&restore_path)?,
            "--output",
            "json",
        ],
    )
    .context("restore Polis state proof object")?;
    let restored_bytes = fs::read(&restore_path)
        .with_context(|| format!("read restored payload {}", restore_path.display()))?;
    let restored_sha256 = sha256_hex(&restored_bytes);
    let checksum_matches = restored_sha256 == payload_sha256;
    if !checksum_matches {
        bail!("restored payload checksum mismatch");
    }

    let missing_case = prove_missing_object(&options, &prefix, &run_id)?;
    let corrupted_case =
        prove_corrupted_restore(&options.out_dir, &payload_sha256, &restored_bytes)?;
    let unsigned_case = prove_unsigned_access_denial(&options, &key)?;

    let taxonomy_path = options.out_dir.join("artifact_durability_taxonomy.json");
    write_taxonomy(&taxonomy_path, &options.bucket, &prefix)?;

    let bucket_name_hash = sha256_hex(options.bucket.as_bytes())[..16].to_string();
    let proof = PolisStorageProofResult {
        schema: PROOF_SCHEMA.to_string(),
        issue: 4913,
        status: "passed".to_string(),
        run_id,
        checked_at_utc: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        aws_profile: options.profile,
        aws_region: options.region,
        aws_account_hash: account_hash,
        aws_account_matches_expected: true,
        bucket_name: options.bucket,
        bucket_name_hash,
        object: StoredObjectProof {
            key,
            version_id: Some(version_id),
            payload_sha256,
            payload_bytes: payload_len,
            metadata_sha256_matches: metadata_sha256 == restored_sha256,
            server_side_encryption: Some(server_side_encryption),
            object_lock_mode: Some(object_lock_mode),
            object_lock_retain_until_date: Some(object_lock_retain_until_date),
        },
        restored_artifact: RestoredArtifactProof {
            restore_ref: "restore/polis_state_snapshot.restored.json".to_string(),
            restored_sha256,
            checksum_matches,
        },
        negative_cases: NegativeCases {
            missing_object: missing_case,
            corrupted_restore: corrupted_case,
            unsigned_access_denial: unsigned_case,
        },
        durability_contract: DurabilityContract {
            target_class: "12-nines-class target; selected S3 backend is vendor 11-nines per-object durability and is therefore a non-12-nines mathematical claim".to_string(),
            backend: "AWS S3 Standard with versioning, default governance object lock, SSE-S3 encryption, public access block, and lifecycle transition policy from #4688".to_string(),
            artifact_taxonomy_ref: "artifact_durability_taxonomy.json".to_string(),
            selected_backend_assumptions: vec![
                "AWS S3 Standard vendor durability is treated as 11 nines per object for the selected single-region backend.".to_string(),
                "Object Lock governance retention and versioning provide immutable reference and recovery semantics for retained proof objects.".to_string(),
                "The #4688 bucket policy supplies public access block, encryption, versioning, lifecycle, and default retention controls.".to_string(),
            ],
            local_proof_scope: vec![
                "write object with checksum metadata".to_string(),
                "read object metadata including version/lock/encryption posture".to_string(),
                "restore object to clean staging directory".to_string(),
                "verify restored checksum".to_string(),
                "prove missing object, corrupted local restore, and unsigned access denial negative cases".to_string(),
            ],
        },
        retained_artifacts: vec![
            "polis_state_snapshot.json".to_string(),
            "restore/polis_state_snapshot.restored.json".to_string(),
            "artifact_durability_taxonomy.json".to_string(),
            "polis_storage_proof_summary.json".to_string(),
        ],
        non_claims: vec![
            "This proof does not claim mathematical 12-nines durability from a single-region S3 bucket.".to_string(),
            "This proof does not retain AWS credentials, raw account ids, or raw AWS error payloads.".to_string(),
            "This proof validates one live write/read/restore cycle and bounded negative cases; it does not prove all future Polis artifacts are automatically archived.".to_string(),
        ],
        redaction: RedactionProof {
            raw_account_id_retained: false,
            full_account_digest_retained: false,
            aws_credentials_retained: false,
            raw_aws_errors_retained: false,
        },
    };

    let summary_path = options.out_dir.join("polis_storage_proof_summary.json");
    fs::write(&summary_path, serde_json::to_vec_pretty(&proof)?)
        .with_context(|| format!("write proof summary {}", summary_path.display()))?;
    Ok(proof)
}

fn prove_missing_object(
    options: &PolisStorageProofOptions,
    prefix: &str,
    run_id: &str,
) -> Result<NegativeCaseProof> {
    let missing_key = format!("{prefix}polis-state/{run_id}/missing-object.json");
    let output = signed_aws_output(
        options,
        &[
            "s3api",
            "head-object",
            "--bucket",
            &options.bucket,
            "--key",
            &missing_key,
            "--output",
            "json",
        ],
    )?;
    if output.status.success() {
        bail!("missing-object negative case unexpectedly succeeded");
    }
    let failure_class = classify_aws_failure(&output);
    if failure_class != "not_found" {
        bail!(
            "missing-object negative case failed with unexpected class: {}",
            failure_class
        );
    }
    Ok(NegativeCaseProof {
        status: "passed".to_string(),
        expected_failure: "missing object must not restore as valid state".to_string(),
        observed_failure_class: "s3_head_object_missing_or_not_found".to_string(),
        raw_error_retained: false,
    })
}

fn prove_corrupted_restore(
    out_dir: &Path,
    expected_sha256: &str,
    restored_bytes: &[u8],
) -> Result<NegativeCaseProof> {
    let mut corrupt = restored_bytes.to_vec();
    corrupt.extend_from_slice(b"\ncorruption-for-negative-proof\n");
    let corrupt_path = out_dir
        .join("restore")
        .join("polis_state_snapshot.corrupt.json");
    fs::write(&corrupt_path, &corrupt)
        .with_context(|| format!("write corrupted restore {}", corrupt_path.display()))?;
    let corrupt_sha = sha256_hex(&corrupt);
    if corrupt_sha == expected_sha256 {
        bail!("corrupted restore negative case did not alter checksum");
    }
    Ok(NegativeCaseProof {
        status: "passed".to_string(),
        expected_failure: "local corrupted restore must fail checksum validation".to_string(),
        observed_failure_class: "checksum_mismatch_detected".to_string(),
        raw_error_retained: false,
    })
}

fn prove_unsigned_access_denial(
    options: &PolisStorageProofOptions,
    key: &str,
) -> Result<NegativeCaseProof> {
    let output = unsigned_aws_output(
        options,
        &[
            "s3api",
            "head-object",
            "--bucket",
            &options.bucket,
            "--key",
            key,
            "--output",
            "json",
        ],
    )?;
    if output.status.success() {
        bail!("unsigned access negative case unexpectedly succeeded");
    }
    let failure_class = classify_aws_failure(&output);
    if failure_class != "access_denied" {
        bail!(
            "unsigned access negative case failed with unexpected class: {}",
            failure_class
        );
    }
    Ok(NegativeCaseProof {
        status: "passed".to_string(),
        expected_failure: "unsigned/public access must be denied for retained Polis state"
            .to_string(),
        observed_failure_class: "unsigned_access_denied_or_forbidden".to_string(),
        raw_error_retained: false,
    })
}

fn write_taxonomy(path: &Path, bucket: &str, prefix: &str) -> Result<()> {
    let taxonomy = json!({
        "schema": TAXONOMY_SCHEMA,
        "issue": 4913,
        "backend": {
            "kind": "aws_s3",
            "bucket_name": bucket,
            "prefix": prefix,
            "durability_posture": "vendor_11_nines_per_object_non_12_nines_claim",
            "controls": [
                "versioning",
                "object_lock_governance_retention",
                "sse_s3_encryption",
                "public_access_block",
                "lifecycle_transition_policy"
            ]
        },
        "artifact_classes": [
            {"artifact_kind":"checkpoint","durability_class":"critical_runtime_state","retention":"365d_minimum_governance_retention","integrity":"sha256_manifest_plus_s3_metadata","restore_required":true},
            {"artifact_kind":"event_log","durability_class":"audit_replay_evidence","retention":"365d_minimum_governance_retention","integrity":"append_order_manifest_plus_sha256","restore_required":true},
            {"artifact_kind":"snapshot","durability_class":"critical_runtime_state","retention":"365d_minimum_governance_retention","integrity":"sha256_manifest_plus_s3_version_id","restore_required":true},
            {"artifact_kind":"diff","durability_class":"replay_delta","retention":"365d_minimum_governance_retention","integrity":"sha256_manifest_plus_parent_snapshot_ref","restore_required":true},
            {"artifact_kind":"freeze_dry_bundle","durability_class":"survival_contract_bundle","retention":"365d_minimum_governance_retention","integrity":"bundle_manifest_sha256_plus_member_hashes","restore_required":true},
            {"artifact_kind":"security_evidence","durability_class":"governance_audit_evidence","retention":"365d_minimum_governance_retention","integrity":"sha256_manifest_and_redacted_summary","restore_required":true}
        ],
        "non_claims": [
            "single-region S3 Standard is not recorded as mathematical 12-nines durability",
            "taxonomy does not make public access acceptable for any Polis state artifact"
        ]
    });
    fs::write(path, serde_json::to_vec_pretty(&taxonomy)?)
        .with_context(|| format!("write taxonomy {}", path.display()))
}

fn build_payload(run_id: &str) -> Value {
    json!({
        "schema": PAYLOAD_SCHEMA,
        "issue": 4913,
        "run_id": run_id,
        "artifact_kind": "snapshot",
        "agent_instance_id": "polis-durable-storage-proof",
        "cycle_id": "cycle-000001",
        "created_at_utc": Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "state": {
            "checkpoint_ref": "checkpoint-000001",
            "event_log_ref": "event-log-000001",
            "snapshot_ref": "snapshot-000001",
            "diff_ref": "diff-000001",
            "freeze_dry_bundle_ref": "freeze-dry-000001"
        },
        "privacy": {
            "contains_secret": false,
            "contains_private_user_content": false,
            "content_class": "synthetic_proof_payload"
        }
    })
}

fn signed_aws_json(options: &PolisStorageProofOptions, args: &[&str]) -> Result<Value> {
    let text = signed_aws_text(options, args)?;
    serde_json::from_str(&text).context("parse AWS JSON output")
}

fn signed_aws_text(options: &PolisStorageProofOptions, args: &[&str]) -> Result<String> {
    let output = signed_aws_output(options, args)?;
    if !output.status.success() {
        bail!("AWS command failed: {}", sanitized_failure(&output));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn signed_aws(options: &PolisStorageProofOptions, args: &[&str]) -> Result<()> {
    let output = signed_aws_output(options, args)?;
    if !output.status.success() {
        bail!("AWS command failed: {}", sanitized_failure(&output));
    }
    Ok(())
}

fn signed_aws_output(options: &PolisStorageProofOptions, args: &[&str]) -> Result<Output> {
    let mut command = Command::new(&options.aws_bin);
    command.args(args);
    command.args(["--profile", &options.profile, "--region", &options.region]);
    run_command(command)
}

fn unsigned_aws_output(options: &PolisStorageProofOptions, args: &[&str]) -> Result<Output> {
    let mut command = Command::new(&options.aws_bin);
    command.arg("--no-sign-request");
    command.args(args);
    command.args(["--region", &options.region]);
    run_command(command)
}

fn run_command(mut command: Command) -> Result<Output> {
    command
        .output()
        .with_context(|| format!("run command {:?}", command.get_program()))
}

fn sanitized_failure(output: &Output) -> String {
    let code = output
        .status
        .code()
        .map(|value| value.to_string())
        .unwrap_or_else(|| "signal".to_string());
    let failure_class = classify_aws_failure(output);
    if failure_class == "not_found" {
        format!("exit={code} class=not_found")
    } else if failure_class == "access_denied" {
        format!("exit={code} class=access_denied")
    } else {
        format!("exit={code} class=aws_command_failed")
    }
}

fn classify_aws_failure(output: &Output) -> &'static str {
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text.push('\n');
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    if text.contains("NoSuchKey")
        || text.contains("NoSuchBucket")
        || text.contains("Not Found")
        || text.contains("NotFound")
        || text.contains("404")
    {
        "not_found"
    } else if text.contains("AccessDenied")
        || text.contains("Forbidden")
        || text.contains("403")
        || text.contains("Unable to locate credentials")
        || text.contains("InvalidAccessKeyId")
    {
        "access_denied"
    } else {
        "aws_command_failed"
    }
}

fn enforce_minimum_retention(value: &str) -> Result<()> {
    let retain_until = DateTime::parse_from_rfc3339(value)
        .with_context(|| format!("parse object lock retain-until date {value:?}"))?
        .with_timezone(&Utc);
    let minimum = Utc::now() + Duration::days(360);
    if retain_until < minimum {
        bail!("S3 object lock retain-until date is shorter than required proof horizon");
    }
    Ok(())
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn path_str(path: &Path) -> Result<&str> {
    path.to_str()
        .ok_or_else(|| anyhow!("path is not valid UTF-8: {}", path.display()))
}

fn validate_nonempty(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        bail!("{field} must not be empty");
    }
    Ok(())
}

fn validate_sha256(value: &str, field: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|ch| ch.is_ascii_hexdigit()) {
        bail!("{field} must be a 64-character sha256 hex digest");
    }
    Ok(())
}

fn validate_path_segment(value: &str, field: &str) -> Result<String> {
    validate_nonempty(value, field)?;
    if value == "." || value == ".." || value.contains('/') || value.contains('\\') {
        bail!("{field} must be a single path segment");
    }
    if !value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        bail!("{field} contains unsupported characters");
    }
    Ok(value.to_string())
}

fn normalize_prefix(value: &str) -> Result<String> {
    validate_nonempty(value, "prefix")?;
    if value.starts_with('/') || value.contains("..") {
        bail!("prefix must be a relative S3 key prefix without parent traversal");
    }
    let mut prefix = value.trim().trim_start_matches("./").to_string();
    if !prefix.ends_with('/') {
        prefix.push('/');
    }
    Ok(prefix)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;

    #[test]
    fn validates_prefix_and_run_id() {
        assert_eq!(
            normalize_prefix("community-memory").unwrap(),
            "community-memory/"
        );
        assert_eq!(
            validate_path_segment("run-001", "run-id").unwrap(),
            "run-001"
        );
        assert!(normalize_prefix("../bad").is_err());
        assert!(validate_path_segment("../bad", "run-id").is_err());
    }

    #[test]
    fn corrupted_restore_negative_detects_checksum_change() {
        let dir = std::env::temp_dir().join(format!(
            "adl-csm-polis-storage-test-{}",
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        fs::create_dir_all(dir.join("restore")).unwrap();
        let payload = br#"{"ok":true}"#;
        let digest = sha256_hex(payload);
        let proof = prove_corrupted_restore(&dir, &digest, payload).unwrap();
        assert_eq!(proof.status, "passed");
        assert_eq!(proof.observed_failure_class, "checksum_mismatch_detected");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn taxonomy_records_non_12_nines_claim() {
        let dir = std::env::temp_dir().join(format!(
            "adl-csm-polis-storage-taxonomy-{}",
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        ));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("taxonomy.json");
        write_taxonomy(&path, "bucket", "community-memory/").unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("vendor_11_nines_per_object_non_12_nines_claim"));
        assert!(text.contains("freeze_dry_bundle"));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn enforces_retention_horizon() {
        let valid =
            (Utc::now() + Duration::days(365)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let too_short =
            (Utc::now() + Duration::days(30)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        assert!(enforce_minimum_retention(&valid).is_ok());
        assert!(enforce_minimum_retention(&too_short).is_err());
        assert!(enforce_minimum_retention("not-a-date").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn classifies_expected_aws_failure_shapes() {
        let not_found = Output {
            status: std::process::ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr: b"An error occurred (404) when calling the HeadObject operation: Not Found"
                .to_vec(),
        };
        assert_eq!(classify_aws_failure(&not_found), "not_found");

        let denied = Output {
            status: std::process::ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr: b"An error occurred (AccessDenied) when calling the HeadObject operation"
                .to_vec(),
        };
        assert_eq!(classify_aws_failure(&denied), "access_denied");

        let throttled = Output {
            status: std::process::ExitStatus::from_raw(1),
            stdout: Vec::new(),
            stderr:
                b"An error occurred (ThrottlingException) when calling the HeadObject operation"
                    .to_vec(),
        };
        assert_eq!(classify_aws_failure(&throttled), "aws_command_failed");
    }
}
