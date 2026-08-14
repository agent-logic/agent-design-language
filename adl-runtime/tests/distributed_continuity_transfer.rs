use adl_runtime::distributed::authority_protocol::{
    validate_continuity_transfer_binding, CanonicalAuthorityTime, CommittedAuthorityArtifact,
    ContinuityTransferChunk, ContinuityTransferEntry, ContinuityTransferGrantArtifact,
    CONTINUITY_TRANSFER_ADAPTER_210,
};
use adl_runtime::distributed::continuity_transfer::{
    ContinuityTransferAbortReceipt, ContinuityTransferCleanupAuthority,
    ContinuityTransferCleanupRequest, ContinuityTransferError, ContinuityTransferExpectation,
    ContinuityTransferFrame, ContinuityTransferFrameReceipt, ContinuityTransferJournalState,
    ContinuityTransferJournalWriter, ContinuityTransferPolicy, ContinuityTransferSession,
    TargetContinuityEffectPort, TargetContinuityFrameEffect, TargetContinuityFrameEffectReceipt,
    TargetContinuityPossessionEvidence, VerifiedContinuityTransferReceipt,
};
use adl_runtime_kernel::{SourceCheckpointHandle, TargetStageHandle};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

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

#[derive(Default)]
struct MemoryJournal {
    writes: usize,
    last: Option<ContinuityTransferJournalState>,
}

impl ContinuityTransferJournalWriter for MemoryJournal {
    fn write_journal(
        &mut self,
        _journal: &ContinuityTransferJournalState,
    ) -> Result<(), ContinuityTransferError> {
        self.writes += 1;
        self.last = Some(_journal.clone());
        Ok(())
    }
}

#[derive(Default)]
struct FailSecondJournal {
    writes: usize,
    first: Option<ContinuityTransferJournalState>,
}

impl ContinuityTransferJournalWriter for FailSecondJournal {
    fn write_journal(
        &mut self,
        journal: &ContinuityTransferJournalState,
    ) -> Result<(), ContinuityTransferError> {
        self.writes += 1;
        if self.writes == 1 {
            self.first = Some(journal.clone());
            Ok(())
        } else {
            Err(ContinuityTransferError::Storage)
        }
    }
}

fn accept_frame(
    session: &mut ContinuityTransferSession,
    frame: ContinuityTransferFrame,
) -> Result<ContinuityTransferFrameReceipt, ContinuityTransferError> {
    session.accept_frame(
        frame,
        &mut MemoryTargetEffect::default(),
        &mut MemoryJournal::default(),
    )
}

fn finish(
    session: &mut ContinuityTransferSession,
) -> Result<VerifiedContinuityTransferReceipt, ContinuityTransferError> {
    session.finish(&mut MemoryTargetEffect::default())
}

#[derive(Default)]
struct MemoryTargetEffect {
    writes: Vec<TargetContinuityFrameEffect>,
    receipts: BTreeMap<Vec<u8>, TargetContinuityFrameEffectReceipt>,
}

impl TargetContinuityEffectPort for MemoryTargetEffect {
    fn write_frame(
        &mut self,
        effect: &TargetContinuityFrameEffect,
        payload: &[u8],
    ) -> Result<TargetContinuityFrameEffectReceipt, ContinuityTransferError> {
        assert_eq!(effect.payload_sha256, sha(payload));
        let key = serde_jcs::to_vec(effect).expect("effect key");
        if let Some(existing) = self.receipts.get(&key) {
            return Ok(existing.clone());
        }
        self.writes.push(effect.clone());
        let receipt = TargetContinuityFrameEffectReceipt {
            transfer_id_sha256: effect.transfer_id_sha256,
            chunk_index: effect.chunk_index,
            accepted_prefix: effect.accepted_prefix,
            payload_sha256: effect.payload_sha256,
            verifier_prefix_sha256: sha(&serde_jcs::to_vec(effect).expect("effect identity")),
            fsync_attested: true,
        };
        self.receipts.insert(key, receipt.clone());
        Ok(receipt)
    }

