# Validation Planning Prompt

Template: 1.0.0

Issue: 5826

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5826/design.md

Diagram: .csdlc/prepared/issues/5826/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday_identity-runtime-v3",
    "proof_role": "Run the crate-internal authority-context proof; external callers must be unable to establish self-consistent attacker trust roots, while canonical signed lineage and governed projection cases remain proven.",
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
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "-E",
      "test(/^birthday_identity::authority_tests::/)",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "5826-core",
    "defer_reason": null
  },
  {
    "lane": "birthday_identity-macos-native-ci-producer",
    "proof_role": "Run the issue-local receipt producer on native macOS at the repaired exact candidate HEAD and retain the full log, authority source manifest, passed-test inventory, and semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5826/produce-native-receipt.rb",
      "--platform",
      "macos",
      "--receipt",
      ".csdlc/evidence/5826/native-platform/macos.json",
      "--semantic-output",
      ".csdlc/evidence/5826/native-platform/macos-semantic.json"
    ],
    "parallel_group": "5826-native-produce",
    "defer_reason": "Fresh exact-head native GitHub Actions proof is required after the trust-bootstrap boundary repair."
  },
  {
    "lane": "birthday_identity-linux-native-ci-producer",
    "proof_role": "Run the issue-local receipt producer on native Linux at the repaired exact candidate HEAD and retain the full log, authority source manifest, passed-test inventory, and semantic output.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5826/produce-native-receipt.rb",
      "--platform",
      "linux",
      "--receipt",
      ".csdlc/evidence/5826/native-platform/linux.json",
      "--semantic-output",
      ".csdlc/evidence/5826/native-platform/linux-semantic.json"
    ],
    "parallel_group": "5826-native-produce",
    "defer_reason": "Fresh exact-head native GitHub Actions proof is required after the trust-bootstrap boundary repair."
  },
  {
    "lane": "birthday_identity-native-ci-receipt-verification",
    "proof_role": "Independently recompute exact HEAD, producer, source-manifest, complete internal-test log, machine-derived passed tests, and semantic-output digests; verify GitHub workflow/run identity and require macOS/Linux equivalence.",
    "acceptance_ids": [
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5826/validate-native-receipts.rb",
      ".csdlc/evidence/5826/native-platform/macos.json",
      ".csdlc/evidence/5826/native-platform/linux.json"
    ],
    "parallel_group": "5826-native-verify",
    "defer_reason": "Blocked until both repaired exact-head producer receipts exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --lib -E test(/^birthday_identity::authority_tests::/) --no-tests=fail --status-level all`
- `ruby .csdlc/prepared/issues/5826/produce-native-receipt.rb --platform macos --receipt .csdlc/evidence/5826/native-platform/macos.json --semantic-output .csdlc/evidence/5826/native-platform/macos-semantic.json`
- `ruby .csdlc/prepared/issues/5826/produce-native-receipt.rb --platform linux --receipt .csdlc/evidence/5826/native-platform/linux.json --semantic-output .csdlc/evidence/5826/native-platform/linux-semantic.json`
- `ruby .csdlc/prepared/issues/5826/validate-native-receipts.rb .csdlc/evidence/5826/native-platform/macos.json .csdlc/evidence/5826/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
