//! Deterministic, fail-closed placement over already-authorized distributed evidence.
//!
//! This module intentionally remains unregistered until integration issue #5878. It consumes
//! committed membership and the verified/admitted projections owned by the preceding distributed
//! modules; it does not verify wire messages, grant leases, activate owners, or mutate authority.

use std::{cmp::Ordering, collections::BTreeMap, fmt};

use super::{
    capability_advertisement::VerifiedCapabilityAdvertisement,
    fencing::FenceReceipt,
    lease::OperationClass,
    membership::{Member, MemberRole, MembershipState},
    resource_weather::{PlacementWeather, WeatherAvailability},
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
    pub now_unix_secs: u64,
    pub requirements: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug)]
pub struct PlacementInputs<'a> {
    pub membership: &'a MembershipState,
    pub capabilities: &'a [VerifiedCapabilityAdvertisement],
    pub weather: &'a [PlacementWeather],
    pub fencing: &'a [FenceReceipt],
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

pub fn decide(
    policy: &PlacementPolicy,
    request: &PlacementRequest,
    inputs: PlacementInputs<'_>,
) -> PlacementResult<PlacementDecision> {
    validate_request(policy, request)?;
    if inputs.membership.epoch() < request.minimum_membership_epoch
        || inputs.membership.committed_log_index() < request.minimum_committed_log_index
        || inputs.membership.epoch() == 0
        || inputs.membership.committed_log_index() == 0
    {
        return Err(PlacementError::StaleMembership);
    }
    if inputs.membership.members().count() > policy.max_inputs
        || inputs.capabilities.len() > policy.max_inputs
        || inputs.weather.len() > policy.max_inputs
        || inputs.fencing.len() > policy.max_inputs
    {
        return Err(PlacementError::ResourceExhausted);
    }

    let members = index_members(inputs.membership)?;
    let capabilities = index_capabilities(policy, inputs.capabilities, &members)?;
    let weather = index_weather(inputs.weather, &members)?;
    let fenced = index_fencing(policy, inputs.membership, inputs.fencing, &members)?;
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
        if !fresh_capability(policy, capability, request.now_unix_secs)
            || !fresh_weather(policy, weather, request.now_unix_secs)
            || capability.certificate_generation != weather.certificate_generation
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

fn index_fencing<'a>(
    policy: &PlacementPolicy,
    membership: &MembershipState,
    values: &'a [FenceReceipt],
    members: &BTreeMap<&str, &Member>,
) -> PlacementResult<BTreeMap<&'a str, &'a FenceReceipt>> {
    let mut result = BTreeMap::new();
    for value in values {
        let trust_domain = std::str::from_utf8(&value.trust_domain_id)
            .map_err(|_| PlacementError::WrongTrustDomain)?;
        if trust_domain != policy.trust_domain {
            return Err(PlacementError::WrongTrustDomain);
        }
        if value.committed_log_index > membership.committed_log_index() {
            return Err(PlacementError::FencingAheadOfMembership);
        }
        let identity = std::str::from_utf8(&value.lineage_id)
            .map_err(|_| PlacementError::InconsistentEvidence)?;
        if !members.contains_key(identity)
            || value.epoch == 0
            || value.committed_log_index == 0
            || value.voter_set_generation == 0
            || (value.operation_class != OperationClass::Fence as u32
                && value.operation_class != OperationClass::Revoke as u32)
            || result.insert(identity, value).is_some()
        {
            return Err(PlacementError::InconsistentEvidence);
        }
    }
    Ok(result)
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
