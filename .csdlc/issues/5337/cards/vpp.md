# Validation Planning Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5337/design.md

Diagram: .csdlc/prepared/issues/5337/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-card-contracts",
    "proof_role": "Required preparation publication gate: validate all six rendered cards, active-template provenance, structure schemas, and canonical lifecycle consistency",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5337"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "issue-local-scope",
    "proof_role": "Required preparation publication gate: verify diff hygiene and absence of shared milestone or product implementation changes",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-review-evidence",
    "proof_role": "Required preparation publication gate: verify bounded subagent review at the substantive revision and typed dispositions for actionable findings",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5337"
    ],
    "parallel_group": "local-review",
    "defer_reason": null
  },
  {
    "lane": "future-corpus-proof",
    "proof_role": "Deferred non-preparation release gate: future product proof for versioned positive and negative fixtures, repeated v1 outcomes, normalization safety, coverage mapping, and nondeterminism disposition",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "characterization"
    ],
    "parallel_group": "future-implementation",
    "defer_reason": "Deferred and not executed: product implementation and focused proof are outside this preparation-only session and remain gated by #5336 plus separate authorization"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5337`
- `git diff --check`
- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5337`
- `cargo test --manifest-path adl/Cargo.toml characterization`

## Failure Semantics

Fail closed on non-current template provenance, invalid cards, generic or implementation-claiming prose, shared-path overlap, missing #5336 dependency, hidden normalization, skipped review, AWS/raw-gh use, or any product-scope execution.

## Handoff

Retain typed evidence before convergence.
