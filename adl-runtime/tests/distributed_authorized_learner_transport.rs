use std::net::SocketAddr;

use adl_runtime::distributed::{
    authority_protocol::AuthorityOperationKind,
    learner_transport::{LearnerIdentity, LearnerMembershipArtifact, LearnerTransportError},
};
use ed25519_dalek::SigningKey;

const CUT: [u8; 32] = [7; 32];
const MEMBERSHIP: [u8; 32] = [8; 32];

fn identity() -> LearnerIdentity {
    LearnerIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-4".to_owned(),
        guardian_id: "guardian-4".to_owned(),
        guardian_control_public_key: SigningKey::from_bytes(&[44; 32]).verifying_key().to_bytes(),
        stable_raft_id: 4,
        certificate_generation: 4,
        boot_generation: 9,
        address: "127.0.0.1:4404".parse::<SocketAddr>().unwrap(),
    }
}

#[test]
fn public_enrollment_is_canonical_membership_artifact() {
    let first = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        2_000,
    )
    .unwrap();
    let second = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        2_000,
    )
    .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.domain, "adl.authority-artifact.membership.v1");
}

#[test]
fn public_removal_is_distinct_canonical_membership_artifact() {
    let enroll = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        2_000,
    )
    .unwrap();
    let remove = LearnerMembershipArtifact::remove_voter(
        identity(),
        CUT,
        MEMBERSHIP,
        2_000,
        "operator_remove",
    )
    .unwrap();
    assert_eq!(remove.domain, "adl.authority-artifact.membership.v1");
    assert_ne!(enroll.sha256, remove.sha256);
}

#[test]
fn artifact_kind_remains_coarse_membership() {
    let artifact = LearnerMembershipArtifact::enroll_non_voting(
        identity(),
        CUT,
        None,
        MEMBERSHIP,
        None,
        2_000,
    )
    .unwrap();
    let rebuilt = adl_runtime::distributed::authority_protocol::CommittedAuthorityArtifact::new(
        AuthorityOperationKind::Membership,
        artifact.bytes.clone(),
    )
    .unwrap();
    assert_eq!(artifact, rebuilt);
}

macro_rules! invalid_identity_case {
    ($name:ident, $mutate:expr) => {
        #[test]
        fn $name() {
            let mut value = identity();
            $mutate(&mut value);
            assert_eq!(
                LearnerMembershipArtifact::enroll_non_voting(
                    value, CUT, None, MEMBERSHIP, None, 2_000,
                ),
                Err(LearnerTransportError::InvalidBinding)
            );
        }
    };
}

invalid_identity_case!(public_empty_domain_denied, |value: &mut LearnerIdentity| {
    value.trust_domain.clear()
});
invalid_identity_case!(public_empty_polis_denied, |value: &mut LearnerIdentity| {
    value.polis_id.clear()
});
invalid_identity_case!(public_empty_node_denied, |value: &mut LearnerIdentity| {
    value.node_id.clear()
});
invalid_identity_case!(
    public_empty_guardian_denied,
    |value: &mut LearnerIdentity| value.guardian_id.clear()
);
invalid_identity_case!(public_zero_raft_id_denied, |value: &mut LearnerIdentity| {
    value.stable_raft_id = 0
});
invalid_identity_case!(
    public_zero_certificate_generation_denied,
    |value: &mut LearnerIdentity| value.certificate_generation = 0
);
invalid_identity_case!(
    public_zero_boot_generation_denied,
    |value: &mut LearnerIdentity| value.boot_generation = 0
);

#[test]
fn public_zero_cut_digest_denied() {
    assert_eq!(
        LearnerMembershipArtifact::enroll_non_voting(
            identity(),
            [0; 32],
            None,
            MEMBERSHIP,
            None,
            2_000,
        ),
        Err(LearnerTransportError::InvalidBinding)
    );
}

#[test]
fn public_zero_membership_digest_denied() {
    assert_eq!(
        LearnerMembershipArtifact::enroll_non_voting(identity(), CUT, None, [0; 32], None, 2_000,),
        Err(LearnerTransportError::InvalidBinding)
    );
}

#[test]
fn public_invalid_deadline_denied() {
    assert_eq!(
        LearnerMembershipArtifact::remove_voter(identity(), CUT, MEMBERSHIP, 0, "operator_remove",),
        Err(LearnerTransportError::InvalidBinding)
    );
}
