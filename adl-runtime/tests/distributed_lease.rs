#[path = "../src/distributed/lease.rs"]
mod lease;

use std::collections::BTreeSet;

use ed25519_dalek::{Signer, SigningKey};
use lease::{
    activation_signature, certificate_body_sha256, encode_certificate, endorse, mutation_signature,
    verify_certificate as verify_certificate_at, AuthorityApplication, AuthorityCertificateBodyV1,
    AuthorityCertificateV1, AuthorityEndorsementV1, AuthorityError, AuthorityLedger,
    AuthorityMembership, ControlCertificatePurpose, LeasePolicy, MutationAuthorization,
    OperationClass, VerifiedAuthority, VoterAuthority, AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
    SIGNING_ALGORITHM_ED25519,
};
use prost::Message;
use sha2::{Digest, Sha256};

const TRUST: &[u8] = b"trust-domain";
const LINEAGE: &[u8] = b"lineage-a";
const NODE: &[u8] = b"node-a";
const HOLDER: &[u8] = b"guardian-a";
const NOW_UNIX_SECONDS: i64 = 1_787_000_100;

fn signer(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

struct Fixture {
    ids: Vec<Vec<u8>>,
    keys: Vec<SigningKey>,
    membership: AuthorityMembership,
}

impl Fixture {
    fn stable() -> Self {
        Self::with_configs(vec![vec![0, 1, 2]])
    }

    fn joint() -> Self {
        Self::with_configs(vec![vec![0, 1, 2], vec![2, 3, 4]])
    }

    fn with_configs(configs: Vec<Vec<usize>>) -> Self {
        let count = configs.iter().flatten().copied().max().unwrap() + 1;
        let ids = (0..count)
            .map(|index| format!("guardian-{index}").into_bytes())
            .collect::<Vec<_>>();
        let keys = (0..count)
            .map(|index| signer(index as u8 + 1))
            .collect::<Vec<_>>();
        let voters = ids
            .iter()
            .zip(&keys)
            .map(|(id, key)| VoterAuthority {
                guardian_id: id.clone(),
                trust_domain_id: TRUST.to_vec(),
                certificate_generation: 7,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: NOW_UNIX_SECONDS - 1_000,
                not_after_unix_seconds: NOW_UNIX_SECONDS + 1_000,
                revoked: false,
                control_public_key: key.verifying_key().to_bytes(),
            })
            .collect();
        let configs = configs
            .into_iter()
            .map(|config| {
                config
                    .into_iter()
                    .map(|index| ids[index].clone())
                    .collect::<BTreeSet<_>>()
            })
            .collect();
        let membership = AuthorityMembership::new(TRUST.to_vec(), 7, 1_000, configs, voters)
            .expect("valid membership");
        Self {
            ids,
            keys,
            membership,
        }
    }

    fn certificate(&self, body: AuthorityCertificateBodyV1, signer_indexes: &[usize]) -> Vec<u8> {
        let mut endorsements = signer_indexes
            .iter()
            .map(|index| endorse(&body, self.ids[*index].clone(), 7, &self.keys[*index]))
            .collect::<Vec<_>>();
        endorsements.sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
        encode_certificate(&AuthorityCertificateV1 {
            body: Some(body),
            endorsements,
        })
        .expect("canonical certificate")
    }
}

fn activation(seed: u8) -> SigningKey {
    signer(seed.saturating_add(100))
}

fn body(
    operation: OperationClass,
    log_index: u64,
    epoch: u64,
    activation: &SigningKey,
) -> AuthorityCertificateBodyV1 {
    AuthorityCertificateBodyV1 {
        schema_version: AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
        trust_domain_id: TRUST.to_vec(),
        lineage_id: LINEAGE.to_vec(),
        voter_set_generation: 7,
        raft_term: 3,
        committed_log_index: log_index,
        epoch,
        holder_node_id: NODE.to_vec(),
        holder_guardian_id: HOLDER.to_vec(),
        activation_key_sha256: Sha256::digest(activation.verifying_key().to_bytes()).to_vec(),
        operation_class: operation as u32,
        issued_unix_seconds: NOW_UNIX_SECONDS,
        issued_nanos: 0,
        lease_duration_millis: 100,
        policy_sha256: policy().sha256().unwrap().to_vec(),
        signing_algorithm: SIGNING_ALGORITHM_ED25519,
    }
}

fn policy() -> LeasePolicy {
    LeasePolicy {
        max_lease_duration_millis: 1_000,
        max_clock_uncertainty_millis: 10,
        message_delay_margin_millis: 5,
        max_snapshot_bytes: 1024 * 1024,
    }
}

fn policy_with_max_lease(max_lease_duration_millis: u64) -> LeasePolicy {
    let mut value = policy();
    value.max_lease_duration_millis = max_lease_duration_millis;
    value
}

fn verify_certificate(
    bytes: &[u8],
    membership: &AuthorityMembership,
) -> Result<VerifiedAuthority, AuthorityError> {
    verify_certificate_at(bytes, membership, NOW_UNIX_SECONDS)
}

fn application<'a>(
    activation: &SigningKey,
    proof: &'a [u8],
    now_elapsed_millis: u64,
    clock_uncertainty_millis: u64,
) -> AuthorityApplication<'a> {
    application_at(
        activation,
        proof,
        NOW_UNIX_SECONDS,
        now_elapsed_millis,
        clock_uncertainty_millis,
    )
}

