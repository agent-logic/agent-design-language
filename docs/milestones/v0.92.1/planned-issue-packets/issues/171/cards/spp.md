# Structured Planning Prompt

Template: 1.0.0

Issue: 171

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Deliver idempotent local issue initialization/observation and execution binding that authorizes only exact observed canonical branch/worktree topology through direct flags or exclusive typed input.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define direct-flag and typed --input schemas, exclusivity, JSON/human parity, and idempotent outcomes for issue and bind commands.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement local issue initialization/show over canonical state and transaction storage without branch-name ownership inference.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement bind using observed canonical repo/branch/worktree topology and exact issue identity, not requested path strings.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Encode same-issue, cross-issue, main-branch, missing, dirty-policy, drift, symlink, and duplicate-registration collision outcomes.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run end-to-end local fixtures for direct flags, typed input, retries, all collision classes, and human/JSON equivalence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain topology proof and stop on ambiguous identity, hidden request files, or requested-not-observed authorization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Remove only issue-owned topology fixtures and prove failed binds left no partial registration, branch, worktree, lock, or state mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-10A owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/171/design.md

Digest: c2518ee14ef6f54cd5a6fbfbe1a4e3656bfa868327156118bc70da4fd6a8e2e4

## Diagram

.csdlc/prepared/issues/171/diagram.mmd

Digest: 3c9d5477db099cca3050bbc9bb85c08b19255fd0ab16a0660a6124b31469b0a8

## Stop Conditions

- Binding trusts requested rather than observed topology, repository identity is ambiguous, or common use still requires request files.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
