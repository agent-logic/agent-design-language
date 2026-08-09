# Validation Planning Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/83/design.md

Diagram: .csdlc/prepared/issues/83/diagram.mmd

## Selected Lanes

[
  {
    "lane": "html-observatory-live-browser",
    "proof_role": "Prove the real browser against Runtime v3 HTTPS/WSS, including controls, redaction, refusal, disconnect, and reconnect.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "node",
      "adl/tools/validate_v092_html_observatory_live.mjs"
    ],
    "parallel_group": "live-browser",
    "defer_reason": "This validator is an issue #83 implementation deliverable and requires the exact live Runtime candidate; it becomes mandatory after the target is created."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed changes and path-boundary drift.",
    "acceptance_ids": [
      "AC-7"
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
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `node adl/tools/validate_v092_html_observatory_live.mjs`
- `git diff --check`

## Failure Semantics

Fail closed into an explicit read-only, stale, denied, version-mismatch, or unavailable state; never synthesize live success.

## Handoff

Retain typed evidence before convergence.
