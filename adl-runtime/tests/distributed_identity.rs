// PVF: lane=exact-child-tests; proof=identity/enrollment positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[path = "../src/distributed/identity.rs"]
mod identity;

use ed25519_dalek::SigningKey;
use identity::{
    operator_root_id, DistributedIdentityStore, EnrollmentClaims, EnrollmentOutcome,
    EnrollmentPolicy, GuardianEnrollmentRole, IdentityError, PublicNodeGuardianIdentity,
    SignedEnrollmentRequest, IDENTITY_SCHEMA,
};
use tempfile::TempDir;

const DOMAIN: &str = "polis.example";
const NOW: u64 = 1_000_000;

fn signing_key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn policy(root: &SigningKey) -> EnrollmentPolicy {
    EnrollmentPolicy::new(DOMAIN, [root.verifying_key()]).unwrap()
}

fn open_store(temp: &TempDir, policy: EnrollmentPolicy) -> DistributedIdentityStore {
    DistributedIdentityStore::open(
        temp.path().canonicalize().unwrap().join("identity.redb"),
        policy,
    )
    .unwrap()
}

fn open_error(path: impl AsRef<std::path::Path>, policy: EnrollmentPolicy) -> IdentityError {
    match DistributedIdentityStore::open(path, policy) {
        Ok(_) => panic!("corrupt or unsafe database path unexpectedly opened"),
        Err(error) => error,
    }
}

fn identity_with_keys(
    node_id_seed: u8,
    guardian_id_seed: u8,
    node_key: &SigningKey,
    guardian_key: &SigningKey,
) -> PublicNodeGuardianIdentity {
    PublicNodeGuardianIdentity {
        schema: IDENTITY_SCHEMA.to_owned(),
        trust_domain: DOMAIN.to_owned(),
        identity_generation: 1,
        node_id: format!("node_{}", hex::encode([node_id_seed; 32])),
        guardian_id: format!("guardian_{}", hex::encode([guardian_id_seed; 32])),
        node_public_key: node_key.verifying_key().to_bytes(),
        guardian_control_public_key: guardian_key.verifying_key().to_bytes(),
    }
}

fn signed_request(
    identity: PublicNodeGuardianIdentity,
    root: &SigningKey,
    node: &SigningKey,
    guardian: &SigningKey,
    nonce_seed: u8,
) -> SignedEnrollmentRequest {
    let claims = EnrollmentClaims::new(
        identity,
        GuardianEnrollmentRole::Voter,
        operator_root_id(&root.verifying_key()),
        [nonce_seed; 32],
        NOW,
        NOW + 300,
    );
    SignedEnrollmentRequest::sign(claims, node, guardian, root).unwrap()
}

#[test]
fn local_node_and_guardian_identity_is_restart_stable() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(1);
    let configured_policy = policy(&root);
    assert_eq!(configured_policy.trust_domain(), DOMAIN);
    assert_eq!(
        configured_policy.approved_root_id(&root.verifying_key()),
        Some(operator_root_id(&root.verifying_key()))
    );
    let first_public = {
        let store = open_store(&temp, configured_policy);
        let identity = store.load_or_create_local_identity(1).unwrap();
        identity.public_identity().clone()
    };

    let restarted = open_store(&temp, policy(&root));
    let restored = restarted.load_local_identity().unwrap();
    assert_eq!(restored.public_identity(), &first_public);
    assert_eq!(
        restarted.database_path(),
        temp.path().canonicalize().unwrap().join("identity.redb")
    );
    assert_eq!(
        restarted.load_or_create_local_identity(2).unwrap_err(),
        IdentityError::IdentityGenerationMismatch
    );
}

#[test]
fn signed_enrollment_is_durable_and_same_generation_is_idempotent() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(2);
    let public = {
        let store = open_store(&temp, policy(&root));
        let local = store.load_or_create_local_identity(1).unwrap();
        let first = local
            .sign_enrollment(
                GuardianEnrollmentRole::Voter,
                &root,
                [1; 32],
                NOW,
                NOW + 300,
            )
            .unwrap();
        assert!(matches!(
            store.enroll(&first, NOW).unwrap(),
            EnrollmentOutcome::Enrolled(_)
        ));

        let duplicate = local
            .sign_enrollment(
                GuardianEnrollmentRole::Voter,
                &root,
                [2; 32],
                NOW + 1,
                NOW + 301,
            )
            .unwrap();
        assert!(matches!(
            store.enroll(&duplicate, NOW + 1).unwrap(),
            EnrollmentOutcome::AlreadyEnrolled(_)
        ));
        assert_eq!(store.enrollment_count().unwrap(), 1);
        local.public_identity().clone()
    };

    let restarted = open_store(&temp, policy(&root));
    let restored = restarted.enrollment(&public.node_id).unwrap().unwrap();
    assert_eq!(restored.request.claims.identity, public);
    assert_eq!(restarted.enrollment_count().unwrap(), 1);
}

