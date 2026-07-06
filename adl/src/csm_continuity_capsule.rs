//! Portable CSM continuity capsule capture and stage support.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::long_lived_agent;

pub const CONTINUITY_CAPSULE_SCHEMA: &str = "adl.csm.continuity_capsule.v1";
pub const CONTINUITY_STAGE_REPORT_SCHEMA: &str = "adl.csm.continuity_capsule_stage_report.v1";
pub const CONTINUITY_RESTORE_REPORT_SCHEMA: &str = "adl.csm.continuity_capsule_restore_report.v1";
pub const CONTINUITY_CAPSULE_FORMAT_VERSION: &str = "csm.continuity-capsule.v1";
pub const SNAPSHOT_SEGMENT_SCHEMA: &str = "adl.csm.snapshot_segment.v1";
pub const SNAPSHOT_SEGMENT_FORMAT_VERSION: &str = "csm.snapshot-segment.v1";

#[derive(Debug, Clone)]
pub struct ContinuityCaptureOptions {
    pub spec_path: PathBuf,
    pub out_dir: PathBuf,
    pub source_host: String,
    pub target_host: String,
}

#[derive(Debug, Clone)]
pub struct ContinuityStageOptions {
    pub bundle_dir: PathBuf,
    pub out_dir: PathBuf,
    pub target_host: String,
}

