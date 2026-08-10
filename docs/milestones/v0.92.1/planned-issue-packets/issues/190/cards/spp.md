# Structured Planning Prompt

Template: 1.0.0

Issue: 190

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After release closeout, publish a portable next-milestone handoff that accounts for all terminal evidence, residual risks, deferred work, and the still-gated V3-R01 retirement eligibility decision.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Resolve the exact terminal INT-02 and release-closeout revisions and inventory every accepted release artifact, evidence digest, residual risk, and rollback-window metric.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the deferred-work register with stable identifiers, owners, dependencies, required proofs, target routing, and explicit incomplete status for every item.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Evaluate V3-R01 eligibility against rollback-window expiry, stability thresholds, historical readability proof, and explicit operator approval; retain ineligible status when any gate is absent.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Remove chat, absolute machine paths, transient worktree references, and untracked state from the handoff; verify every source and evidence link is repository-portable.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run the issue validator over terminal references, residual/deferred denominators, ownership, eligibility gates, digests, portability, and placeholders.",
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
    "action": "Obtain independent exact-head handoff review and publish without starting the downstream milestone, deleting v2, or rewriting v0.92.1 history.",
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

- Issue INT-03 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/190/design.md

Digest: e46b767906a5c65aa50729bc1faefb52783e36721c76a521cb3a275af5defc64

## Diagram

.csdlc/prepared/issues/190/diagram.mmd

Digest: 8be8160e7d214f92b42ef8b4873801816eb8dfd5d53a268108fc1c5c38bf2fe0

## Stop Conditions

- A residual risk or deferred item lacks an owner
- Rollback window is still active for V3-R01
- The packet depends on untracked or machine-local state
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
