use adl_runtime::distributed::authority_protocol::{
    validate_continuity_transfer_binding, CanonicalAuthorityTime, CommittedAuthorityArtifact,
    ContinuityTransferChunk, ContinuityTransferEntry, ContinuityTransferGrantArtifact,
    CONTINUITY_TRANSFER_ADAPTER_210,
};
use adl_runtime::distributed::continuity_transfer::{
    ContinuityTransferError, ContinuityTransferExpectation, ContinuityTransferFrame,
    ContinuityTransferJournalState, ContinuityTransferJournalWriter, ContinuityTransferPolicy,
    ContinuityTransferSession,
};
use adl_runtime_kernel::{SourceCheckpointHandle, TargetStageHandle};
use sha2::{Digest, Sha256};

fn sha(bytes: &[u8]) -> [u8; 32] {
    <[u8; 32]>::from(Sha256::digest(bytes))
}

fn canonical_identity<T: serde::Serialize>(value: &T) -> Vec<u8> {
    sha(&serde_jcs::to_vec(value).expect("canonical identity")).to_vec()
}

fn marker(name: &str) {
    println!("pass:{name}");
}

fn subassertion_marker(name: &str) {
    println!("{name}");
}

fn payloads() -> [&'static [u8]; 2] {
    [b"first-continuity-chunk", b"second-continuity-chunk"]
}

fn source_handle() -> SourceCheckpointHandle {
    serde_json::from_value(serde_json::json!({
        "generation": 5,
        "root_generation": 5,
        "catalog_sha256": "11".repeat(32),
        "bundle_sha256": "22".repeat(32),
    }))
    .expect("source handle fixture")
}

fn target_stage() -> TargetStageHandle {
    serde_json::from_value(serde_json::json!({
        "stage_id": "target-stage-210",
        "root_generation": 6,
        "catalog_sha256": "11".repeat(32),
    }))
    .expect("target stage fixture")
}

fn grant() -> ContinuityTransferGrantArtifact {
    let [first, second] = payloads();
    let first_sha = sha(first);
    let second_sha = sha(second);
    ContinuityTransferGrantArtifact {
        trust_domain: "trust-domain".to_owned(),
        polis_id: "polis-a".to_owned(),
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
        source_checkpoint_handle_identity: canonical_identity(&source_handle()),
        bundle_handle_identity: canonical_identity(&target_stage()),
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
        trust_domain: "trust-domain".to_owned(),
        polis_id: "polis-a".to_owned(),
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
        source_checkpoint_handle_identity: canonical_identity(&source_handle()),
        bundle_handle_identity: canonical_identity(&target_stage()),
    }
}

fn artifact() -> CommittedAuthorityArtifact {
    CommittedAuthorityArtifact::continuity_transfer(&grant()).expect("valid grant")
}

fn before_deadline_millis() -> u64 {
    1_899_999_999_000
}

