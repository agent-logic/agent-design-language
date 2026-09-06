# Validation Planning Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Focused validation must prove PR #615 closing-link truth, zero net csdlc-v2 source/test diff, v3 real-issue canary behavior, typed v2 structural validity, and diff hygiene before publication.

## Lane Inputs

Design: .csdlc/prepared/issues/596/design.md

Diagram: .csdlc/prepared/issues/596/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-596-remediation-regression",
    "proof_role": "Prove #596 local lifecycle, PR #615 closing linkage, non-closing #505/#534 linkage, and zero csdlc-v2 source/test diff through an issue-owned validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/596/validate-remediation-regression.sh"
    ],
    "parallel_group": "policy",
    "defer_reason": null
  },
  {
    "lane": "v3-real-issue-canary",
    "proof_role": "Prove the v3 canary surface still exercises real issue records without granting v3 lifecycle authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2200,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "typed-v2-structural-validation",
    "proof_role": "Prove #596 remains structurally valid under the current live v2 lifecycle authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "issue",
      "--issue",
      "596"
    ],
    "parallel_group": "policy",
    "defer_reason": null
  },
  {
    "lane": "exact-range-diff-hygiene",
    "proof_role": "Prove origin/main...HEAD has no whitespace errors before publication.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "policy",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/596/validate-remediation-regression.sh`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test real_issue_canary`
- `.adl/bin/csdlc-v2/csdlc-validate issue --issue 596`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed if PR #615 cannot visibly close #596, if #505/#534 would be closed, if any csdlc-v2 source/test file appears in the origin/main...HEAD diff, if typed validation fails, or if v3 canary evidence is stale.

## Handoff

Retain typed evidence before convergence.
