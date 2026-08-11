# Validation Planning Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/203/design.md

Diagram: .csdlc/prepared/issues/203/diagram.mmd

## Selected Lanes

[
  {
    "lane": "identity-lease-fencing-authority",
    "proof_role": "Prove exact forty-four-case denominator: certificate_enroll, certificate_rotate_overlap, certificate_successor_post_overlap, certificate_revoke, certificate_compromise_identity_fence, lease_grant, lease_renewal, lease_revoke, fence_commit, activate_after_safety, owner_commit, exact_retry_published, restart_reanchor_safe, barrier_pending_blocks_all_reads, unsigned_certificate_rejected, wrong_issuer_rejected, wrong_certificate_purpose_rejected, wrong_certificate_domain_rejected, stale_certificate_generation_rejected, token_artifact_digest_mismatch, reconstructed_endorsements_rejected, wrong_authority_membership_rejected, stale_lease_index_rejected, stale_lease_epoch_rejected, wrong_activation_possession_rejected, activate_before_safety_rejected, floor_precedes_ledger_revocation, local_clock_unsafe_no_effect, local_clock_rollback_no_effect, crash_after_certificate_effect, crash_after_fence_floor, crash_after_ledger_effect, crash_after_local_anchor, crash_after_result, crash_before_checkpoint, crash_after_checkpoint, stale_read_permit_rejected, stale_mutation_permit_rejected, read_to_mutation_escalation_rejected, wrong_lineage_permit_rejected, coherent_rollback_rejected, corrupt_noncanonical_oversized_rejected, state_or_lock_symlink_rejected, capacity_n_plus_one_no_partial. Crash/bounds cases mechanically enumerate init CAS, dual open, before/after each effect and receipt, anchor, result, checkpoint, marker/view flip, opened-handle growth/inode replacement, and exact restart without partial authority.",
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
    "budget_seconds": 1800,
    "budget_tokens": 24000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--no-tests=fail"
    ],
    "parallel_group": "203-runtime",
    "defer_reason": "Deferred until all five dependencies merge and this issue creates the exact adapter and focused test targets; fail closed on missing targets, zero tests, or any missing/extra/duplicate name, result, marker, or declared subassertion."
  },
  {
    "lane": "identity-lease-fencing-clippy",
    "proof_role": "Reject warnings and API misuse across the exact concrete authority target.",
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
      "distributed_identity_lease_authority",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "203-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "identity-lease-fencing-producer",
    "proof_role": "Produce exact Git/source/command/stream/timing/protected-digest and forty-four-case evidence.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/203/produce-proof-receipt.rb"
    ],
    "parallel_group": "203-proof",
    "defer_reason": "Deferred until the exact producer exists; fail closed on dirty protected source, wrong case count, missing/extra/duplicate name or subassertion, nonpassing result, or nonzero status."
  },
  {
    "lane": "identity-lease-fencing-receipt",
    "proof_role": "Bind exact protected source, commands, forty-four cases/subassertions, strict Clippy, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/203/validate-proof-receipt.rb"
    ],
    "parallel_group": "203-proof",
    "defer_reason": "Deferred until validator and post-finalize immutable evidence exist; fail closed until exact reviewed source and all cases/subassertions are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- -D warnings`
- `ruby .csdlc/prepared/issues/203/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/203/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing/invalid token, artifact mismatch, raw-store access, stale or escalated grant, wrong authority/time/membership/floor binding, local-clock canonical drift, partial publication, conflicting retry, rollback, corruption, capacity, unsafe path, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
