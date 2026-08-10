# Structured Planning Prompt

Template: 1.0.0

Issue: 157

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Transfer approved repository, domain, brand, and vendor control by copy-first, non-destructive operations; preserve exact refs and visibility; prove asksifu and Horust were untouched; and retain explicit rollback/legacy dispositions.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Require terminal CORP-03 and CORP-04, freeze the seven-repository allowlist, the asksifu/Horust no-touch list, target visibility matrix, and all domain/vendor control surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Copy each approved repository to Agent Logic, verify immutable ref parity and ownership, keep Agent Design Language public, and keep every other company repository private.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Verify founder repositories remain present and unmodified; record legacy public-repository and redirect dispositions without deletion or destructive history edits.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Transfer or verify domains, registrar roles, brand/vendor ownership, Apps, webhooks, packages, Pages, OIDC, and repository references against the manifest.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Capture source/destination identity, exact refs, visibility, owner, provider readbacks, and rollback instructions in digest-bound receipts.",
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
    "action": "Run negative checks for same-name/fork conflicts, visibility drift, excluded-repository changes, missing control surfaces, and destructive source operations; stop before cutover on any failure.",
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
    "action": "Complete exact-head review and publish the redacted control manifest and copy/cutover runbook only after live readback matches every declared disposition.",
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

- Issue CORP-05 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/157/design.md

Digest: eeb3b8e93bc40236ab5ce5a0b28f65b63712ff384e18bda066e62542110c3227

## Diagram

.csdlc/prepared/issues/157/diagram.mmd

Digest: 4cf19da8205792d1c0e4cb0ba60348b1b1bed294af99066ecf7485cfd1fa7f1f

## Stop Conditions

- A source ref cannot be reproduced
- A same-name or fork conflict is unresolved
- A private repository would become public
- asksifu or Horust would change
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
