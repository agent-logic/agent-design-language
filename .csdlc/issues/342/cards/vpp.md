# Validation Planning Prompt

Template: 1.0.0

Issue: 342

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/342/design.md

Diagram: .csdlc/prepared/issues/342/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-342-focused",
    "proof_role": "Ten-package completeness, audio and metadata parity, redaction, playback, and ownership-boundary validation",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      "adl/tools/test_podcast_launch_packet.sh"
    ],
    "parallel_group": "sprint8-issue-342",
    "defer_reason": "Deferred until the issue is bound, all declared dependencies pass, and the owned validator or proof target is implemented; missing target, zero tests, or any failure blocks publication."
  },
  {
    "lane": "issue-342-diff-hygiene",
    "proof_role": "Reject malformed tracked changes before exact-head review.",
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
    "parallel_group": "sprint8-hygiene",
    "defer_reason": "Run after the issue has a bounded candidate diff."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_podcast_launch_packet.sh`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, ownership, authority, privacy, validation, exact-revision, or review drift; preserve evidence and route separate defects without widening the issue.

## Handoff

Retain typed evidence before convergence.
