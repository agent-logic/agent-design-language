# Validation Planning Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/519/design.md

Diagram: .csdlc/prepared/issues/519/diagram.mmd

## Selected Lanes

[
  {
    "lane": "publication-linkage",
    "proof_role": "Prove exact and unambiguous publication and closing relationships.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--linkage"
    ],
    "parallel_group": "publication",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "exact-head",
    "proof_role": "Prove the packet and artifact digests bind the exact reviewed candidate revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--exact-head"
    ],
    "parallel_group": "publication",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "artifact-redaction",
    "proof_role": "Prove credentials, private payloads, and machine-local paths are absent.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/519/validate-publication-candidate.rb",
      "--redaction"
    ],
    "parallel_group": "publication",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact packet diff hygiene before review.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --linkage`
- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --exact-head`
- `ruby .csdlc/prepared/issues/519/validate-publication-candidate.rb --redaction`
- `git diff --check`

## Failure Semantics

Fail closed on unmet predecessor authority, revision drift, ambiguous linkage, redaction failure, or any attempted release mutation.

## Handoff

Retain typed evidence before convergence.
