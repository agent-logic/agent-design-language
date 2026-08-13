//! Deterministic, fail-closed placement over already-authorized distributed evidence.
//!
//! This module intentionally remains unregistered until integration issue #5878. It consumes
//! committed membership and the verified/admitted projections owned by the preceding distributed
//! modules; it does not verify wire messages, grant leases, activate owners, or mutate authority.

use std::{cmp::Ordering, collections::BTreeMap, fmt, sync::Mutex, time::SystemTime};

use serde::Deserialize;
use sha2::{Digest, Sha256};

#[cfg(not(test))]
use super::authority_store_adapters::{AuthorityBoundFencingStore, AuthorityBoundLeaseLedger};
use super::{
    capability_advertisement::{CapabilityAdvertisementVerifier, VerifiedCapabilityAdvertisement},
    certificates::CertificatePurpose,
    fencing::FenceReceipt,
    lease::{decode_certificate, LeaseState, OperationClass, AUTHORITY_SNAPSHOT_SCHEMA},
    membership::{Member, MemberRole, MembershipState},
    resource_weather::{
        PlacementWeather, ResourceWeatherCertificateAuthority, ResourceWeatherStore,
        WeatherAvailability,
    },
};
#[cfg(test)]
use super::{fencing::FencingStore, lease::AuthorityLedger};

#[cfg(not(test))]
type PlacementLeaseAuthority = AuthorityBoundLeaseLedger;
#[cfg(test)]
type PlacementLeaseAuthority = AuthorityLedger;
#[cfg(not(test))]
type PlacementFencingAuthority = AuthorityBoundFencingStore;
#[cfg(test)]
type PlacementFencingAuthority = FencingStore;

fn placement_lease_snapshot(ledger: &PlacementLeaseAuthority) -> PlacementResult<Vec<u8>> {
    ledger
        .snapshot()
        .map_err(|_| PlacementError::InconsistentEvidence)
}

fn placement_fencing_floor(
    store: &PlacementFencingAuthority,
    lineage_id: &[u8],
) -> PlacementResult<Option<FenceReceipt>> {
    #[cfg(not(test))]
    return store
        .floor(lineage_id)
        .map_err(|_| PlacementError::InconsistentEvidence);
    #[cfg(test)]
    return Ok(store.floor(lineage_id).cloned());
}

const MAX_TEXT_BYTES: usize = 128;
const ABSOLUTE_MAX_INPUTS: usize = 4096;
const ABSOLUTE_MAX_REQUIREMENTS: usize = 256;
const ABSOLUTE_MAX_REQUIRED_UNITS: u32 = 1_000_000_000;
const ABSOLUTE_MAX_TOTAL_REQUIRED_UNITS: u64 = 1_000_000_000_000;
const ABSOLUTE_MAX_REQUIRED_SLOTS: u16 = 16_384;
const ABSOLUTE_MAX_FUTURE_SKEW_SECS: u64 = 300;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlacementError {
    InvalidPolicy,
    InvalidRequest,
    NonCanonicalRequirements,
    ResourceExhausted,
    StaleMembership,
    WrongTrustDomain,
    InconsistentEvidence,
    FencingAheadOfMembership,
    NoEligibleTarget,
    RevisionDrift,
    AuthorityUnavailable,
}

impl PlacementError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidRequest => "invalid_request",
            Self::NonCanonicalRequirements => "noncanonical_requirements",
            Self::ResourceExhausted => "resource_exhausted",
            Self::StaleMembership => "stale_membership",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::InconsistentEvidence => "inconsistent_evidence",
            Self::FencingAheadOfMembership => "fencing_ahead_of_membership",
            Self::NoEligibleTarget => "no_eligible_target",
            Self::RevisionDrift => "revision_drift",
            Self::AuthorityUnavailable => "authority_unavailable",
        }
    }
}

impl fmt::Display for PlacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for PlacementError {}
pub type PlacementResult<T> = Result<T, PlacementError>;

#[derive(Clone, Debug)]
pub struct PlacementPolicy {
    trust_domain: String,
    max_inputs: usize,
    max_requirements: usize,
    max_required_units: u32,
    max_total_required_units: u64,
    max_required_slots: u16,
    max_pressure_permille: u16,
    max_future_skew_secs: u64,
}

