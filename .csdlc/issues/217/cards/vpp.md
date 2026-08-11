# Validation Planning Prompt

Template: 1.0.0

Issue: 217

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/217/design.md

Diagram: .csdlc/prepared/issues/217/diagram.mmd

## Selected Lanes

[
  {
    "lane": "source-native-packet",
    "proof_role": "Authenticate the restored source packet at detached source revision c640066f with the unchanged exact-source validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/209/validate-native-receipts.rb",
      ".csdlc/evidence/209/native-platform/macos.json",
      ".csdlc/evidence/209/native-platform/linux.json"
    ],
    "parallel_group": "217-source",
    "defer_reason": null
  },
  {
    "lane": "retained-native-proof",
    "proof_role": "Validate complete retained packet digests/provenance and ancestry or protected-tree equivalence at the current final head.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      ".csdlc/evidence/209/native-validation-manifest.json"
    ],
    "parallel_group": "217-retained",
    "defer_reason": "The issue-owned validator is implemented only after independent design approval and execution binding."
  },
  {
    "lane": "retained-proof-regressions",
    "proof_role": "Prove success and fail-closed behavior for squash equivalence, drift, tampering, missing files, provenance, semantics, and unrelated history.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      "--self-test"
    ],
    "parallel_group": "217-retained",
    "defer_reason": "The issue-owned regression mode is implemented only after independent design approval and execution binding."
  },
  {
    "lane": "preparation-contract",
    "proof_role": "Validate all six issue cards, design/diagram digests, path hygiene, and review/publication truth.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "217"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/209/validate-native-receipts.rb .csdlc/evidence/209/native-platform/macos.json .csdlc/evidence/209/native-platform/linux.json`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb .csdlc/evidence/209/native-validation-manifest.json`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb --self-test`
- `.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 217`

## Failure Semantics

Fail closed on missing or changed packet bytes, digest/provenance mismatch, unconfined paths, incomplete protected inventory, source relationship ambiguity, protected-source drift, stale typed truth, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
