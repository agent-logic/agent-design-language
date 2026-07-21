# Validation Planning Prompt

Template: 1.0.0

Issue: 5589

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5589/design.md

Diagram: .csdlc/prepared/issues/5589/diagram.mmd

## Selected Lanes

[
  {
    "lane": "parity-c-live-governance",
    "proof_role": "Prove signed Freedom Gate/AEE gate-before-actuation and denial, appeal, revocation, quarantine, replay, and expiry negatives on live work",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_live_governance"
    ],
    "parallel_group": "parity-c-live",
    "defer_reason": null
  },
  {
    "lane": "parity-c-delegation-resources",
    "proof_role": "Prove attenuating delegation, resource bounds, cancellation precedence, retry/idempotency, saturation, and cleanup",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_delegation_resources"
    ],
    "parallel_group": "parity-c-live",
    "defer_reason": null
  },
  {
    "lane": "parity-c-provider-scheduler-tools",
    "proof_role": "Prove live multi-agent Shepherd/provider/scheduler/governed-tool execution and provider/scheduler negative classifications",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_provider_scheduler_tools"
    ],
    "parallel_group": "parity-c-live",
    "defer_reason": null
  },
  {
    "lane": "parity-c-private-identity",
    "proof_role": "Prove authoritative identity, private-state partitioning, restart persistence, revocation, cross-identity rejection, and evidence redaction",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_private_identity"
    ],
    "parallel_group": "parity-c-state",
    "defer_reason": null
  },
  {
    "lane": "parity-c-time-continuity",
    "proof_role": "Prove qualified time, authenticated checkpoints, non-authoritative lifelog, restart/no-duplicate continuity, corruption/replay rejection, failure isolation, and final shutdown checkpoint",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_time_continuity"
    ],
    "parallel_group": "parity-c-state",
    "defer_reason": null
  },
  {
    "lane": "parity-c-production-credit",
    "proof_role": "Reject DegradedOperationExecutor, fixture, mock, metadata-only, library-only, and fixed-bootstrap parity credit and inventory production/COTS adapters",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_production_credit"
    ],
    "parallel_group": "parity-c-quality",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-full-kernel",
    "proof_role": "Run the complete canonical Runtime v3 kernel suite after focused Parity-C proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "parity-c-full",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-strict-lint",
    "proof_role": "Prove format and strict warning-free Runtime v3 code",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--all-features",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "parity-c-quality",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-budget",
    "proof_role": "Measure exact Runtime v3 source lines and test count and account for placeholder/duplicate deletion",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "adl/tools/report_runtime_v3_loc.sh"
    ],
    "parallel_group": "parity-c-quality",
    "defer_reason": null
  },
  {
    "lane": "parity-c-boundary-scan",
    "proof_role": "Reject Runtime v2, AWS, credential, machine-local, degraded-credit, and cross-lane ownership drift",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "parity_c_boundary_contract"
    ],
    "parallel_group": "parity-c-quality",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify exact issue-branch patch hygiene",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_live_governance`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_delegation_resources`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_provider_scheduler_tools`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_private_identity`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_time_continuity`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_production_credit`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings`
- `bash adl/tools/report_runtime_v3_loc.sh`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml parity_c_boundary_contract`
- `git diff --check`

## Failure Semantics

Fail closed on absent clean #5591 review truth, claim collision, unpinned contract drift, degraded or fixture-only credit, governance bypass, widened delegation, leaked resources, cancellation/revocation races, identity/private-state disclosure, unqualified time, ambiguous recovery, duplicate side effects, provider/tool misclassification, skipped proof, AWS or Runtime v2 use, budget breach, actionable review findings, or non-green exact-revision state.

## Handoff

Retain typed evidence before convergence.
