# Validation Planning Prompt

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/201/design.md

Diagram: .csdlc/prepared/issues/201/diagram.mmd

## Selected Lanes

[
  {
    "lane": "committed-authority-protocol",
    "proof_role": "Prove exact forty-two-case denominator: current_three_voter_finalize, exact_retry_returns_cached_result, signer_rotation_current_generation, joint_majority_each_config, finalize_at_deadline, three_node_checkpoint_restart_reconcile, missing_quorum, duplicate_signer, wrong_voter, signer_unavailable, expired_signer_cert, stale_membership, config_digest_mismatch, joint_old_only, joint_new_only, joint_union_majority_only, joint_duplicate_guardian_reuse, declared_finalize_time_after_deadline, finalize_before_prepare_time, replay_with_regressed_finalize_time, local_clock_skew_apply_parity, checkpoint_object_collision, node_a_local_before_cas, node_a_cas_before_final_marker, node_b_local_before_cas, node_b_cas_before_final_marker, node_c_local_before_cas, node_c_cas_before_final_marker, checkpoint_result_retry_digest_mismatch, coherent_rollback_rejected, corrupt_journal_rejected, corrupt_retry_cache_rejected, capacity_n_plus_one_no_partial, state_symlink_rejected, lock_symlink_rejected, legacy_fence_voter_rejected, legacy_activate_owner_rejected, legacy_activate_shepherd_rejected, legacy_acquire_observatory_rejected, legacy_demote_voter_rejected, exact_store_artifact_bytes_retained, artifact_bytes_digest_substitution_rejected.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 20000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--no-tests=fail"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": "Deferred until PR #197 is merged and this issue creates the focused target; fail closed on a missing target/source, zero tests, any result not mapping exactly once to all forty-two canonical case names, any artifact byte/digest/operation mismatch, or any public/caller-substitutable artifact view."
  },
  {
    "lane": "committed-authority-protocol-clippy",
    "proof_role": "Reject warnings and API misuse across the same bounded core protocol and private artifact-view surface.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "201-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "committed-authority-protocol-producer",
    "proof_role": "Produce machine-derived execution artifacts with exact name, result, marker, command, stream, timing, Git source, protected-digest, and private artifact byte/digest parity for all forty-two declared cases.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/201/produce-proof-receipt.rb"
    ],
    "parallel_group": "201-proof",
    "defer_reason": "Deferred until the exact producer exists; fail closed on dirty protected source, a case count other than forty-two, any missing/extra/duplicate canonical name, artifact byte/digest mismatch, nonpassing result, or nonzero status."
  },
  {
    "lane": "committed-authority-protocol-receipt",
    "proof_role": "Bind exact protected source, commands, the exact forty-two-case name/result/marker denominator including retained artifact bytes, strict Clippy, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
    ],
    "parallel_group": "201-proof",
    "defer_reason": "Deferred until validator and post-finalize immutable evidence exist; fail closed until exact reviewed source, all forty-two names/results/markers, and exact artifact byte/digest parity are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_protocol -- -D warnings`
- `ruby .csdlc/prepared/issues/201/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/201/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing or invalid quorum endorsements, signer unavailability, stale membership, domain/index/time mismatch, incomplete protocol checkpoint, rollback, retry conflict, legacy direct authority, corruption, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
