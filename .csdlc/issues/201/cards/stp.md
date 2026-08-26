# Structured Task Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and publish only the committed quorum authority protocol, its private exact-artifact views for sealed #199/#200/#203 consumers, and a separate sealed #210-only ContinuityTransferGrantProjection for exact continuity-transfer variants; the #210 projection is borrowed, read-only, operation-bound, nonconstructible, and cannot authorize generic payload conversion, signing, migration, fencing, activation, serving, or concrete store effects.

## Deliverables

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_authority_protocol.rs
- .csdlc/prepared/issues/201/produce-proof-receipt.rb
- .csdlc/prepared/issues/201/validate-proof-receipt.rb
- .csdlc/evidence/201
- .csdlc/issues/201

## Acceptance

1. AC-1: Prepare and finalize entries bind polis, trust domain, exact membership epoch/index and voter-set digest, operation kind, checkpoint, payload digest, quorum-attested time, and operation id.
2. AC-2: Finalization requires opaque local signer custody and strict distinct current-voter quorum over the committed intent; raw keys and caller endorsements are unavailable.
3. AC-3: Replicated apply is deterministic and uses actual committed OpenRaft IDs; local clocks may gate endorsement but never applied result.
4. AC-4: Valid finalize apply returns pending with no token; only runtime-held local identity, root, and checkpoint authority may publish after exact reconciliation.
5. AC-5: Exact retries return retained canonical results; conflicting reuse, relabeling, stale custody, rollback, corruption, and capacity violations fail before publication.
6. AC-6: Snapshot-replicated current authority contains only stable polis, epoch, membership, configuration, and voter truth; the exact current boot vector remains runtime-external, while every prepared operation freezes its complete historical canonical boot vector plus stable-authority digest for finalization, publication, restore, and install revalidation. After boot rotation, reopen must build and install a snapshot before any new Prepare; a stale cut must reject byte-for-byte without state mutation and the exact current cut must succeed, while duplicate, reordered, zero, missing, extra, or non-JCS boot vectors fail closed.
7. AC-7: Successful publication yields only opaque operation tokens and sealed operation-specific views; #201 performs no downstream side effect.
8. AC-8: The exact ordered 86 names are current_three_voter_finalize, exact_retry_returns_cached_result, signer_rotation_current_generation, joint_majority_each_config, finalize_at_deadline, three_node_checkpoint_restart_reconcile, missing_quorum, duplicate_signer, wrong_voter, signer_unavailable, expired_signer_cert, stale_membership, config_digest_mismatch, joint_old_only, joint_new_only, joint_union_majority_only, joint_duplicate_guardian_reuse, declared_finalize_time_after_deadline, finalize_before_prepare_time, replay_with_regressed_finalize_time, local_clock_skew_apply_parity, checkpoint_object_collision, node_a_local_before_cas, node_a_cas_before_final_marker, node_b_local_before_cas, node_b_cas_before_final_marker, node_c_local_before_cas, node_c_cas_before_final_marker, checkpoint_result_retry_digest_mismatch, coherent_rollback_rejected, corrupt_journal_rejected, corrupt_retry_cache_rejected, capacity_n_plus_one_no_partial, state_symlink_rejected, lock_symlink_rejected, legacy_fence_voter_rejected, legacy_activate_owner_rejected, legacy_activate_shepherd_rejected, legacy_acquire_observatory_rejected, legacy_demote_voter_rejected, exact_store_artifact_bytes_retained, artifact_bytes_digest_substitution_rejected, sealed_continuity_transfer_projection, continuity_projection_consumer_confusion_rejected, continuity_projection_wrong_lineage_rejected, continuity_projection_wrong_source_checkpoint_handle_rejected, continuity_projection_wrong_bundle_handle_rejected, snapshot_valid_multi_prepared_finalized_restart, snapshot_current_polis_mismatch, snapshot_current_epoch_mismatch, snapshot_current_membership_mismatch, snapshot_current_boot_mismatch, snapshot_prepared_polis_mismatch, snapshot_prepared_epoch_mismatch, snapshot_prepared_membership_mismatch, snapshot_prepared_boot_mismatch, snapshot_later_prepared_custody_mismatch, snapshot_legacy_owner_injection, snapshot_legacy_shepherd_injection, snapshot_legacy_observatory_injection, snapshot_legacy_fence_injection, snapshot_legacy_demotion_injection, snapshot_finalized_missing_proposal, snapshot_finalized_missing_endorsements, snapshot_finalized_wrong_operation, snapshot_finalized_insufficient_quorum, snapshot_finalized_duplicate_quorum, snapshot_finalized_bad_signature, snapshot_finalized_stale_certificate, snapshot_finalized_wrong_boot, snapshot_finalized_invalid_time, snapshot_finalized_wrong_prepare_index, snapshot_finalized_wrong_finalize_index, snapshot_custody_omitted, snapshot_custody_reencoded, snapshot_custody_injected, snapshot_custody_substituted, snapshot_custody_byte_digest_mismatch, snapshot_evidence_omitted, snapshot_evidence_reencoded, snapshot_evidence_injected, snapshot_evidence_substituted, snapshot_evidence_byte_digest_mismatch, validator_available_divergent_rejected, validator_available_ancestral_passed, validator_unavailable_protected_fallback_passed. ADL_ISSUE_201_CASE_V2 result passed is required exactly for current_three_voter_finalize, exact_retry_returns_cached_result, joint_majority_each_config, finalize_at_deadline, three_node_checkpoint_restart_reconcile, local_clock_skew_apply_parity, exact_store_artifact_bytes_retained, sealed_continuity_transfer_projection, snapshot_valid_multi_prepared_finalized_restart, validator_available_ancestral_passed, validator_unavailable_protected_fallback_passed. Result reconciled is required exactly for node_a_local_before_cas, node_a_cas_before_final_marker, node_b_local_before_cas, node_b_cas_before_final_marker, node_c_local_before_cas, node_c_cas_before_final_marker. Every other named case defaults to rejected, yielding exactly passed=11, reconciled=6, rejected=69, selected=86; all required gates must pass before publication.

## Dependencies

- Issue #191 / PR #197 externally reviewed and merged as an ancestor
- Current merged MembershipState, AuthorityMembership, certificate identity, and secure OpenRaft transport contracts
- Issue #201 live GitHub contract
- Issues #199 and #200 remain blocked until this issue merges

## Inputs

- agent-logic/agent-design-language#201
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/issues/142 and its reviewed operational design as read-only umbrella truth

## Non Goals

- OpenRaft learner, joint, final, demotion, or rejoin membership coordination (#199)
- Certificate, lease, fence, owner, Shepherd, migration, or recovery store side effects (#200)
- Kernel continuity export/import or snapshot catalog materialization
- Guardian/API/WSS or Observatory listener integration
- Models, AWS, live demonstration, final #142 delivery, merge without operator authorization, or lifecycle closeout