impl PlacementPolicy {
    pub fn new(trust_domain: impl Into<String>) -> PlacementResult<Self> {
        Self::with_bounds(trust_domain, 256, 64, 1_000_000, 10_000_000, 4096, 900, 5)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_bounds(
        trust_domain: impl Into<String>,
        max_inputs: usize,
        max_requirements: usize,
        max_required_units: u32,
        max_total_required_units: u64,
        max_required_slots: u16,
        max_pressure_permille: u16,
        max_future_skew_secs: u64,
    ) -> PlacementResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_inputs,
            max_requirements,
            max_required_units,
            max_total_required_units,
            max_required_slots,
            max_pressure_permille,
            max_future_skew_secs,
        };
        if !valid_text(&policy.trust_domain)
            || max_inputs == 0
            || max_inputs > ABSOLUTE_MAX_INPUTS
            || max_requirements == 0
            || max_requirements > ABSOLUTE_MAX_REQUIREMENTS
            || max_required_units == 0
            || max_required_units > ABSOLUTE_MAX_REQUIRED_UNITS
            || max_total_required_units == 0
            || max_total_required_units > ABSOLUTE_MAX_TOTAL_REQUIRED_UNITS
            || max_total_required_units < u64::from(max_required_units)
            || max_required_slots == 0
            || max_required_slots > ABSOLUTE_MAX_REQUIRED_SLOTS
            || max_pressure_permille > 1_000
            || max_future_skew_secs > ABSOLUTE_MAX_FUTURE_SKEW_SECS
        {
            return Err(PlacementError::InvalidPolicy);
        }
        Ok(policy)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CapabilityRequirement {
    pub capability: String,
    pub required_units: u32,
}

impl CapabilityRequirement {
    pub fn new(capability: impl Into<String>, required_units: u32) -> Self {
        Self {
            capability: capability.into(),
            required_units,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementRequest {
    pub lineage_id: String,
    pub minimum_membership_epoch: u64,
    pub minimum_committed_log_index: u64,
    pub required_slots: u16,
    pub requirements: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug)]
pub struct PlacementInputs<'a> {
    pub membership: &'a MembershipState,
    pub capabilities: &'a PlacementCapabilitySnapshot,
    pub weather: &'a PlacementWeatherSnapshot,
    pub fencing: &'a PlacementFencingSnapshot,
}

/// Opaque capability evidence produced only by signature and certificate verification.
#[derive(Clone, Debug)]
pub struct PlacementCapabilitySnapshot {
    captured_at_unix_secs: u64,
    rows: Vec<VerifiedCapabilityAdvertisement>,
}

impl PlacementCapabilitySnapshot {
    pub fn capture(
        verifier: &CapabilityAdvertisementVerifier,
        signed_advertisements: &[Vec<u8>],
        now_unix_secs: u64,
    ) -> PlacementResult<Self> {
        let rows = signed_advertisements
            .iter()
            .map(|bytes| {
                verifier
                    .decode_and_verify(bytes, now_unix_secs)
                    .map_err(|_| PlacementError::InconsistentEvidence)
            })
            .collect::<PlacementResult<Vec<_>>>()?;
        Ok(Self {
            captured_at_unix_secs: now_unix_secs,
            rows,
        })
    }

    #[cfg(test)]
    pub fn from_rows_for_test(
        now_unix_secs: u64,
        rows: &[VerifiedCapabilityAdvertisement],
    ) -> Self {
        Self {
            captured_at_unix_secs: now_unix_secs,
            rows: rows.to_vec(),
        }
    }
}

/// Opaque weather projected from the durable verified-advertisement store at service time.
#[derive(Clone, Debug)]
pub struct PlacementWeatherSnapshot {
    captured_at_unix_secs: u64,
    rows: Vec<BoundWeather>,
}

#[derive(Clone, Debug)]
struct BoundWeather {
    row: PlacementWeather,
    certificate_id: String,
}

impl std::ops::Deref for BoundWeather {
    type Target = PlacementWeather;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

impl PlacementWeatherSnapshot {
    pub fn capture<C: ResourceWeatherCertificateAuthority>(
        store: &ResourceWeatherStore,
        certificates: &C,
        now_unix_secs: u64,
    ) -> PlacementResult<Self> {
        let projected = store
            .snapshot(certificates, now_unix_secs)
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        let rows = projected
            .into_iter()
            .map(|row| {
                let authorized = certificates
                    .authorize_weather(
                        &row.holder_id,
                        CertificatePurpose::AdvertisementSigning,
                        row.certificate_generation,
                        now_unix_secs,
                    )
                    .map_err(|_| PlacementError::InconsistentEvidence)?;
                Ok(BoundWeather {
                    row,
                    certificate_id: authorized.certificate_id,
                })
            })
            .collect::<PlacementResult<Vec<_>>>()?;
        Ok(Self {
            captured_at_unix_secs: now_unix_secs,
            rows,
        })
    }

