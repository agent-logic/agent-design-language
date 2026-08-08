# Validation Planning Prompt

Template: 1.0.0

Issue: 5820

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5820/design.md

Diagram: .csdlc/prepared/issues/5820/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-lifecycle-unit",
    "proof_role": "Prove Guardian restart policy, lifecycle aggregation, and nonce-bound pre-restart probe synchronization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--bin",
      "adl-runtime-lifecycle-soak",
      "--no-default-features"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "production-guardian-macos-stress",
    "proof_role": "Run 100 ten-second production Guardian windows on macOS and prove authenticated HTTPS/WSS, forced child failure, bounded restart, durable continuity, clean logs, and shutdown.",
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
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh",
      "--suite",
      "stress_100x10s"
    ],
    "parallel_group": "runtime-production",
    "defer_reason": null
  },
  {
    "lane": "native-platform-receipts",
    "proof_role": "Recompute exact-head digest bindings for the acceptance-eligible macOS and Linux production artifacts and retain the acceptance-authorized native Windows blocker as blocked evidence, never as a pass.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_runtime_native_receipts.rb",
      ".csdlc/evidence/5820/runtime-native-receipts.json"
    ],
    "parallel_group": "platform-evidence",
    "defer_reason": null
  },
  {
    "lane": "exact-head-hygiene",
    "proof_role": "Reject whitespace damage before exact-head review and issue-closing publication.",
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
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-lifecycle-soak --no-default-features`
- `bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh --suite stress_100x10s`
- `ruby adl/tools/validate_v092_runtime_native_receipts.rb .csdlc/evidence/5820/runtime-native-receipts.json`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
