# Structured Planning Prompt

Template: 1.0.0

Issue: 185

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Exercise the distributed security and failure matrix across separated identities, approved TLS and mTLS trust, capabilities, leases, cross-polis messages, malformed traffic, pre-auth surfaces, and provider failures.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the identity and trust-domain matrix for voters, governed agents, Shepherd, operator, and Observatory; verify distinct keys and roles and prove Shepherd has no voting authority.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory every production TLS and mTLS endpoint and verify its chain to an approved trust anchor, hostname and peer identity binding, revocation posture, and absence of self-signed production certificates.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Generate and execute forged, stale, wrong-domain, missing-capability, cross-polis, malformed, and pre-auth disclosure attempts through production ingress, retaining typed denial receipts for each exact input.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Inject provider timeout, stall, malformed output, and partial failure while recording pre/post committed state, authority, lease, and resource invariants.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run the issue validator over the exact security denominator, certificate evidence, input/output digests, denial receipts, and invariant snapshots; fail on missing classes or aggregate self-attestation.",
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
    "action": "Complete independent exact-head security review, remediate all blocking findings, and publish without widening into certificate issuance or unrelated Runtime redesign.",
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

- Issue DRT-05 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/185/design.md

Digest: c82ccdf59d8ad146aa7b890df3fc4026b3eb9c435db852d35b03cbd1798fce20

## Diagram

.csdlc/prepared/issues/185/diagram.mmd

Digest: bf58756f4081f0bee0a74d9685fcf68bf2e628fce343982e80c519302f060a24

## Stop Conditions

- Any role shares an unauthorized key
- A production path accepts self-signed TLS
- A denied operation mutates state
- Receipt totals are not producer-derived
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
