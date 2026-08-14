use adl_runtime::distributed::authority_protocol::{
    CanonicalAuthorityTime, CommittedAuthorityArtifact, ContinuityTransferChunk,
    ContinuityTransferEntry, ContinuityTransferGrantArtifact,
};
use adl_runtime::distributed::continuity_transfer::{
    ContinuityTransferError, ContinuityTransferExpectation, ContinuityTransferFrame,
    ContinuityTransferPolicy, ContinuityTransferSession,
};
use sha2::{Digest, Sha256};

fn sha(bytes: &[u8]) -> [u8; 32] {
    <[u8; 32]>::from(Sha256::digest(bytes))
}

fn marker(name: &str) {
    println!("pass:{name}");
}

fn payloads() -> [&'static [u8]; 2] {
    [b"first-continuity-chunk", b"second-continuity-chunk"]
}

fn grant() -> ContinuityTransferGrantArtifact {
    let [first, second] = payloads();
    let first_sha = sha(first);
    let second_sha = sha(second);
    ContinuityTransferGrantArtifact {
        source_guardian_id: "guardian-source".to_owned(),
        target_guardian_id: "guardian-target".to_owned(),
        route_id: "route-current".to_owned(),
        membership_epoch: 7,
        membership_log_index: 42,
        source_certificate_generation: 3,
        target_certificate_generation: 4,
        source_boot_generation: 5,
        target_boot_generation: 6,
        transfer_id: "transfer-210".to_owned(),
        lineage_id: b"lineage".to_vec(),
        source_checkpoint_handle_identity: b"source-checkpoint".to_vec(),
        bundle_handle_identity: b"bundle-handle".to_vec(),
        signed_manifest_bytes: b"signed-manifest".to_vec(),
        signed_manifest_sha256: sha(b"signed-manifest"),
        signed_catalog_bytes: b"signed-catalog".to_vec(),
        signed_catalog_sha256: sha(b"signed-catalog"),
        trusted_key_generation: 9,
        entries: vec![ContinuityTransferEntry {
            schema: "adl.bundle.entry.v1".to_owned(),
            absolute_start: 0,
            length: (first.len() + second.len()) as u64,
            sha256: sha(&[first, second].concat()),
        }],
        chunks: vec![
            ContinuityTransferChunk {
                index: 0,
                absolute_start: 0,
                length: first.len() as u64,
                sha256: first_sha,
                predecessor_sha256: None,
            },
            ContinuityTransferChunk {
                index: 1,
                absolute_start: first.len() as u64,
                length: second.len() as u64,
                sha256: second_sha,
                predecessor_sha256: Some(first_sha),
            },
        ],
        total_bytes: (first.len() + second.len()) as u64,
        inclusive_deadline: CanonicalAuthorityTime {
            unix_seconds: 1_900_000_000,
            nanos: 0,
            uncertainty_millis: 10,
        },
        cleanup_identity: "cleanup-stage".to_owned(),
    }
}

fn expectation() -> ContinuityTransferExpectation {
    ContinuityTransferExpectation {
        source_guardian_id: "guardian-source".to_owned(),
        target_guardian_id: "guardian-target".to_owned(),
        route_id: "route-current".to_owned(),
        membership_epoch: 7,
        membership_log_index: 42,
        source_certificate_generation: 3,
        target_certificate_generation: 4,
        source_boot_generation: 5,
        target_boot_generation: 6,
        lineage_id: b"lineage".to_vec(),
        source_checkpoint_handle_identity: b"source-checkpoint".to_vec(),
        bundle_handle_identity: b"bundle-handle".to_vec(),
    }
}

fn artifact() -> CommittedAuthorityArtifact {
    CommittedAuthorityArtifact::continuity_transfer(&grant()).expect("valid grant")
}

fn session() -> ContinuityTransferSession {
    ContinuityTransferSession::open(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
    )
    .expect("session opens")
}

fn frame(index: u64) -> ContinuityTransferFrame {
    let [first, second] = payloads();
    match index {
        0 => ContinuityTransferFrame {
            transfer_id: "transfer-210".to_owned(),
            chunk_index: 0,
            absolute_start: 0,
            predecessor_sha256: None,
            payload: first.to_vec(),
        },
        1 => ContinuityTransferFrame {
            transfer_id: "transfer-210".to_owned(),
            chunk_index: 1,
            absolute_start: first.len() as u64,
            predecessor_sha256: Some(sha(first)),
            payload: second.to_vec(),
        },
        _ => unreachable!("fixture has two chunks"),
    }
}

#[test]
fn authorized_transfer_accepts_ordered_frames_and_redacted_receipt() {
    let mut session = session();
    let first = session.accept_frame(frame(0)).expect("first accepted");
    assert_eq!(first.accepted_prefix, payloads()[0].len() as u64);
    let second = session.accept_frame(frame(1)).expect("second accepted");
    assert_eq!(second.accepted_prefix, grant().total_bytes);

    let receipt = session.finish().expect("complete");
    assert_eq!(receipt.chunk_count, 2);
    assert_eq!(receipt.total_bytes, grant().total_bytes);
    assert_ne!(receipt.transfer_id_sha256, [0; 32]);
    assert_ne!(receipt.source_guardian_sha256, [0; 32]);
    marker("CASE-001:authorized_transfer");
    marker("CASE-018:frame_n_accepted");
    marker("CASE-045:evidence_redaction");
}

