# Validation Planning Prompt

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/200/design.md

Diagram: .csdlc/prepared/issues/200/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authority-reconciliation-barrier",
    "proof_role": "Prove exact thirty-six-case denominator: happy_single_step, happy_multi_step, exact_retry_cached_result, pending_blocks_read, pending_blocks_mutation, published_permit_current, missing_201_token, public_token_forgery_denied, legacy_command_denied, wrong_domain, wrong_polis, wrong_node, wrong_guardian, wrong_boot, wrong_protocol_instance, wrong_membership, wrong_operation_kind, wrong_adapter_version, wrong_time_digest, conflicting_retry, reordered_step, duplicate_step, missing_step, forged_step_receipt, crash_after_journal, crash_each_step, crash_after_result, crash_before_checkpoint, crash_after_checkpoint, coherent_rollback, capacity_n_plus_one_no_partial, state_or_lock_symlink_rejected, corrupt_journal_rejected, noncanonical_state_rejected, opened_handle_growth_rejected, checkpoint_object_collision. crash_each_step and checkpoint cases must mechanically enumerate every before/after effect, receipt fsync, result, CAS, marker, view flip, and restart outcome; capacity and opened-handle cases prove no partial mutation or unbounded allocation.",
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
      "distributed_authority_reconciliation",
      "--no-tests=fail"
    ],
    "parallel_group": "200-runtime",
    "defer_reason": "Deferred until #191 and #201 merge and this issue creates adl-runtime/tests/distributed_authority_reconciliation.rs plus adl-runtime/src/distributed/authority_reconciliation.rs; fail closed on missing targets, zero tests, or any result not mapping exactly once to all thirty-six names and required subassertions."
  },
  {
    "lane": "authority-reconciliation-clippy",
    "proof_role": "Reject warnings and API misuse across the exact barrier target.",
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
      "distributed_authority_reconciliation",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "200-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "authority-reconciliation-producer",
    "proof_role": "Produce exact source, command, stream, timing, Git, protected-digest, and thirty-six-case name/result/marker evidence.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/200/produce-proof-receipt.rb"
    ],
    "parallel_group": "200-proof",
    "defer_reason": "Deferred until exact producer exists; fail closed on dirty protected source, wrong case count, missing/extra/duplicate name or subassertion, nonpassing result, or nonzero status."
  },
  {
    "lane": "authority-reconciliation-receipt",
    "proof_role": "Bind exact protected source, commands, thirty-six cases and subassertions, strict Clippy, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/200/validate-proof-receipt.rb"
    ],
    "parallel_group": "200-proof",
    "defer_reason": "Deferred until validator and post-finalize immutable evidence exist; fail closed until exact reviewed source and all cases/subassertions are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_reconciliation --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authority_reconciliation -- -D warnings`
- `ruby .csdlc/prepared/issues/200/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/200/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing/invalid token, public adapter/receipt/permit construction, wrong authority/time/membership/checkpoint binding, partial publication, conflicting retry, step-order/receipt mismatch, rollback, corruption, capacity, unsafe path, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
