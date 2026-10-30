//! Authenticated, bounded and redacted distributed-runtime projection.
//!
//! WP-04.16 owns production registration. This module owns the v1 schema and
//! the coherent read across the authority-owned snapshots introduced by #133.

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

#[cfg(not(test))]
use super::authority_store_adapters::{
    AuthorityBoundCertificateStore, AuthorityBoundFencingStore, AuthorityBoundLeaseLedger,
};
#[cfg(test)]
use super::{
    certificates::DistributedCertificateStore, fencing::FencingStore, lease::AuthorityLedger,
};
use super::{
    failure_detection::{
        FailureClass, FailureDetector, FailureFreshness, FailureMembershipSnapshot,
    },
    lease::AuthorityMembership,
    membership::{MemberRole, MembershipState},
    migration::MigrationStore,
    placement::{PlacementClock, PlacementService},
    recovery::RecoveryStore,
};

#[cfg(not(test))]
type ProjectionCertificateSource = AuthorityBoundCertificateStore;
#[cfg(test)]
type ProjectionCertificateSource = DistributedCertificateStore;
#[cfg(not(test))]
type ProjectionLeaseSource = AuthorityBoundLeaseLedger;
#[cfg(test)]
type ProjectionLeaseSource = AuthorityLedger;
#[cfg(not(test))]
type ProjectionFencingSource = AuthorityBoundFencingStore;
#[cfg(test)]
type ProjectionFencingSource = FencingStore;

pub const PROJECTION_SCHEMA_V1: &str = "adl.distributed.projection.v1";
const MAX_CREDENTIAL_BYTES: usize = 4096;

#[derive(Clone, Eq, PartialEq)]
pub struct ProjectionReferenceKey([u8; 32]);

impl ProjectionReferenceKey {
    pub fn new(bytes: [u8; 32]) -> ProjectionResult<Self> {
        if bytes == [0; 32] {
            return Err(ProjectionError::InvalidPolicy);
        }
        Ok(Self(bytes))
    }
}

impl fmt::Debug for ProjectionReferenceKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ProjectionReferenceKey([REDACTED])")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionPolicy {
    pub trust_domain: String,
    pub reference_key: ProjectionReferenceKey,
    pub max_nodes: usize,
    pub max_certificates: usize,
    pub max_leases: usize,
    pub max_fences: usize,
    pub max_placements: usize,
    pub max_migrations: usize,
    pub max_recoveries: usize,
    pub max_response_bytes: usize,
}

