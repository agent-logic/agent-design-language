# Validation Planning Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/282/design.md

Diagram: .csdlc/prepared/issues/282/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-dependency-caches",
    "proof_role": "Validate canonical terminal caches for #279, #280, and #281 before #282 review/publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-finish",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof",
      "--validate-cached-issue",
      "279"
    ],
    "parallel_group": "282-serial-01-terminal",
    "defer_reason": null
  },
  {
    "lane": "qualification-packet-validator",
    "proof_role": "Validate the #282 exact-revision qualification packet, runbook, review retention, residual-risk, and non-claim sections.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/evidence/282/validate_qualification_packet.py",
      ".csdlc/evidence/282/production-polis-interface-qualification.md"
    ],
    "parallel_group": "282-serial-02-packet",
    "defer_reason": null
  },
  {
    "lane": "typed-issue-validation",
    "proof_role": "Validate #282 lifecycle/card truth after bound implementation evidence is recorded.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-282-production-polis-qualification",
      "issue",
      "--issue",
      "282"
    ],
    "parallel_group": "282-serial-03-lifecycle",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace, conflict marker, and patch hygiene defects before exact-head review.",
    "acceptance_ids": [
      "AC-5"
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
    "parallel_group": "282-serial-04-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-finish --root /Volumes/FastWork/adl-worktrees/adl-issue-279-observatory-accessibility-responsive-ux-proof --validate-cached-issue 279`
- `python3 .csdlc/evidence/282/validate_qualification_packet.py .csdlc/evidence/282/production-polis-interface-qualification.md`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root /Volumes/FastWork/adl-worktrees/adl-issue-282-production-polis-qualification issue --issue 282`
- `git diff --check`

## Failure Semantics

Fail closed on stale terminal evidence, missing exact revisions, out-of-scope implementation claims, validator failure, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
