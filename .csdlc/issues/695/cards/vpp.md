# Validation Planning Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/695/design.md

Diagram: .csdlc/prepared/issues/695/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-agent-partial-continuity",
    "proof_role": "Prove schema, cadence, all-resident coverage, isolation, lineage, restore, spool bounds, and S3 failure recovery with accelerated deterministic fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "agent_partial_checkpoint",
      "--lib"
    ],
    "parallel_group": "agent-partials",
    "defer_reason": null
  },
  {
    "lane": "runtime-agent-continuity-api",
    "proof_role": "Prove roster and detail API fields, null semantics, state enums, freshness transitions, and privacy exclusions.",
    "acceptance_ids": [
      "AC-7",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "agent_roster"
    ],
    "parallel_group": "agent-partials",
    "defer_reason": null
  },
  {
    "lane": "runtime-agent-continuity-observatory",
    "proof_role": "Prove Observatory backing-model and snapshot/archive rendering for every declared state without inferred success.",
    "acceptance_ids": [
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "node",
      "demos/html-observatory/tests/agent_continuity.test.mjs"
    ],
    "parallel_group": "agent-partials",
    "defer_reason": null
  },
  {
    "lane": "runtime-agent-checkpoint-terraform",
    "proof_role": "Prove private encrypted versioned S3, least-privilege IAM, lifecycle retention, and no live apply.",
    "acceptance_ids": [
      "AC-5",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "infra/aws/runtime/agent-checkpoint-archive/validate.sh"
    ],
    "parallel_group": "agent-partials",
    "defer_reason": null
  },
  {
    "lane": "runtime-agent-partial-production-shape",
    "proof_role": "Exercise multiple coordinator cycles, resident roster mutation, durable local state, and restart restoration through the ControlService production path.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "agent_partial_checkpoint_coordinator_tracks_roster_cycles_and_restart_restore",
      "--lib"
    ],
    "parallel_group": "agent-partials-serial",
    "defer_reason": null
  },
  {
    "lane": "acceptance-denominator",
    "proof_role": "Require one passing non-zero proving test or assertion set for every acceptance row before review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/695/validate-acceptance.sh"
    ],
    "parallel_group": "agent-partials-final",
    "defer_reason": null
  },
  {
    "lane": "format-and-diff",
    "proof_role": "Reject Rust formatting and committed whitespace defects.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "agent-partials",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_partial_checkpoint --lib`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster`
- `node demos/html-observatory/tests/agent_continuity.test.mjs`
- `bash infra/aws/runtime/agent-checkpoint-archive/validate.sh`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_partial_checkpoint_coordinator_tracks_roster_cycles_and_restart_restore --lib`
- `bash .csdlc/prepared/issues/695/validate-acceptance.sh`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`

## Failure Semantics

Fail closed for invalid configuration, partial integrity or lineage mismatch, unbounded retention, overlapping authority, secret leakage, cloud-coupled readiness, zero-test proof, missing implementation validators, or unresolved exact-head findings. During S3 failure, record degraded archive health while preserving Runtime availability.

## Handoff

Retain typed evidence before convergence.