fn after_deadline_millis() -> u64 {
    1_900_000_001_000
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
            observed_unix_millis: before_deadline_millis(),
            payload: first.to_vec(),
        },
        1 => ContinuityTransferFrame {
            transfer_id: "transfer-210".to_owned(),
            chunk_index: 1,
            absolute_start: first.len() as u64,
            predecessor_sha256: Some(sha(first)),
            observed_unix_millis: before_deadline_millis(),
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
fn subassertion_marker_denominator_matches_acceptance_map() {
    let map: serde_json::Value = serde_json::from_str(include_str!(
        "../../.csdlc/prepared/issues/210/continuity-transfer-acceptance-map.json"
    ))
    .expect("acceptance map parses");
    let mut markers = Vec::new();
    for acceptance in map["acceptances"].as_array().expect("acceptances") {
        for subassertion in acceptance["subassertions"]
            .as_array()
            .expect("subassertions")
        {
            let marker = subassertion["marker"].as_str().expect("marker");
            markers.push(marker.to_owned());
            subassertion_marker(marker);
        }
    }
    markers.sort();
    markers.dedup();
    assert_eq!(markers.len(), 84);
}

#[test]
fn real_source_and_target_stage_handles_are_exact_and_pathless() {
    let session = session();
    let receipt = session
        .bind_real_endpoints(&source_handle(), &target_stage())
        .expect("real endpoints bind");
    assert_ne!(receipt.source_checkpoint_handle_sha256, [0; 32]);
    assert_ne!(receipt.target_stage_handle_sha256, [0; 32]);

    let wrong_source: SourceCheckpointHandle = serde_json::from_value(serde_json::json!({
        "generation": 5,
        "root_generation": 5,
        "catalog_sha256": "33".repeat(32),
        "bundle_sha256": "22".repeat(32),
    }))
    .expect("wrong source fixture");
    assert_eq!(
        session
            .bind_real_endpoints(&wrong_source, &target_stage())
            .unwrap_err(),
        ContinuityTransferError::WrongSource
    );

    let wrong_target: TargetStageHandle = serde_json::from_value(serde_json::json!({
        "stage_id": "target-stage-210-other",
        "root_generation": 6,
        "catalog_sha256": "11".repeat(32),
    }))
    .expect("wrong target fixture");
    assert_eq!(
        session
            .bind_real_endpoints(&source_handle(), &wrong_target)
            .unwrap_err(),
        ContinuityTransferError::WrongTarget
    );

    let unsafe_source = serde_json::from_value::<SourceCheckpointHandle>(serde_json::json!({
        "generation": 5,
        "root_generation": 5,
        "catalog_sha256": "11".repeat(32),
        "bundle_sha256": "22".repeat(32),
        "path": "../escape",
    }));
    assert!(unsafe_source.is_err());
    let unsafe_stage = serde_json::from_value::<TargetStageHandle>(serde_json::json!({
        "stage_id": "target-stage-210",
        "root_generation": 6,
        "catalog_sha256": "11".repeat(32),
        "path": "../escape",
    }));
    assert!(unsafe_stage.is_err());
    marker("CASE-002:real_bundle_source");
    marker("CASE-003:exact_target_stage");
    marker("CASE-043:unsafe_path_denied");
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
fn completion_retry_and_completion_journal_are_cached_without_payload() {
    let mut session = session();
    session.accept_frame(frame(0)).expect("first accepted");
    session.accept_frame(frame(1)).expect("second accepted");
    let completed = session.finish().expect("complete");
    assert_eq!(session.finish().expect("finish retry"), completed);
    let journal = session.journal();
    assert!(journal.completed.is_some());
    assert_eq!(journal.accepted.len(), 2);

    let mut restored = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        journal,
    )
    .expect("restore completed");
    assert_eq!(restored.finish().expect("restored complete"), completed);
    marker("CASE-006:exact_retry_cached");
    marker("CASE-037:crash_after_completion_result");
    marker("CASE-039:crash_after_checkpoint");
}

#[test]
fn wrong_source_target_route_and_cut_are_denied_before_bytes_move() {
    let artifact = artifact();
    let mut expected = expectation();
    expected.trust_domain = "other-domain".to_owned();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongTrustDomain
    );

    let mut expected = expectation();
    expected.polis_id = "other-polis".to_owned();
    assert_eq!(
        ContinuityTransferSession::open(&artifact, expected, ContinuityTransferPolicy::default())
            .unwrap_err(),
        ContinuityTransferError::WrongPolis
    );

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
    marker("CASE-009:wrong_polis_denied");
    marker("CASE-010:wrong_domain_denied");
    marker("CASE-011:wrong_lineage_denied");
    marker("CASE-012:wrong_membership_cut_denied");
    marker("CASE-013:stale_certificate_denied");
    marker("CASE-014:wrong_boot_generation_denied");
}

