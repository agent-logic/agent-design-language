# Validation Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/518/design.md

Diagram: .csdlc/prepared/issues/518/diagram.mmd

## Selected Lanes

[
  {
    "lane": "canonical-doc-inventory",
    "proof_role": "Verify the 737-document baseline inventory and hashes; final candidate must be refreshed after issue 517.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--inventory"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "link-check",
    "proof_role": "Check extracted local inline Markdown links in the audited documents and packet; remote reachability, bare refs and final acceptance are outside this local check.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--links"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "claim-audit",
    "proof_role": "Verify retained diagnostic/creation snapshot parity, 15 finding dispositions and explicit pending final acceptance; not product proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--claims"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove repository diff hygiene for the exact documentation candidate.",
    "acceptance_ids": [
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
  },
  {
    "lane": "final-handoff-gate",
    "proof_role": "Fail closed until issue 517 passing reviewed merge and final candidate independent review have been incorporated.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/518/validate-documentation-handoff.rb",
      "--final"
    ],
    "parallel_group": "final",
    "defer_reason": "Issue 517 is still in flight; preliminary local checks are not final acceptance."
  },
  {
    "lane": "cargo-manifest-inventory",
    "proof_role": "Audit all tracked Cargo manifests: TOML, package and inherited version identity, local dependency paths, offline no-deps Cargo workspace metadata. Not dependency builds, security, lock reproducibility or release-version approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/518/audit-cargo-manifests.py",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --inventory`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --links`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --claims`
- `git diff --check`
- `ruby .csdlc/prepared/issues/518/validate-documentation-handoff.rb --final`
- `python3 .csdlc/prepared/issues/518/audit-cargo-manifests.py --check`

## Failure Semantics

Fail closed on an unmet predecessor, incomplete document denominator, unresolved input, unsupported claim, hidden residual risk, or candidate drift.

## Handoff

Retain typed evidence before convergence.
