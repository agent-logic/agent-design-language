# Validation Planning Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5827/design.md

Diagram: .csdlc/prepared/issues/5827/diagram.mmd

## Selected Lanes

[
  {
    "lane": "birthday_continuity-runtime-v3",
    "proof_role": "Run the exact integration target with seven deterministic authority, replay, tamper, path, and discontinuity tests plus the sealed-policy doc proof.",
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
      "--test",
      "birthday_continuity",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "parallel_group": "5827-core",
    "defer_reason": null
  },
  {
    "lane": "birthday_continuity-macos-native-ci-producer",
    "proof_role": "Run the issue-local structured receipt producer on native macOS at the published exact head.",
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
      ".csdlc/prepared/issues/5827/produce-native-receipt.rb",
      "--platform",
      "macos",
      "--receipt",
      ".csdlc/evidence/5827/native-platform/macos.json",
      "--semantic-output",
      ".csdlc/evidence/5827/native-platform/macos-semantic.json"
    ],
    "parallel_group": "5827-native-produce",
    "defer_reason": "Mandatory publication-triggered proof; missing exact-head macOS receipt blocks final review and merge."
  },
  {
    "lane": "birthday_continuity-linux-native-ci-producer",
    "proof_role": "Run the issue-local structured receipt producer on native Linux at the published exact head.",
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
      ".csdlc/prepared/issues/5827/produce-native-receipt.rb",
      "--platform",
      "linux",
      "--receipt",
      ".csdlc/evidence/5827/native-platform/linux.json",
      "--semantic-output",
      ".csdlc/evidence/5827/native-platform/linux-semantic.json"
    ],
    "parallel_group": "5827-native-produce",
    "defer_reason": "Mandatory publication-triggered proof; missing exact-head Linux receipt blocks final review and merge."
  },
  {
    "lane": "birthday_continuity-native-ci-receipt-verification",
    "proof_role": "Recompute exact-head receipt, manifest, log, inventory, semantic digests and workflow provenance, then require macOS/Linux equivalence.",
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
      ".csdlc/prepared/issues/5827/validate-native-receipts.rb",
      ".csdlc/evidence/5827/native-platform/macos.json",
      ".csdlc/evidence/5827/native-platform/linux.json"
    ],
    "parallel_group": "5827-native-verify",
    "defer_reason": "Mandatory publication-triggered verification; missing or mismatched native receipts block final review and merge."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test birthday_continuity --no-tests=fail --status-level all`
- `ruby .csdlc/prepared/issues/5827/produce-native-receipt.rb --platform macos --receipt .csdlc/evidence/5827/native-platform/macos.json --semantic-output .csdlc/evidence/5827/native-platform/macos-semantic.json`
- `ruby .csdlc/prepared/issues/5827/produce-native-receipt.rb --platform linux --receipt .csdlc/evidence/5827/native-platform/linux.json --semantic-output .csdlc/evidence/5827/native-platform/linux-semantic.json`
- `ruby .csdlc/prepared/issues/5827/validate-native-receipts.rb .csdlc/evidence/5827/native-platform/macos.json .csdlc/evidence/5827/native-platform/linux.json`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