fn application_at<'a>(
    activation: &SigningKey,
    proof: &'a [u8],
    now_unix_seconds: i64,
    now_elapsed_millis: u64,
    clock_uncertainty_millis: u64,
) -> AuthorityApplication<'a> {
    application_at_nanos(
        activation,
        proof,
        now_unix_seconds,
        0,
        now_elapsed_millis,
        clock_uncertainty_millis,
    )
}

fn application_at_nanos<'a>(
    activation: &SigningKey,
    proof: &'a [u8],
    now_unix_seconds: i64,
    now_unix_nanos: u32,
    now_elapsed_millis: u64,
    clock_uncertainty_millis: u64,
) -> AuthorityApplication<'a> {
    AuthorityApplication {
        now_unix_seconds,
        now_unix_nanos,
        now_elapsed_millis,
        clock_uncertainty_millis,
        activation_public_key: activation.verifying_key().to_bytes(),
        activation_proof: proof,
    }
}

fn authorization<'a>(
    holder_guardian_id: &'a [u8],
    epoch: u64,
    now_elapsed_millis: u64,
    applied_log_index: u64,
    sequence: u64,
    mutation_sha256: [u8; 32],
    activation_proof: &'a [u8],
) -> MutationAuthorization<'a> {
    MutationAuthorization {
        lineage_id: LINEAGE,
        holder_guardian_id,
        epoch,
        now_elapsed_millis,
        applied_log_index,
        sequence,
        mutation_sha256,
        activation_proof,
    }
}

fn apply(
    ledger: &mut AuthorityLedger,
    fixture: &Fixture,
    body: AuthorityCertificateBodyV1,
    activation: &SigningKey,
    now: u64,
) -> Result<(), AuthorityError> {
    let proof = activation_signature(&body, activation);
    let certificate = fixture.certificate(body, &[0, 1]);
    ledger
        .apply(
            &certificate,
            &fixture.membership,
            application(activation, &proof, now, 5),
        )
        .map(|_| ())
}

#[test]
fn stable_and_joint_openraft_membership_enforce_exact_quorum() {
    let activation = activation(1);
    let stable = Fixture::stable();
    let stable_body = body(OperationClass::LeaseGrant, 10, 1, &activation);
    assert!(verify_certificate(
        &stable.certificate(stable_body.clone(), &[0, 1]),
        &stable.membership
    )
    .is_ok());
    assert_eq!(
        verify_certificate(&stable.certificate(stable_body, &[0]), &stable.membership),
        Err(AuthorityError::QuorumNotReached)
    );

    let joint = Fixture::joint();
    let joint_body = body(OperationClass::LeaseGrant, 11, 1, &activation);
    // Three of the five voters is a union majority, but {0,1,2} has only voter 2
    // in the new configuration and therefore cannot authorize joint consensus.
    assert_eq!(
        verify_certificate(
            &joint.certificate(joint_body.clone(), &[0, 1, 2]),
            &joint.membership
        ),
        Err(AuthorityError::QuorumNotReached)
    );
    assert!(verify_certificate(
        &joint.certificate(joint_body, &[0, 1, 2, 3]),
        &joint.membership
    )
    .is_ok());
}

