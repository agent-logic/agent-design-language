# Validation Planning Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/208/design.md

Diagram: .csdlc/prepared/issues/208/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-kernel-continuity",
    "proof_role": "Prove exact thirty-six-case denominator: internal_listener_config_valid, nonloopback_bind_rejected, guardian_identity_distinct, guardian_mtls_authorized, unknown_client_certificate_denied, bearer_only_denied, agent_control_identity_denied, replay_rejected, wrong_kernel_instance_denied, real_quiesce_checkpoint, signed_bundle_export, export_bounds, export_exact_retry, source_resume, source_resume_exact_retry, isolated_stage, isolated_import_validate, wrong_manifest_signature, wrong_topology, wrong_config, wrong_service_set, wrong_service_schema, corrupt_content, oversized_bundle, caller_path_rejected, symlink_path_rejected, deadline_before_effect, cancellation_no_partial, restart_after_accept, crash_after_bundle_commit, target_discard, discard_exact_retry, zero_residue, dual_open, evidence_redaction, public_surface_absent. Retry/crash cases mechanically enumerate accepted journal, every kernel effect/receipt, stream interruption, result, checkpoint, marker, response cache and reply loss on client/server; bounds enumerate N/N+1 frames, blobs, services, bytes, records and opened-handle replacement.",
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
      "kernel_continuity_client",
      "--no-tests=fail"
    ],
    "parallel_group": "208-runtime",
    "defer_reason": "Deferred until #191 merges and this issue creates the exact client/server targets; fail closed on missing targets, zero tests, or missing, extra, duplicate, nonpassing case/result/marker/subassertion."
  },
  {
    "lane": "guardian-kernel-continuity-clippy",
    "proof_role": "Reject warnings and API misuse across the exact client/server target.",
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
      "kernel_continuity_client",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "208-runtime",
    "defer_reason": "Deferred until the owned focused target exists; fail closed on warnings, missing target, or missing source."
  },
  {
    "lane": "guardian-kernel-continuity-producer",
    "proof_role": "Produce exact Git, source, command, stream, timing, protected-digest and thirty-six-case evidence.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/208/produce-proof-receipt.rb"
    ],
    "parallel_group": "208-producer",
    "defer_reason": "Deferred until the exact producer exists; fail closed on dirty protected source, wrong case count, missing, extra, duplicate or nonpassing case/subassertion, or nonzero status."
  },
  {
    "lane": "guardian-kernel-continuity-receipt",
    "proof_role": "After producer completion, bind exact protected source, commands, thirty-six cases/subassertions, strict Clippy, immutable evidence introduction, review, and squash-merge-safe validation.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/208/validate-proof-receipt.rb"
    ],
    "parallel_group": "208-receipt",
    "defer_reason": "Deferred until producer, validator, typed finalize, post-finalize immutable evidence and review exist; fail closed until exact source and all cases/subassertions are bound."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test kernel_continuity_client -- -D warnings`
- `ruby .csdlc/prepared/issues/208/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/208/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on unsafe bind/root, missing or stale TLS identity, replay, wrong kernel/domain/polis/generation, public-route access, caller path, synthetic or corrupt snapshot, topology/configuration/service/content mismatch, deadline ambiguity, partial effect, rollback, capacity, residue, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
