// PVF: lane=exact-child-tests; proof=authenticated coherent bounded redacted projection;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/capability_advertisement.rs"]
mod capability_advertisement;
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
#[allow(dead_code)]
#[path = "../src/distributed/failure_detection.rs"]
mod failure_detection;
#[allow(dead_code)]
#[path = "../src/distributed/fencing.rs"]
mod fencing;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;
#[allow(dead_code)]
#[path = "../src/distributed/membership.rs"]
mod membership;
#[allow(dead_code)]
#[path = "../src/distributed/migration.rs"]
mod migration;
mod integrated_serving_authority_snapshot {
    pub use adl_runtime::distributed::integrated_serving_authority_snapshot::*;
}
mod shepherd_serving_eligibility {
    pub use adl_runtime::distributed::shepherd_serving_eligibility::*;
}
#[cfg(feature = "internal-test-fixtures")]
mod distributed {
    pub use adl_runtime::distributed::{
        authority_protocol, observatory_serving_eligibility, polis_runtime, serving_authority,
        shepherd_serving_eligibility,
    };
}
#[allow(dead_code)]
#[path = "../src/distributed/placement.rs"]
mod placement;
#[allow(dead_code)]
#[path = "../src/distributed/projection.rs"]
mod projection;
#[allow(dead_code)]
#[path = "../src/distributed/recovery.rs"]
mod recovery;
#[allow(dead_code)]
#[path = "../src/distributed/resource_weather.rs"]
mod resource_weather;
#[allow(dead_code)]
#[path = "../src/distributed/snapshot_catalog.rs"]
mod snapshot_catalog;