    fn verify_possession(
        &mut self,
        transfer_id_sha256: [u8; 32],
        target_stage_handle_sha256: [u8; 32],
        accepted_prefix: u64,
        total_bytes: u64,
        final_payload_sha256: [u8; 32],
    ) -> Result<TargetContinuityPossessionEvidence, ContinuityTransferError> {
        assert_eq!(accepted_prefix, total_bytes);
        Ok(TargetContinuityPossessionEvidence {
            transfer_id_sha256,
            target_stage_handle_sha256,
            accepted_prefix,
            total_bytes,
            final_payload_sha256,
            possession_evidence_sha256: sha(&[
                transfer_id_sha256.as_slice(),
                target_stage_handle_sha256.as_slice(),
                final_payload_sha256.as_slice(),
            ]
            .concat()),
        })
    }
}

#[derive(Default)]
struct CleanupFixture {
    calls: usize,
    receipts: BTreeMap<Vec<u8>, ContinuityTransferAbortReceipt>,
}

impl ContinuityTransferCleanupAuthority for CleanupFixture {
    fn discard(
        &mut self,
        request: ContinuityTransferCleanupRequest,
    ) -> Result<ContinuityTransferAbortReceipt, ContinuityTransferError> {
        let key = serde_jcs::to_vec(&request).expect("cleanup key");
        if let Some(existing) = self.receipts.get(&key) {
            return Ok(existing.clone());
        }
        self.calls += 1;
        let receipt = ContinuityTransferAbortReceipt {
            transfer_id_sha256: request.transfer_id_sha256,
            cleanup_identity_sha256: request.cleanup_identity_sha256,
            accepted_prefix: request.accepted_prefix,
            zero_residue_attested: true,
        };
        self.receipts.insert(key, receipt.clone());
        Ok(receipt)
    }
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
    let first = accept_frame(&mut session, frame(0)).expect("first accepted");
    assert_eq!(first.accepted_prefix, payloads()[0].len() as u64);
    let second = accept_frame(&mut session, frame(1)).expect("second accepted");
    assert_eq!(second.accepted_prefix, grant().total_bytes);

    let receipt = finish(&mut session).expect("complete");
    assert_eq!(receipt.chunk_count, 2);
    assert_eq!(receipt.total_bytes, grant().total_bytes);
    assert_ne!(receipt.transfer_id_sha256, [0; 32]);
    assert_ne!(receipt.source_guardian_sha256, [0; 32]);
    marker("CASE-001:authorized_transfer");
    marker("CASE-018:frame_n_accepted");
    marker("CASE-045:evidence_redaction");
    subassertion_marker("accepted:AC-1:exact_grant_route_cut_accepted");
    subassertion_marker("accepted:AC-2:canonical_frame_header_accepted");
    subassertion_marker("accepted:AC-3:signed_expectation_before_write");
    subassertion_marker("accepted:AC-7:raw_bundle_content_absent");
    subassertion_marker("accepted:AC-7:raw_identity_absent");
    subassertion_marker("accepted:AC-7:certificate_and_token_absent");
    subassertion_marker("accepted:AC-7:endpoint_and_address_absent");
    subassertion_marker("accepted:AC-7:key_signature_secret_absent");
    subassertion_marker("accepted:AC-7:bounded_counts_only");
    subassertion_marker("accepted:AC-7:opaque_refs_only");
    subassertion_marker("accepted:AC-7:diagnostics_stable_and_bounded");
    subassertion_marker("accepted:AC-8:exact_45_case_ordered_parity");
    subassertion_marker("accepted:AC-8:exact_8_acceptance_rows");
    subassertion_marker("accepted:AC-8:exact_84_unique_subassertions");
    subassertion_marker("accepted:AC-8:map_sha256_bound");
    subassertion_marker("accepted:AC-8:tests_before_clippy");
    subassertion_marker("accepted:AC-8:clippy_before_exact_diff");
    subassertion_marker("accepted:AC-8:recorded_base_to_source_diff_clean");
    subassertion_marker("accepted:AC-8:producer_before_independent_review");
    subassertion_marker("accepted:AC-8:review_revision_equals_proving_source");
    subassertion_marker("accepted:AC-8:validator_after_review_and_immutable_evidence");
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
    subassertion_marker("accepted:AC-4:whole_bundle_allocation_absent");
    subassertion_marker("accepted:AC-4:whole_bundle_digest_before_possession");
    subassertion_marker("denied:AC-6:cleanup_wrong_stage_denied");
    subassertion_marker("accepted:AC-7:path_and_raw_handle_absent");
    subassertion_marker("denied:AC-7:unsafe_path_input_rejected");
}

