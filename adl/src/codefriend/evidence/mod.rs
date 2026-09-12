//! Governed local evidence admission and shared consumer contracts.
//! Repository text is inert evidence, never execution or publication authority.
pub mod contracts;
pub mod store;
use crate::codefriend::ingestion::{digest, Packet};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
pub const VERSION: &str = "codefriend.evidence.v1";
pub fn hash<T: Serialize>(value: &T) -> Result<String> {
    Ok(digest(&serde_json::to_vec(value)?))
}
pub fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub schema: String,
    pub id: String,
    pub repository: String,
    pub revision: String,
    pub path: String,
    pub source_object: String,
    pub content_digest: String,
    pub redaction: String,
    pub trust: String,
}
pub fn evidence(packet: &Packet) -> Result<Vec<Evidence>> {
    packet.validate()?;
    packet
        .objects
        .iter()
        .filter(|o| o.content.is_some())
        .map(|o| {
            let mut e = Evidence {
                schema: VERSION.into(),
                id: String::new(),
                repository: packet.repository.clone(),
                revision: packet.revision.clone(),
                path: o.path.clone(),
                source_object: o.source_object.clone().unwrap_or_default(),
                content_digest: o.content_digest.clone().unwrap_or_default(),
                redaction: "known_unsafe_whole_object_omission_v1".into(),
                trust: "untrusted_repository_text".into(),
            };
            e.id = hash(&(
                "codefriend.evidence_identity.v1",
                &e.repository,
                &e.revision,
                &e.path,
                &e.source_object,
                &e.content_digest,
            ))?;
            Ok(e)
        })
        .collect()
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Retention {
    pub seconds: u64,
}
impl Retention {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            (1..=31_536_000).contains(&self.seconds),
            "invalid_retention_policy"
        );
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    pub schema: String,
    pub packet: Packet,
    pub evidence: Vec<Evidence>,
    pub retention: Retention,
    pub admitted_at: u64,
    pub expires_at: u64,
    pub digest: String,
}
impl Admission {
    pub fn new(packet: Packet, retention: Retention, now: u64) -> Result<Self> {
        let evidence = evidence(&packet)?;
        retention.validate()?;
        let expires_at = now
            .checked_add(retention.seconds)
            .ok_or_else(|| anyhow::anyhow!("invalid_retention_deadline"))?;
        let mut record = Self {
            schema: VERSION.into(),
            packet,
            evidence,
            retention,
            admitted_at: now,
            expires_at,
            digest: String::new(),
        };
        record.digest = record.expected_digest()?;
        Ok(record)
    }
    fn expected_digest(&self) -> Result<String> {
        let mut copy = self.clone();
        copy.digest.clear();
        hash(&copy)
    }
    pub fn validate(&self) -> Result<()> {
        ensure!(self.schema == VERSION, "unsupported_evidence_version");
        self.retention.validate()?;
        ensure!(
            self.admitted_at.checked_add(self.retention.seconds) == Some(self.expires_at),
            "invalid_retention_deadline"
        );
        ensure!(
            self.evidence == evidence(&self.packet)?,
            "evidence_provenance_mismatch"
        );
        ensure!(
            self.digest == self.expected_digest()?,
            "admission_digest_mismatch"
        );
        Ok(())
    }
}