#[test]
fn membership_rejects_small_sets_duplicate_keys_and_invalid_identity_bounds() {
    let key = signer(1).verifying_key().to_bytes();
    let ids = [b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
    let duplicate_key_voters = ids
        .iter()
        .map(|id| VoterAuthority {
            guardian_id: id.clone(),
            trust_domain_id: TRUST.to_vec(),
            certificate_generation: 1,
            purpose: ControlCertificatePurpose::AuthorityEndorsement,
            not_before_unix_seconds: NOW_UNIX_SECONDS - 1,
            not_after_unix_seconds: NOW_UNIX_SECONDS + 1,
            revoked: false,
            control_public_key: key,
        })
        .collect();
    assert_eq!(
        AuthorityMembership::new(
            TRUST.to_vec(),
            1,
            1,
            vec![ids.iter().cloned().collect()],
            duplicate_key_voters,
        ),
        Err(AuthorityError::DuplicateControlKey)
    );
    let fixture = Fixture::stable();
    assert_eq!(
        AuthorityMembership::new(
            vec![b'x'; 129],
            7,
            1,
            vec![fixture.ids.iter().cloned().collect()],
            fixture.membership.voters.values().cloned().collect(),
        ),
        Err(AuthorityError::InvalidMembership)
    );
}

#[test]
fn endorsements_bind_body_identity_generation_algorithm_and_strict_signature() {
    let fixture = Fixture::stable();
    let activation = activation(2);
    let body = body(OperationClass::LeaseGrant, 20, 1, &activation);
    let certificate = fixture.certificate(body.clone(), &[0, 1]);
    let verified = verify_certificate(&certificate, &fixture.membership).expect("valid quorum");
    assert_eq!(verified.body_sha256, certificate_body_sha256(&body));
    assert_eq!(verified.signer_guardian_ids.len(), 2);

    let mut decoded = AuthorityCertificateV1::decode(certificate.as_slice()).unwrap();
    decoded.endorsements[0].certificate_generation += 1;
    let drift = encode_certificate(&decoded).unwrap();
    assert_eq!(
        verify_certificate(&drift, &fixture.membership),
        Err(AuthorityError::StaleMembership)
    );

    decoded = AuthorityCertificateV1::decode(certificate.as_slice()).unwrap();
    decoded.endorsements[0].signature = vec![0xff; 64];
    let invalid_point_or_scalar = encode_certificate(&decoded).unwrap();
    assert_eq!(
        verify_certificate(&invalid_point_or_scalar, &fixture.membership),
        Err(AuthorityError::InvalidEndorsement)
    );

    decoded = AuthorityCertificateV1::decode(certificate.as_slice()).unwrap();
    decoded.endorsements[0].signing_algorithm = 2;
    let wrong_algorithm = encode_certificate(&decoded).unwrap();
    assert_eq!(
        verify_certificate(&wrong_algorithm, &fixture.membership),
        Err(AuthorityError::InvalidEndorsement)
    );
}

#[test]
fn voter_authorization_is_live_purpose_domain_expiry_and_revocation_bound() {
    let activation = activation(12);
    let body = body(OperationClass::LeaseGrant, 25, 1, &activation);
    let mut revoked = Fixture::stable();
    let certificate = revoked.certificate(body.clone(), &[0, 1]);
    revoked
        .membership
        .voters
        .get_mut(&revoked.ids[0])
        .unwrap()
        .revoked = true;
    assert_eq!(
        verify_certificate(&certificate, &revoked.membership),
        Err(AuthorityError::CertificateUnauthorized)
    );

    let mut wrong_purpose = Fixture::stable();
    let certificate = wrong_purpose.certificate(body.clone(), &[0, 1]);
    wrong_purpose
        .membership
        .voters
        .get_mut(&wrong_purpose.ids[0])
        .unwrap()
        .purpose = ControlCertificatePurpose::Transport;
    assert_eq!(
        verify_certificate(&certificate, &wrong_purpose.membership),
        Err(AuthorityError::CertificateUnauthorized)
    );

    let mut expired = Fixture::stable();
    let certificate = expired.certificate(body.clone(), &[0, 1]);
    expired
        .membership
        .voters
        .get_mut(&expired.ids[0])
        .unwrap()
        .not_after_unix_seconds = NOW_UNIX_SECONDS;
    assert_eq!(
        verify_certificate(&certificate, &expired.membership),
        Err(AuthorityError::CertificateUnauthorized)
    );

    let mut wrong_domain = Fixture::stable();
    let certificate = wrong_domain.certificate(body, &[0, 1]);
    wrong_domain
        .membership
        .voters
        .get_mut(&wrong_domain.ids[0])
        .unwrap()
        .trust_domain_id = b"other-domain".to_vec();
    assert_eq!(
        verify_certificate(&certificate, &wrong_domain.membership),
        Err(AuthorityError::CertificateUnauthorized)
    );
}

#[test]
fn copied_and_superseded_domain_signatures_never_authorize() {
    let fixture = Fixture::stable();
    let activation = activation(3);
    let body = body(OperationClass::LeaseGrant, 30, 1, &activation);
    let mut certificate =
        AuthorityCertificateV1::decode(fixture.certificate(body.clone(), &[0, 1]).as_slice())
            .unwrap();
    certificate.endorsements[0].signer_guardian_id = fixture.ids[2].clone();
    certificate
        .endorsements
        .sort_by(|a, b| a.signer_guardian_id.cmp(&b.signer_guardian_id));
    assert_eq!(
        verify_certificate(
            &encode_certificate(&certificate).unwrap(),
            &fixture.membership
        ),
        Err(AuthorityError::InvalidEndorsement)
    );

    let body_bytes = body.encode_to_vec();
    let mut legacy = Sha256::new();
    legacy.update(b"ADL-AUTHORITY-CERTIFICATE-V1\0");
    legacy.update(body_bytes);
    let legacy_signature = fixture.keys[0].sign(&<[u8; 32]>::from(legacy.finalize()));
    let mut endorsements = vec![
        AuthorityEndorsementV1 {
            signer_guardian_id: fixture.ids[0].clone(),
            certificate_generation: 7,
            signing_algorithm: SIGNING_ALGORITHM_ED25519,
            signature: legacy_signature.to_bytes().to_vec(),
        },
        endorse(&body, fixture.ids[1].clone(), 7, &fixture.keys[1]),
    ];
    endorsements.sort_by(|a, b| a.signer_guardian_id.cmp(&b.signer_guardian_id));
    let legacy = encode_certificate(&AuthorityCertificateV1 {
        body: Some(body),
        endorsements,
    })
    .unwrap();
    assert_eq!(
        verify_certificate(&legacy, &fixture.membership),
        Err(AuthorityError::InvalidEndorsement)
    );
}

#[test]
fn canonical_wire_rejects_unknown_duplicate_nonminimal_unsorted_and_truncated_data() {
    let fixture = Fixture::stable();
    let activation = activation(4);
    let body = body(OperationClass::LeaseGrant, 40, 1, &activation);
    let certificate = fixture.certificate(body.clone(), &[0, 1]);

    let mut unknown = certificate.clone();
    unknown.extend_from_slice(&[0x98, 0x06, 0x01]); // unknown top-level field 99
    assert_eq!(
        verify_certificate(&unknown, &fixture.membership),
        Err(AuthorityError::NonCanonicalEncoding)
    );

    let mut nonminimal_key = vec![0x8a, 0x00];
    nonminimal_key.extend_from_slice(&certificate[1..]);
    assert_eq!(
        verify_certificate(&nonminimal_key, &fixture.membership),
        Err(AuthorityError::NonCanonicalEncoding)
    );

    let mut truncated = certificate.clone();
    truncated.pop();
    assert!(verify_certificate(&truncated, &fixture.membership).is_err());

    let mut decoded = AuthorityCertificateV1::decode(certificate.as_slice()).unwrap();
    decoded.endorsements.reverse();
    assert_eq!(
        verify_certificate(&decoded.encode_to_vec(), &fixture.membership),
        Err(AuthorityError::NonCanonicalEncoding)
    );

    let duplicate = body_with_suffix_certificate(&fixture, body, &[0x08, 0x01]);
    assert_eq!(
        verify_certificate(&duplicate, &fixture.membership),
        Err(AuthorityError::NonCanonicalEncoding)
    );
}

#[test]
fn certificate_context_rejects_wrong_domain_generation_applied_index_and_operation() {
    let fixture = Fixture::stable();
    let activation = activation(5);
    let mut candidate = body(OperationClass::LeaseGrant, 50, 1, &activation);
    candidate.trust_domain_id = b"other-domain".to_vec();
    assert_eq!(
        verify_certificate(
            &fixture.certificate(candidate, &[0, 1]),
            &fixture.membership,
        ),
        Err(AuthorityError::WrongTrustDomain)
    );
    let mut candidate = body(OperationClass::LeaseGrant, 51, 1, &activation);
    candidate.voter_set_generation = 8;
    assert_eq!(
        verify_certificate(
            &fixture.certificate(candidate, &[0, 1]),
            &fixture.membership,
        ),
        Err(AuthorityError::StaleMembership)
    );
    let candidate = body(OperationClass::LeaseGrant, 1_001, 1, &activation);
    assert_eq!(
        verify_certificate(
            &fixture.certificate(candidate, &[0, 1]),
            &fixture.membership,
        ),
        Err(AuthorityError::StaleAppliedIndex)
    );
    let mut candidate = body(OperationClass::LeaseGrant, 52, 1, &activation);
    candidate.operation_class = 99;
    assert_eq!(
        verify_certificate(
            &fixture.certificate(candidate, &[0, 1]),
            &fixture.membership,
        ),
        Err(AuthorityError::InvalidOperationClass)
    );
    let mut exact_second = body(OperationClass::LeaseGrant, 53, 1, &activation);
    exact_second.issued_nanos = 0;
    assert!(verify_certificate(
        &fixture.certificate(exact_second, &[0, 1]),
        &fixture.membership,
    )
    .is_ok());
}

#[test]
fn ledger_serializes_grant_renewal_owner_commit_and_revocation() {
    let fixture = Fixture::stable();
    let activation = activation(6);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::LeaseGrant, 60, 1, &activation),
        &activation,
        100,
    )
    .unwrap();
    assert_eq!(ledger.lease(LINEAGE).unwrap().deadline_elapsed_millis, 200);
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::LeaseRenewal, 61, 1, &activation),
        &activation,
        150,
    )
    .unwrap();
    assert_eq!(ledger.lease(LINEAGE).unwrap().deadline_elapsed_millis, 250);
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::OwnerCommit, 62, 1, &activation),
        &activation,
        160,
    )
    .unwrap();
    assert_eq!(ledger.lease(LINEAGE).unwrap().deadline_elapsed_millis, 250);
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::Revoke, 63, 1, &activation),
        &activation,
        170,
    )
    .unwrap();
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 170, 63, 1, [1; 32], &[])),
        Err(AuthorityError::LeaseRevoked)
    );
    assert_eq!(ledger.applied_log_index(), 63);
}

