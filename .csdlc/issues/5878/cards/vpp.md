# Validation Planning Prompt

Template: 1.0.0

Issue: 5878

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5878/design.md

Diagram: .csdlc/prepared/issues/5878/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Run the exact nonzero distributed_guardian target through the production library registration and prove bounded Prost transport plus quorum authority replay and wrong-domain rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_guardian",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "production-distributed-guardian",
    "proof_role": "Execute the exact integration target on one native runner and produce bounded digest-bound logs, native host provenance, and machine-derived negative cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_distributed_guardian.sh"
    ],
    "parallel_group": "integration",
    "defer_reason": null
  },
  {
    "lane": "native-hosted-matrix",
    "proof_role": "Dispatch the exact branch head to standard macOS, Linux, and Windows GitHub runners, run the production producer once per platform, retain distinct receipt fragments, and aggregate them without substituting platform labels.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "gh",
      "workflow",
      "run",
      "wp04-native-distributed.yml",
      "--ref",
      "codex/5878-final-distributed-integration"
    ],
    "parallel_group": "native",
    "defer_reason": null
  },
  {
    "lane": "native-distributed-receipts",
    "proof_role": "Recompute all artifact and runner-provenance digests and require exactly one distinct native macOS, Linux, and Windows receipt at the same exact source revision.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_distributed_native_receipts.rb"
    ],
    "parallel_group": "native-verify",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Recompute exact protected source artifacts, nonzero command logs, machine-derived negative cases, and the three distinct native receipt bindings.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5878/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_guardian --no-tests=fail`
- `bash adl/tools/validate_v092_distributed_guardian.sh`
- `gh workflow run wp04-native-distributed.yml --ref codex/5878-final-distributed-integration`
- `ruby adl/tools/validate_v092_distributed_native_receipts.rb`
- `ruby .csdlc/prepared/issues/5878/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
