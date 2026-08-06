# Validation Planning Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5819/design.md

Diagram: .csdlc/prepared/issues/5819/diagram.mmd

## Selected Lanes

[
  {
    "lane": "copy-evidence-contract",
    "proof_role": "Validate exact repository order and visibility, source-before/destination-after/source-after manifests, Git and LFS parity receipts, the complete non-Git surface disposition denominator, names-only secret and variable evidence, authenticated-confirmation identifiers, #5888 handoff, controls, and zero unexplained drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5819/validate-migration-evidence.rb"
    ],
    "parallel_group": "copy-contract",
    "defer_reason": null
  },
  {
    "lane": "github-copy-live-proof",
    "proof_role": "Recompute API-visible repository and configuration digests for all five sources, five destinations, and two controls; verify exact identities, visibility, branches, HEADs, refs, Actions state, and absent control destinations; and validate authenticated issue comments that bind organization readiness plus each repository's Actions-before-push, LFS, platform-disposition, and source-immutability receipts.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5819/verify-live-repositories.rb"
    ],
    "parallel_group": "github-live",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in tracked copy evidence and contract changes.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5819/validate-migration-evidence.rb`
- `ruby .csdlc/prepared/issues/5819/verify-live-repositories.rb`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