use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::SigningKey;
use failure_detection::{
    FailureDetector, FailurePolicy, FailureProbeClaims, FailureThresholds, ProbeAuthority,
    ProbeResult, SignedFailureProbe, FAILURE_PROBE_SCHEMA,
};
use fencing::{
    FencingCheckpoint, FencingCheckpointAuthority, FencingError, FencingPolicy, FencingStore,
    TEST_FENCING_STORE_ACCESS,
};
use lease::{
    AuthorityLedger, AuthorityMembership, ControlCertificatePurpose, LeasePolicy, VoterAuthority,
    TEST_LEASE_STORE_ACCESS,
};
use membership::{
    CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
    MembershipState,
};
use migration::{
    MigrationCheckpoint, MigrationCheckpointAuthority, MigrationClock, MigrationPolicy,
    MigrationStore,
};
use placement::{PlacementClock, PlacementDecision, PlacementPolicy, PlacementService};
use projection::*;
use recovery::{
    RecoveryCheckpoint, RecoveryCheckpointAuthority, RecoveryClock, RecoveryPolicy, RecoveryStore,
    RecoveryTime,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

const DOMAIN: &str = "polis.test";

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

#[derive(Debug)]
struct Auth(bool);
impl ProjectionAuthorizer for Auth {
    fn authorize(&self, _: &ProjectionRequest) -> ProjectionResult<()> {
        self.0.then_some(()).ok_or(ProjectionError::Unauthorized)
    }
}
#[derive(Debug)]
struct Clock;
impl ProjectionClock for Clock {
    fn now(&self) -> ProjectionResult<ProjectionTime> {
        Ok(ProjectionTime {
            unix_secs: 100,
            elapsed_millis: 100,
        })
    }
}

#[derive(Debug)]
struct ClockAt(u64);
impl ProjectionClock for ClockAt {
    fn now(&self) -> ProjectionResult<ProjectionTime> {
        Ok(ProjectionTime {
            unix_secs: self.0,
            elapsed_millis: self.0,
        })
    }
}

#[derive(Debug)]
struct RecordingClock(Arc<AtomicBool>);
impl ProjectionClock for RecordingClock {
    fn now(&self) -> ProjectionResult<ProjectionTime> {
        self.0.store(true, Ordering::SeqCst);
        Ok(ProjectionTime {
            unix_secs: 100,
            elapsed_millis: 100,
        })
    }
}

#[derive(Default)]
struct ProbeKeys {
    identities: BTreeMap<String, (u64, ed25519_dalek::VerifyingKey)>,
    members: BTreeSet<(String, u64)>,
}
impl ProbeAuthority for ProbeKeys {
    fn current_observer_identity(
        &self,
        observer_node_id: &str,
    ) -> Option<(u64, ed25519_dalek::VerifyingKey)> {
        self.identities.get(observer_node_id).copied()
    }

    fn is_member(&self, node_id: &str, membership_epoch: u64) -> bool {
        self.members
            .contains(&(node_id.to_owned(), membership_epoch))
    }
}
#[derive(Clone, Copy, Debug)]
struct PlacementNow;
impl PlacementClock for PlacementNow {
    fn now_unix_secs(&self) -> placement::PlacementResult<u64> {
        Ok(100)
    }
}

#[derive(Clone, Debug, Default)]
struct MigrationAuthority(Arc<Mutex<Option<MigrationCheckpoint>>>);
impl MigrationCheckpointAuthority for MigrationAuthority {
    fn current(&self) -> migration::MigrationResult<Option<MigrationCheckpoint>> {
        Ok(*self.0.lock().unwrap())
    }
    fn compare_and_swap(
        &self,
        expected: Option<MigrationCheckpoint>,
        next: MigrationCheckpoint,
    ) -> migration::MigrationResult<()> {
        let mut value = self.0.lock().unwrap();
        if *value != expected {
            return Err(migration::MigrationError::Rollback);
        }
        *value = Some(next);
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
struct MigrationNow;
impl MigrationClock for MigrationNow {
    fn now_millis(&self) -> migration::MigrationResult<u64> {
        Ok(100)
    }
}

#[derive(Clone, Debug, Default)]
struct RecoveryAuthority(Arc<Mutex<Option<RecoveryCheckpoint>>>);
impl RecoveryCheckpointAuthority for RecoveryAuthority {
    fn current(&self) -> recovery::RecoveryResult<Option<RecoveryCheckpoint>> {
        Ok(*self.0.lock().unwrap())
    }
    fn compare_and_swap(
        &self,
        expected: Option<RecoveryCheckpoint>,
        next: RecoveryCheckpoint,
    ) -> recovery::RecoveryResult<()> {
        let mut value = self.0.lock().unwrap();
        if *value != expected {
            return Err(recovery::RecoveryError::Rollback);
        }
        *value = Some(next);
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
struct RecoveryNow;
impl RecoveryClock for RecoveryNow {
    fn now(&self) -> recovery::RecoveryResult<RecoveryTime> {
        Ok(RecoveryTime {
            unix_seconds: 1,
            unix_nanos: 0,
            elapsed_millis: 100,
            clock_uncertainty_millis: 0,
        })
    }
}

#[derive(Clone, Debug, Default)]
struct FenceAuthority(Arc<Mutex<Option<FencingCheckpoint>>>);
impl FencingCheckpointAuthority for FenceAuthority {
    fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
        Ok(*self.0.lock().unwrap())
    }
    fn compare_and_swap(
        &self,
        expected: Option<FencingCheckpoint>,
        next: FencingCheckpoint,
    ) -> Result<(), FencingError> {
        let mut value = self.0.lock().unwrap();
        if *value != expected {
            return Err(FencingError::Rollback);
        }
        *value = Some(next);
        Ok(())
    }
}

fn base_membership() -> MembershipState {
    let mut state = MembershipState::new(MembershipPolicy::new(DOMAIN, 8, 16).unwrap());
    for (offset, suffix) in ["a", "b", "c"].into_iter().enumerate() {
        let join_sequence = offset as u64 * 2 + 1;
        let promote_sequence = join_sequence + 1;
        state
            .apply(&CommittedMembershipEvent {
                schema: membership::MEMBERSHIP_EVENT_SCHEMA.into(),
                trust_domain: DOMAIN.into(),
                event_id: [join_sequence as u8; 32],
                epoch: join_sequence,
                committed_log_index: join_sequence,
                operation: MembershipOperation::Join {
                    member: Member {
                        node_id: format!("node-{suffix}"),
                        guardian_id: format!("guardian-{suffix}"),
                        identity_generation: 1,
                        guardian_control_public_key: key(offset as u8 + 1)
                            .verifying_key()
                            .to_bytes(),
                        role: MemberRole::NonVoting,
                    },
                },
            })
            .unwrap();
        state
            .apply(&CommittedMembershipEvent {
                schema: membership::MEMBERSHIP_EVENT_SCHEMA.into(),
                trust_domain: DOMAIN.into(),
                event_id: [promote_sequence as u8; 32],
                epoch: promote_sequence,
                committed_log_index: promote_sequence,
                operation: MembershipOperation::Promote {
                    node_id: format!("node-{suffix}"),
                },
            })
            .unwrap();
    }
    state
}

fn authority_membership() -> AuthorityMembership {
    authority_membership_at(DOMAIN, 6)
}

fn authority_membership_at(domain: &str, committed_log_index: u64) -> AuthorityMembership {
    let ids = [
        b"guardian-a".to_vec(),
        b"guardian-b".to_vec(),
        b"guardian-c".to_vec(),
    ];
    let voters = ids
        .iter()
        .enumerate()
        .map(|(index, id)| VoterAuthority {
            guardian_id: id.clone(),
            trust_domain_id: domain.as_bytes().to_vec(),
            certificate_generation: 1,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: 1,
            not_after_unix_seconds: 10_000,
            revoked: false,
            control_public_key: key(index as u8 + 1).verifying_key().to_bytes(),
        })
        .collect();
    AuthorityMembership::new(
        domain.as_bytes().to_vec(),
        1,
        committed_log_index,
        vec![ids.into_iter().collect::<BTreeSet<_>>()],
        voters,
    )
    .unwrap()
}

fn unrelated_authority_membership() -> AuthorityMembership {
    let ids = [
        b"guardian-x".to_vec(),
        b"guardian-y".to_vec(),
        b"guardian-z".to_vec(),
    ];
    let voters = ids
        .iter()
        .enumerate()
        .map(|(index, id)| VoterAuthority {
            guardian_id: id.clone(),
            trust_domain_id: DOMAIN.as_bytes().to_vec(),
            certificate_generation: 1,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: 1,
            not_after_unix_seconds: 10_000,
            revoked: false,
            control_public_key: key(index as u8 + 10).verifying_key().to_bytes(),
        })
        .collect();
    AuthorityMembership::new(
        DOMAIN.as_bytes().to_vec(),
        1,
        6,
        vec![ids.into_iter().collect::<BTreeSet<_>>()],
        voters,
    )
    .unwrap()
}

fn certificate(
    root: &SigningKey,
    holder: &str,
    purpose: CertificatePurpose,
    generation: u64,
    subject_seed: u8,
) -> AuthorityCertificate {
    AuthorityCertificate::issue(
        CertificateBody::new(
            DOMAIN,
            holder,
            purpose,
            generation,
            CertificateValidity {
                issued_at_unix_secs: 50,
                expires_at_unix_secs: 500,
            },
            key(subject_seed).verifying_key(),
            &root.verifying_key(),
        ),
        root,
    )
    .unwrap()
}

fn healthy_failure_detector() -> FailureDetector {
    let observer = key(70);
    let mut authority = ProbeKeys::default();
    authority
        .identities
        .insert("node-local".into(), (1, observer.verifying_key()));
    authority.members.insert(("node-local".into(), 6));
    for suffix in ["a", "b", "c"] {
        authority.members.insert((format!("node-{suffix}"), 6));
    }
    let mut detector = FailureDetector::new(
        FailurePolicy::new(
            DOMAIN,
            "node-local",
            6,
            FailureThresholds {
                suspect_after_secs: 5,
                unavailable_after_secs: 10,
                evidence_window_secs: 5,
                flap_window_secs: 30,
            },
            1,
        )
        .unwrap(),
    );
    for (offset, suffix) in ["a", "b", "c"].into_iter().enumerate() {
        let probe = SignedFailureProbe::sign(
            FailureProbeClaims {
                schema: FAILURE_PROBE_SCHEMA.into(),
                trust_domain: DOMAIN.into(),
                membership_epoch: 6,
                observer_node_id: "node-local".into(),
                observer_identity_generation: 1,
                subject_node_id: format!("node-{suffix}"),
                sequence: offset as u64 + 1,
                observed_at_unix_secs: 100,
                expires_at_unix_secs: 120,
                result: ProbeResult::Reachable,
            },
            &observer,
        )
        .unwrap();
        detector.observe(&authority, &probe, 100).unwrap();
    }
    detector
}

fn assert_schema_keys(openapi: &serde_json::Value, schema_name: &str, value: serde_json::Value) {
    let schema = &openapi["components"]["schemas"][schema_name];
    assert_eq!(schema["additionalProperties"], false, "{schema_name}");
    let expected = schema["required"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    let actual = value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "{schema_name}");
}

fn marker(case: &str, result: &str) {
    println!(
        "ADL_ISSUE_5877_NEGATIVE_CASE_V1 {}",
        serde_json::json!({"case":case,"result":result})
    );
}

fn policy() -> ProjectionPolicy {
    ProjectionPolicy {
        trust_domain: DOMAIN.into(),
        reference_key: ProjectionReferenceKey::new([7; 32]).unwrap(),
        max_nodes: 8,
        max_certificates: 16,
        max_leases: 8,
        max_fences: 8,
        max_placements: 8,
        max_migrations: 8,
        max_recoveries: 8,
        max_response_bytes: 65_536,
    }
}
fn request() -> ProjectionRequest {
    ProjectionRequest {
        version: "v1".into(),
        trust_domain: DOMAIN.into(),
        credential: b"credential".to_vec(),
    }
}

#[test]
fn coherent_projection_is_deterministic_redacted_and_openapi_aligned() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let membership = base_membership();
    let authority_membership = authority_membership();
    let cert_root = key(30);
    let certificates = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        root.join("certificates.redb"),
        CertificatePolicy::new(DOMAIN, [cert_root.verifying_key()]).unwrap(),
    )
    .unwrap();
    let failure = FailureDetector::new(
        FailurePolicy::new(
            DOMAIN,
            "node-a",
            6,
            FailureThresholds {
                suspect_after_secs: 5,
                unavailable_after_secs: 10,
                evidence_window_secs: 5,
                flap_window_secs: 30,
            },
            1,
        )
        .unwrap(),
    );
    let ledger = AuthorityLedger::new(
        &TEST_LEASE_STORE_ACCESS,
        LeasePolicy {
            max_lease_duration_millis: 1_000,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
            max_lineages: 8,
            max_snapshot_bytes: 1024 * 1024,
        },
    )
    .unwrap();
    for name in ["fencing", "migration", "recovery"] {
        std::fs::create_dir(root.join(name)).unwrap();
    }
    let fencing = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        root.join("fencing").canonicalize().unwrap(),
        FencingPolicy {
            max_lineages: 8,
            max_receipts: 16,
            max_state_bytes: 1024 * 1024,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
        },
        Arc::new(FenceAuthority::default()),
    )
    .unwrap();
    let placement = PlacementService::new(PlacementPolicy::new(DOMAIN).unwrap(), PlacementNow);
    placement
        .seed_decision_for_snapshot_test(
            PlacementDecision {
                lineage_id: "lineage-a".into(),
                node_id: "node-a".into(),
                guardian_id: "guardian-a".into(),
                membership_epoch: 6,
                committed_log_index: 6,
                capability_sequence: 1,
                weather_sequence: 1,
                pressure_permille: 100,
                remaining_slots: 2,
            },
            100,
        )
        .unwrap();
    let migrations = MigrationStore::create(
        root.join("migration").canonicalize().unwrap(),
        MigrationPolicy::new(DOMAIN).unwrap(),
        Arc::new(MigrationAuthority::default()),
        Arc::new(MigrationNow),
    )
    .unwrap();
    let recoveries = RecoveryStore::create(
        root.join("recovery").canonicalize().unwrap(),
        RecoveryPolicy::new(DOMAIN).unwrap(),
        Arc::new(RecoveryAuthority::default()),
        Arc::new(RecoveryNow),
    )
    .unwrap();

    let run = || {
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries,
            },
        )
        .unwrap()
    };
    let first = run();
    let second = run();
    assert_eq!(first, second);
    let first_len = first.len();
    let text = String::from_utf8(first).unwrap();
    for secret in [
        "credential",
        "node-a",
        "guardian-a",
        "lineage-a",
        "certificate_bytes",
        "signature",
        "public_key",
        "/Volumes/",
    ] {
        assert!(!text.contains(secret), "redacted {secret}");
    }
    let projection: DistributedProjectionV1 = serde_json::from_str(&text).unwrap();
    assert_eq!(projection.schema, PROJECTION_SCHEMA_V1);
    assert_eq!(projection.nodes.len(), 3);
    assert_eq!(projection.placements.len(), 1);
    assert!(!projection.ready);
    let openapi: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/api/runtime-v3/v1/distributed.openapi.json"
    ))
    .unwrap();
    assert_eq!(openapi["openapi"], "3.1.0");
    let operation = &openapi["paths"]["/runtime/v3/distributed/projection"]["get"];
    assert_eq!(operation["operationId"], "getDistributedProjectionV1");
    assert_eq!(
        operation["security"][0]["bearerAuth"],
        serde_json::json!([])
    );
    for status in ["200", "400", "401", "403", "406", "413", "500", "503"] {
        assert!(
            operation["responses"].get(status).is_some(),
            "status {status}"
        );
    }
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let schema = &openapi["components"]["schemas"]["DistributedProjectionV1"];
    let actual = value
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let required = schema["required"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, required);
    assert_eq!(schema["additionalProperties"], false);
    assert!(first_len <= policy().max_response_bytes);
    assert!(first_len > 256);
    let mut exact_node_bound = policy();
    exact_node_bound.max_nodes = 3;
    assert_eq!(
        project_v1(
            &exact_node_bound,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries
            }
        )
        .unwrap()
        .len(),
        first_len
    );
    let mut node_bounded = policy();
    node_bounded.max_nodes = 2;
    assert_eq!(
        project_v1(
            &node_bounded,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries
            }
        ),
        Err(ProjectionError::ResourceExhausted)
    );
    let mut byte_bounded = policy();
    byte_bounded.max_response_bytes = first_len;
    assert_eq!(
        project_v1(
            &byte_bounded,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries
            }
        )
        .unwrap()
        .len(),
        first_len
    );
    byte_bounded.max_response_bytes = first_len - 1;
    assert_eq!(
        project_v1(
            &byte_bounded,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries
            }
        ),
        Err(ProjectionError::ResourceExhausted)
    );
    for (case, result) in [
        ("deterministic_jcs", "recovered"),
        ("raw_evidence_redaction", "rejected"),
        ("openapi_parity", "recovered"),
        ("response_byte_bound", "rejected"),
    ] {
        println!(
            "ADL_ISSUE_5877_NEGATIVE_CASE_V1 {}",
            serde_json::json!({"case":case,"result":result})
        );
    }
}

