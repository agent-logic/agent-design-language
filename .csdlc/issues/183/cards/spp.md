# Structured Planning Prompt

Template: 1.0.0

Issue: 183

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After terminal #142 proof, launch the real Wuji three-voter topology, prove governed 3-to-2-to-1 behavior, lease fencing, snapshot/restart continuity, replay, and verified cleanup at one exact revision.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Fail closed unless issue #142 is terminal, its merge SHA is ancestral to the tested revision, and its retained Guardian, API, WSS, and WP-04.16 proofs verify at their recorded digests.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Launch three independent production voters with distinct identities, credentials, ports, state roots, and processes; attach three governed agents, one non-voting Shepherd, and one quorum-leased Observatory without direct executor bypass.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Drive governed work through three-voter quorum, remove one voter and prove two-voter continuity, then remove a second voter and prove one-voter mutation denial with term, commit-index, and state-digest receipts.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Expire the old Observatory lease before successor binding, attempt stale-owner writes, and retain producer denial and fencing receipts tied to the active term and lease epoch.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Create a snapshot, terminate and restart a voter from independently materialized state, verify agent identity continuity and committed-state parity, then replay retained inputs and compare exact digests.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "After every phase and failure path, stop agents, Shepherd, Observatory, and voters; verify process, port, state-root, and provider cleanup before advancing.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Run the issue-specific validator and production proof, then obtain independent exact-head review; retain the terminal #142 gate and never substitute an open PR or in-process service object.",
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

- Issue DRT-03 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/183/design.md

Digest: 26a25176e2fdb18edf6579258ad14ae345c0818f305618bcdac843cfe3683054

## Diagram

.csdlc/prepared/issues/183/diagram.mmd

Digest: 469ca4f85682066c7b50970aa1824c2a777ce2ac0b1d3872948b778c125b8f9d

## Stop Conditions

- #142 is not terminal with passing retained proof
- Any voter shares identity or state
- One-voter mutation succeeds
- Cleanup cannot be verified
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