#[test]
fn activation_possession_epoch_safety_and_clock_bounds_fail_closed() {
    let fixture = Fixture::stable();
    let first = activation(7);
    let clone = activation(8);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::LeaseGrant, 70, 1, &first),
        &first,
        100,
    )
    .unwrap();

    let copied = body(OperationClass::LeaseRenewal, 71, 1, &first);
    let copied_certificate = fixture.certificate(copied.clone(), &[0, 1]);
    let copied_proof = activation_signature(&copied, &clone);
    assert_eq!(
        ledger.apply(
            &copied_certificate,
            &fixture.membership,
            application(&clone, &copied_proof, 150, 5),
        ),
        Err(AuthorityError::ActivationPossession)
    );

    let epoch_two = body(OperationClass::Activate, 71, 2, &clone);
    let epoch_two_certificate = fixture.certificate(epoch_two.clone(), &[0, 1]);
    let epoch_two_proof = activation_signature(&epoch_two, &clone);
    assert_eq!(
        ledger.apply(
            &epoch_two_certificate,
            &fixture.membership,
            application(&clone, &epoch_two_proof, 214, 5),
        ),
        Err(AuthorityError::LeaseExpired)
    );
    let stale_epoch = body(OperationClass::Activate, 71, 1, &clone);
    let stale_epoch_proof = activation_signature(&stale_epoch, &clone);
    assert_eq!(
        ledger.apply(
            &fixture.certificate(stale_epoch, &[0, 1]),
            &fixture.membership,
            application(&clone, &stale_epoch_proof, 215, 5),
        ),
        Err(AuthorityError::StaleEpoch)
    );
    let epoch_gap = body(OperationClass::Activate, 71, 3, &clone);
    let epoch_gap_proof = activation_signature(&epoch_gap, &clone);
    assert_eq!(
        ledger.apply(
            &fixture.certificate(epoch_gap, &[0, 1]),
            &fixture.membership,
            application(&clone, &epoch_gap_proof, 215, 5),
        ),
        Err(AuthorityError::EpochGap)
    );
    ledger
        .apply(
            &epoch_two_certificate,
            &fixture.membership,
            application(&clone, &epoch_two_proof, 215, 5),
        )
        .unwrap();

    let uncertain = body(OperationClass::LeaseRenewal, 72, 2, &clone);
    let uncertain_certificate = fixture.certificate(uncertain.clone(), &[0, 1]);
    let uncertain_proof = activation_signature(&uncertain, &clone);
    assert_eq!(
        ledger.apply(
            &uncertain_certificate,
            &fixture.membership,
            application(&clone, &uncertain_proof, 216, 11),
        ),
        Err(AuthorityError::ClockUncertain)
    );
}