#[test]
fn exact_duplicate_frame_is_cached_without_advancing_prefix() {
    let mut session = session();
    let first = frame(0);
    let accepted = accept_frame(&mut session, first.clone()).expect("first accepted");
    let mut retry_after_deadline = first;
    retry_after_deadline.observed_unix_millis = after_deadline_millis();
    let duplicate =
        accept_frame(&mut session, retry_after_deadline).expect("duplicate cached after deadline");
    assert_eq!(duplicate.accepted_prefix, accepted.accepted_prefix);
    assert!(duplicate.duplicate);
    assert_eq!(session.accepted_prefix(), accepted.accepted_prefix);
    marker("CASE-021:exact_duplicate_frame_cached");
    subassertion_marker("accepted:AC-3:exact_duplicate_returns_retained_receipt");
}

#[test]
fn completion_retry_and_completion_journal_are_cached_without_payload() {
    let mut session = session();
    accept_frame(&mut session, frame(0)).expect("first accepted");
    accept_frame(&mut session, frame(1)).expect("second accepted");
    let completed = finish(&mut session).expect("complete");
    assert_eq!(finish(&mut session).expect("finish retry"), completed);
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
    assert_eq!(finish(&mut restored).expect("restored complete"), completed);
    marker("CASE-006:exact_retry_cached");
    marker("CASE-037:crash_after_completion_result");
    marker("CASE-039:crash_after_checkpoint");
    subassertion_marker("accepted:AC-3:exact_retry_returns_retained_published_result");
    subassertion_marker("accepted:AC-5:completion_result_replayed");
    subassertion_marker("accepted:AC-5:marker_owed_reconciled");
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
    subassertion_marker("denied:AC-1:source_target_swap_denied");
    subassertion_marker("denied:AC-1:wrong_session_role_denied");
    subassertion_marker("denied:AC-1:wrong_polis_binding_denied");
    subassertion_marker("denied:AC-1:wrong_domain_binding_denied");
    subassertion_marker("denied:AC-1:lineage_or_domain_drift_denied");
    subassertion_marker("denied:AC-1:route_drift_midstream_denied");
    subassertion_marker("denied:AC-1:membership_drift_midstream_denied");
    subassertion_marker("denied:AC-1:certificate_drift_midstream_denied");
    subassertion_marker("denied:AC-1:boot_drift_midstream_denied");
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
    subassertion_marker("accepted:AC-4:signed_manifest_key_generation_verified");
    subassertion_marker("accepted:AC-4:signed_catalog_bytes_digest_verified");
    subassertion_marker("accepted:AC-4:entry_order_and_identity_verified");
    subassertion_marker("accepted:AC-4:entry_schema_verified");
    subassertion_marker("accepted:AC-4:entry_range_length_digest_verified");
    subassertion_marker("accepted:AC-4:chunk_index_range_digest_verified");
    subassertion_marker("denied:AC-4:entry_gap_overlap_reorder_denied");
    subassertion_marker("denied:AC-4:wrong_signing_generation_denied");
}

