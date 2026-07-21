# Validation Planning Prompt

Template: 1.0.0

Issue: 5590

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5590/design.md

Diagram: .csdlc/prepared/issues/5590/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-secure-config-access",
    "proof_role": "Prove init validation, TLS-only configured local/remote access, actual listener discovery, and negative transport/authority cases",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
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
      "configuration"
    ],
    "parallel_group": "runtime-v3-access",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-observatory-http-websocket",
    "proof_role": "Prove authenticated live HTTP and WebSocket Observatory state plus origin, bearer, session, malformed, and oversized-frame negatives",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
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
      "observatory"
    ],
    "parallel_group": "runtime-v3-access",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-guardian-recovery",
    "proof_role": "Prove external launch, signal forwarding, child reaping, bounded restart, pressure serialization, checkpoint restore, and invalid-config/intended-stop negatives",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "guardian"
    ],
    "parallel_group": "runtime-v3-operations",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-vector-telemetry",
    "proof_role": "Prove redacted stderr to Vector routing and truthful collector-degraded kernel behavior without custom OTel",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 450,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "telemetry"
    ],
    "parallel_group": "runtime-v3-operations",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-rollback-soak",
    "proof_role": "Prove bounded guardian soak and explicit selector rollback without automatic cutover, Runtime v2 source edits, or AWS",
    "acceptance_ids": [
      "AC-5",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/run_runtime_v3_guardian_soak.sh"
    ],
    "parallel_group": "runtime-v3-full",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-full-kernel",
    "proof_role": "Run the complete independent Runtime v3 kernel test suite",
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
    "parallel_group": "runtime-v3-full",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-strict-quality",
    "proof_role": "Prove strict lint plus exact dependency, LoC, module-growth, test-count, boundary, and evidence hygiene",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "parallel_group": "runtime-v3-quality",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify the exact issue patch has no whitespace or unrelated product changes",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
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

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml configuration`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml observatory`
- `cargo test --manifest-path adl-runtime/Cargo.toml guardian`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml telemetry`
- `bash adl/tools/run_runtime_v3_guardian_soak.sh`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets --all-features -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on missing #5591 integration eligibility, claim collision, plaintext access, weak or missing authentication, fake discovery, WebSocket policy bypass, secret leakage, unbounded restart, lost pressure state, collector-owned liveness, Runtime v2 edits, AWS use, hard-coded IPs, sidecars, deferred proof, budget breach, actionable review findings, or non-green exact-revision state.

## Handoff

Retain typed evidence before convergence.
