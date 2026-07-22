use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::VerifyingKey;

use crate::{ErrorCode, Limits, RecordError, RecordKind, Result};

#[derive(Debug, Clone)]
pub struct TrustEntry {
    pub verifying_key: VerifyingKey,
    pub profile_version: u16,
    pub allowed_kinds: BTreeSet<RecordKind>,
    pub not_before: u64,
    pub not_after: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TrustPolicy {
    entries: BTreeMap<String, TrustEntry>,
}

impl TrustPolicy {
    pub fn new(entries: BTreeMap<String, TrustEntry>, limits: &Limits) -> Result<Self> {
        if entries.is_empty() || entries.len() > limits.max_metadata_entries {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "trust entry bound exceeded",
            ));
        }
        if entries
            .keys()
            .any(|key| key.is_empty() || key.len() > limits.max_string_bytes)
        {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "trust key id bound exceeded",
            ));
        }
        Ok(Self { entries })
    }

    pub(crate) fn authorize(
        &self,
        key_id: &str,
        profile_version: u16,
        kind: RecordKind,
        logical_time: u64,
    ) -> Result<&VerifyingKey> {
        let entry = self
            .entries
            .get(key_id)
            .ok_or_else(|| RecordError::new(ErrorCode::Trust, "unknown signing key"))?;
        if entry.revoked
            || entry.profile_version != profile_version
            || !entry.allowed_kinds.contains(&kind)
            || logical_time < entry.not_before
            || logical_time > entry.not_after
        {
            return Err(RecordError::new(
                ErrorCode::Trust,
                "trust policy rejected envelope",
            ));
        }
        Ok(&entry.verifying_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplayToken {
    pub key_id: String,
    pub subject_id: String,
    pub record_id: String,
    pub sequence: u64,
    pub payload_digest: [u8; 32],
}

pub trait ReplayGuard {
    fn admit(&mut self, token: ReplayToken) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct InMemoryReplayGuard {
    maximum: usize,
    admitted: BTreeSet<ReplayToken>,
    last_sequence: BTreeMap<(String, String), u64>,
}

impl InMemoryReplayGuard {
    pub fn new(limits: &Limits) -> Self {
        Self {
            maximum: limits.max_replay_entries,
            admitted: BTreeSet::new(),
            last_sequence: BTreeMap::new(),
        }
    }
}

impl ReplayGuard for InMemoryReplayGuard {
    fn admit(&mut self, token: ReplayToken) -> Result<()> {
        if self.admitted.len() >= self.maximum {
            return Err(RecordError::new(
                ErrorCode::Bounds,
                "replay guard capacity reached",
            ));
        }
        if self.admitted.contains(&token) {
            return Err(RecordError::new(ErrorCode::Replay, "envelope replayed"));
        }
        let stream = (token.key_id.clone(), token.subject_id.clone());
        if self
            .last_sequence
            .get(&stream)
            .is_some_and(|last| token.sequence <= *last)
        {
            return Err(RecordError::new(
                ErrorCode::Replay,
                "sequence rollback or reuse",
            ));
        }
        self.last_sequence.insert(stream, token.sequence);
        self.admitted.insert(token);
        Ok(())
    }
}