#[test]
fn wrong_trust_domain_is_rejected_and_audited() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(3);
    let store = open_store(&temp, policy(&root));
    let other = identity::LocalNodeGuardianIdentity::generate("other.example", 1).unwrap();
    let request = other
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [3; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();

    assert_eq!(
        store.enroll(&request, NOW).unwrap_err(),
        IdentityError::WrongTrustDomain
    );
    let audit = store.audit_events().unwrap();
    assert_eq!(audit.last().unwrap().reason_code, "wrong_trust_domain");
    assert_eq!(audit.last().unwrap().result, "rejected");
}

#[test]
fn consumed_nonce_cannot_be_replayed() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(4);
    let store = open_store(&temp, policy(&root));
    let local = store.load_or_create_local_identity(1).unwrap();
    let request = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [4; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();

    store.enroll(&request, NOW).unwrap();
    assert_eq!(
        store.enroll(&request, NOW + 1).unwrap_err(),
        IdentityError::Replay
    );
    assert_eq!(store.enrollment_count().unwrap(), 1);
}

#[test]
fn node_and_guardian_proof_of_possession_are_both_required() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(5);
    let store = open_store(&temp, policy(&root));
    let local = store.load_or_create_local_identity(1).unwrap();

    let mut bad_node = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [5; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    bad_node.node_proof_signature[0] ^= 0x80;
    assert_eq!(
        store.enroll(&bad_node, NOW).unwrap_err(),
        IdentityError::InvalidNodeProof
    );

    let mut bad_guardian = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [6; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    bad_guardian.guardian_proof_signature[0] ^= 0x80;
    assert_eq!(
        store.enroll(&bad_guardian, NOW).unwrap_err(),
        IdentityError::InvalidGuardianProof
    );
    assert_eq!(store.enrollment_count().unwrap(), 0);
}

#[test]
fn unapproved_operator_root_and_malformed_signature_fail_closed() {
    let temp = tempfile::tempdir().unwrap();
    let approved_root = signing_key(6);
    let unapproved_root = signing_key(7);
    let store = open_store(&temp, policy(&approved_root));
    let local = store.load_or_create_local_identity(1).unwrap();
    let unapproved = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &unapproved_root,
            [7; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    assert_eq!(
        store.enroll(&unapproved, NOW).unwrap_err(),
        IdentityError::OperatorRootNotApproved
    );

    let mut malformed = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &approved_root,
            [8; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    malformed.operator_signature.truncate(63);
    assert_eq!(
        store.enroll(&malformed, NOW).unwrap_err(),
        IdentityError::MalformedSignature
    );
}

#[test]
fn expired_and_future_requests_fail_closed() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(8);
    let store = open_store(&temp, policy(&root));
    let local = store.load_or_create_local_identity(1).unwrap();
    let expired = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [9; 32],
            NOW - 300,
            NOW,
        )
        .unwrap();
    assert_eq!(
        store.enroll(&expired, NOW).unwrap_err(),
        IdentityError::RequestExpired
    );

    let future = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [10; 32],
            NOW + 1,
            NOW + 301,
        )
        .unwrap();
    assert_eq!(
        store.enroll(&future, NOW).unwrap_err(),
        IdentityError::RequestNotYetValid
    );
}

#[test]
fn one_guardian_control_key_cannot_represent_two_voters() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(9);
    let store = open_store(&temp, policy(&root));
    let first_node = signing_key(10);
    let second_node = signing_key(11);
    let shared_guardian = signing_key(12);

    let first = signed_request(
        identity_with_keys(10, 10, &first_node, &shared_guardian),
        &root,
        &first_node,
        &shared_guardian,
        11,
    );
    store.enroll(&first, NOW).unwrap();

    let second = signed_request(
        identity_with_keys(11, 11, &second_node, &shared_guardian),
        &root,
        &second_node,
        &shared_guardian,
        12,
    );
    assert_eq!(
        store.enroll(&second, NOW).unwrap_err(),
        IdentityError::DuplicateGuardianControlKey
    );
    assert_eq!(store.enrollment_count().unwrap(), 1);
}