    #[cfg(test)]
    pub fn from_rows_for_test(now_unix_secs: u64, rows: &[(PlacementWeather, String)]) -> Self {
        Self {
            captured_at_unix_secs: now_unix_secs,
            rows: rows
                .iter()
                .map(|(row, certificate_id)| BoundWeather {
                    row: row.clone(),
                    certificate_id: certificate_id.clone(),
                })
                .collect(),
        }
    }
}

/// A complete, opaque projection of fencing floors for one exact membership revision.
///
/// Production callers can construct this only through governed lease and fencing adapters for
/// every committed member identity. This prevents a caller from making a fenced node eligible by
/// merely omitting its receipt from a caller-selected slice.
#[derive(Clone, Debug)]
pub struct PlacementFencingSnapshot {
    membership_epoch: u64,
    committed_log_index: u64,
    fenced: BTreeMap<String, FenceReceipt>,
}

impl PlacementFencingSnapshot {
    pub fn capture(
        policy: &PlacementPolicy,
        membership: &MembershipState,
        ledger: &PlacementLeaseAuthority,
        store: &PlacementFencingAuthority,
    ) -> PlacementResult<Self> {
        let bytes = placement_lease_snapshot(ledger)?;
        let snapshot: AuthoritySnapshotEnvelopeView =
            serde_json::from_slice(&bytes).map_err(|_| PlacementError::InconsistentEvidence)?;
        if snapshot.body.schema != AUTHORITY_SNAPSHOT_SCHEMA
            || snapshot.body.applied_log_index != membership.committed_log_index()
        {
            return Err(PlacementError::StaleMembership);
        }
        let lineages = snapshot
            .body
            .leases
            .into_iter()
            .map(PlacementLineageBinding::try_from)
            .collect::<PlacementResult<Vec<_>>>()?;
        let receipts = lineages
            .iter()
            .map(|lineage| {
                placement_fencing_floor(store, lineage.lineage_id.as_bytes())
                    .map(|receipt| receipt.map(|receipt| (lineage.lineage_id.clone(), receipt)))
            })
            .collect::<PlacementResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        build_fencing_snapshot(policy, membership, lineages, receipts)
    }

    #[cfg(test)]
    pub fn from_receipts_for_test(
        policy: &PlacementPolicy,
        membership: &MembershipState,
        receipts: &[FenceReceipt],
    ) -> PlacementResult<Self> {
        let lineages = receipts
            .iter()
            .map(|receipt| {
                let lineage_id = std::str::from_utf8(&receipt.lineage_id)
                    .map_err(|_| PlacementError::InconsistentEvidence)?;
                let node_id = lineage_id
                    .strip_prefix("lineage-")
                    .map(|suffix| format!("node-{suffix}"))
                    .ok_or(PlacementError::InconsistentEvidence)?;
                let member = membership
                    .members()
                    .find(|member| member.node_id == node_id)
                    .ok_or(PlacementError::InconsistentEvidence)?;
                Ok(PlacementLineageBinding {
                    lineage_id: lineage_id.to_owned(),
                    node_id: member.node_id.clone(),
                    guardian_id: member.guardian_id.clone(),
                    epoch: receipt.epoch,
                    committed_log_index: receipt.committed_log_index,
                    operation_class: receipt.operation_class,
                    revoked: true,
                })
            })
            .collect::<PlacementResult<Vec<_>>>()?;
        let receipts = receipts
            .iter()
            .map(|receipt| {
                let lineage_id = std::str::from_utf8(&receipt.lineage_id)
                    .map_err(|_| PlacementError::InconsistentEvidence)?;
                Ok((lineage_id.to_owned(), receipt.clone()))
            })
            .collect::<PlacementResult<BTreeMap<_, _>>>()?;
        build_fencing_snapshot(policy, membership, lineages, receipts)
    }

    #[cfg(test)]
    pub fn missing_floor_for_test(
        policy: &PlacementPolicy,
        membership: &MembershipState,
        lineage_id: &str,
        node_id: &str,
        guardian_id: &str,
    ) -> PlacementResult<Self> {
        build_fencing_snapshot(
            policy,
            membership,
            vec![PlacementLineageBinding {
                lineage_id: lineage_id.to_owned(),
                node_id: node_id.to_owned(),
                guardian_id: guardian_id.to_owned(),
                epoch: 1,
                committed_log_index: membership.committed_log_index(),
                operation_class: OperationClass::Fence as u32,
                revoked: true,
            }],
            BTreeMap::new(),
        )
    }

