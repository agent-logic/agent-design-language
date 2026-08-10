# Validation Planning Prompt

Template: 1.0.0

Issue: 143

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/143/design.md

Diagram: .csdlc/prepared/issues/143/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-adr-packet-contract",
    "proof_role": "Validate candidate numbering, status, required sections, index completeness, evidence links, source boundaries, and forbidden claims",
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
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/143/validate-v092-adrs.rb"
    ],
    "parallel_group": "adr-docs",
    "defer_reason": null
  },
  {
    "lane": "v092-adr-diff-hygiene",
    "proof_role": "Reject malformed documentation changes across the exact issue delta",
    "acceptance_ids": [
      "AC-9"
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
    "parallel_group": "adr-docs",
    "defer_reason": null
  },
  {
    "lane": "v092-adr-independent-review",
    "proof_role": "Fresh exact-head architecture, security-boundary, and documentation review of all candidate dispositions",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/143/review-request.json"
    ],
    "parallel_group": "pre-publication-review",
    "defer_reason": "Runs only after the complete candidate packet and deterministic validation are current."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/143/validate-v092-adrs.rb`
- `git diff --check origin/main...HEAD`
- `.adl/bin/csdlc-v2/csdlc-review --root . --request .csdlc/prepared/issues/143/review-request.json`

## Failure Semantics

Fail closed on number collision, missing required section, broken or non-repository evidence, unsupported architectural claim, accidental Accepted status, production cross-polis claim, unresolved review finding, or stale revision.

## Handoff

Retain typed evidence before convergence.