#[test]
fn request_and_authority_failures_are_closed_and_bounded() {
    let mut invalid = request();
    invalid.version = "v2".into();
    let unavailable = PlacementService::new(PlacementPolicy::new(DOMAIN).unwrap(), PlacementNow);
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let membership = base_membership();
    let authority_membership = authority_membership();
    let cert_root = key(31);
    let certificates = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        root.join("cert.redb"),
        CertificatePolicy::new(DOMAIN, [cert_root.verifying_key()]).unwrap(),
    )
    .unwrap();
    let failure = FailureDetector::new(
        FailurePolicy::new(
            DOMAIN,
            "node-a",
            6,
            FailureThresholds {
                suspect_after_secs: 5,
                unavailable_after_secs: 10,
                evidence_window_secs: 5,
                flap_window_secs: 30,
            },
            1,
        )
        .unwrap(),
    );
    let ledger = AuthorityLedger::new(
        &TEST_LEASE_STORE_ACCESS,
        LeasePolicy {
            max_lease_duration_millis: 1_000,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
            max_lineages: 8,
            max_snapshot_bytes: 1024 * 1024,
        },
    )
    .unwrap();
    for name in ["fencing", "migration", "recovery"] {
        std::fs::create_dir(root.join(name)).unwrap();
    }
    let fencing = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        root.join("fencing").canonicalize().unwrap(),
        FencingPolicy {
            max_lineages: 8,
            max_receipts: 16,
            max_state_bytes: 1024 * 1024,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
        },
        Arc::new(FenceAuthority::default()),
    )
    .unwrap();
    let migrations = MigrationStore::create(
        root.join("migration").canonicalize().unwrap(),
        MigrationPolicy::new(DOMAIN).unwrap(),
        Arc::new(MigrationAuthority::default()),
        Arc::new(MigrationNow),
    )
    .unwrap();
    let recoveries = RecoveryStore::create(
        root.join("recovery").canonicalize().unwrap(),
        RecoveryPolicy::new(DOMAIN).unwrap(),
        Arc::new(RecoveryAuthority::default()),
        Arc::new(RecoveryNow),
    )
    .unwrap();
    let sources = || ProjectionSources {
        membership: &membership,
        authority_membership: &authority_membership,
        certificates: &certificates,
        failure_detector: &failure,
        lease_ledger: &ledger,
        fencing: &fencing,
        placement: &unavailable,
        migrations: &migrations,
        recoveries: &recoveries,
    };
    assert_eq!(
        project_v1(&policy(), &invalid, &Auth(true), &Clock, sources()),
        Err(ProjectionError::UnsupportedVersion)
    );
    assert_eq!(
        project_v1(&policy(), &request(), &Auth(false), &Clock, sources()),
        Err(ProjectionError::Unauthorized)
    );
    let clock_was_read = Arc::new(AtomicBool::new(false));
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(false),
            &RecordingClock(clock_was_read.clone()),
            sources()
        ),
        Err(ProjectionError::Unauthorized)
    );
    assert!(!clock_was_read.load(Ordering::SeqCst));
    assert_eq!(
        project_v1(&policy(), &request(), &Auth(true), &Clock, sources()),
        Err(ProjectionError::IncoherentCut)
    );
    let ahead_authority = authority_membership_at(DOMAIN, 7);
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &ahead_authority,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &unavailable,
                migrations: &migrations,
                recoveries: &recoveries,
            }
        ),
        Err(ProjectionError::IncoherentCut)
    );
    assert_eq!(
        ProjectionReferenceKey::new([0; 32]),
        Err(ProjectionError::InvalidPolicy)
    );
    let wrong_domain_authority = authority_membership_at("other.test", 6);
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &wrong_domain_authority,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &unavailable,
                migrations: &migrations,
                recoveries: &recoveries,
            }
        ),
        Err(ProjectionError::WrongTrustDomain)
    );
    let unrelated_authority = unrelated_authority_membership();
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &unrelated_authority,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &unavailable,
                migrations: &migrations,
                recoveries: &recoveries,
            }
        ),
        Err(ProjectionError::IncoherentCut)
    );

    let behind_authority = authority_membership_at(DOMAIN, 5);
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &behind_authority,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &unavailable,
                migrations: &migrations,
                recoveries: &recoveries,
            }
        ),
        Err(ProjectionError::IncoherentCut)
    );

    let unknown_member = PlacementService::new(PlacementPolicy::new(DOMAIN).unwrap(), PlacementNow);
    unknown_member
        .seed_decision_for_snapshot_test(
            PlacementDecision {
                lineage_id: "lineage-z".into(),
                node_id: "node-z".into(),
                guardian_id: "guardian-z".into(),
                membership_epoch: 6,
                committed_log_index: 6,
                capability_sequence: 1,
                weather_sequence: 1,
                pressure_permille: 100,
                remaining_slots: 2,
            },
            100,
        )
        .unwrap();
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &unknown_member,
                migrations: &migrations,
                recoveries: &recoveries,
            }
        ),
        Err(ProjectionError::IncoherentCut)
    );
    for (case, result) in [
        ("unauthorized_request", "denied"),
        ("authorization_before_read", "denied"),
        ("unsupported_version", "rejected"),
        ("wrong_authority_domain", "rejected"),
        ("authority_voter_membership_mismatch", "rejected"),
        ("authority_index_behind_membership", "rejected"),
        ("authority_index_ahead_of_membership", "rejected"),
        ("unknown_member_reference", "rejected"),
        ("zero_reference_key", "rejected"),
        ("placement_authority_unavailable", "fail_closed"),
    ] {
        marker(case, result);
    }
}

