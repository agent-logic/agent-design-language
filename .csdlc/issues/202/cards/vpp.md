# Validation Planning Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/202/design.md

Diagram: .csdlc/prepared/issues/202/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authorized-learner-transport",
    "proof_role": "Prove exact thirty-six-case denominator: real_four_node_learner_replication, current_voter_cut_unchanged, excluded_node_recovery_learner, learner_promotion_route_handoff, exact_retry_session, reconnect_boot_rotation, certificate_overlap_authorized, missing_201_token, public_caller_denied, wrong_operation_kind, wrong_domain, wrong_polis, wrong_learner, wrong_guardian, wrong_certificate_generation, expired_certificate, revoked_certificate, wrong_boot_generation, wrong_address, learner_vote_rpc_denied, learner_endorsement_denied, learner_finalize_denied, learner_mutation_denied, learner_renewal_denied, learner_shepherd_denied, learner_observatory_denied, exclusion_ordinary_session_denied, exclusion_wrong_recovery_token, stale_admission, replay_conflict, oversized_frame, truncated_frame, capacity_n_plus_one_no_partial, crash_before_exclusion_checkpoint, crash_after_exclusion_checkpoint, state_or_lock_symlink_rejected.",
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
      "distributed_authorized_learner_transport",
      "--no-tests=fail"
    ],
    "parallel_group": "202-runtime",
    "defer_reason": "Deferred until #191 and #201 merge and this issue creates adl-runtime/tests/distributed_authorized_learner_transport.rs plus adl-runtime/src/distributed/learner_transport.rs; fail closed on missing targets, zero tests, or any result not mapping exactly once to all thirty-six names."
  },
  {
    "lane": "authorized-learner-transport-clippy",
    "proof_role": "Reject warnings and API misuse across the exact learner/exclusion target.",
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
      "distributed_authorized_learner_transport",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "202-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "authorized-learner-transport-producer",
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
      ".csdlc/prepared/issues/202/produce-proof-receipt.rb"
    ],
    "parallel_group": "202-proof",
    "defer_reason": "Deferred until exact producer exists; fail closed on dirty protected source, wrong case count, missing/extra/duplicate name, nonpassing result, or nonzero status."
  },
  {
    "lane": "authorized-learner-transport-receipt",
    "proof_role": "Bind exact protected source, commands, thirty-six cases, strict Clippy, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
    ],
    "parallel_group": "202-proof",
    "defer_reason": "Deferred until validator and post-finalize immutable evidence exist; fail closed until exact reviewed source and all thirty-six cases are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_authorized_learner_transport -- -D warnings`
- `ruby .csdlc/prepared/issues/202/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/202/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing/invalid token, voter-cut drift, learner role escalation, wrong identity/cert/boot/address, exclusion bypass, stale connection, replay conflict, checkpoint ambiguity, rollback, corruption, capacity, unsafe paths, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