#[derive(Debug, Clone)]
pub struct ContinuityRestoreOptions {
    pub bundle_dir: PathBuf,
    pub out_dir: PathBuf,
    pub target_host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityCommandResult {
    pub schema: String,
    pub format_version: String,
    pub runtime_owner: String,
    pub operation: String,
    pub status: String,
    pub bundle_dir: PathBuf,
    pub manifest_ref: String,
    pub report_ref: Option<String>,
    pub agent_instance_id: Option<String>,
    pub target_host: String,
    pub event_count: usize,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BundleManifest {
    schema: String,
    format_version: String,
    runtime_owner: String,
    created_at: String,
    agent_instance_id: String,
    source_host: String,
    target_host: String,
    manifest_ref: String,
    state_root_ref: String,
    artifacts: Vec<BundleArtifact>,
    #[serde(default)]
    binary_segments: Vec<BinarySegmentRef>,
    rebind_policy: Value,
    observability: Value,
    future_readability: Value,
    non_claims: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BundleArtifact {
    role: String,
    source_ref: String,
    bundle_ref: String,
    sha256: String,
    bytes: u64,
    required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BinarySegmentRef {
    role: String,
    source_ref: String,
    segment_ref: String,
    sha256: String,
    payload_sha256: String,
    bytes: u64,
    schema: String,
    format_version: String,
    hash_address: String,
    required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotSegmentMetadata {
    pub schema: String,
    pub format_version: String,
    pub package_name: String,
    pub type_name: String,
    pub semantic_version: String,
    pub segment_kind: String,
    pub source_ref: String,
    pub payload_media_type: String,
    pub reserved_fields_policy: String,
    pub unknown_field_policy: String,
    pub compatibility_guarantee: String,
    pub redaction_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedSnapshotSegment {
    pub metadata: SnapshotSegmentMetadata,
    pub payload: Vec<u8>,
    pub payload_sha256: String,
}

pub fn capture_capsule(options: ContinuityCaptureOptions) -> Result<ContinuityCommandResult> {
    validate_target_host(&options.target_host)?;
    let loaded = long_lived_agent::load_spec(&options.spec_path)?;
    let bundle_dir = options.out_dir;
    if bundle_dir.exists() {
        fs::remove_dir_all(&bundle_dir)
            .with_context(|| format!("failed clearing {}", bundle_dir.display()))?;
    }
    fs::create_dir_all(bundle_dir.join("state"))
        .with_context(|| format!("failed creating {}", bundle_dir.join("state").display()))?;

    let mut artifacts = Vec::new();
    for spec in artifact_specs() {
        let source = loaded.state_root.join(spec.ref_path);
        if !source.exists() {
            if spec.required {
                bail!(
                    "continuity capsule capture missing required runtime artifact: {}",
                    spec.ref_path
                );
            }
            continue;
        }
        let dest = bundle_dir.join("state").join(spec.ref_path);
        copy_artifact_tree(&source, &dest)
            .with_context(|| format!("failed copying runtime artifact {}", spec.ref_path))?;
        scan_tree_for_portability(&dest, &loaded.state_root)?;
        collect_artifacts(
            &dest,
            &format!("state/{}", spec.ref_path),
            spec,
            &mut artifacts,
        )?;
    }

    let binary_segments = write_binary_segments(&bundle_dir, &loaded.state_root)?;
    let manifest = BundleManifest {
        schema: CONTINUITY_CAPSULE_SCHEMA.to_string(),
        format_version: CONTINUITY_CAPSULE_FORMAT_VERSION.to_string(),
        runtime_owner: "csm".to_string(),
        created_at: Utc::now().to_rfc3339(),
        agent_instance_id: loaded.spec.agent_instance_id.clone(),
        source_host: options.source_host,
        target_host: options.target_host.clone(),
        manifest_ref: "continuity_capsule_manifest.json".to_string(),
        state_root_ref: "state/".to_string(),
        artifacts,
        binary_segments,
        rebind_policy: rebind_policy(&options.target_host),
        observability: observability_contract("capture"),
        future_readability: json!({
            "schema_stability": "format_version_required",
            "reader_guarantee": "future readers must reject unknown major versions",
            "portable_paths": "bundle_relative_refs_only",
            "binary_segment_policy": {
                "format_version": SNAPSHOT_SEGMENT_FORMAT_VERSION,
                "manifest_linkage": "binary segments are hash-addressed from this JSON manifest",
                "protobuf_decision": "evaluated in issue #4933; deferred until codegen/toolchain ownership is explicit"
            }
        }),
        non_claims: continuity_capsule_non_claims(),
    };
    let manifest_path = bundle_dir.join("continuity_capsule_manifest.json");
    write_json_pretty(&manifest_path, &manifest)?;
    scan_tree_for_portability(&manifest_path, &loaded.state_root)?;
    emit_continuity_event(
        "continuity_capsule_capture",
        "completed",
        &loaded.spec.agent_instance_id,
        &options.target_host,
        json!({
            "bundle_ref": ".",
            "manifest_ref": "continuity_capsule_manifest.json",
            "artifact_count": manifest.artifacts.len()
        }),
    );
    Ok(ContinuityCommandResult {
        schema: "adl.csm.continuity_capsule_command_result.v1".to_string(),
        format_version: CONTINUITY_CAPSULE_FORMAT_VERSION.to_string(),
        runtime_owner: "csm".to_string(),
        operation: "capture".to_string(),
        status: "captured".to_string(),
        bundle_dir,
        manifest_ref: "continuity_capsule_manifest.json".to_string(),
        report_ref: None,
        agent_instance_id: Some(loaded.spec.agent_instance_id),
        target_host: options.target_host,
        event_count: 1,
        non_claims: continuity_capsule_non_claims(),
    })
}

pub fn stage_capsule(options: ContinuityStageOptions) -> Result<ContinuityCommandResult> {
    let manifest_path = options.bundle_dir.join("continuity_capsule_manifest.json");
    let manifest: BundleManifest = read_json(&manifest_path)?;
    validate_manifest(&manifest)?;
    validate_target_host(&options.target_host)?;
    scan_tree_for_portability(&options.bundle_dir, &options.bundle_dir)?;
    validate_binary_segments(&manifest, &options.bundle_dir)?;

    if options.out_dir.exists() {
        fs::remove_dir_all(&options.out_dir)
            .with_context(|| format!("failed clearing {}", options.out_dir.display()))?;
    }
    fs::create_dir_all(options.out_dir.join("staged_state")).with_context(|| {
        format!(
            "failed creating {}",
            options.out_dir.join("staged_state").display()
        )
    })?;
    for artifact in &manifest.artifacts {
        let source = options.bundle_dir.join(&artifact.bundle_ref);
        if !source.exists() {
            bail!(
                "continuity capsule stage missing bundle artifact {}",
                artifact.bundle_ref
            );
        }
        if sha256_file(&source)? != artifact.sha256 {
            bail!(
                "continuity capsule stage hash mismatch for {}",
                artifact.bundle_ref
            );
        }
        let relative = artifact
            .bundle_ref
            .strip_prefix("state/")
            .unwrap_or(artifact.bundle_ref.as_str());
        let dest = options.out_dir.join("staged_state").join(relative);
        copy_artifact_tree(&source, &dest)?;
    }

    let status = if options.target_host == "ec2" {
        "blocked"
    } else {
        "staged"
    };
    let blockers = if status == "blocked" {
        json!([{
            "blocker": "live_ec2_transfer_not_attempted",
            "required_profile": "agent-logic-admin",
            "required_operator_action": "provide approved EC2 target and transfer authorization"
        }])
    } else {
        json!([])
    };
    let report = json!({
        "schema": CONTINUITY_STAGE_REPORT_SCHEMA,
        "format_version": CONTINUITY_CAPSULE_FORMAT_VERSION,
        "runtime_owner": "csm",
        "agent_instance_id": manifest.agent_instance_id,
        "target_host": options.target_host,
        "status": status,
        "staged_state_ref": "staged_state/",
        "manifest_ref": "continuity_capsule_manifest.json",
        "artifact_count": manifest.artifacts.len(),
        "blockers": blockers,
        "rebind_policy": rebind_policy(&options.target_host),
        "observability": observability_contract("stage"),
        "non_claims": continuity_capsule_non_claims()
    });
    let report_path = options.out_dir.join("stage_report.json");
    write_json_pretty(&report_path, &report)?;
    emit_continuity_event(
        "continuity_capsule_stage",
        status,
        &manifest.agent_instance_id,
        &options.target_host,
        json!({
            "bundle_ref": ".",
            "report_ref": "stage_report.json",
            "artifact_count": manifest.artifacts.len()
        }),
    );
    Ok(ContinuityCommandResult {
        schema: "adl.csm.continuity_capsule_command_result.v1".to_string(),
        format_version: CONTINUITY_CAPSULE_FORMAT_VERSION.to_string(),
        runtime_owner: "csm".to_string(),
        operation: "stage".to_string(),
        status: status.to_string(),
        bundle_dir: options.bundle_dir,
        manifest_ref: "continuity_capsule_manifest.json".to_string(),
        report_ref: Some("stage_report.json".to_string()),
        agent_instance_id: Some(manifest.agent_instance_id),
        target_host: options.target_host,
        event_count: 1,
        non_claims: continuity_capsule_non_claims(),
    })
}

pub fn restore_capsule(options: ContinuityRestoreOptions) -> Result<ContinuityCommandResult> {
    let manifest_path = options.bundle_dir.join("continuity_capsule_manifest.json");
    let manifest: BundleManifest = read_json(&manifest_path)?;
    validate_manifest(&manifest)?;
    validate_target_host(&options.target_host)?;
    scan_tree_for_portability(&options.bundle_dir, &options.bundle_dir)?;
    validate_binary_segments(&manifest, &options.bundle_dir)?;

    if options.out_dir.exists() {
        fs::remove_dir_all(&options.out_dir)
            .with_context(|| format!("failed clearing {}", options.out_dir.display()))?;
    }
    fs::create_dir_all(options.out_dir.join("state")).with_context(|| {
        format!(
            "failed creating restored state {}",
            options.out_dir.join("state").display()
        )
    })?;

    for artifact in &manifest.artifacts {
        let source = options.bundle_dir.join(&artifact.bundle_ref);
        if !source.exists() {
            bail!(
                "continuity capsule restore missing bundle artifact {}",
                artifact.bundle_ref
            );
        }
        if sha256_file(&source)? != artifact.sha256 {
            bail!(
                "continuity capsule restore hash mismatch for {}",
                artifact.bundle_ref
            );
        }
        let relative = artifact
            .bundle_ref
            .strip_prefix("state/")
            .unwrap_or(artifact.bundle_ref.as_str());
        let dest = options.out_dir.join("state").join(relative);
        copy_artifact_tree(&source, &dest)?;
    }

    let spec: long_lived_agent::AgentSpec =
        read_json(&options.out_dir.join("state").join("agent_spec.locked.json"))?;
    let spec_yaml = serde_yaml::to_string(&spec)?;
    fs::write(options.out_dir.join("agent.yaml"), spec_yaml).with_context(|| {
        format!(
            "failed writing {}",
            options.out_dir.join("agent.yaml").display()
        )
    })?;

    let report = json!({
        "schema": CONTINUITY_RESTORE_REPORT_SCHEMA,
        "format_version": CONTINUITY_CAPSULE_FORMAT_VERSION,
        "runtime_owner": "csm",
        "agent_instance_id": manifest.agent_instance_id,
        "target_host": options.target_host,
        "status": "restored",
        "restored_spec_ref": "agent.yaml",
        "restored_state_ref": "state/",
        "manifest_ref": "continuity_capsule_manifest.json",
        "artifact_count": manifest.artifacts.len(),
        "rebind_policy": rebind_policy(&options.target_host),
        "observability": observability_contract("restore"),
        "non_claims": [
            "not_secret_material_restore",
            "not_provider_auth_restore",
            "not_multi_region_disaster_recovery"
        ]
    });
    write_json_pretty(&options.out_dir.join("restore_report.json"), &report)?;
    emit_continuity_event(
        "continuity_capsule_restore",
        "restored",
        &manifest.agent_instance_id,
        &options.target_host,
        json!({
            "bundle_ref": ".",
            "report_ref": "restore_report.json",
            "restored_spec_ref": "agent.yaml",
            "artifact_count": manifest.artifacts.len()
        }),
    );
    Ok(ContinuityCommandResult {
        schema: "adl.csm.continuity_capsule_command_result.v1".to_string(),
        format_version: CONTINUITY_CAPSULE_FORMAT_VERSION.to_string(),
        runtime_owner: "csm".to_string(),
        operation: "restore".to_string(),
        status: "restored".to_string(),
        bundle_dir: options.bundle_dir,
        manifest_ref: "continuity_capsule_manifest.json".to_string(),
        report_ref: Some("restore_report.json".to_string()),
        agent_instance_id: Some(manifest.agent_instance_id),
        target_host: options.target_host,
        event_count: 1,
        non_claims: vec![
            "not_secret_material_restore".to_string(),
            "not_provider_auth_restore".to_string(),
            "not_multi_region_disaster_recovery".to_string(),
        ],
    })
}

fn validate_manifest(manifest: &BundleManifest) -> Result<()> {
    if manifest.schema != CONTINUITY_CAPSULE_SCHEMA {
        bail!(
            "unsupported continuity capsule manifest schema: {}",
            manifest.schema
        );
    }
    if manifest.format_version != CONTINUITY_CAPSULE_FORMAT_VERSION {
        bail!(
            "unsupported continuity capsule format version: {}",
            manifest.format_version
        );
    }
    if manifest.runtime_owner != "csm" {
        bail!("continuity capsule manifest runtime_owner must be csm");
    }
    if manifest.artifacts.is_empty() {
        bail!("continuity capsule manifest must list retained artifacts");
    }
    for spec in artifact_specs().into_iter().filter(|spec| spec.required) {
        if !manifest
            .artifacts
            .iter()
            .any(|artifact| artifact.role == spec.role && artifact.source_ref == spec.ref_path)
        {
            bail!(
                "continuity capsule manifest missing required artifact role {} ({})",
                spec.role,
                spec.ref_path
            );
        }
    }
    for artifact in &manifest.artifacts {
        validate_relative_ref(&artifact.bundle_ref)?;
        validate_relative_ref(&artifact.source_ref)?;
    }
    for segment in &manifest.binary_segments {
        validate_relative_ref(&segment.segment_ref)?;
        validate_relative_ref(&segment.source_ref)?;
        if segment.schema != SNAPSHOT_SEGMENT_SCHEMA {
            bail!(
                "unsupported continuity capsule binary segment schema: {}",
                segment.schema
            );
        }
        if segment.format_version != SNAPSHOT_SEGMENT_FORMAT_VERSION {
            bail!(
                "unsupported continuity capsule binary segment format version: {}",
                segment.format_version
            );
        }
        if segment.hash_address != format!("sha256:{}", segment.sha256) {
            bail!(
                "continuity capsule binary segment hash address mismatch for {}",
                segment.segment_ref
            );
        }
    }
    Ok(())
}

fn validate_target_host(target_host: &str) -> Result<()> {
    match target_host {
        "local" | "wuji" | "ec2-staging" | "ec2" => Ok(()),
        other => bail!("unsupported continuity capsule target host: {other}"),
    }
}

fn validate_relative_ref(value: &str) -> Result<()> {
    let path = Path::new(value);
    if path.is_absolute() {
        bail!("continuity capsule artifact refs must be relative: {value}");
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        bail!("continuity capsule artifact refs must not escape bundle: {value}");
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct ArtifactSpec {
    role: &'static str,
    ref_path: &'static str,
    required: bool,
}

fn artifact_specs() -> Vec<ArtifactSpec> {
    vec![
        ArtifactSpec {
            role: "runtime_identity",
            ref_path: "agent_spec.locked.json",
            required: true,
        },
        ArtifactSpec {
            role: "recoverable_status",
            ref_path: "status.json",
            required: true,
        },
        ArtifactSpec {
            role: "daemon_status",
            ref_path: "daemon_status.json",
            required: true,
        },
        ArtifactSpec {
            role: "continuity",
            ref_path: "continuity.json",
            required: true,
        },
        ArtifactSpec {
            role: "continuity_checkpoint",
            ref_path: "continuity_checkpoint.json",
            required: true,
        },
        ArtifactSpec {
            role: "continuity_replay_manifest",
            ref_path: "continuity_replay_manifest.json",
            required: true,
        },
        ArtifactSpec {
            role: "dag_run_ledger",
            ref_path: "cycle_ledger.jsonl",
            required: true,
        },
        ArtifactSpec {
            role: "memory_index",
            ref_path: "memory_index.json",
            required: true,
        },
        ArtifactSpec {
            role: "provider_binding_history",
            ref_path: "provider_binding_history.jsonl",
            required: true,
        },
        ArtifactSpec {
            role: "observability_tail",
            ref_path: "operator_events.jsonl",
            required: true,
        },
        ArtifactSpec {
            role: "cycle_artifacts",
            ref_path: "cycles",
            required: false,
        },
    ]
}

fn collect_artifacts(
    path: &Path,
    bundle_ref: &str,
    spec: ArtifactSpec,
    artifacts: &mut Vec<BundleArtifact>,
) -> Result<()> {
    if path.is_dir() {
        let mut entries = fs::read_dir(path)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let child_ref = format!(
                "{}/{}",
                bundle_ref.trim_end_matches('/'),
                entry.file_name().to_string_lossy()
            );
            collect_artifacts(&entry.path(), &child_ref, spec, artifacts)?;
        }
        return Ok(());
    }
    artifacts.push(BundleArtifact {
        role: spec.role.to_string(),
        source_ref: bundle_ref
            .strip_prefix("state/")
            .unwrap_or(bundle_ref)
            .to_string(),
        bundle_ref: bundle_ref.to_string(),
        sha256: sha256_file(path)?,
        bytes: fs::metadata(path)?.len(),
        required: spec.required,
    });
    Ok(())
}

fn write_binary_segments(
    bundle_dir: &Path,
    forbidden_root: &Path,
) -> Result<Vec<BinarySegmentRef>> {
    let segment_specs = [(
        "continuity_checkpoint_snapshot",
        "continuity_checkpoint.json",
        "segments/continuity_checkpoint.snapshot.segment",
        true,
    )];
    let mut refs = Vec::new();
    for (role, source_ref, segment_ref, required) in segment_specs {
        let payload_path = bundle_dir.join("state").join(source_ref);
        if !payload_path.exists() {
            if required {
                bail!("continuity capsule binary segment missing source artifact: {source_ref}");
            }
            continue;
        }
        let payload = fs::read(&payload_path).with_context(|| {
            format!("failed reading segment payload {}", payload_path.display())
        })?;
        validate_segment_payload(&payload, &payload_path, forbidden_root)?;
        let metadata = SnapshotSegmentMetadata {
            schema: SNAPSHOT_SEGMENT_SCHEMA.to_string(),
            format_version: SNAPSHOT_SEGMENT_FORMAT_VERSION.to_string(),
            package_name: "adl.csm.snapshot_segment".to_string(),
            type_name: "ContinuityCheckpointSnapshotSegment".to_string(),
            semantic_version: "1.0.0".to_string(),
            segment_kind: "snapshot".to_string(),
            source_ref: source_ref.to_string(),
            payload_media_type: "application/json".to_string(),
            reserved_fields_policy: "new fields require a minor version and must not change existing field meaning".to_string(),
            unknown_field_policy: "reader accepts same-major newer minor metadata and rejects unknown major format versions".to_string(),
            compatibility_guarantee: "payload bytes remain hash-verifiable and manifest-addressed; JSON continuity capsule v1 remains the reviewable control plane".to_string(),
            redaction_policy: "segment writer rejects host-private paths and credential-like JSON keys before writing".to_string(),
        };
        let segment = encode_snapshot_segment(&metadata, &payload)?;
        let segment_path = bundle_dir.join(segment_ref);
        if let Some(parent) = segment_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&segment_path, &segment)
            .with_context(|| format!("failed writing {}", segment_path.display()))?;
        scan_tree_for_portability(&segment_path, forbidden_root)?;
        let decoded = decode_snapshot_segment(&segment)?;
        if decoded.metadata != metadata {
            bail!("continuity capsule binary segment metadata failed round trip for {source_ref}");
        }
        let segment_sha256 = sha256_bytes(&segment);
        refs.push(BinarySegmentRef {
            role: role.to_string(),
            source_ref: source_ref.to_string(),
            segment_ref: segment_ref.to_string(),
            sha256: segment_sha256.clone(),
            payload_sha256: decoded.payload_sha256,
            bytes: segment.len() as u64,
            schema: SNAPSHOT_SEGMENT_SCHEMA.to_string(),
            format_version: SNAPSHOT_SEGMENT_FORMAT_VERSION.to_string(),
            hash_address: format!("sha256:{segment_sha256}"),
            required,
        });
    }
    Ok(refs)
}

fn validate_binary_segments(manifest: &BundleManifest, bundle_dir: &Path) -> Result<()> {
    let required_segments = [(
        "continuity_checkpoint_snapshot",
        "continuity_checkpoint.json",
    )];
    for (required_role, required_source_ref) in required_segments {
        if !manifest.binary_segments.iter().any(|segment| {
            segment.role == required_role && segment.source_ref == required_source_ref
        }) {
            bail!(
                "continuity capsule manifest missing required binary segment role {} ({})",
                required_role,
                required_source_ref
            );
        }
    }
    for segment in &manifest.binary_segments {
        let path = bundle_dir.join(&segment.segment_ref);
        if !path.exists() {
            bail!(
                "continuity capsule binary segment missing bundle artifact {}",
                segment.segment_ref
            );
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("failed reading binary segment {}", path.display()))?;
        if sha256_bytes(&bytes) != segment.sha256 {
            bail!(
                "continuity capsule binary segment hash mismatch for {}",
                segment.segment_ref
            );
        }
        let source_artifact = manifest
            .artifacts
            .iter()
            .find(|artifact| artifact.source_ref == segment.source_ref)
            .with_context(|| {
                format!(
                    "continuity capsule binary segment source artifact missing for {}",
                    segment.source_ref
                )
            })?;
        if source_artifact.sha256 != segment.payload_sha256 {
            bail!(
                "continuity capsule binary segment payload hash does not match retained artifact {}",
                segment.source_ref
            );
        }
        let source_path = bundle_dir.join("state").join(&segment.source_ref);
        if !source_path.exists() {
            bail!(
                "continuity capsule binary segment source artifact file missing {}",
                segment.source_ref
            );
        }
        if sha256_file(&source_path)? != segment.payload_sha256 {
            bail!(
                "continuity capsule binary segment payload hash does not match source file {}",
                segment.source_ref
            );
        }
        let decoded = decode_snapshot_segment(&bytes)?;
        if decoded.metadata.schema != segment.schema
            || decoded.metadata.format_version != segment.format_version
            || decoded.metadata.source_ref != segment.source_ref
            || decoded.payload_sha256 != segment.payload_sha256
        {
            bail!(
                "continuity capsule binary segment manifest linkage mismatch for {}",
                segment.segment_ref
            );
        }
        validate_segment_payload(&decoded.payload, &path, bundle_dir)?;
    }
    Ok(())
}

const SNAPSHOT_SEGMENT_MAGIC: &[u8; 9] = b"ADLCSMSEG";
const SNAPSHOT_SEGMENT_HEADER_LEN: usize = 9 + 2 + 2 + 2 + 4 + 8 + 32;
const SNAPSHOT_SEGMENT_KIND: u16 = 1;
const SNAPSHOT_SEGMENT_MAJOR: u16 = 1;
const SNAPSHOT_SEGMENT_MINOR: u16 = 0;

pub fn encode_snapshot_segment(
    metadata: &SnapshotSegmentMetadata,
    payload: &[u8],
) -> Result<Vec<u8>> {
    if metadata.schema != SNAPSHOT_SEGMENT_SCHEMA {
        bail!("snapshot segment metadata schema must be {SNAPSHOT_SEGMENT_SCHEMA}");
    }
    if metadata.format_version != SNAPSHOT_SEGMENT_FORMAT_VERSION {
        bail!("snapshot segment metadata format_version must be {SNAPSHOT_SEGMENT_FORMAT_VERSION}");
    }
    let metadata_bytes = serde_json::to_vec(metadata)?;
    let metadata_len: u32 = metadata_bytes
        .len()
        .try_into()
        .context("snapshot segment metadata too large")?;
    let payload_len: u64 = payload
        .len()
        .try_into()
        .context("snapshot segment payload too large")?;
    let payload_digest = Sha256::digest(payload);
    let mut out =
        Vec::with_capacity(SNAPSHOT_SEGMENT_HEADER_LEN + metadata_bytes.len() + payload.len());
    out.extend_from_slice(SNAPSHOT_SEGMENT_MAGIC);
    out.extend_from_slice(&SNAPSHOT_SEGMENT_MAJOR.to_be_bytes());
    out.extend_from_slice(&SNAPSHOT_SEGMENT_MINOR.to_be_bytes());
    out.extend_from_slice(&SNAPSHOT_SEGMENT_KIND.to_be_bytes());
    out.extend_from_slice(&metadata_len.to_be_bytes());
    out.extend_from_slice(&payload_len.to_be_bytes());
    out.extend_from_slice(&payload_digest);
    out.extend_from_slice(&metadata_bytes);
    out.extend_from_slice(payload);
    Ok(out)
}

pub fn decode_snapshot_segment(bytes: &[u8]) -> Result<DecodedSnapshotSegment> {
    if bytes.len() < SNAPSHOT_SEGMENT_HEADER_LEN {
        bail!("snapshot segment is truncated before header");
    }
    let mut offset = 0;
    if &bytes[offset..offset + SNAPSHOT_SEGMENT_MAGIC.len()] != SNAPSHOT_SEGMENT_MAGIC {
        bail!("snapshot segment has unsupported magic");
    }
    offset += SNAPSHOT_SEGMENT_MAGIC.len();
    let major = read_u16(bytes, &mut offset)?;
    let _minor = read_u16(bytes, &mut offset)?;
    if major != SNAPSHOT_SEGMENT_MAJOR {
        bail!("unsupported snapshot segment major version: {major}");
    }
    let kind = read_u16(bytes, &mut offset)?;
    if kind != SNAPSHOT_SEGMENT_KIND {
        bail!("unsupported snapshot segment kind: {kind}");
    }
    let metadata_len = read_u32(bytes, &mut offset)? as usize;
    let payload_len = read_u64(bytes, &mut offset)? as usize;
    let expected_digest = bytes
        .get(offset..offset + 32)
        .context("snapshot segment is truncated before payload digest")?;
    offset += 32;
    let metadata_end = offset
        .checked_add(metadata_len)
        .context("snapshot segment metadata length overflow")?;
    let payload_end = metadata_end
        .checked_add(payload_len)
        .context("snapshot segment payload length overflow")?;
    if bytes.len() != payload_end {
        bail!("snapshot segment length does not match header");
    }
    let metadata: SnapshotSegmentMetadata = serde_json::from_slice(&bytes[offset..metadata_end])
        .context("failed parsing snapshot segment metadata")?;
    let payload = bytes[metadata_end..payload_end].to_vec();
    let actual_digest = Sha256::digest(&payload);
    if expected_digest != actual_digest.as_slice() {
        bail!("snapshot segment payload hash mismatch");
    }
    Ok(DecodedSnapshotSegment {
        metadata,
        payload,
        payload_sha256: format!("{actual_digest:x}"),
    })
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16> {
    let end = *offset + 2;
    let chunk = bytes
        .get(*offset..end)
        .context("snapshot segment is truncated while reading u16")?;
    *offset = end;
    Ok(u16::from_be_bytes([chunk[0], chunk[1]]))
}

fn read_u32(bytes: &[u8], offset: &mut usize) -> Result<u32> {
    let end = *offset + 4;
    let chunk = bytes
        .get(*offset..end)
        .context("snapshot segment is truncated while reading u32")?;
    *offset = end;
    Ok(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
}

fn read_u64(bytes: &[u8], offset: &mut usize) -> Result<u64> {
    let end = *offset + 8;
    let chunk = bytes
        .get(*offset..end)
        .context("snapshot segment is truncated while reading u64")?;
    *offset = end;
    Ok(u64::from_be_bytes([
        chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
    ]))
}

fn validate_segment_payload(payload: &[u8], path: &Path, forbidden_root: &Path) -> Result<()> {
    let text = std::str::from_utf8(payload).unwrap_or_default();
    let forbidden = forbidden_root.display().to_string();
    if forbidden_root.is_absolute() && !forbidden.is_empty() && text.contains(&forbidden) {
        bail!(
            "continuity capsule binary segment portability scan found host-private absolute path in {}",
            path.display()
        );
    }
    for marker in host_private_path_markers() {
        if text.contains(marker) {
            bail!(
                "continuity capsule binary segment portability scan found host-private absolute path in {}",
                path.display()
            );
        }
    }
    if let Ok(value) = serde_json::from_str::<Value>(text) {
        scan_json_for_secret_keys(&value, path)?;
    }
    Ok(())
}

fn copy_artifact_tree(source: &Path, dest: &Path) -> Result<()> {
    if source.is_dir() {
        fs::create_dir_all(dest)?;
        let mut entries = fs::read_dir(source)?.collect::<std::io::Result<Vec<_>>>()?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            copy_artifact_tree(&entry.path(), &dest.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, dest)?;
    }
    Ok(())
}

fn scan_tree_for_portability(path: &Path, forbidden_root: &Path) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            scan_tree_for_portability(&entry?.path(), forbidden_root)?;
        }
        return Ok(());
    }
    let text = fs::read_to_string(path).unwrap_or_default();
    let forbidden = forbidden_root.display().to_string();
    if forbidden_root.is_absolute() && !forbidden.is_empty() && text.contains(&forbidden) {
        bail!(
            "continuity capsule portability scan found host-private absolute path in {}",
            path.display()
        );
    }
    for marker in host_private_path_markers() {
        if text.contains(marker) {
            bail!(
                "continuity capsule portability scan found host-private absolute path in {}",
                path.display()
            );
        }
    }
    if let Ok(value) = serde_json::from_str::<Value>(&text) {
        scan_json_for_secret_keys(&value, path)?;
    }
    Ok(())
}

fn host_private_path_markers() -> &'static [&'static str] {
    &[
        "/Users/",
        "/home/",
        "/workspace/",
        "/workspaces/",
        "/private/tmp/",
        "/var/folders/",
        "/tmp/",
        "C:\\",
    ]
}

fn scan_json_for_secret_keys(value: &Value, path: &Path) -> Result<()> {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let lower = key.to_ascii_lowercase();
                if lower.contains("secret")
                    || lower.contains("token")
                    || lower.contains("password")
                    || lower.contains("api_key")
                    || lower.contains("credential")
                {
                    bail!(
                        "continuity capsule portability scan found credential-like key `{}` in {}",
                        key,
                        path.display()
                    );
                }
                scan_json_for_secret_keys(child, path)?;
            }
        }
        Value::Array(values) => {
            for child in values {
                scan_json_for_secret_keys(child, path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed reading {}", path.display()))?;
    Ok(sha256_bytes(&bytes))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("failed reading {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed parsing {}", path.display()))
}

fn write_json_pretty(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes).with_context(|| format!("failed writing {}", path.display()))
}

fn rebind_policy(target_host: &str) -> Value {
    json!({
        "schema": "adl.csm.continuity_capsule_rebind_policy.v1",
        "target_host": target_host,
        "identity": "preserve_agent_instance_id_and_locked_spec",
        "capabilities": "re-evaluate_on_target_control_plane",
        "provider_auth": "excluded_from_bundle_rebind_from_target_provider_environment",
        "paths": "bundle_relative_refs_only",
        "provider_bindings": "historical_bindings_retained_new_bindings_required_after_stage",
        "aws": {
            "default_profile": "agent-logic-admin",
            "live_ec2_transfer": "blocked_without_operator_authorization"
        }
    })
}

fn observability_contract(operation: &str) -> Value {
    let (event_stages, retained_refs) = match operation {
        "restore" => (
            vec!["continuity_capsule_restore"],
            vec!["operator_events.jsonl", "restore_report.json"],
        ),
        _ => (
            vec!["continuity_capsule_capture", "continuity_capsule_stage"],
            vec![
                "operator_events.jsonl",
                "continuity_capsule_manifest.json",
                "stage_report.json",
            ],
        ),
    };
    json!({
        "schema": "adl.csm.continuity_capsule_observability.v1",
        "operation": operation,
        "event_command": "csm",
        "event_stages": event_stages,
        "otel_service_name": "csm-runtime-daemon",
        "retained_refs": retained_refs
    })
}

fn continuity_capsule_non_claims() -> Vec<String> {
    vec![
        "not_live_multi_region_disaster_recovery".to_string(),
        "not_secret_material_capture".to_string(),
        "not_live_ec2_transfer_without_operator_authorization".to_string(),
    ]
}

fn emit_continuity_event(
    stage: &str,
    result: &str,
    agent_instance_id: &str,
    target_host: &str,
    details: Value,
) {
    let details_text = serde_json::to_string(&details).unwrap_or_else(|_| "{}".to_string());
    crate::observability::emit_event(
        "csm",
        stage,
        result,
        &[
            ("process_class", "csm_runtime_daemon"),
            ("agent_instance_id", agent_instance_id),
            ("target_host", target_host),
            ("otel_service_name", "csm-runtime-daemon"),
            ("runtime_role", "csm_runtime"),
            ("migration_format", CONTINUITY_CAPSULE_FORMAT_VERSION),
            ("details", details_text.as_str()),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_metadata() -> SnapshotSegmentMetadata {
        SnapshotSegmentMetadata {
            schema: SNAPSHOT_SEGMENT_SCHEMA.to_string(),
            format_version: SNAPSHOT_SEGMENT_FORMAT_VERSION.to_string(),
            package_name: "adl.csm.snapshot_segment".to_string(),
            type_name: "ContinuityCheckpointSnapshotSegment".to_string(),
            semantic_version: "1.0.0".to_string(),
            segment_kind: "snapshot".to_string(),
            source_ref: "continuity_checkpoint.json".to_string(),
            payload_media_type: "application/json".to_string(),
            reserved_fields_policy:
                "new fields require a minor version and must not change existing field meaning"
                    .to_string(),
            unknown_field_policy:
                "reader accepts same-major newer minor metadata and rejects unknown major format versions"
                    .to_string(),
            compatibility_guarantee:
                "payload bytes remain hash-verifiable and manifest-addressed".to_string(),
            redaction_policy:
                "segment writer rejects host-private paths and credential-like JSON keys before writing"
                    .to_string(),
        }
    }

    #[test]
    fn csm_snapshot_segment_round_trips_with_deterministic_hash() {
        let payload = br#"{"cycle_id":"cycle-000001","state":"recoverable"}"#;
        let segment = encode_snapshot_segment(&test_metadata(), payload).expect("encode");
        let decoded = decode_snapshot_segment(&segment).expect("decode");

        assert_eq!(decoded.metadata, test_metadata());
        assert_eq!(decoded.payload, payload);
        assert_eq!(decoded.payload_sha256, sha256_bytes(payload));
        assert_eq!(
            sha256_bytes(&segment),
            sha256_bytes(&encode_snapshot_segment(&test_metadata(), payload).expect("re-encode"))
        );
    }

    #[test]
    fn csm_snapshot_segment_rejects_unknown_major_version() {
        let mut segment = encode_snapshot_segment(&test_metadata(), br#"{"state":"recoverable"}"#)
            .expect("encode");
        let major_offset = SNAPSHOT_SEGMENT_MAGIC.len();
        segment[major_offset..major_offset + 2].copy_from_slice(&2u16.to_be_bytes());

        let err = decode_snapshot_segment(&segment).expect_err("unknown major rejected");
        assert!(err
            .to_string()
            .contains("unsupported snapshot segment major version"));
    }

    #[test]
    fn csm_snapshot_segment_rejects_truncated_payload() {
        let mut segment = encode_snapshot_segment(&test_metadata(), br#"{"state":"recoverable"}"#)
            .expect("encode");
        segment.pop();

        let err = decode_snapshot_segment(&segment).expect_err("truncated segment rejected");
        assert!(err
            .to_string()
            .contains("snapshot segment length does not match header"));
    }

    #[test]
    fn csm_snapshot_segment_rejects_redaction_violations() {
        let payload = br#"{"api_key":"not-exportable"}"#;
        let root = Path::new("/safe/root");
        let err = validate_segment_payload(payload, Path::new("segment"), root)
            .expect_err("credential key rejected");

        assert!(err.to_string().contains("credential-like key"));
    }

    #[test]
    fn csm_snapshot_segment_manifest_linkage_is_hash_addressed() {
        let payload = br#"{"cycle_id":"cycle-000001"}"#;
        let segment = encode_snapshot_segment(&test_metadata(), payload).expect("encode");
        let decoded = decode_snapshot_segment(&segment).expect("decode");
        let segment_sha = sha256_bytes(&segment);
        let segment_ref = BinarySegmentRef {
            role: "continuity_checkpoint_snapshot".to_string(),
            source_ref: decoded.metadata.source_ref.clone(),
            segment_ref: "segments/continuity_checkpoint.snapshot.segment".to_string(),
            sha256: segment_sha.clone(),
            payload_sha256: decoded.payload_sha256,
            bytes: segment.len() as u64,
            schema: decoded.metadata.schema,
            format_version: decoded.metadata.format_version,
            hash_address: format!("sha256:{segment_sha}"),
            required: true,
        };

        assert_eq!(
            segment_ref.hash_address,
            format!("sha256:{}", segment_ref.sha256)
        );
        assert_eq!(segment_ref.schema, SNAPSHOT_SEGMENT_SCHEMA);
        assert_eq!(segment_ref.format_version, SNAPSHOT_SEGMENT_FORMAT_VERSION);
    }
}