#[test]
fn policy_digest_and_future_issuance_are_rejected_before_state_mutation() {
    let fixture = Fixture::stable();
    let activation = activation(13);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    let mut wrong_policy = body(OperationClass::LeaseGrant, 75, 1, &activation);
    wrong_policy.policy_sha256 = vec![0x55; 32];
    let proof = activation_signature(&wrong_policy, &activation);
    let certificate = fixture.certificate(wrong_policy, &[0, 1]);
    assert_eq!(
        ledger.apply(
            &certificate,
            &fixture.membership,
            application(&activation, &proof, 100, 5),
        ),
        Err(AuthorityError::PolicyMismatch)
    );
    let mut future = body(OperationClass::LeaseGrant, 76, 1, &activation);
    future.issued_unix_seconds = NOW_UNIX_SECONDS + 1;
    let proof = activation_signature(&future, &activation);
    let certificate = fixture.certificate(future, &[0, 1]);
    assert_eq!(
        ledger.apply(
            &certificate,
            &fixture.membership,
            application(&activation, &proof, 100, 5),
        ),
        Err(AuthorityError::InvalidCertificate)
    );
    let mut same_second_future = body(OperationClass::LeaseGrant, 77, 1, &activation);
    same_second_future.issued_nanos = 900_000_000;
    let proof = activation_signature(&same_second_future, &activation);
    let certificate = fixture.certificate(same_second_future, &[0, 1]);
    assert_eq!(
        ledger.apply(
            &certificate,
            &fixture.membership,
            application_at_nanos(&activation, &proof, NOW_UNIX_SECONDS, 100_000_000, 100, 5,),
        ),
        Err(AuthorityError::InvalidCertificate)
    );
    assert_eq!(ledger.applied_log_index(), 0);
}

