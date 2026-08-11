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
    "lane": "historical-c640-packet",
    "proof_role": "Verify the exact ten-path historical denominator, create a detached c640 worktree, overlay evidence, set the original GitHub environment, and run the unchanged #209 validator as provenance-only proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/verify-historical-c640-packet.rb",
      ".csdlc/prepared/issues/217/historical-c640-denominator.json"
    ],
    "parallel_group": "217-historical",
    "defer_reason": "Implemented only after the second independent full-package review passes."
  },
  {
    "lane": "fresh-native-producer-contract",
    "proof_role": "Prove the producer consumes the exact seventeen-path denominator and writes confined issue #217 artifacts.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/produce-native-receipt.rb",
      "--self-test"
    ],
    "parallel_group": "217-contract",
    "defer_reason": "Implemented only after the second independent full-package review passes."
  },
  {
    "lane": "fresh-native-linux-macos",
    "proof_role": "At reviewed producer head H, produce and aggregate the fresh exact Linux/macOS packet; retain only its exact ten-path denominator and evidence in H2 without protected drift or recursive triggering.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      ".csdlc/evidence/217/retained-proof-denominator.json"
    ],
    "parallel_group": "217-native",
    "defer_reason": "Runs on GitHub Actions at exact reviewed H after implementation review and publication; missing fresh proof blocks H2 retention and merge."
  },
  {
    "lane": "retained-proof-regressions",
    "proof_role": "Prove exact ten-path evidence and seventeen-path source denominators, complete provenance, ancestry/equivalence, protected drift, semantic, path-confinement, and tamper behavior.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/217/validate-retained-native-proof.rb",
      "--self-test"
    ],
    "parallel_group": "217-contract",
    "defer_reason": "Implemented only after the second independent full-package review passes."
  },
  {
    "lane": "preparation-contract",
    "proof_role": "Validate the two reviewed denominators and reject malformed preparation patches before implementation begins.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "jq",
      "-e",
      ".expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17",
      ".csdlc/prepared/issues/217/protected-source-denominator.json"
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

- `ruby .csdlc/prepared/issues/217/verify-historical-c640-packet.rb .csdlc/prepared/issues/217/historical-c640-denominator.json`
- `ruby .csdlc/prepared/issues/217/produce-native-receipt.rb --self-test`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb .csdlc/evidence/217/retained-proof-denominator.json`
- `ruby .csdlc/prepared/issues/217/validate-retained-native-proof.rb --self-test`
- `jq -e .expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17 .csdlc/prepared/issues/217/protected-source-denominator.json`

## Failure Semantics

Fail closed on missing or changed packet bytes, digest/provenance mismatch, unconfined paths, incomplete protected inventory, source relationship ambiguity, protected-source drift, stale typed truth, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
