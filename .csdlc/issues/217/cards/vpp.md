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
    "lane": "historical-denominator-contract",
    "proof_role": "Require exactly ten unique historical evidence paths with SHA-256 digests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "jq",
      "-e",
      ".expected_file_count == 10 and (.files | length) == 10 and ([.files[].path] | unique | length) == 10 and ([.files[].sha256 | test(\"^[0-9a-f]{64}$\")] | all)",
      ".csdlc/prepared/issues/217/historical-c640-denominator.json"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  },
  {
    "lane": "protected-source-denominator-contract",
    "proof_role": "Require exactly seventeen unique protected source paths.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "jq",
      "-e",
      ".expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17",
      ".csdlc/prepared/issues/217/protected-source-denominator.json"
    ],
    "parallel_group": "217-prep",
    "defer_reason": null
  },
  {
    "lane": "preparation-diff-contract",
    "proof_role": "Reject malformed preparation patches or whitespace errors.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "git",
      "diff",
      "--check",
      "--",
      ".csdlc/issues/217",
      ".csdlc/prepared/issues/217"
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

- `jq -e .expected_file_count == 10 and (.files | length) == 10 and ([.files[].path] | unique | length) == 10 and ([.files[].sha256 | test("^[0-9a-f]{64}$")] | all) .csdlc/prepared/issues/217/historical-c640-denominator.json`
- `jq -e .expected_path_count == 17 and (.paths | length) == 17 and (.paths | unique | length) == 17 .csdlc/prepared/issues/217/protected-source-denominator.json`
- `git diff --check -- .csdlc/issues/217 .csdlc/prepared/issues/217`

## Failure Semantics

Fail closed on missing or changed packet bytes, digest/provenance mismatch, unconfined paths, incomplete protected inventory, source relationship ambiguity, protected-source drift, stale typed truth, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