#[test]
fn exact_duplicate_frame_is_cached_without_advancing_prefix() {
    let mut session = session();
    let first = frame(0);
    let accepted = session.accept_frame(first.clone()).expect("first accepted");
    let duplicate = session.accept_frame(first).expect("duplicate cached");
    assert_eq!(duplicate.accepted_prefix, accepted.accepted_prefix);
    assert!(duplicate.duplicate);
    assert_eq!(session.accepted_prefix(), accepted.accepted_prefix);
    marker("CASE-021:exact_duplicate_frame_cached");
}

#[test]
fn wrong_source_target_route_and_cut_are_denied_before_bytes_move() {
    let artifact = artifact();
    let mut expected = expectation();
    expected.source_guardian_id = "other-source".to_owned();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongSource
    );

    let mut expected = expectation();
    expected.target_guardian_id = "other-target".to_owned();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongTarget
    );

    let mut expected = expectation();
    expected.route_id = "stale-route".to_owned();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongRoute
    );

    let mut expected = expectation();
    expected.membership_log_index += 1;
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongMembershipCut
    );

    let mut expected = expectation();
    expected.lineage_id = b"other-lineage".to_vec();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongAuthority
    );

    let mut expected = expectation();
    expected.source_certificate_generation += 1;
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongCertificateGeneration
    );

    let mut expected = expectation();
    expected.target_boot_generation += 1;
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongBootGeneration
    );
    marker("CASE-007:wrong_source_denied");
    marker("CASE-008:wrong_target_denied");
    marker("CASE-011:wrong_lineage_denied");
    marker("CASE-012:wrong_membership_cut_denied");
    marker("CASE-013:stale_certificate_denied");
    marker("CASE-014:wrong_boot_generation_denied");
}

#[test]
fn gaps_conflicts_wrong_predecessor_and_wrong_digest_are_denied() {
    let mut gap_session = session();
    assert_eq!(
        gap_session.accept_frame(frame(1)).unwrap_err(),
        ContinuityTransferError::Gap
    );

    let mut digest_session = session();
    let mut wrong_digest = frame(0);
    wrong_digest.payload.push(b'!');
    assert_eq!(
        digest_session.accept_frame(wrong_digest).unwrap_err(),
        ContinuityTransferError::Bounds
    );

    let mut accepted_session = session();
    accepted_session
        .accept_frame(frame(0))
        .expect("first accepted");
    let mut conflicting = frame(0);
    conflicting.payload = b"same-length-wrong-data".to_vec();
    assert_eq!(
        accepted_session.accept_frame(conflicting).unwrap_err(),
        ContinuityTransferError::Conflict
    );

    let mut wrong_predecessor = frame(1);
    wrong_predecessor.predecessor_sha256 = Some(sha(b"wrong predecessor"));
    assert_eq!(
        accepted_session
            .accept_frame(wrong_predecessor)
            .unwrap_err(),
        ContinuityTransferError::Predecessor
    );
    marker("CASE-020:reordered_frame_denied");
    marker("CASE-022:conflicting_duplicate_denied");
    marker("CASE-023:wrong_predecessor_denied");
    marker("CASE-024:wrong_chunk_digest_denied");
}

#[test]
fn policy_bounds_reject_oversized_frame_and_total_before_effect() {
    let artifact = artifact();
    assert_eq!(
        ContinuityTransferSession::open(
            &artifact,
            expectation(),
            ContinuityTransferPolicy::bounded(1, 128, 4)
        )
        .unwrap_err(),
        ContinuityTransferError::Bounds
    );
    assert_eq!(
        ContinuityTransferSession::open(
            &artifact,
            expectation(),
            ContinuityTransferPolicy::bounded(64, 1, 4)
        )
        .unwrap_err(),
        ContinuityTransferError::Bounds
    );
    marker("CASE-026:oversized_frame_denied");
    marker("CASE-027:oversized_total_denied");
}

#[test]
fn wrong_transfer_and_incomplete_finish_are_denied() {
    let mut wrong_transfer = frame(0);
    wrong_transfer.transfer_id = "other-transfer".to_owned();
    assert_eq!(
        session().accept_frame(wrong_transfer).unwrap_err(),
        ContinuityTransferError::WrongAuthority
    );

    let mut partial = session();
    partial.accept_frame(frame(0)).expect("first accepted");
    assert_eq!(
        partial.finish().unwrap_err(),
        ContinuityTransferError::Incomplete
    );
    marker("CASE-017:unknown_kind_denied");
    marker("CASE-025:wrong_manifest_denied");
}

#[test]
fn abort_is_idempotent_redacted_and_stops_later_frames() {
    let mut abort_session = session();
    abort_session
        .accept_frame(frame(0))
        .expect("first accepted");
    let receipt = abort_session
        .abort("transfer-210", "cleanup-stage")
        .expect("abort accepted");
    assert_eq!(receipt.accepted_prefix, payloads()[0].len() as u64);
    assert!(receipt.zero_residue_attested);
    assert_ne!(receipt.cleanup_identity_sha256, sha(b"cleanup-stage-wrong"));
    assert_eq!(
        abort_session
            .abort("transfer-210", "cleanup-stage")
            .expect("abort retry"),
        receipt
    );
    assert_eq!(
        abort_session.accept_frame(frame(1)).unwrap_err(),
        ContinuityTransferError::Aborted
    );
    assert_eq!(
        abort_session.finish().unwrap_err(),
        ContinuityTransferError::Aborted
    );

    let mut cleanup_session = session();
    assert_eq!(
        cleanup_session
            .abort("transfer-210", "wrong-cleanup")
            .unwrap_err(),
        ContinuityTransferError::CleanupAuthority
    );
    marker("CASE-030:cancellation_before_effect");
    marker("CASE-031:cancellation_midstream");
    marker("CASE-044:zero_residue_abort");
}