#[test]
fn readiness_requires_exact_active_identity_and_transport_and_refs_are_keyed() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let membership = base_membership();
    let authority_membership = authority_membership();
    let cert_root = key(80);
    let certificates = DistributedCertificateStore::open(
        &TEST_CERTIFICATE_STORE_ACCESS,
        root.join("certificates.redb"),
        CertificatePolicy::new(DOMAIN, [cert_root.verifying_key()]).unwrap(),
    )
    .unwrap();
    let failure = healthy_failure_detector();
    let ledger = AuthorityLedger::new(
        &TEST_LEASE_STORE_ACCESS,
        LeasePolicy {
            max_lease_duration_millis: 1_000,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
            max_lineages: 8,
            max_snapshot_bytes: 1024 * 1024,
        },
    )
    .unwrap();
    for name in ["fencing", "migration", "recovery"] {
        std::fs::create_dir(root.join(name)).unwrap();
    }
    let fencing = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        root.join("fencing").canonicalize().unwrap(),
        FencingPolicy {
            max_lineages: 8,
            max_receipts: 16,
            max_state_bytes: 1024 * 1024,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
        },
        Arc::new(FenceAuthority::default()),
    )
    .unwrap();
    let placement = PlacementService::new(PlacementPolicy::new(DOMAIN).unwrap(), PlacementNow);
    placement
        .seed_decision_for_snapshot_test(
            PlacementDecision {
                lineage_id: "lineage-a".into(),
                node_id: "node-a".into(),
                guardian_id: "guardian-a".into(),
                membership_epoch: 6,
                committed_log_index: 6,
                capability_sequence: 1,
                weather_sequence: 1,
                pressure_permille: 100,
                remaining_slots: 2,
            },
            100,
        )
        .unwrap();
    let migrations = MigrationStore::create(
        root.join("migration").canonicalize().unwrap(),
        MigrationPolicy::new(DOMAIN).unwrap(),
        Arc::new(MigrationAuthority::default()),
        Arc::new(MigrationNow),
    )
    .unwrap();
    let recoveries = RecoveryStore::create(
        root.join("recovery").canonicalize().unwrap(),
        RecoveryPolicy::new(DOMAIN).unwrap(),
        Arc::new(RecoveryAuthority::default()),
        Arc::new(RecoveryNow),
    )
    .unwrap();
    let project = |projection_policy: &ProjectionPolicy| {
        let bytes = project_v1(
            projection_policy,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries,
            },
        )
        .unwrap();
        serde_json::from_slice::<DistributedProjectionV1>(&bytes).unwrap()
    };

    assert!(!project(&policy()).ready);
    certificates
        .activate(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate(
                &cert_root,
                "node-a",
                CertificatePurpose::NodeIdentity,
                1,
                81,
            ),
            60,
        )
        .unwrap();
    assert!(!project(&policy()).ready);
    let mut certificate_bound = policy();
    certificate_bound.max_certificates = 1;
    assert_eq!(project(&certificate_bound).certificates.len(), 1);
    certificates
        .activate(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate(&cert_root, "node-a", CertificatePurpose::Transport, 1, 82),
            60,
        )
        .unwrap();
    assert_eq!(
        project_v1(
            &certificate_bound,
            &request(),
            &Auth(true),
            &Clock,
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries,
            },
        ),
        Err(ProjectionError::ResourceExhausted)
    );
    for (offset, suffix) in ["b", "c"].into_iter().enumerate() {
        certificates
            .activate(
                &TEST_CERTIFICATE_STORE_ACCESS,
                &certificate(
                    &cert_root,
                    &format!("node-{suffix}"),
                    CertificatePurpose::NodeIdentity,
                    1,
                    84 + offset as u8 * 2,
                ),
                60,
            )
            .unwrap();
        certificates
            .activate(
                &TEST_CERTIFICATE_STORE_ACCESS,
                &certificate(
                    &cert_root,
                    &format!("node-{suffix}"),
                    CertificatePurpose::Transport,
                    1,
                    85 + offset as u8 * 2,
                ),
                60,
            )
            .unwrap();
    }
    let ready = project(&policy());
    assert!(ready.ready);

    let mut alternate_policy = policy();
    alternate_policy.reference_key = ProjectionReferenceKey::new([8; 32]).unwrap();
    let alternate = project(&alternate_policy);
    assert_ne!(ready.nodes[0].node_id, alternate.nodes[0].node_id);
    assert_ne!(
        ready.placements[0].lineage_id,
        alternate.placements[0].lineage_id
    );
    assert!(ready.nodes[0].node_id.starts_with("id_"));
    assert!(!format!("{:?}", policy().reference_key).contains(&hex::encode([7; 32])));

    certificates
        .activate(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate(
                &cert_root,
                "node-a",
                CertificatePurpose::NodeIdentity,
                2,
                83,
            ),
            70,
        )
        .unwrap();
    assert!(!project(&policy()).ready);
    assert_eq!(
        project_v1(
            &policy(),
            &request(),
            &Auth(true),
            &ClockAt(121),
            ProjectionSources {
                membership: &membership,
                authority_membership: &authority_membership,
                certificates: &certificates,
                failure_detector: &failure,
                lease_ledger: &ledger,
                fencing: &fencing,
                placement: &placement,
                migrations: &migrations,
                recoveries: &recoveries,
            },
        ),
        Err(ProjectionError::StaleCut)
    );
    for (case, result) in [
        ("identity_without_transport", "rejected"),
        ("exact_identity_and_transport", "recovered"),
        ("certificate_count_n_plus_one", "rejected"),
        ("identity_rotation_overlap", "rejected"),
        ("keyed_opaque_references", "recovered"),
        ("stale_authority_cut", "rejected"),
    ] {
        marker(case, result);
    }
}

