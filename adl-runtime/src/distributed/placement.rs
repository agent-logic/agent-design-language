//! Deterministic, fail-closed placement over already-authorized distributed evidence.
//!
//! This module intentionally remains unregistered until integration issue #5878. It consumes
//! committed membership and the verified/admitted projections owned by the preceding distributed
//! modules; it does not verify wire messages, grant leases, activate owners, or mutate authority.

use std::{cmp::Ordering, collections::BTreeMap, fmt, time::SystemTime};

use serde::Deserialize;

use super::{
    capability_advertisement::VerifiedCapabilityAdvertisement,
    certificates::{CertificatePurpose, DistributedCertificateStore},
    fencing::{FenceReceipt, FencingStore},
    lease::{AuthorityLedger, LeaseState, OperationClass, AUTHORITY_SNAPSHOT_SCHEMA},
    membership::{Member, MemberRole, MembershipState},
    resource_weather::{PlacementWeather, ResourceWeatherStore, WeatherAvailability},
};

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
    pub capabilities: &'a [VerifiedCapabilityAdvertisement],
    pub weather: &'a PlacementWeatherSnapshot,
    pub fencing: &'a PlacementFencingSnapshot,
}

/// Opaque weather projected from the durable verified-advertisement store at service time.
#[derive(Clone, Debug)]
pub struct PlacementWeatherSnapshot {
    captured_at_unix_secs: u64,
    rows: Vec<PlacementWeather>,
}

impl PlacementWeatherSnapshot {
    pub fn capture(
        store: &ResourceWeatherStore,
        certificates: &DistributedCertificateStore,
        now_unix_secs: u64,
    ) -> PlacementResult<Self> {
        let rows = store
            .snapshot(certificates, now_unix_secs)
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        Ok(Self {
            captured_at_unix_secs: now_unix_secs,
            rows,
        })
    }

    #[cfg(test)]
    pub fn from_rows_for_test(now_unix_secs: u64, rows: &[PlacementWeather]) -> Self {
        Self {
            captured_at_unix_secs: now_unix_secs,
            rows: rows.to_vec(),
        }
    }
}

/// A complete, opaque projection of fencing floors for one exact membership revision.
///
/// Production callers can construct this only by querying the authoritative `FencingStore` for
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
        ledger: &AuthorityLedger,
        store: &FencingStore,
    ) -> PlacementResult<Self> {
        let bytes = ledger
            .snapshot()
            .map_err(|_| PlacementError::InconsistentEvidence)?;
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
            .filter_map(|lineage| {
                store
                    .floor(lineage.lineage_id.as_bytes())
                    .cloned()
                    .map(|receipt| (lineage.lineage_id.clone(), receipt))
            })
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
                revoked: true,
            }],
            BTreeMap::new(),
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

pub struct PlacementService<'a, C> {
    policy: PlacementPolicy,
    certificates: &'a DistributedCertificateStore,
    clock: C,
}

impl<'a, C: PlacementClock> PlacementService<'a, C> {
    pub fn new(
        policy: PlacementPolicy,
        certificates: &'a DistributedCertificateStore,
        clock: C,
    ) -> Self {
        Self {
            policy,
            certificates,
            clock,
        }
    }

    pub fn decide(
        &self,
        request: &PlacementRequest,
        inputs: PlacementInputs<'_>,
    ) -> PlacementResult<PlacementDecision> {
        let now_unix_secs = self.clock.now_unix_secs()?;
        if inputs.weather.captured_at_unix_secs != now_unix_secs {
            return Err(PlacementError::InconsistentEvidence);
        }
        decide(
            &self.policy,
            self.certificates,
            now_unix_secs,
            request,
            inputs,
        )
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

impl Rank {
    fn compare(self, other: Self) -> Ordering {
        self.pressure_permille
            .cmp(&other.pressure_permille)
            .then_with(|| other.remaining_slots.cmp(&self.remaining_slots))
    }
}

fn decide(
    policy: &PlacementPolicy,
    certificates: &DistributedCertificateStore,
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
        || inputs.capabilities.len() > policy.max_inputs
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
    let capabilities = index_capabilities(policy, inputs.capabilities, &members)?;
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
            || certificates
                .authorize(
                    &member.guardian_id,
                    CertificatePurpose::AdvertisementSigning,
                    capability.certificate_generation,
                    now_unix_secs,
                )
                .ok()
                .is_none_or(|authorized| {
                    authorized.certificate_id != capability.certificate_id
                        || authorized.holder_id != capability.issuer_id
                        || authorized.generation != capability.certificate_generation
                        || authorized.purpose != CertificatePurpose::AdvertisementSigning
                })
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
    values: &'a [PlacementWeather],
    members: &BTreeMap<&str, &Member>,
) -> PlacementResult<BTreeMap<&'a str, &'a PlacementWeather>> {
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
    revoked: bool,
}

impl TryFrom<LeaseState> for PlacementLineageBinding {
    type Error = PlacementError;

    fn try_from(lease: LeaseState) -> PlacementResult<Self> {
        Ok(Self {
            lineage_id: String::from_utf8(lease.lineage_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
            node_id: String::from_utf8(lease.holder_node_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
            guardian_id: String::from_utf8(lease.holder_guardian_id)
                .map_err(|_| PlacementError::InconsistentEvidence)?,
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
