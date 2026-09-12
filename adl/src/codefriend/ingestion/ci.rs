//! CI provenance is separate from the common packet and never supplies source authority.
use super::{local, unsafe_content, AdmissionInput, Packet, Scope};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::OpenOptions, io::Write, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema: String,
    pub packet_id: String,
    pub source_revision: String,
    pub candidate_revision: String,
    pub metadata: BTreeMap<String, String>,
    /// Acquisition does not establish external artifact delivery.
    pub delivery: String,
}

pub fn validate_metadata(metadata: &BTreeMap<String, String>) -> Result<()> {
    ensure!(metadata.len() <= 3, "invalid_ci_metadata");
    for (key, value) in metadata {
        ensure!(
            value.len() <= 100 && !value.is_empty(),
            "invalid_ci_metadata"
        );
        ensure!(!unsafe_content("", value), "unsafe_ci_metadata");
        let valid = match key.as_str() {
            "run_id" | "run_attempt" => value.bytes().all(|b| b.is_ascii_digit()),
            "job" => value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'),
            _ => false,
        };
        ensure!(valid, "invalid_ci_metadata");
    }
    Ok(())
}

pub fn acquire(
    checkout: &Path,
    repository: &str,
    revision: &str,
    scope: Scope,
    candidate_revision: &str,
    metadata: BTreeMap<String, String>,
) -> Result<(Packet, Receipt)> {
    // Validate all provenance before any capture or durable write.
    ensure!(
        super::object_id(candidate_revision),
        "exact_candidate_revision_required"
    );
    validate_metadata(&metadata)?;
    let packet = local::acquire(checkout, repository, revision, scope)?;
    let receipt = Receipt {
        schema: "codefriend.ci_acquisition_receipt.v1".into(),
        packet_id: packet.packet_id.clone(),
        source_revision: packet.revision.clone(),
        candidate_revision: candidate_revision.into(),
        metadata,
        delivery: "not_established".into(),
    };
    Ok((packet, receipt))
}

impl Receipt {
    pub fn validate(&self, packet: &Packet) -> Result<()> {
        packet.validate()?;
        ensure!(
            self.schema == "codefriend.ci_acquisition_receipt.v1"
                && self.packet_id == packet.packet_id
                && self.source_revision == packet.revision
                && super::object_id(&self.candidate_revision)
                && self.delivery == "not_established",
            "invalid_ci_receipt"
        );
        validate_metadata(&self.metadata)
    }
}

/// A failure may leave the already-published packet; it never reports delivered evidence.
pub fn write(
    checkout: &Path,
    output: &Path,
    receipt_output: &Path,
    packet: &Packet,
    receipt: &Receipt,
) -> Result<()> {
    receipt.validate(packet)?;
    let parent = receipt_output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("receipt_parent_not_found"))?;
    let root = checkout
        .canonicalize()
        .map_err(|_| anyhow::anyhow!("checkout_not_found"))?;
    ensure!(!parent.starts_with(root), "receipt_inside_source_rejected");
    local::write_packet(checkout, output, packet)?;
    let admitted = AdmissionInput::read(output)?;
    receipt.validate(admitted.packet())?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(receipt_output)
        .map_err(|_| anyhow::anyhow!("receipt_create_failed_or_exists"))?;
    file.write_all(&serde_json::to_vec_pretty(receipt)?)
        .map_err(|_| anyhow::anyhow!("receipt_write_failed"))?;
    file.sync_all()
        .map_err(|_| anyhow::anyhow!("receipt_sync_failed"))?;
    Ok(())
}