#[test]
fn openapi_nested_dtos_enums_limits_security_and_errors_match_rust_contract() {
    let openapi: serde_json::Value = serde_json::from_str(include_str!(
        "../../docs/api/runtime-v3/v1/distributed.openapi.json"
    ))
    .unwrap();
    assert_schema_keys(
        &openapi,
        "ProjectedNode",
        serde_json::to_value(ProjectedNode {
            node_id: "id_0".into(),
            guardian_id: "id_1".into(),
            identity_generation: 1,
            role: "voter".into(),
            peer_state: "connected".into(),
            failure_class: Some("healthy".into()),
            failure_reason: "healthy_evidence".into(),
            failure_freshness: "fresh".into(),
            advertisement_freshness: "verified_at_decision".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedCertificate",
        serde_json::to_value(ProjectedCertificate {
            certificate_id: "id_2".into(),
            holder_id: "id_0".into(),
            holder_kind: "node".into(),
            purpose: "transport".into(),
            generation: 1,
            health: "active".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedLease",
        serde_json::to_value(ProjectedLease {
            lineage_id: "id_3".into(),
            holder_node_id: Some("id_0".into()),
            holder_guardian_id: Some("id_1".into()),
            epoch: 1,
            committed_log_index: 1,
            certificate_generation: 1,
            health: "active".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedFence",
        serde_json::to_value(ProjectedFence {
            lineage_id: "id_3".into(),
            epoch: 2,
            committed_log_index: 2,
            voter_set_generation: 1,
            operation: "fence".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedPlacement",
        serde_json::to_value(ProjectedPlacement {
            lineage_id: "id_3".into(),
            node_id: "id_0".into(),
            guardian_id: "id_1".into(),
            freshness: "verified_at_decision".into(),
            capacity: "available".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedMigration",
        serde_json::to_value(ProjectedMigration {
            migration_id: "id_4".into(),
            lineage_id: "id_3".into(),
            source_node_id: "id_0".into(),
            source_guardian_id: "id_1".into(),
            target_node_id: "id_5".into(),
            target_guardian_id: "id_6".into(),
            phase: "prepared".into(),
        })
        .unwrap(),
    );
    assert_schema_keys(
        &openapi,
        "ProjectedRecovery",
        serde_json::to_value(ProjectedRecovery {
            recovery_id: "id_7".into(),
            migration_id: "id_4".into(),
            lineage_id: "id_3".into(),
            owner_node_id: None,
            owner_guardian_id: None,
            phase: "pending".into(),
            reason: "timeout".into(),
            operator_required: true,
        })
        .unwrap(),
    );

    let schemas = &openapi["components"]["schemas"];
    assert_eq!(schemas["OpaqueId"]["pattern"], "^id_[0-9a-f]{64}$");
    assert_eq!(schemas["OpaqueId"]["maxLength"], 67);
    assert_eq!(schemas["ProjectionId"]["pattern"], "^prj_[0-9a-f]{64}$");
    assert_eq!(
        schemas["DistributedProjectionV1"]["properties"]["nodes"]["maxItems"],
        4096
    );
    assert_eq!(
        schemas["DistributedProjectionV1"]["properties"]["certificates"]["maxItems"],
        16384
    );
    assert_eq!(
        schemas["ProjectedFence"]["properties"]["operation"]["enum"],
        serde_json::json!(["fence", "revoke"])
    );
    assert_eq!(
        schemas["ProjectedLease"]["properties"]["health"]["enum"],
        serde_json::json!(["active", "expired", "revoked"])
    );
    assert_eq!(openapi["x-adl-max-response-bytes"], 65_536);
    assert_eq!(schemas["Uint64"]["maximum"], serde_json::json!(u64::MAX));
    assert_eq!(
        schemas["PositiveUint64"]["maximum"],
        serde_json::json!(u64::MAX)
    );
    for (schema, field, expected) in [
        (
            "ProjectedCertificate",
            "purpose",
            serde_json::json!([
                "node_identity",
                "guardian_control",
                "transport",
                "advertisement_signing",
                "snapshot_signing"
            ]),
        ),
        (
            "ProjectedCertificate",
            "health",
            serde_json::json!([
                "active",
                "rotation_overlap",
                "superseded",
                "not_yet_valid",
                "expired",
                "revoked",
                "fenced"
            ]),
        ),
        (
            "ProjectedNode",
            "failure_reason",
            serde_json::json!([
                "no_evidence",
                "healthy_evidence",
                "suspected_failure",
                "quorum_unavailable",
                "network_partition",
                "recovery_evidence",
                "flapping_evidence"
            ]),
        ),
        (
            "ProjectedMigration",
            "phase",
            serde_json::json!([
                "prepared",
                "quiesced",
                "checkpointed",
                "transferred",
                "validated",
                "fenced",
                "activated",
                "committed",
                "aborted"
            ]),
        ),
        (
            "ProjectedRecovery",
            "phase",
            serde_json::json!([
                "assessing",
                "planned",
                "cleanup_pending",
                "target_discarded",
                "rollback_pending",
                "fence_pending",
                "fenced",
                "operator_required",
                "activate_pending",
                "restored",
                "commit_pending",
                "committed"
            ]),
        ),
        (
            "ProjectedRecovery",
            "reason",
            serde_json::json!([
                "assessing_interrupted_migration",
                "planned_recovery",
                "target_cleanup_required",
                "target_discarded",
                "rollback_required",
                "fencing_required",
                "fenced",
                "operator_required",
                "activation_required",
                "restored",
                "commit_required",
                "committed"
            ]),
        ),
    ] {
        assert_eq!(schemas[schema]["properties"][field]["enum"], expected);
    }
    assert_eq!(
        openapi["components"]["securitySchemes"]["bearerAuth"],
        serde_json::json!({"type":"http","scheme":"bearer"})
    );
    let operation = &openapi["paths"]["/runtime/v3/distributed/projection"]["get"];
    assert_eq!(
        operation["security"],
        serde_json::json!([{"bearerAuth":[]} ])
    );
    let trust_domain = operation["parameters"]
        .as_array()
        .unwrap()
        .iter()
        .find(|parameter| parameter["name"] == "X-ADL-Trust-Domain")
        .unwrap();
    assert_eq!(trust_domain["in"], "header");
    assert_eq!(trust_domain["required"], true);
    assert_eq!(trust_domain["schema"]["minLength"], 1);
    assert_eq!(trust_domain["schema"]["maxLength"], 128);
    for (status, response, schema) in [
        ("400", "Projection400", "ProjectionError400"),
        ("401", "Projection401", "ProjectionError401"),
        ("403", "Projection403", "ProjectionError403"),
        ("406", "Projection406", "ProjectionError406"),
        ("413", "Projection413", "ProjectionError413"),
        ("500", "Projection500", "ProjectionError500"),
        ("503", "Projection503", "ProjectionError503"),
    ] {
        assert_eq!(
            operation["responses"][status]["$ref"],
            format!("#/components/responses/{response}")
        );
        assert_eq!(
            openapi["components"]["responses"][response]["content"]["application/json"]["schema"]
                ["$ref"],
            format!("#/components/schemas/{schema}")
        );
    }
    let errors = schemas["ProjectionError"]["properties"]["code"]["enum"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let rust_errors = [
        ProjectionError::InvalidPolicy,
        ProjectionError::InvalidRequest,
        ProjectionError::Unauthorized,
        ProjectionError::UnsupportedVersion,
        ProjectionError::WrongTrustDomain,
        ProjectionError::StaleCut,
        ProjectionError::IncoherentCut,
        ProjectionError::MalformedAuthority,
        ProjectionError::ResourceExhausted,
        ProjectionError::Serialization,
    ]
    .into_iter()
    .map(ProjectionError::code)
    .collect::<BTreeSet<_>>();
    assert_eq!(errors, rust_errors);
    marker("nested_openapi_parity", "recovered");
}