#[test]
fn stale_replay_quorum_loss_and_malicious_minority_cannot_mutate() {
    let fixture = Fixture::stable();
    let activation = activation(9);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    let grant = body(OperationClass::LeaseGrant, 80, 1, &activation);
    apply(&mut ledger, &fixture, grant.clone(), &activation, 100).unwrap();
    let certificate = fixture.certificate(grant.clone(), &[0, 1]);
    assert_eq!(
        ledger.apply(
            &certificate,
            &fixture.membership,
            application(
                &activation,
                &activation_signature(&grant, &activation),
                101,
                5,
            ),
        ),
        Err(AuthorityError::Replay)
    );

    let renewal = body(OperationClass::LeaseRenewal, 81, 1, &activation);
    let minority_certificate = fixture.certificate(renewal.clone(), &[0]);
    assert_eq!(
        ledger.apply(
            &minority_certificate,
            &fixture.membership,
            application(
                &activation,
                &activation_signature(&renewal, &activation),
                150,
                5,
            ),
        ),
        Err(AuthorityError::QuorumNotReached)
    );
    assert_eq!(ledger.applied_log_index(), 80);
}

#[test]
fn mutation_sink_enforces_holder_epoch_deadline_and_applied_index() {
    let fixture = Fixture::stable();
    let owner_activation = activation(10);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::LeaseGrant, 90, 1, &owner_activation),
        &owner_activation,
        100,
    )
    .unwrap();
    let mutation = Sha256::digest(b"mutation-one").into();
    let lease = ledger.lease(LINEAGE).unwrap().clone();
    let clone = activation(15);
    let clone_proof = mutation_signature(&lease, 90, 1, mutation, &clone);
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 199, 90, 1, mutation, &clone_proof,)),
        Err(AuthorityError::ActivationPossession)
    );
    let proof = mutation_signature(&lease, 90, 1, mutation, &owner_activation);
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 199, 90, 1, mutation, &proof)),
        Ok(())
    );
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 199, 90, 1, mutation, &proof)),
        Err(AuthorityError::Replay)
    );
    assert_eq!(
        ledger.authorize_mutation(authorization(b"other", 1, 199, 90, 2, [2; 32], &[])),
        Err(AuthorityError::HolderMismatch)
    );
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 199, 89, 2, [2; 32], &[])),
        Err(AuthorityError::StaleAppliedIndex)
    );
    assert_eq!(
        ledger.authorize_mutation(authorization(HOLDER, 1, 200, 90, 2, [2; 32], &[])),
        Err(AuthorityError::LeaseExpired)
    );
}

