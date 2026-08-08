# Validation Planning Prompt

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/27/design.md

Diagram: .csdlc/prepared/issues/27/diagram.mmd

## Selected Lanes

[
  {
    "lane": "native-receipt-policy-focused",
    "proof_role": "Prove role order independence, duplicate rejection, verifier-only allowance, rename safety, clean-worktree enforcement, and ancestry rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_runtime_native_receipts.rb",
      "--self-test-policy"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "native-receipt-packet-integration",
    "proof_role": "Preserve digest recomputation and exact platform denominator checks against the final WP-03 proof revision.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "adl/tools/validate_v092_runtime_native_receipts.rb",
      ".csdlc/evidence/5820/runtime-native-receipts.json"
    ],
    "parallel_group": "issue-local",
    "defer_reason": "WP-03 is generating final exact-head macOS and Linux native proofs; the existing packet predates later product changes and correctly fails closed."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in the bounded validator delta.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
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

Seconds: 1200

Tokens: 10000

## Commands

- `ruby adl/tools/validate_v092_runtime_native_receipts.rb --self-test-policy`
- `ruby adl/tools/validate_v092_runtime_native_receipts.rb .csdlc/evidence/5820/runtime-native-receipts.json`
- `git diff --check`

## Failure Semantics

Fail closed on malformed revisions, duplicate roles, denominator drift, disallowed Git paths, or any retained receipt validation failure.

## Handoff

Retain typed evidence before convergence.