    #[cfg(test)]
    pub fn active_successor_for_test(
        policy: &PlacementPolicy,
        membership: &MembershipState,
        lease: LeaseState,
        receipt: FenceReceipt,
    ) -> PlacementResult<Self> {
        let lineage_id = std::str::from_utf8(&receipt.lineage_id)
            .map_err(|_| PlacementError::InconsistentEvidence)?
            .to_owned();
        let binding = PlacementLineageBinding::try_from(lease)?;
        build_fencing_snapshot(
            policy,
            membership,
            vec![binding],
            BTreeMap::from([(lineage_id, receipt)]),
        )
    }
}

pub trait PlacementClock: fmt::Debug + Send + Sync {
    fn now_unix_secs(&self) -> PlacementResult<u64>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemPlacementClock;

impl PlacementClock for SystemPlacementClock {
    fn now_unix_secs(&self) -> PlacementResult<u64> {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .map_err(|_| PlacementError::InconsistentEvidence)
    }
}

pub struct PlacementService<C> {
    policy: PlacementPolicy,
    clock: C,
    authority: Mutex<PlacementAuthorityState>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlacementAuthorityRevision {
    sequence: u64,
    content_sha256: [u8; 32],
}

impl PlacementAuthorityRevision {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn content_sha256(&self) -> [u8; 32] {
        self.content_sha256
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedPlacementRow {
    lineage_ref: String,
    node_ref: String,
    guardian_ref: String,
    capability_sequence: u64,
    weather_sequence: u64,
    freshness: PlacementFreshness,
    capacity: PlacementCapacityBand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementFreshness {
    VerifiedAtDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlacementCapacityBand {
    Available,
    Constrained,
    Unavailable,
}

impl RedactedPlacementRow {
    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }

    pub fn node_ref(&self) -> &str {
        &self.node_ref
    }

    pub fn guardian_ref(&self) -> &str {
        &self.guardian_ref
    }

    pub fn capability_sequence(&self) -> u64 {
        self.capability_sequence
    }

    pub fn weather_sequence(&self) -> u64 {
        self.weather_sequence
    }

    pub fn freshness(&self) -> PlacementFreshness {
        self.freshness
    }

    pub fn capacity(&self) -> PlacementCapacityBand {
        self.capacity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedPlacementSnapshot {
    trust_domain: String,
    membership_epoch: u64,
    committed_log_index: u64,
    revision: PlacementAuthorityRevision,
    rows: Vec<RedactedPlacementRow>,
}

impl RedactedPlacementSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn membership_epoch(&self) -> u64 {
        self.membership_epoch
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn revision(&self) -> PlacementAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedPlacementRow> {
        self.rows.iter()
    }
}

#[derive(Clone, Debug)]
struct StoredPlacementDecision {
    decision: PlacementDecision,
    captured_at_unix_secs: u64,
}

#[derive(Debug, Default)]
struct PlacementAuthorityState {
    available: bool,
    sequence: u64,
    membership_epoch: u64,
    committed_log_index: u64,
    decisions: BTreeMap<String, StoredPlacementDecision>,
}

impl<C: PlacementClock> PlacementService<C> {
    pub fn new(policy: PlacementPolicy, clock: C) -> Self {
        Self {
            policy,
            clock,
            authority: Mutex::new(PlacementAuthorityState::default()),
        }
    }

    pub fn decide(
        &self,
        request: &PlacementRequest,
        inputs: PlacementInputs<'_>,
    ) -> PlacementResult<PlacementDecision> {
        let now_unix_secs = self.clock.now_unix_secs()?;
        if inputs.capabilities.captured_at_unix_secs != now_unix_secs
            || inputs.weather.captured_at_unix_secs != now_unix_secs
        {
            return Err(PlacementError::InconsistentEvidence);
        }
        let decision = decide(&self.policy, now_unix_secs, request, inputs)?;
        self.retain_decision(decision.clone(), now_unix_secs)?;
        Ok(decision)
    }

    pub fn remove_decision(&self, lineage_id: &str) -> PlacementResult<bool> {
        if !valid_text(lineage_id) {
            return Err(PlacementError::InvalidRequest);
        }
        let mut authority = self
            .authority
            .lock()
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        if !authority.decisions.contains_key(lineage_id) {
            return Ok(false);
        }
        let next = authority
            .sequence
            .checked_add(1)
            .ok_or(PlacementError::ResourceExhausted)?;
        authority.decisions.remove(lineage_id);
        authority.sequence = next;
        Ok(true)
    }

    pub fn authority_revision(&self) -> PlacementResult<PlacementAuthorityRevision> {
        let authority = self
            .authority
            .lock()
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        if !authority.available {
            return Err(PlacementError::AuthorityUnavailable);
        }
        Ok(placement_revision(&authority))
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: PlacementAuthorityRevision,
    ) -> PlacementResult<RedactedPlacementSnapshot> {
        let authority = self
            .authority
            .lock()
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        if !authority.available {
            return Err(PlacementError::AuthorityUnavailable);
        }
        let revision = placement_revision(&authority);
        if revision != expected_revision {
            return Err(PlacementError::RevisionDrift);
        }
        if authority.decisions.len() > self.policy.max_inputs {
            return Err(PlacementError::ResourceExhausted);
        }
        let rows = authority
            .decisions
            .values()
            .map(|stored| RedactedPlacementRow {
                lineage_ref: projection_ref(b"lineage", stored.decision.lineage_id.as_bytes()),
                node_ref: projection_ref(b"node", stored.decision.node_id.as_bytes()),
                guardian_ref: projection_ref(b"guardian", stored.decision.guardian_id.as_bytes()),
                capability_sequence: stored.decision.capability_sequence,
                weather_sequence: stored.decision.weather_sequence,
                freshness: PlacementFreshness::VerifiedAtDecision,
                capacity: if stored.decision.remaining_slots == 0 {
                    PlacementCapacityBand::Unavailable
                } else if stored.decision.pressure_permille >= 800 {
                    PlacementCapacityBand::Constrained
                } else {
                    PlacementCapacityBand::Available
                },
            })
            .collect();
        Ok(RedactedPlacementSnapshot {
            trust_domain: self.policy.trust_domain.clone(),
            membership_epoch: authority.membership_epoch,
            committed_log_index: authority.committed_log_index,
            revision,
            rows,
        })
    }

    #[cfg(test)]
    pub(crate) fn seed_decision_for_snapshot_test(
        &self,
        decision: PlacementDecision,
        captured_at_unix_secs: u64,
    ) -> PlacementResult<()> {
        self.retain_decision(decision, captured_at_unix_secs)
    }

    fn retain_decision(
        &self,
        decision: PlacementDecision,
        captured_at_unix_secs: u64,
    ) -> PlacementResult<()> {
        let mut authority = self
            .authority
            .lock()
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        let incoming_cut = (decision.membership_epoch, decision.committed_log_index);
        let current_cut = (authority.membership_epoch, authority.committed_log_index);
        if current_cut != (0, 0) && incoming_cut < current_cut {
            return Err(PlacementError::StaleMembership);
        }
        let replaces_cut = current_cut != (0, 0) && incoming_cut > current_cut;
        let is_new = !authority.decisions.contains_key(&decision.lineage_id);
        if !replaces_cut && is_new && authority.decisions.len() >= self.policy.max_inputs {
            return Err(PlacementError::ResourceExhausted);
        }
        let next = authority
            .sequence
            .checked_add(1)
            .ok_or(PlacementError::ResourceExhausted)?;
        if replaces_cut {
            authority.decisions.clear();
        }
        authority.membership_epoch = decision.membership_epoch;
        authority.committed_log_index = decision.committed_log_index;
        authority.available = true;
        authority.decisions.insert(
            decision.lineage_id.clone(),
            StoredPlacementDecision {
                decision,
                captured_at_unix_secs,
            },
        );
        authority.sequence = next;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementDecision {
    pub lineage_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub membership_epoch: u64,
    pub committed_log_index: u64,
    pub capability_sequence: u64,
    pub weather_sequence: u64,
    pub pressure_permille: u16,
    pub remaining_slots: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rank {
    pressure_permille: u16,
    remaining_slots: u16,
}

fn placement_revision(authority: &PlacementAuthorityState) -> PlacementAuthorityRevision {
    let mut digest = Sha256::new();
    digest.update(b"ADL-PLACEMENT-AUTHORITY-REVISION-V1\0");
    digest.update(authority.membership_epoch.to_be_bytes());
    digest.update(authority.committed_log_index.to_be_bytes());
    for stored in authority.decisions.values() {
        let decision = &stored.decision;
        for value in [
            decision.lineage_id.as_bytes(),
            decision.node_id.as_bytes(),
            decision.guardian_id.as_bytes(),
        ] {
            digest.update((value.len() as u64).to_be_bytes());
            digest.update(value);
        }
        digest.update(decision.capability_sequence.to_be_bytes());
        digest.update(decision.weather_sequence.to_be_bytes());
        digest.update(stored.captured_at_unix_secs.to_be_bytes());
        digest.update(decision.pressure_permille.to_be_bytes());
        digest.update(decision.remaining_slots.to_be_bytes());
    }
    PlacementAuthorityRevision {
        sequence: authority.sequence,
        content_sha256: digest.finalize().into(),
    }
}

fn projection_ref(kind: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"adl-projection-ref-v1");
    digest.update((kind.len() as u64).to_be_bytes());
    digest.update(kind);
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
    format!("id_{}", hex::encode(digest.finalize()))
}

impl Rank {
    fn compare(self, other: Self) -> Ordering {
        self.pressure_permille
            .cmp(&other.pressure_permille)
            .then_with(|| other.remaining_slots.cmp(&self.remaining_slots))
    }
}

fn decide(
    policy: &PlacementPolicy,
    now_unix_secs: u64,
    request: &PlacementRequest,
    inputs: PlacementInputs<'_>,
) -> PlacementResult<PlacementDecision> {
    validate_request(policy, request)?;
    if membership_trust_domain(inputs.membership)? != policy.trust_domain {
        return Err(PlacementError::WrongTrustDomain);
    }
    if inputs.membership.epoch() < request.minimum_membership_epoch
        || inputs.membership.committed_log_index() < request.minimum_committed_log_index
        || inputs.membership.epoch() == 0
        || inputs.membership.committed_log_index() == 0
    {
        return Err(PlacementError::StaleMembership);
    }
    if inputs.membership.members().count() > policy.max_inputs
        || inputs.capabilities.rows.len() > policy.max_inputs
        || inputs.weather.rows.len() > policy.max_inputs
    {
        return Err(PlacementError::ResourceExhausted);
    }

    if inputs.fencing.membership_epoch != inputs.membership.epoch()
        || inputs.fencing.committed_log_index != inputs.membership.committed_log_index()
    {
        return Err(PlacementError::StaleMembership);
    }

    let members = index_members(inputs.membership)?;
    let capabilities = index_capabilities(policy, &inputs.capabilities.rows, &members)?;
    let weather = index_weather(&inputs.weather.rows, &members)?;
    let fenced = &inputs.fencing.fenced;
    let mut candidates = Vec::new();

    for member in inputs.membership.members() {
        if member.role != MemberRole::Voter
            || fenced.contains_key(member.node_id.as_str())
            || fenced.contains_key(member.guardian_id.as_str())
        {
            continue;
        }
        let Some(capability) = lookup(&capabilities, member) else {
            continue;
        };
        let Some(weather) = lookup(&weather, member) else {
            continue;
        };
        if capability.issuer_id != member.guardian_id
            || weather.holder_id != member.guardian_id
            || capability.issuer_id != weather.holder_id
            || capability.certificate_generation != weather.certificate_generation
            || capability.certificate_id != weather.certificate_id
            || !fresh_capability(policy, capability, now_unix_secs)
            || !fresh_weather(policy, weather, now_unix_secs)
            || weather.availability == WeatherAvailability::Unavailable
            || !weather.advisory_only
        {
            continue;
        }
        let Some(slots) = weather.available_slots else {
            continue;
        };
        let Some(pressure) = weather.pressure_permille else {
            continue;
        };
        if slots < request.required_slots || pressure > policy.max_pressure_permille {
            continue;
        }
        if !requirements_met(capability, &request.requirements) {
            continue;
        }
        candidates.push((
            Rank {
                pressure_permille: pressure,
                remaining_slots: slots - request.required_slots,
            },
            member,
            capability,
            weather,
        ));
    }

    candidates.sort_by(|left, right| {
        left.0
            .compare(right.0)
            .then_with(|| left.1.node_id.cmp(&right.1.node_id))
            .then_with(|| left.1.guardian_id.cmp(&right.1.guardian_id))
    });
    let Some((rank, member, capability, weather)) = candidates.first() else {
        return Err(PlacementError::NoEligibleTarget);
    };
    Ok(PlacementDecision {
        lineage_id: request.lineage_id.clone(),
        node_id: member.node_id.clone(),
        guardian_id: member.guardian_id.clone(),
        membership_epoch: inputs.membership.epoch(),
        committed_log_index: inputs.membership.committed_log_index(),
        capability_sequence: capability.sequence,
        weather_sequence: weather.sequence,
        pressure_permille: rank.pressure_permille,
        remaining_slots: rank.remaining_slots,
    })
}

fn validate_request(policy: &PlacementPolicy, request: &PlacementRequest) -> PlacementResult<()> {
    if !valid_text(&request.lineage_id)
        || request.minimum_membership_epoch == 0
        || request.minimum_committed_log_index == 0
        || request.required_slots == 0
        || request.required_slots > policy.max_required_slots
        || request.requirements.is_empty()
        || request.requirements.len() > policy.max_requirements
    {
        return Err(PlacementError::InvalidRequest);
    }
    let mut total = 0_u64;
    for requirement in &request.requirements {
        if !valid_text(&requirement.capability)
            || requirement.required_units == 0
            || requirement.required_units > policy.max_required_units
        {
            return Err(PlacementError::InvalidRequest);
        }
        total = total
            .checked_add(u64::from(requirement.required_units))
            .ok_or(PlacementError::ResourceExhausted)?;
    }
    if total > policy.max_total_required_units {
        return Err(PlacementError::ResourceExhausted);
    }
    if request
        .requirements
        .windows(2)
        .any(|pair| pair[0].capability >= pair[1].capability)
    {
        return Err(PlacementError::NonCanonicalRequirements);
    }
    Ok(())
}

fn index_members(membership: &MembershipState) -> PlacementResult<BTreeMap<&str, &Member>> {
    let mut result = BTreeMap::new();
    for member in membership.members() {
        for identity in [member.node_id.as_str(), member.guardian_id.as_str()] {
            if result.insert(identity, member).is_some() {
                return Err(PlacementError::InconsistentEvidence);
            }
        }
    }
    Ok(result)
}

fn index_capabilities<'a>(
    policy: &PlacementPolicy,
    values: &'a [VerifiedCapabilityAdvertisement],
    members: &BTreeMap<&str, &Member>,
) -> PlacementResult<BTreeMap<&'a str, &'a VerifiedCapabilityAdvertisement>> {
    let mut result = BTreeMap::new();
    for value in values {
        if value.trust_domain != policy.trust_domain {
            return Err(PlacementError::WrongTrustDomain);
        }
        if !members.contains_key(value.issuer_id.as_str())
            || value.certificate_generation == 0
            || value.sequence == 0
            || result.insert(value.issuer_id.as_str(), value).is_some()
        {
            return Err(PlacementError::InconsistentEvidence);
        }
    }
    Ok(result)
}

fn index_weather<'a>(
    values: &'a [BoundWeather],
    members: &BTreeMap<&str, &Member>,
) -> PlacementResult<BTreeMap<&'a str, &'a BoundWeather>> {
    let mut result = BTreeMap::new();
    for value in values {
        if !members.contains_key(value.holder_id.as_str())
            || result.insert(value.holder_id.as_str(), value).is_some()
        {
            return Err(PlacementError::InconsistentEvidence);
        }
    }
    Ok(result)
}

#[derive(Clone, Debug)]
struct PlacementLineageBinding {
    lineage_id: String,
    node_id: String,
    guardian_id: String,
    epoch: u64,
    committed_log_index: u64,
    operation_class: u32,
    revoked: bool,
}

impl TryFrom<LeaseState> for PlacementLineageBinding {
    type Error = PlacementError;

    fn try_from(lease: LeaseState) -> PlacementResult<Self> {
        let certificate = decode_certificate(&lease.certificate_bytes)
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        let body = certificate
            .body
            .ok_or(PlacementError::InconsistentEvidence)?;
        if body.lineage_id != lease.lineage_id
            || body.holder_node_id != lease.holder_node_id
            || body.holder_guardian_id != lease.holder_guardian_id
            || body.epoch != lease.epoch
            || body.committed_log_index != lease.committed_log_index
        {
            return Err(PlacementError::InconsistentEvidence);
        }
        Ok(Self {
            lineage_id: String::from_utf8(lease.lineage_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
            node_id: String::from_utf8(lease.holder_node_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
            guardian_id: String::from_utf8(lease.holder_guardian_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
            epoch: lease.epoch,
            committed_log_index: lease.committed_log_index,
            operation_class: body.operation_class,
            revoked: lease.revoked,
        })
    }
}

fn build_fencing_snapshot(
    policy: &PlacementPolicy,
    membership: &MembershipState,
    lineages: Vec<PlacementLineageBinding>,
    receipts: BTreeMap<String, FenceReceipt>,
) -> PlacementResult<PlacementFencingSnapshot> {
    if membership_trust_domain(membership)? != policy.trust_domain {
        return Err(PlacementError::WrongTrustDomain);
    }
    let members = index_members(membership)?;
    if lineages.len() > policy.max_inputs || receipts.len() > policy.max_inputs {
        return Err(PlacementError::ResourceExhausted);
    }
    let mut result = BTreeMap::new();
    let mut seen_lineages = BTreeMap::new();
    for lineage in lineages {
        if !valid_text(&lineage.lineage_id)
            || !valid_text(&lineage.node_id)
            || !valid_text(&lineage.guardian_id)
            || lineage.epoch == 0
            || lineage.committed_log_index == 0
            || seen_lineages
                .insert(lineage.lineage_id.clone(), ())
                .is_some()
        {
            return Err(PlacementError::InconsistentEvidence);
        }
        let Some(member) = members.get(lineage.node_id.as_str()) else {
            return Err(PlacementError::InconsistentEvidence);
        };
        if member.guardian_id != lineage.guardian_id {
            return Err(PlacementError::InconsistentEvidence);
        }
        let receipt = receipts.get(&lineage.lineage_id);
        if lineage.revoked && receipt.is_none() {
            return Err(PlacementError::InconsistentEvidence);
        }
        let Some(value) = receipt else {
            continue;
        };
        let trust_domain = std::str::from_utf8(&value.trust_domain_id)
            .map_err(|_| PlacementError::WrongTrustDomain)?;
        if trust_domain != policy.trust_domain {
            return Err(PlacementError::WrongTrustDomain);
        }
        if value.committed_log_index > membership.committed_log_index() {
            return Err(PlacementError::FencingAheadOfMembership);
        }
        if value.lineage_id != lineage.lineage_id.as_bytes()
            || value.epoch == 0
            || value.committed_log_index == 0
            || value.voter_set_generation == 0
            || (value.operation_class != OperationClass::Fence as u32
                && value.operation_class != OperationClass::Revoke as u32)
        {
            return Err(PlacementError::InconsistentEvidence);
        }
        if !lineage.revoked {
            if value.epoch < lineage.epoch {
                continue;
            }
            if value.epoch == lineage.epoch
                && value.committed_log_index < lineage.committed_log_index
                && matches!(lineage.operation_class, 2 | 4 | 5)
            {
                continue;
            }
            return Err(PlacementError::InconsistentEvidence);
        }
        if value.epoch != lineage.epoch
            || value.committed_log_index != lineage.committed_log_index
            || value.operation_class != lineage.operation_class
        {
            return Err(PlacementError::InconsistentEvidence);
        }
        result.insert(lineage.node_id.clone(), value.clone());
        result.insert(lineage.guardian_id.clone(), value.clone());
    }
    if receipts
        .keys()
        .any(|lineage| !seen_lineages.contains_key(lineage))
    {
        return Err(PlacementError::InconsistentEvidence);
    }
    Ok(PlacementFencingSnapshot {
        membership_epoch: membership.epoch(),
        committed_log_index: membership.committed_log_index(),
        fenced: result,
    })
}

fn lookup<'a, T>(values: &'a BTreeMap<&str, T>, member: &Member) -> Option<&'a T> {
    values
        .get(member.node_id.as_str())
        .or_else(|| values.get(member.guardian_id.as_str()))
}

fn fresh_capability(
    policy: &PlacementPolicy,
    value: &VerifiedCapabilityAdvertisement,
    now: u64,
) -> bool {
    let future_limit = now.saturating_add(policy.max_future_skew_secs);
    value.measured_at_unix_secs <= future_limit
        && now < value.expires_at_unix_secs
        && now < value.verification_deadline_unix_secs
}

fn fresh_weather(policy: &PlacementPolicy, value: &PlacementWeather, now: u64) -> bool {
    let future_limit = now.saturating_add(policy.max_future_skew_secs);
    value.certificate_generation > 0
        && value.sequence > 0
        && value.sampled_at_unix_secs <= future_limit
        && now < value.expires_at_unix_secs
}

fn requirements_met(
    capability: &VerifiedCapabilityAdvertisement,
    requirements: &[CapabilityRequirement],
) -> bool {
    requirements.iter().all(|requirement| {
        capability
            .capabilities
            .binary_search_by(|evidence| evidence.capability.cmp(&requirement.capability))
            .ok()
            .is_some_and(|index| {
                capability.capabilities[index].observed_units >= requirement.required_units
            })
    })
}

fn valid_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_TEXT_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

#[derive(Deserialize)]
struct MembershipSnapshotEnvelopeView {
    body: MembershipSnapshotBodyView,
}

#[derive(Deserialize)]
struct MembershipSnapshotBodyView {
    schema: String,
    trust_domain: String,
}

#[derive(Deserialize)]
struct AuthoritySnapshotEnvelopeView {
    body: AuthoritySnapshotBodyView,
}

#[derive(Deserialize)]
struct AuthoritySnapshotBodyView {
    schema: String,
    applied_log_index: u64,
    leases: Vec<LeaseState>,
}

fn membership_trust_domain(membership: &MembershipState) -> PlacementResult<String> {
    let bytes = membership
        .snapshot()
        .map_err(|_| PlacementError::InconsistentEvidence)?;
    let snapshot: MembershipSnapshotEnvelopeView =
        serde_json::from_slice(&bytes).map_err(|_| PlacementError::InconsistentEvidence)?;
    if snapshot.body.schema != super::membership::MEMBERSHIP_SNAPSHOT_SCHEMA
        || !valid_text(&snapshot.body.trust_domain)
    {
        return Err(PlacementError::InconsistentEvidence);
    }
    Ok(snapshot.body.trust_domain)
}