#[test]
fn canonical_snapshot_restores_exact_state_and_rejects_tamper_and_bounds() {
    let mut fixture = Fixture::stable();
    let initial_activation = activation(11);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    apply(
        &mut ledger,
        &fixture,
        body(OperationClass::LeaseGrant, 100, 1, &initial_activation),
        &initial_activation,
        100,
    )
    .unwrap();
    let snapshot = ledger.snapshot().unwrap();
    let mut unproved_prefix_membership = fixture.membership.clone();
    unproved_prefix_membership.committed_log_index = 101;
    assert_eq!(
        AuthorityLedger::restore(
            policy(),
            &snapshot,
            &unproved_prefix_membership,
            NOW_UNIX_SECONDS,
        )
        .unwrap_err(),
        AuthorityError::SnapshotCorrupt
    );
    fixture.membership.committed_log_index = 100;
    let mut mismatched_policy = policy();
    mismatched_policy.max_clock_uncertainty_millis = 9;
    assert_eq!(
        AuthorityLedger::restore(
            mismatched_policy,
            &snapshot,
            &fixture.membership,
            NOW_UNIX_SECONDS,
        )
        .unwrap_err(),
        AuthorityError::SnapshotCorrupt
    );
    let mut restored =
        AuthorityLedger::restore(policy(), &snapshot, &fixture.membership, NOW_UNIX_SECONDS)
            .unwrap();
    assert_eq!(restored.applied_log_index(), 100);
    assert!(restored.lease(LINEAGE).unwrap().revoked);
    assert_eq!(
        restored.authorize_mutation(authorization(HOLDER, 1, 101, 100, 1, [1; 32], &[])),
        Err(AuthorityError::LeaseRevoked)
    );
    let replacement = activation(14);
    let mut replacement_body = body(OperationClass::Activate, 101, 2, &replacement);
    replacement_body.issued_unix_seconds = NOW_UNIX_SECONDS + 1;
    let replacement_proof = activation_signature(&replacement_body, &replacement);
    fixture.membership.committed_log_index = 101;
    restored
        .apply(
            &fixture.certificate(replacement_body, &[0, 1]),
            &fixture.membership,
            application_at(
                &replacement,
                &replacement_proof,
                NOW_UNIX_SECONDS + 1,
                215,
                5,
            ),
        )
        .unwrap();
    let mutation = Sha256::digest(b"post-recovery-mutation").into();
    let proof = mutation_signature(
        restored.lease(LINEAGE).unwrap(),
        101,
        1,
        mutation,
        &replacement,
    );
    assert_eq!(
        restored.authorize_mutation(authorization(HOLDER, 2, 216, 101, 1, mutation, &proof)),
        Ok(())
    );

    let mut tampered = snapshot;
    let offset = tampered.len() / 2;
    tampered[offset] ^= 1;
    assert!(
        AuthorityLedger::restore(policy(), &tampered, &fixture.membership, NOW_UNIX_SECONDS,)
            .is_err()
    );
    let mut too_small = policy();
    too_small.max_snapshot_bytes = 1024;
    assert!(AuthorityLedger::restore(
        too_small,
        &vec![b'x'; 1025],
        &fixture.membership,
        NOW_UNIX_SECONDS,
    )
    .is_err());
}

#[test]
fn restart_uses_portable_wall_safety_anchor_not_foreign_elapsed_clock() {
    let mut fixture = Fixture::stable();
    let initial = activation(16);
    let portable_policy = policy_with_max_lease(2_000);
    let mut grant = body(OperationClass::LeaseGrant, 110, 1, &initial);
    grant.issued_unix_seconds = NOW_UNIX_SECONDS;
    grant.policy_sha256 = portable_policy.sha256().unwrap().to_vec();
    let proof = activation_signature(&grant, &initial);
    let certificate = fixture.certificate(grant, &[0, 1]);
    let mut ledger = AuthorityLedger::new(portable_policy.clone()).unwrap();
    ledger
        .apply(
            &certificate,
            &fixture.membership,
            application(&initial, &proof, 100, 5),
        )
        .unwrap();
    let snapshot = ledger.snapshot().unwrap();
    fixture.membership.committed_log_index = 110;
    let mut restored = AuthorityLedger::restore(
        portable_policy.clone(),
        &snapshot,
        &fixture.membership,
        NOW_UNIX_SECONDS,
    )
    .unwrap();
    let replacement = activation(17);
    let mut replacement_body = body(OperationClass::Activate, 111, 2, &replacement);
    replacement_body.lease_duration_millis = 2_000;
    replacement_body.policy_sha256 = portable_policy.sha256().unwrap().to_vec();
    let replacement_proof = activation_signature(&replacement_body, &replacement);
    let replacement_certificate = fixture.certificate(replacement_body, &[0, 1]);
    fixture.membership.committed_log_index = 111;
    assert_eq!(
        restored.apply(
            &replacement_certificate,
            &fixture.membership,
            application_at(
                &replacement,
                &replacement_proof,
                NOW_UNIX_SECONDS,
                10_000_000,
                5,
            ),
        ),
        Err(AuthorityError::LeaseExpired)
    );
    restored
        .apply(
            &replacement_certificate,
            &fixture.membership,
            application_at(&replacement, &replacement_proof, NOW_UNIX_SECONDS + 1, 1, 5),
        )
        .unwrap();
}