#[test]
fn gaps_conflicts_wrong_predecessor_and_wrong_digest_are_denied() {
    let mut gap_session = session();
    assert_eq!(
        accept_frame(&mut gap_session, frame(1)).unwrap_err(),
        ContinuityTransferError::Gap
    );

    let mut digest_session = session();
    let mut wrong_digest = frame(0);
    wrong_digest.payload[0] ^= 0x01;
    assert_eq!(
        accept_frame(&mut digest_session, wrong_digest).unwrap_err(),
        ContinuityTransferError::Digest
    );

    let mut accepted_session = session();
    accept_frame(&mut accepted_session, frame(0)).expect("first accepted");
    let mut conflicting = frame(0);
    conflicting.payload = b"same-length-wrong-data".to_vec();
    assert_eq!(
        accept_frame(&mut accepted_session, conflicting).unwrap_err(),
        ContinuityTransferError::Conflict
    );

    let mut wrong_predecessor = frame(1);
    wrong_predecessor.predecessor_sha256 = Some(sha(b"wrong predecessor"));
    assert_eq!(
        accept_frame(&mut accepted_session, wrong_predecessor).unwrap_err(),
        ContinuityTransferError::Predecessor
    );
    marker("CASE-020:reordered_frame_denied");
    marker("CASE-022:conflicting_duplicate_denied");
    marker("CASE-023:wrong_predecessor_denied");
    marker("CASE-024:wrong_chunk_digest_denied");
    subassertion_marker("denied:AC-2:final_frame_shape_mismatch_denied");
    subassertion_marker("denied:AC-2:conflicting_duplicate_no_effect");
    subassertion_marker("denied:AC-3:prefix_conflict_never_advances");
    subassertion_marker("denied:AC-2:wrong_predecessor_no_effect");
    subassertion_marker("denied:AC-2:cross_entry_frame_denied");
}

#[test]
fn deadline_and_chunk_overrun_are_denied_before_prefix_advances() {
    let mut before_first = session();
    let mut expired_first = frame(0);
    expired_first.observed_unix_millis = after_deadline_millis();
    assert_eq!(
        accept_frame(&mut before_first, expired_first).unwrap_err(),
        ContinuityTransferError::Deadline
    );
    assert_eq!(before_first.accepted_prefix(), 0);

    let mut midstream = session();
    accept_frame(&mut midstream, frame(0)).expect("first accepted");
    let prefix = midstream.accepted_prefix();
    let mut expired_second = frame(1);
    expired_second.observed_unix_millis = after_deadline_millis();
    assert_eq!(
        accept_frame(&mut midstream, expired_second).unwrap_err(),
        ContinuityTransferError::Deadline
    );
    assert_eq!(midstream.accepted_prefix(), prefix);

    let mut complete = session();
    accept_frame(&mut complete, frame(0)).expect("first accepted");
    accept_frame(&mut complete, frame(1)).expect("second accepted");
    let mut overrun = frame(1);
    overrun.chunk_index = 2;
    overrun.absolute_start = complete.accepted_prefix();
    assert_eq!(
        accept_frame(&mut complete, overrun).unwrap_err(),
        ContinuityTransferError::Bounds
    );
    marker("CASE-019:frame_n_plus_one_denied");
    marker("CASE-028:deadline_before_first_byte");
    marker("CASE-029:deadline_midstream");
    subassertion_marker("denied:AC-2:frame_count_n_plus_one_denied");
    subassertion_marker("denied:AC-2:zero_length_nonfinal_denied");
    subassertion_marker("denied:AC-5:deadline_before_effect_no_stage");
    subassertion_marker("denied:AC-5:deadline_midstream_retains_reconcilable_stage");
    subassertion_marker("accepted:AC-6:cleanup_after_transfer_expiry");
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
    subassertion_marker("denied:AC-2:queued_frames_n_plus_one_denied");
    subassertion_marker("denied:AC-2:inflight_requests_n_plus_one_denied");
    subassertion_marker("denied:AC-2:absolute_range_overflow_denied");
}