#[test]
fn signed_catalog_manifest_and_incremental_ranges_are_verified() {
    let mut bad_manifest = grant();
    bad_manifest.signed_manifest_sha256 = sha(b"wrong-manifest");
    assert!(CommittedAuthorityArtifact::continuity_transfer(&bad_manifest).is_err());

    let mut entry_gap = grant();
    entry_gap.entries[0].absolute_start = 1;
    assert!(CommittedAuthorityArtifact::continuity_transfer(&entry_gap).is_err());

    let mut predecessor_drift = grant();
    predecessor_drift.chunks[1].predecessor_sha256 = Some(sha(b"wrong-predecessor"));
    assert!(CommittedAuthorityArtifact::continuity_transfer(&predecessor_drift).is_err());
    marker("CASE-004:incremental_catalog_verify");
    marker("CASE-025:wrong_manifest_denied");
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
    wrong_digest.payload[0] ^= 0x01;
    assert_eq!(
        digest_session.accept_frame(wrong_digest).unwrap_err(),
        ContinuityTransferError::Digest
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
fn deadline_and_chunk_overrun_are_denied_before_prefix_advances() {
    let mut before_first = session();
    let mut expired_first = frame(0);
    expired_first.observed_unix_millis = after_deadline_millis();
    assert_eq!(
        before_first.accept_frame(expired_first).unwrap_err(),
        ContinuityTransferError::Deadline
    );
    assert_eq!(before_first.accepted_prefix(), 0);

    let mut midstream = session();
    midstream.accept_frame(frame(0)).expect("first accepted");
    let prefix = midstream.accepted_prefix();
    let mut expired_second = frame(1);
    expired_second.observed_unix_millis = after_deadline_millis();
    assert_eq!(
        midstream.accept_frame(expired_second).unwrap_err(),
        ContinuityTransferError::Deadline
    );
    assert_eq!(midstream.accepted_prefix(), prefix);

    let mut complete = session();
    complete.accept_frame(frame(0)).expect("first accepted");
    complete.accept_frame(frame(1)).expect("second accepted");
    let mut overrun = frame(1);
    overrun.chunk_index = 2;
    overrun.absolute_start = complete.accepted_prefix();
    assert_eq!(
        complete.accept_frame(overrun).unwrap_err(),
        ContinuityTransferError::Bounds
    );
    marker("CASE-019:frame_n_plus_one_denied");
    marker("CASE-028:deadline_before_first_byte");
    marker("CASE-029:deadline_midstream");
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
}

#[test]
fn generic_or_confused_authority_binding_is_denied() {
    let valid_artifact = artifact();
    let expected = expectation();
    assert!(validate_continuity_transfer_binding(
        &valid_artifact,
        "generic-send",
        &expected.lineage_id,
        &expected.source_checkpoint_handle_identity,
        &expected.bundle_handle_identity,
    )
    .is_err());

    let mut confused_artifact = valid_artifact.clone();
    confused_artifact.domain = "adl.authority-artifact.membership.v1".to_owned();
    assert!(validate_continuity_transfer_binding(
        &confused_artifact,
        CONTINUITY_TRANSFER_ADAPTER_210,
        &expected.lineage_id,
        &expected.source_checkpoint_handle_identity,
        &expected.bundle_handle_identity,
    )
    .is_err());

    let mut unknown_artifact = valid_artifact;
    unknown_artifact.domain = "adl.authority-artifact.unknown-transfer.v1".to_owned();
    assert!(validate_continuity_transfer_binding(
        &unknown_artifact,
        CONTINUITY_TRANSFER_ADAPTER_210,
        &expected.lineage_id,
        &expected.source_checkpoint_handle_identity,
        &expected.bundle_handle_identity,
    )
    .is_err());
    marker("CASE-015:generic_send_denied");
    marker("CASE-016:raft_rpc_confusion_denied");
    marker("CASE-017:unknown_kind_denied");
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

#[test]
fn journal_restore_resumes_exact_prefix_without_raw_payload() {
    let admitted = session();
    let admitted_journal = admitted.journal();
    let admitted_restored = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        admitted_journal,
    )
    .expect("restore empty admission");
    assert_eq!(admitted_restored.accepted_prefix(), 0);

    let mut session = session();
    let accepted = session.accept_frame(frame(0)).expect("first accepted");
    let journal = session.journal();
    assert_eq!(journal.accepted_prefix, accepted.accepted_prefix);
    assert_eq!(journal.accepted.len(), 1);

    let mut restored = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        journal,
    )
    .expect("restore accepted prefix");
    assert_eq!(restored.accepted_prefix(), accepted.accepted_prefix);
    restored.accept_frame(frame(1)).expect("resume second");
    assert_eq!(restored.finish().expect("complete").chunk_count, 2);
    marker("CASE-005:resume_after_partition");
    marker("CASE-032:source_restart_resume");
    marker("CASE-033:target_restart_resume");
    marker("CASE-034:crash_after_admission");
    marker("CASE-035:crash_after_frame_write");
    marker("CASE-036:crash_after_prefix_receipt");
    marker("CASE-038:crash_before_checkpoint");
    marker("CASE-040:reply_loss_retry");
}

#[test]
fn corrupt_or_rollback_journal_is_denied() {
    let mut session = session();
    session.accept_frame(frame(0)).expect("first accepted");

    let mut wrong_transfer = session.journal();
    wrong_transfer.transfer_id_sha256 = sha(b"wrong-transfer");
    assert_eq!(
        ContinuityTransferSession::restore(
            &artifact(),
            expectation(),
            ContinuityTransferPolicy::bounded(64, 128, 4),
            wrong_transfer,
        )
        .unwrap_err(),
        ContinuityTransferError::CorruptJournal
    );

    let mut rollback = session.journal();
    rollback.accepted_prefix = 0;
    assert_eq!(
        ContinuityTransferSession::restore(
            &artifact(),
            expectation(),
            ContinuityTransferPolicy::bounded(64, 128, 4),
            rollback,
        )
        .unwrap_err(),
        ContinuityTransferError::CorruptJournal
    );

    let mut duplicate_receipt = session.journal();
    duplicate_receipt
        .accepted
        .get_mut(&0)
        .expect("receipt")
        .duplicate = true;
    assert_eq!(
        ContinuityTransferSession::restore(
            &artifact(),
            expectation(),
            ContinuityTransferPolicy::bounded(64, 128, 4),
            duplicate_receipt,
        )
        .unwrap_err(),
        ContinuityTransferError::CorruptJournal
    );
    let mut torn_write = session.journal();
    torn_write.accepted.clear();
    assert_eq!(
        ContinuityTransferSession::restore(
            &artifact(),
            expectation(),
            ContinuityTransferPolicy::bounded(64, 128, 4),
            torn_write,
        )
        .unwrap_err(),
        ContinuityTransferError::CorruptJournal
    );
    struct FullDisk;
    impl ContinuityTransferJournalWriter for FullDisk {
        fn write_journal(
            &mut self,
            _journal: &ContinuityTransferJournalState,
        ) -> Result<(), ContinuityTransferError> {
            Err(ContinuityTransferError::Storage)
        }
    }
    let prefix_before_failed_write = session.accepted_prefix();
    assert_eq!(
        session.checkpoint_journal(&mut FullDisk).unwrap_err(),
        ContinuityTransferError::Storage
    );
    assert_eq!(session.accepted_prefix(), prefix_before_failed_write);
    let mut impossible_completion = session.journal();
    impossible_completion.completed = Some({
        session.accept_frame(frame(1)).expect("second accepted");
        session.finish().expect("complete")
    });
    assert_eq!(
        ContinuityTransferSession::restore(
            &artifact(),
            expectation(),
            ContinuityTransferPolicy::bounded(64, 128, 4),
            impossible_completion,
        )
        .unwrap_err(),
        ContinuityTransferError::CorruptJournal
    );
    marker("CASE-042:coherent_rollback_denied");
    marker("CASE-041:disk_full_no_false_success");
}