#[test]
fn bounded_nonce_capacity_rejects_additional_work() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(13);
    let bounded = policy(&root).with_limits(4, 1, 8).unwrap();
    let store = open_store(&temp, bounded);
    let local = store.load_or_create_local_identity(1).unwrap();
    let first = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [13; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    store.enroll(&first, NOW).unwrap();

    let second = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [14; 32],
            NOW + 1,
            NOW + 301,
        )
        .unwrap();
    assert_eq!(
        store.enroll(&second, NOW + 1).unwrap_err(),
        IdentityError::ResourceExhausted
    );
}

#[test]
fn audit_records_are_bounded_and_redact_identity_material() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(14);
    let bounded = policy(&root).with_limits(4, 4, 2).unwrap();
    let store = open_store(&temp, bounded);
    let local = store.load_or_create_local_identity(1).unwrap();
    let public = local.public_identity().clone();

    for nonce in [15_u8, 16, 17] {
        let mut request = local
            .sign_enrollment(
                GuardianEnrollmentRole::Voter,
                &root,
                [nonce; 32],
                NOW,
                NOW + 300,
            )
            .unwrap();
        request.operator_signature[0] ^= 1;
        assert_eq!(
            store.enroll(&request, NOW).unwrap_err(),
            IdentityError::InvalidOperatorSignature
        );
    }

    let audit = store.audit_events().unwrap();
    assert_eq!(audit.len(), 2);
    let encoded = serde_json::to_string(&audit).unwrap();
    assert!(!encoded.contains(&public.node_id));
    assert!(!encoded.contains(&public.guardian_id));
    assert!(!encoded.contains(&hex::encode(public.guardian_control_public_key)));
}

#[test]
fn oversized_request_is_rejected_before_unbounded_serialization_and_safely_audited() {
    let temp = tempfile::tempdir().unwrap();
    let root = signing_key(15);
    let store = open_store(&temp, policy(&root));
    let local = store.load_or_create_local_identity(1).unwrap();
    let mut request = local
        .sign_enrollment(
            GuardianEnrollmentRole::Voter,
            &root,
            [18; 32],
            NOW,
            NOW + 300,
        )
        .unwrap();
    request.operator_signature = vec![0; 1_000_000];

    assert_eq!(
        store.enroll(&request, NOW).unwrap_err(),
        IdentityError::RequestTooLarge
    );
    let audit = store.audit_events().unwrap();
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].reason_code, "request_too_large");
    assert_eq!(audit[0].request_sha256.len(), 64);
}

#[test]
fn corrupt_audit_metadata_event_and_nonce_fail_closed_on_restart() {
    let root = signing_key(16);

    let corrupt_meta = tempfile::tempdir().unwrap();
    {
        let store = open_store(&corrupt_meta, policy(&root));
        store.corrupt_audit_sequence_for_test(&[1, 2, 3]).unwrap();
    }
    assert_eq!(
        open_error(
            corrupt_meta
                .path()
                .canonicalize()
                .unwrap()
                .join("identity.redb"),
            policy(&root),
        ),
        IdentityError::DurableStateCorrupt
    );

    let corrupt_event = tempfile::tempdir().unwrap();
    {
        let store = open_store(&corrupt_event, policy(&root));
        store
            .corrupt_audit_event_for_test(1, br#"{"schema":"wrong"}"#)
            .unwrap();
    }
    assert_eq!(
        open_error(
            corrupt_event
                .path()
                .canonicalize()
                .unwrap()
                .join("identity.redb"),
            policy(&root),
        ),
        IdentityError::DurableStateCorrupt
    );

    let corrupt_nonce = tempfile::tempdir().unwrap();
    {
        let store = open_store(&corrupt_nonce, policy(&root));
        store.corrupt_nonce_for_test(&[1, 2, 3], NOW).unwrap();
    }
    assert_eq!(
        open_error(
            corrupt_nonce
                .path()
                .canonicalize()
                .unwrap()
                .join("identity.redb"),
            policy(&root),
        ),
        IdentityError::DurableStateCorrupt
    );
}

#[cfg(unix)]
#[test]
fn symlinked_database_parent_is_rejected() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let canonical_temp = temp.path().canonicalize().unwrap();
    let real_parent = canonical_temp.join("real");
    std::fs::create_dir(&real_parent).unwrap();
    let linked_parent = canonical_temp.join("linked");
    symlink(&real_parent, &linked_parent).unwrap();

    assert_eq!(
        open_error(
            linked_parent.join("identity.redb"),
            policy(&signing_key(17))
        ),
        IdentityError::DatabasePathIsSymlink
    );
    assert!(!real_parent.join("identity.redb").exists());
}