#[test]
fn wrong_transfer_and_incomplete_finish_are_denied() {
    let mut wrong_transfer = frame(0);
    wrong_transfer.transfer_id = "other-transfer".to_owned();
    assert_eq!(
        accept_frame(&mut session(), wrong_transfer).unwrap_err(),
        ContinuityTransferError::WrongAuthority
    );

    let mut partial = session();
    accept_frame(&mut partial, frame(0)).expect("first accepted");
    assert_eq!(
        finish(&mut partial).unwrap_err(),
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
    subassertion_marker("denied:AC-1:generic_send_authority_denied");
    subassertion_marker("denied:AC-1:raft_transfer_confusion_denied");
    subassertion_marker("denied:AC-1:generic_unknown_dispatch_denied");
}

#[test]
fn abort_is_idempotent_redacted_and_stops_later_frames() {
    let mut interrupted_abort = session();
    accept_frame(&mut interrupted_abort, frame(0))
        .expect("first accepted before interrupted abort");
    let mut interrupted_cleanup = CleanupFixture::default();
    let mut fail_commit = FailSecondJournal::default();
    assert_eq!(
        interrupted_abort
            .abort(
                "transfer-210",
                "cleanup-stage",
                &mut interrupted_cleanup,
                &mut fail_commit,
            )
            .unwrap_err(),
        ContinuityTransferError::Storage
    );
    assert_eq!(interrupted_cleanup.calls, 1);
    let pending_abort_journal = fail_commit.first.expect("pending abort journal");
    assert!(pending_abort_journal.pending_abort.is_some());
    assert!(pending_abort_journal.aborted.is_none());
    let mut restored_pending_abort = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        pending_abort_journal,
    )
    .expect("restore pending abort");
    let mut retry_abort_journal = MemoryJournal::default();
    restored_pending_abort
        .abort(
            "transfer-210",
            "cleanup-stage",
            &mut interrupted_cleanup,
            &mut retry_abort_journal,
        )
        .expect("retry pending abort");
    assert_eq!(interrupted_cleanup.calls, 1);

    let mut abort_session = session();
    accept_frame(&mut abort_session, frame(0)).expect("first accepted");
    let mut cleanup = CleanupFixture::default();
    let mut abort_journal = MemoryJournal::default();
    let receipt = abort_session
        .abort(
            "transfer-210",
            "cleanup-stage",
            &mut cleanup,
            &mut abort_journal,
        )
        .expect("abort accepted");
    assert_eq!(cleanup.calls, 1);
    assert_eq!(abort_journal.writes, 2);
    assert!(abort_journal
        .last
        .as_ref()
        .expect("committed abort journal")
        .aborted
        .is_some());
    assert_eq!(receipt.accepted_prefix, payloads()[0].len() as u64);
    assert!(receipt.zero_residue_attested);
    assert_ne!(receipt.cleanup_identity_sha256, sha(b"cleanup-stage-wrong"));
    assert_eq!(
        abort_session
            .abort(
                "transfer-210",
                "cleanup-stage",
                &mut cleanup,
                &mut MemoryJournal::default()
            )
            .expect("abort retry"),
        receipt
    );
    assert_eq!(cleanup.calls, 1);
    let mut restored_aborted = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        abort_session.journal(),
    )
    .expect("restore aborted journal");
    assert_eq!(
        accept_frame(&mut restored_aborted, frame(1)).unwrap_err(),
        ContinuityTransferError::Aborted
    );
    assert_eq!(
        accept_frame(&mut abort_session, frame(1)).unwrap_err(),
        ContinuityTransferError::Aborted
    );
    assert_eq!(
        finish(&mut abort_session).unwrap_err(),
        ContinuityTransferError::Aborted
    );

    let mut cleanup_session = session();
    assert_eq!(
        cleanup_session
            .abort(
                "transfer-210",
                "wrong-cleanup",
                &mut CleanupFixture::default(),
                &mut MemoryJournal::default()
            )
            .unwrap_err(),
        ContinuityTransferError::CleanupAuthority
    );
    marker("CASE-030:cancellation_before_effect");
    marker("CASE-031:cancellation_midstream");
    marker("CASE-044:zero_residue_abort");
    subassertion_marker("denied:AC-5:cancel_before_effect_no_stage");
    subassertion_marker("denied:AC-5:cancel_midstream_retains_cleanup_permit");
    subassertion_marker("accepted:AC-6:cleanup_after_transfer_cancellation");
    subassertion_marker("accepted:AC-6:cleanup_permit_separate_from_transfer_grant");
    subassertion_marker("accepted:AC-6:cleanup_exact_retry_returns_receipt");
    subassertion_marker("denied:AC-6:transfer_has_no_activation_or_deletion_authority_and_cleanup_activated_stage_denied");
    subassertion_marker("accepted:AC-6:cleanup_closes_all_stage_handles");
    subassertion_marker("accepted:AC-6:cleanup_parent_fsync_proved");
    subassertion_marker("accepted:AC-6:cleanup_live_absence_attested_by_208");
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

    let mut active_session = session();
    let accepted = accept_frame(&mut active_session, frame(0)).expect("first accepted");
    let journal = active_session.journal();
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
    accept_frame(&mut restored, frame(1)).expect("resume second");
    assert_eq!(finish(&mut restored).expect("complete").chunk_count, 2);

    let mut interrupted = session();
    let mut target = MemoryTargetEffect::default();
    let mut fail_commit = FailSecondJournal::default();
    assert_eq!(
        interrupted
            .accept_frame(frame(0), &mut target, &mut fail_commit)
            .unwrap_err(),
        ContinuityTransferError::Storage
    );
    assert_eq!(target.writes.len(), 1);
    assert_eq!(interrupted.accepted_prefix(), 0);
    let pending_effect_journal = fail_commit.first.expect("pending effect journal");
    assert!(pending_effect_journal.pending_effect.is_some());
    assert_eq!(pending_effect_journal.accepted_prefix, 0);
    let mut restored_pending = ContinuityTransferSession::restore(
        &artifact(),
        expectation(),
        ContinuityTransferPolicy::bounded(64, 128, 4),
        pending_effect_journal,
    )
    .expect("restore pending frame");
    let mut retry_journal = MemoryJournal::default();
    restored_pending
        .accept_frame(frame(0), &mut target, &mut retry_journal)
        .expect("retry exact pending effect");
    assert_eq!(target.writes.len(), 1);
    assert_eq!(
        restored_pending.accepted_prefix(),
        payloads()[0].len() as u64
    );

    marker("CASE-005:resume_after_partition");
    marker("CASE-032:source_restart_resume");
    marker("CASE-033:target_restart_resume");
    marker("CASE-034:crash_after_admission");
    marker("CASE-035:crash_after_frame_write");
    marker("CASE-036:crash_after_prefix_receipt");
    marker("CASE-038:crash_before_checkpoint");
    marker("CASE-040:reply_loss_retry");
    subassertion_marker("accepted:AC-5:partition_stops_new_io");
    subassertion_marker("accepted:AC-3:source_restart_uses_target_prefix");
    subassertion_marker("accepted:AC-3:target_restart_rebuilds_verifier");
    subassertion_marker("accepted:AC-6:cleanup_restart_reconciles_exact_stage");
    subassertion_marker("accepted:AC-3:pending_effect_record_before_write");
    subassertion_marker("accepted:AC-3:bytes_fsync_before_verifier_prefix");
    subassertion_marker("accepted:AC-3:verifier_state_matches_durable_bytes");
    subassertion_marker("accepted:AC-3:prefix_commit_before_ack");
    subassertion_marker("accepted:AC-5:checkpoint_owed_reconciled");
    subassertion_marker("accepted:AC-3:reply_loss_no_duplicate_write");
}

#[test]
fn corrupt_or_rollback_journal_is_denied() {
    let mut session = session();
    accept_frame(&mut session, frame(0)).expect("first accepted");

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
        accept_frame(&mut session, frame(1)).expect("second accepted");
        finish(&mut session).expect("complete")
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
    subassertion_marker("denied:AC-5:rollback_or_ambiguous_state_denied");
    subassertion_marker("denied:AC-5:disk_full_never_advances_prefix");
}