impl ProjectionPolicy {
    fn validate(&self) -> ProjectionResult<()> {
        if !valid_text(&self.trust_domain, 128)
            || !(1..=4096).contains(&self.max_nodes)
            || !(1..=16_384).contains(&self.max_certificates)
            || !(1..=4096).contains(&self.max_leases)
            || !(1..=4096).contains(&self.max_fences)
            || !(1..=4096).contains(&self.max_placements)
            || !(1..=4096).contains(&self.max_migrations)
            || !(1..=4096).contains(&self.max_recoveries)
            || !(256..=65_536).contains(&self.max_response_bytes)
        {
            return Err(ProjectionError::InvalidPolicy);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionRequest {
    pub version: String,
    pub trust_domain: String,
    pub credential: Vec<u8>,
}

pub trait ProjectionAuthorizer: fmt::Debug + Send + Sync {
    fn authorize(&self, request: &ProjectionRequest) -> ProjectionResult<()>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionTime {
    pub unix_secs: u64,
    pub elapsed_millis: u64,
}

pub trait ProjectionClock: fmt::Debug + Send + Sync {
    fn now(&self) -> ProjectionResult<ProjectionTime>;
}

pub struct ProjectionSources<'a, C> {
    pub membership: &'a MembershipState,
    pub authority_membership: &'a AuthorityMembership,
    pub certificates: &'a ProjectionCertificateSource,
    pub failure_detector: &'a FailureDetector,
    pub lease_ledger: &'a ProjectionLeaseSource,
    pub fencing: &'a ProjectionFencingSource,
    pub placement: &'a PlacementService<C>,
    pub migrations: &'a MigrationStore,
    pub recoveries: &'a RecoveryStore,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DistributedProjectionV1 {
    pub schema: String,
    pub projection_id: String,
    pub captured_at_unix_secs: u64,
    pub membership_epoch: u64,
    pub committed_log_index: u64,
    pub ready: bool,
    pub nodes: Vec<ProjectedNode>,
    pub certificates: Vec<ProjectedCertificate>,
    pub leases: Vec<ProjectedLease>,
    pub fences: Vec<ProjectedFence>,
    pub placements: Vec<ProjectedPlacement>,
    pub migrations: Vec<ProjectedMigration>,
    pub recoveries: Vec<ProjectedRecovery>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedNode {
    pub node_id: String,
    pub guardian_id: String,
    pub identity_generation: u64,
    pub role: String,
    pub peer_state: String,
    pub failure_class: Option<String>,
    pub failure_reason: String,
    pub failure_freshness: String,
    pub advertisement_freshness: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedCertificate {
    pub certificate_id: String,
    pub holder_id: String,
    pub holder_kind: String,
    pub purpose: String,
    pub generation: u64,
    pub health: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedLease {
    pub lineage_id: String,
    pub holder_node_id: Option<String>,
    pub holder_guardian_id: Option<String>,
    pub epoch: u64,
    pub committed_log_index: u64,
    pub certificate_generation: u64,
    pub health: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedFence {
    pub lineage_id: String,
    pub epoch: u64,
    pub committed_log_index: u64,
    pub voter_set_generation: u64,
    pub operation: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedPlacement {
    pub lineage_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub freshness: String,
    pub capacity: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedMigration {
    pub migration_id: String,
    pub lineage_id: String,
    pub source_node_id: String,
    pub source_guardian_id: String,
    pub target_node_id: String,
    pub target_guardian_id: String,
    pub phase: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectedRecovery {
    pub recovery_id: String,
    pub migration_id: String,
    pub lineage_id: String,
    pub owner_node_id: Option<String>,
    pub owner_guardian_id: Option<String>,
    pub phase: String,
    pub reason: String,
    pub operator_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionError {
    InvalidPolicy,
    InvalidRequest,
    Unauthorized,
    UnsupportedVersion,
    WrongTrustDomain,
    StaleCut,
    IncoherentCut,
    MalformedAuthority,
    ResourceExhausted,
    Serialization,
}

impl ProjectionError {
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidRequest => "invalid_request",
            Self::Unauthorized => "unauthorized",
            Self::UnsupportedVersion => "unsupported_version",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::StaleCut => "stale_cut",
            Self::IncoherentCut => "incoherent_cut",
            Self::MalformedAuthority => "malformed_authority",
            Self::ResourceExhausted => "resource_exhausted",
            Self::Serialization => "serialization_failure",
        }
    }
}

pub type ProjectionResult<T> = Result<T, ProjectionError>;

pub fn project_v1<C: PlacementClock>(
    policy: &ProjectionPolicy,
    request: &ProjectionRequest,
    authorizer: &dyn ProjectionAuthorizer,
    clock: &dyn ProjectionClock,
    sources: ProjectionSources<'_, C>,
) -> ProjectionResult<Vec<u8>> {
    policy.validate()?;
    if request.version != "v1" {
        return Err(ProjectionError::UnsupportedVersion);
    }
    if request.trust_domain != policy.trust_domain {
        return Err(ProjectionError::WrongTrustDomain);
    }
    if request.credential.is_empty() || request.credential.len() > MAX_CREDENTIAL_BYTES {
        return Err(ProjectionError::InvalidRequest);
    }
    authorizer.authorize(request)?;
    let time = clock.now()?;
    let mut projection = collect(policy, time, sources)?;
    projection.projection_id.clear();
    let canonical = serde_jcs::to_vec(&projection).map_err(|_| ProjectionError::Serialization)?;
    projection.projection_id = format!("prj_{}", hex::encode(Sha256::digest(canonical)));
    let bytes = serde_jcs::to_vec(&projection).map_err(|_| ProjectionError::Serialization)?;
    if bytes.len() > policy.max_response_bytes {
        return Err(ProjectionError::ResourceExhausted);
    }
    Ok(bytes)
}

fn collect<C: PlacementClock>(
    policy: &ProjectionPolicy,
    time: ProjectionTime,
    sources: ProjectionSources<'_, C>,
) -> ProjectionResult<DistributedProjectionV1> {
    if sources.membership.trust_domain() != policy.trust_domain
        || sources.authority_membership.trust_domain_id != policy.trust_domain.as_bytes()
    {
        return Err(ProjectionError::WrongTrustDomain);
    }
    let epoch = sources.membership.epoch();
    let index = sources.membership.committed_log_index();
    if epoch == 0 || index == 0 || sources.authority_membership.committed_log_index != index {
        return Err(ProjectionError::IncoherentCut);
    }
    let membership_voters = sources
        .membership
        .members()
        .filter(|member| member.role == MemberRole::Voter)
        .map(|member| {
            (
                member.guardian_id.as_bytes().to_vec(),
                member.guardian_control_public_key,
            )
        })
        .collect::<BTreeMap<_, _>>();
    if membership_voters.len() != sources.authority_membership.voters.len()
        || membership_voters.iter().any(|(guardian_id, control_key)| {
            sources
                .authority_membership
                .voters
                .get(guardian_id)
                .is_none_or(|voter| voter.control_public_key != *control_key)
        })
        || sources
            .authority_membership
            .voters
            .keys()
            .any(|guardian_id| !membership_voters.contains_key(guardian_id))
    {
        return Err(ProjectionError::IncoherentCut);
    }
    let failure_membership = failure_membership(sources.membership);

    let cert_r = sources
        .certificates
        .authority_revision()
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let fail_r = sources
        .failure_detector
        .authority_revision(&failure_membership, time.unix_secs)
        .map_err(failure_error)?;
    let lease_r = sources
        .lease_ledger
        .authority_revision()
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let fence_r = sources
        .fencing
        .authority_revision()
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let place_r = sources
        .placement
        .authority_revision()
        .map_err(placement_error)?;
    let migration_r = sources
        .migrations
        .authority_revision()
        .map_err(migration_error)?;
    let recovery_r = sources
        .recoveries
        .authority_revision()
        .map_err(recovery_error)?;

    let certificates = sources
        .certificates
        .redacted_snapshot_at(cert_r, time.unix_secs)
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let failures = sources
        .failure_detector
        .redacted_snapshot_at(fail_r, &failure_membership, time.unix_secs)
        .map_err(failure_error)?;
    let leases = sources
        .lease_ledger
        .redacted_snapshot_at(lease_r, sources.authority_membership, time.elapsed_millis)
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let fences = sources
        .fencing
        .redacted_snapshot_at(fence_r, sources.authority_membership)
        .map_err(|_| ProjectionError::MalformedAuthority)?;
    let placements = sources
        .placement
        .redacted_snapshot_at(place_r)
        .map_err(placement_error)?;
    let migrations = sources
        .migrations
        .redacted_snapshot_at(migration_r)
        .map_err(migration_error)?;
    let recoveries = sources
        .recoveries
        .redacted_snapshot_at(recovery_r)
        .map_err(recovery_error)?;

    if sources
        .certificates
        .authority_revision()
        .map_err(|_| ProjectionError::MalformedAuthority)?
        != cert_r
        || sources
            .failure_detector
            .authority_revision(&failure_membership, time.unix_secs)
            .map_err(failure_error)?
            != fail_r
        || sources
            .lease_ledger
            .authority_revision()
            .map_err(|_| ProjectionError::MalformedAuthority)?
            != lease_r
        || sources
            .fencing
            .authority_revision()
            .map_err(|_| ProjectionError::MalformedAuthority)?
            != fence_r
        || sources
            .placement
            .authority_revision()
            .map_err(placement_error)?
            != place_r
        || sources
            .migrations
            .authority_revision()
            .map_err(migration_error)?
            != migration_r
        || sources
            .recoveries
            .authority_revision()
            .map_err(recovery_error)?
            != recovery_r
        || sources.membership.epoch() != epoch
        || sources.membership.committed_log_index() != index
    {
        return Err(ProjectionError::IncoherentCut);
    }

    for domain in [
        certificates.trust_domain(),
        failures.trust_domain(),
        leases.trust_domain(),
        fences.trust_domain(),
        placements.trust_domain(),
        migrations.trust_domain(),
        recoveries.trust_domain(),
    ] {
        if domain != policy.trust_domain {
            return Err(ProjectionError::WrongTrustDomain);
        }
    }
    if failures.membership_epoch() != epoch
        || failures.committed_log_index() != index
        || placements.membership_epoch() != epoch
        || placements.committed_log_index() != index
        || certificates.captured_at_unix_secs() != time.unix_secs
        || failures.captured_at_unix_secs() != time.unix_secs
    {
        return Err(ProjectionError::IncoherentCut);
    }
    if failures
        .rows()
        .any(|row| row.freshness() == FailureFreshness::Stale)
    {
        return Err(ProjectionError::StaleCut);
    }
    let counts = [
        sources.membership.members().count(),
        certificates.rows().len(),
        leases.rows().len(),
        fences.rows().len(),
        placements.rows().len(),
        migrations.rows().len(),
        recoveries.rows().len(),
    ];
    let limits = [
        policy.max_nodes,
        policy.max_certificates,
        policy.max_leases,
        policy.max_fences,
        policy.max_placements,
        policy.max_migrations,
        policy.max_recoveries,
    ];
    if counts
        .into_iter()
        .zip(limits)
        .any(|(count, limit)| count > limit)
    {
        return Err(ProjectionError::ResourceExhausted);
    }

    let member_count = sources.membership.members().count();
    let authority_node_refs = sources
        .membership
        .members()
        .map(|member| projection_ref(b"node", member.node_id.as_bytes()))
        .collect::<BTreeSet<_>>();
    let authority_guardian_refs = sources
        .membership
        .members()
        .map(|member| projection_ref(b"guardian", member.guardian_id.as_bytes()))
        .collect::<BTreeSet<_>>();
    let member_pairs = sources
        .membership
        .members()
        .map(|member| {
            (
                projection_ref(b"node", member.node_id.as_bytes()),
                projection_ref(b"guardian", member.guardian_id.as_bytes()),
            )
        })
        .collect::<BTreeSet<_>>();
    if authority_node_refs.len() != member_count
        || authority_guardian_refs.len() != member_count
        || member_pairs.len() != member_count
    {
        return Err(ProjectionError::MalformedAuthority);
    }
    let failure_map = failures
        .rows()
        .map(|row| (row.node_ref().to_owned(), row))
        .collect::<BTreeMap<_, _>>();
    if failure_map.len() != authority_node_refs.len()
        || failure_map
            .keys()
            .any(|id| !authority_node_refs.contains(id))
    {
        return Err(ProjectionError::IncoherentCut);
    }
    let placement_nodes = placements
        .rows()
        .map(|row| row.node_ref().to_owned())
        .collect::<BTreeSet<_>>();
    let migration_authority_unambiguous = migrations
        .rows()
        .all(|row| !(row.source_authoritative() && row.target_authoritative()));

    let mut nodes = Vec::with_capacity(authority_node_refs.len());
    for member in sources.membership.members() {
        let authority_node_id = projection_ref(b"node", member.node_id.as_bytes());
        let authority_guardian_id = projection_ref(b"guardian", member.guardian_id.as_bytes());
        let failure = failure_map
            .get(&authority_node_id)
            .ok_or(ProjectionError::IncoherentCut)?;
        if failure.guardian_ref() != authority_guardian_id {
            return Err(ProjectionError::IncoherentCut);
        }
        nodes.push(ProjectedNode {
            node_id: keyed_ref(&policy.reference_key, b"node", &authority_node_id),
            guardian_id: keyed_ref(&policy.reference_key, b"guardian", &authority_guardian_id),
            identity_generation: member.identity_generation,
            role: match member.role {
                MemberRole::NonVoting => "non_voting",
                MemberRole::Voter => "voter",
            }
            .to_owned(),
            peer_state: peer_state(failure.class()).to_owned(),
            failure_class: failure.class().map(snake_debug),
            failure_reason: snake_debug(failure.reason()),
            failure_freshness: snake_debug(failure.freshness()),
            advertisement_freshness: if placement_nodes.contains(failure.node_ref()) {
                "verified_at_decision"
            } else {
                "unavailable"
            }
            .to_owned(),
        });
    }
    nodes.sort_by(|a, b| a.node_id.cmp(&b.node_id));

    let mut projected_certificates = Vec::with_capacity(certificates.rows().len());
    let mut certificate_authority = Vec::with_capacity(certificates.rows().len());
    for row in certificates.rows() {
        let (holder_id, holder_kind) = if let Some(id) = row.node_ref() {
            if row.guardian_ref().is_some() || !authority_node_refs.contains(id) {
                return Err(ProjectionError::IncoherentCut);
            }
            (keyed_ref(&policy.reference_key, b"node", id), "node")
        } else if let Some(id) = row.guardian_ref() {
            if !authority_guardian_refs.contains(id) {
                return Err(ProjectionError::IncoherentCut);
            }
            (
                keyed_ref(&policy.reference_key, b"guardian", id),
                "guardian",
            )
        } else {
            return Err(ProjectionError::MalformedAuthority);
        };
        certificate_authority.push((
            row.node_ref().map(str::to_owned),
            snake_debug(row.purpose()),
            snake_debug(row.health()),
            row.generation(),
        ));
        projected_certificates.push(ProjectedCertificate {
            certificate_id: keyed_ref(&policy.reference_key, b"certificate", row.certificate_ref()),
            holder_id,
            holder_kind: holder_kind.to_owned(),
            purpose: snake_debug(row.purpose()),
            generation: row.generation(),
            health: snake_debug(row.health()),
        });
    }
    projected_certificates.sort_by(|a, b| a.certificate_id.cmp(&b.certificate_id));
    if !all_unique(
        projected_certificates
            .iter()
            .map(|row| row.certificate_id.as_str()),
    ) {
        return Err(ProjectionError::MalformedAuthority);
    }

    let mut projected_leases = leases
        .rows()
        .map(|row| {
            validate_optional_member_pair(
                row.holder_node_ref(),
                row.holder_guardian_ref(),
                &member_pairs,
            )?;
            Ok(ProjectedLease {
                lineage_id: keyed_ref(&policy.reference_key, b"lineage", row.lineage_ref()),
                holder_node_id: row
                    .holder_node_ref()
                    .map(|id| keyed_ref(&policy.reference_key, b"node", id)),
                holder_guardian_id: row
                    .holder_guardian_ref()
                    .map(|id| keyed_ref(&policy.reference_key, b"guardian", id)),
                epoch: row.epoch(),
                committed_log_index: row.committed_log_index(),
                certificate_generation: row.certificate_generation(),
                health: snake_debug(row.health()),
            })
        })
        .collect::<ProjectionResult<Vec<_>>>()?;
    projected_leases.sort_by(|a, b| a.lineage_id.cmp(&b.lineage_id));
    if !all_unique(projected_leases.iter().map(|row| row.lineage_id.as_str())) {
        return Err(ProjectionError::MalformedAuthority);
    }
    let mut projected_fences = fences
        .rows()
        .map(|row| ProjectedFence {
            lineage_id: keyed_ref(&policy.reference_key, b"lineage", row.lineage_ref()),
            epoch: row.epoch(),
            committed_log_index: row.committed_log_index(),
            voter_set_generation: row.voter_set_generation(),
            operation: match row.operation_class() {
                3 => "fence",
                6 => "revoke",
                _ => "unknown",
            }
            .to_owned(),
        })
        .collect::<Vec<_>>();
    if projected_fences
        .iter()
        .any(|row| row.operation == "unknown")
    {
        return Err(ProjectionError::MalformedAuthority);
    }
    projected_fences.sort_by(|a, b| a.lineage_id.cmp(&b.lineage_id));
    if !all_unique(projected_fences.iter().map(|row| row.lineage_id.as_str())) {
        return Err(ProjectionError::MalformedAuthority);
    }
    let mut projected_placements = placements
        .rows()
        .map(|row| {
            validate_member_pair(row.node_ref(), row.guardian_ref(), &member_pairs)?;
            Ok(ProjectedPlacement {
                lineage_id: keyed_ref(&policy.reference_key, b"lineage", row.lineage_ref()),
                node_id: keyed_ref(&policy.reference_key, b"node", row.node_ref()),
                guardian_id: keyed_ref(&policy.reference_key, b"guardian", row.guardian_ref()),
                freshness: snake_debug(row.freshness()),
                capacity: snake_debug(row.capacity()),
            })
        })
        .collect::<ProjectionResult<Vec<_>>>()?;
    projected_placements.sort_by(|a, b| a.lineage_id.cmp(&b.lineage_id));
    if !all_unique(
        projected_placements
            .iter()
            .map(|row| row.lineage_id.as_str()),
    ) {
        return Err(ProjectionError::MalformedAuthority);
    }
    let mut projected_migrations = migrations
        .rows()
        .map(|row| {
            validate_member_pair(
                row.source_node_ref(),
                row.source_guardian_ref(),
                &member_pairs,
            )?;
            validate_member_pair(
                row.target_node_ref(),
                row.target_guardian_ref(),
                &member_pairs,
            )?;
            Ok(ProjectedMigration {
                migration_id: keyed_ref(&policy.reference_key, b"migration", row.migration_ref()),
                lineage_id: keyed_ref(&policy.reference_key, b"lineage", row.lineage_ref()),
                source_node_id: keyed_ref(&policy.reference_key, b"node", row.source_node_ref()),
                source_guardian_id: keyed_ref(
                    &policy.reference_key,
                    b"guardian",
                    row.source_guardian_ref(),
                ),
                target_node_id: keyed_ref(&policy.reference_key, b"node", row.target_node_ref()),
                target_guardian_id: keyed_ref(
                    &policy.reference_key,
                    b"guardian",
                    row.target_guardian_ref(),
                ),
                phase: snake_debug(row.phase()),
            })
        })
        .collect::<ProjectionResult<Vec<_>>>()?;
    projected_migrations.sort_by(|a, b| a.migration_id.cmp(&b.migration_id));
    if !all_unique(
        projected_migrations
            .iter()
            .map(|row| row.migration_id.as_str()),
    ) {
        return Err(ProjectionError::MalformedAuthority);
    }
    let mut projected_recoveries = recoveries
        .rows()
        .map(|row| {
            validate_member_pair(
                row.source_node_ref(),
                row.source_guardian_ref(),
                &member_pairs,
            )?;
            validate_member_pair(
                row.target_node_ref(),
                row.target_guardian_ref(),
                &member_pairs,
            )?;
            validate_optional_member_pair(
                row.owner_node_ref(),
                row.owner_guardian_ref(),
                &member_pairs,
            )?;
            Ok(ProjectedRecovery {
                recovery_id: keyed_ref(&policy.reference_key, b"recovery", row.recovery_ref()),
                migration_id: keyed_ref(&policy.reference_key, b"migration", row.migration_ref()),
                lineage_id: keyed_ref(&policy.reference_key, b"lineage", row.lineage_ref()),
                owner_node_id: row
                    .owner_node_ref()
                    .map(|id| keyed_ref(&policy.reference_key, b"node", id)),
                owner_guardian_id: row
                    .owner_guardian_ref()
                    .map(|id| keyed_ref(&policy.reference_key, b"guardian", id)),
                phase: snake_debug(row.phase()),
                reason: snake_debug(row.reason()),
                operator_required: row.operator_required(),
            })
        })
        .collect::<ProjectionResult<Vec<_>>>()?;
    projected_recoveries.sort_by(|a, b| a.recovery_id.cmp(&b.recovery_id));
    if !all_unique(
        projected_recoveries
            .iter()
            .map(|row| row.recovery_id.as_str()),
    ) {
        return Err(ProjectionError::MalformedAuthority);
    }

    let ready = nodes.iter().all(|node| {
        let authority_node_ref = sources
            .membership
            .members()
            .find(|member| {
                keyed_ref(
                    &policy.reference_key,
                    b"node",
                    &projection_ref(b"node", member.node_id.as_bytes()),
                ) == node.node_id
            })
            .map(|member| {
                (
                    projection_ref(b"node", member.node_id.as_bytes()),
                    member.identity_generation,
                )
            });
        let Some((authority_node_ref, identity_generation)) = authority_node_ref else {
            return false;
        };
        let exact = |purpose: &str, expected_generation: Option<u64>| {
            let rows = certificate_authority
                .iter()
                .filter(|(holder, row_purpose, health, _generation)| {
                    holder.as_deref() == Some(authority_node_ref.as_str())
                        && row_purpose == purpose
                        && matches!(health.as_str(), "active" | "rotation_overlap")
                })
                .collect::<Vec<_>>();
            rows.len() == 1
                && rows[0].2 == "active"
                && expected_generation.is_none_or(|expected| rows[0].3 == expected)
        };
        node.failure_freshness == "fresh"
            && exact("node_identity", Some(identity_generation))
            && exact("transport", None)
    }) && projected_leases
        .iter()
        .all(|lease| lease.health == "active")
        && migration_authority_unambiguous
        && projected_recoveries
            .iter()
            .all(|recovery| !recovery.operator_required);
    Ok(DistributedProjectionV1 {
        schema: PROJECTION_SCHEMA_V1.to_owned(),
        projection_id: String::new(),
        captured_at_unix_secs: time.unix_secs,
        membership_epoch: epoch,
        committed_log_index: index,
        ready,
        nodes,
        certificates: projected_certificates,
        leases: projected_leases,
        fences: projected_fences,
        placements: projected_placements,
        migrations: projected_migrations,
        recoveries: projected_recoveries,
    })
}

fn failure_membership(authority: &MembershipState) -> FailureMembershipSnapshot {
    #[cfg(not(test))]
    {
        FailureMembershipSnapshot::from_membership(authority)
    }
    #[cfg(test)]
    {
        FailureMembershipSnapshot::from_test_rows(
            authority.trust_domain(),
            authority.epoch(),
            authority.committed_log_index(),
            authority
                .members()
                .map(|member| (member.node_id.clone(), member.guardian_id.clone()))
                .collect(),
        )
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

fn keyed_ref(key: &ProjectionReferenceKey, kind: &[u8], authority_ref: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(&key.0).expect("fixed HMAC key length");
    mac.update(b"adl-reviewer-projection-ref-v1");
    mac.update(&(kind.len() as u64).to_be_bytes());
    mac.update(kind);
    mac.update(&(authority_ref.len() as u64).to_be_bytes());
    mac.update(authority_ref.as_bytes());
    format!("id_{}", hex::encode(mac.finalize().into_bytes()))
}

fn validate_member_pair(
    node: &str,
    guardian: &str,
    members: &BTreeSet<(String, String)>,
) -> ProjectionResult<()> {
    if !members.contains(&(node.to_owned(), guardian.to_owned())) {
        return Err(ProjectionError::IncoherentCut);
    }
    Ok(())
}

fn validate_optional_member_pair(
    node: Option<&str>,
    guardian: Option<&str>,
    members: &BTreeSet<(String, String)>,
) -> ProjectionResult<()> {
    match (node, guardian) {
        (Some(node), Some(guardian)) => validate_member_pair(node, guardian, members),
        (None, None) => Ok(()),
        _ => Err(ProjectionError::MalformedAuthority),
    }
}

fn all_unique<'a>(mut values: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = BTreeSet::new();
    values.all(|value| seen.insert(value))
}

fn snake_debug(value: impl fmt::Debug) -> String {
    let source = format!("{value:?}");
    let mut result = String::with_capacity(source.len());
    for (index, ch) in source.chars().enumerate() {
        if ch.is_ascii_uppercase() && index != 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

fn peer_state(class: Option<FailureClass>) -> &'static str {
    match class {
        Some(FailureClass::Healthy | FailureClass::Recovered) => "connected",
        Some(_) => "degraded",
        None => "unavailable",
    }
}
fn valid_text(value: &str, max: usize) -> bool {
    !value.is_empty() && value.len() <= max && value.chars().all(|ch| !ch.is_control())
}
fn map_error(code: &str) -> ProjectionError {
    match code {
        "wrong_trust_domain" | "invalid_membership" | "stale_membership" => {
            ProjectionError::WrongTrustDomain
        }
        "resource_exhausted" | "snapshot_too_large" => ProjectionError::ResourceExhausted,
        "revision_drift" | "authority_unavailable" | "rollback" => ProjectionError::IncoherentCut,
        _ => ProjectionError::MalformedAuthority,
    }
}
fn failure_error(error: super::failure_detection::FailureError) -> ProjectionError {
    map_error(error.code())
}
fn placement_error(error: super::placement::PlacementError) -> ProjectionError {
    map_error(error.code())
}
fn migration_error(error: super::migration::MigrationError) -> ProjectionError {
    map_error(error.code())
}
fn recovery_error(error: super::recovery::RecoveryError) -> ProjectionError {
    map_error(error.code())
}
