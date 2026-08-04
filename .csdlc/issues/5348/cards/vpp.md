# Validation Planning Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5348/design.md

Diagram: .csdlc/prepared/issues/5348/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-doctor-5348",
    "proof_role": "Validate #5348 canonical record, six generated cards, design and diagram digests, and bound claim truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5348"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "request-driven-preparation-validation",
    "proof_role": "Run the issue-local csdlc-validate request for #5348 preparation proof without finalizing or executing the ceremony.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5348/validate-preparation.json"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove the preparation diff has no whitespace or patch hygiene errors.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
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
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "future-predecessor-live-merge-ancestry",
    "proof_role": "Future execution gate: observe #5359 live merge and verify its merge SHA is ancestral to the exact #5348 execution base.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "future-execution-gate",
      "observe-5359-live-merge-and-git-merge-base-is-ancestor"
    ],
    "parallel_group": "blocked-future",
    "defer_reason": "Deferred because #5359 is still open as of 2026-08-04 and #5348 ceremony execution is out of scope for preparation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5348`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . --request .csdlc/prepared/issues/5348/validate-preparation.json`
- `git diff --check`
- `future-execution-gate observe-5359-live-merge-and-git-merge-base-is-ancestor`

## Failure Semantics

Fail closed on missing live predecessor merge, stale ancestry, incomplete release evidence, or hidden implementation need.

## Handoff

Retain typed evidence before convergence.