#[test]
fn delayed_apply_never_extends_the_signed_portable_lease_deadline() {
    let mut fixture = Fixture::stable();
    let initial = activation(18);
    let delayed_policy = policy_with_max_lease(10_000);
    let mut grant = body(OperationClass::LeaseGrant, 120, 1, &initial);
    grant.lease_duration_millis = 5_000;
    grant.policy_sha256 = delayed_policy.sha256().unwrap().to_vec();
    let proof = activation_signature(&grant, &initial);
    let certificate = fixture.certificate(grant, &[0, 1]);
    let mut ledger = AuthorityLedger::new(delayed_policy.clone()).unwrap();
    ledger
        .apply(
            &certificate,
            &fixture.membership,
            application_at(&initial, &proof, NOW_UNIX_SECONDS + 3, 1_000, 5),
        )
        .unwrap();
    assert_eq!(
        ledger.lease(LINEAGE).unwrap().deadline_elapsed_millis,
        3_000
    );

    let snapshot = ledger.snapshot().unwrap();
    fixture.membership.committed_log_index = 120;
    let mut restored = AuthorityLedger::restore(
        delayed_policy.clone(),
        &snapshot,
        &fixture.membership,
        NOW_UNIX_SECONDS + 3,
    )
    .unwrap();
    let replacement = activation(19);
    let mut replacement_body = body(OperationClass::Activate, 121, 2, &replacement);
    replacement_body.issued_unix_seconds = NOW_UNIX_SECONDS + 4;
    replacement_body.lease_duration_millis = 5_000;
    replacement_body.policy_sha256 = delayed_policy.sha256().unwrap().to_vec();
    let replacement_proof = activation_signature(&replacement_body, &replacement);
    let replacement_certificate = fixture.certificate(replacement_body, &[0, 1]);
    fixture.membership.committed_log_index = 121;
    assert_eq!(
        restored.apply(
            &replacement_certificate,
            &fixture.membership,
            application_at(
                &replacement,
                &replacement_proof,
                NOW_UNIX_SECONDS + 4,
                10_000_000,
                5,
            ),
        ),
        Err(AuthorityError::LeaseExpired)
    );
    restored
        .apply(
            &replacement_certificate,
            &fixture.membership,
            application_at(&replacement, &replacement_proof, NOW_UNIX_SECONDS + 6, 1, 5),
        )
        .unwrap();
}

#[test]
fn mid_second_expired_certificate_cannot_gain_local_elapsed_authority() {
    let fixture = Fixture::stable();
    let activation = activation(20);
    let mut grant = body(OperationClass::LeaseGrant, 130, 1, &activation);
    grant.issued_nanos = 0;
    grant.lease_duration_millis = 500;
    let proof = activation_signature(&grant, &activation);
    let certificate = fixture.certificate(grant, &[0, 1]);
    let mut ledger = AuthorityLedger::new(policy()).unwrap();
    assert_eq!(
        ledger.apply(
            &certificate,
            &fixture.membership,
            application_at_nanos(&activation, &proof, NOW_UNIX_SECONDS, 900_000_000, 10, 5,),
        ),
        Err(AuthorityError::LeaseExpired)
    );
    assert_eq!(ledger.applied_log_index(), 0);
}

fn body_with_suffix_certificate(
    fixture: &Fixture,
    body: AuthorityCertificateBodyV1,
    suffix: &[u8],
) -> Vec<u8> {
    let canonical = fixture.certificate(body, &[0, 1]);
    assert_eq!(canonical[0], 0x0a);
    let (body_len, length_bytes) = read_varint(&canonical[1..]);
    let body_start = 1 + length_bytes;
    let body_end = body_start + body_len as usize;
    let new_len = body_len + suffix.len() as u64;
    let mut result = vec![0x0a];
    write_varint(new_len, &mut result);
    result.extend_from_slice(&canonical[body_start..body_end]);
    result.extend_from_slice(suffix);
    result.extend_from_slice(&canonical[body_end..]);
    result
}

fn read_varint(bytes: &[u8]) -> (u64, usize) {
    let mut value = 0;
    for (index, byte) in bytes.iter().enumerate() {
        value |= u64::from(byte & 0x7f) << (index * 7);
        if byte & 0x80 == 0 {
            return (value, index + 1);
        }
    }
    panic!("truncated fixture varint")
}

fn write_varint(mut value: u64, bytes: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        bytes.push(byte);
        if value == 0 {
            return;
        }
    }
}
