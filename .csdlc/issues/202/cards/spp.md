# Structured Planning Prompt

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #191 and #201 merge, bind and implement one token-authorized replication-only learner topology plus shared pending exclusion, prove it over real Quinn/OpenRaft with restart and fault coverage, independently review it, and publish a ready unmerged PR before releasing #199.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #191 and #201 merge ancestrally, bind #202 and freeze exact learner token payload, role-bound session, RPC allowlist, pending exclusion, recovery exception, checkpoint, and publication contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement only the learner topology/session, shared pending-exclusion authority, and narrow existing transport/authority-protocol/PolisRuntime integration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove real fourth-node replication, denied authority, exclusion, governed recovery, reconnect, rotation, exact retry, crash, rollback, corruption, capacity, and path safety.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a ready PR closing #202, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Adding a learner never changes voter membership, quorum, stable voter ids, or ordinary voter route authority
- No caller, config, address, certificate alone, local state, or public constructor can create learner or exclusion authority
- A learner can replicate only and cannot vote or invoke any authority/client/serving operation
- Pending exclusion denies ordinary authority from its published generation onward and only an exact later token may grant replication-only recovery
- No admission or exclusion becomes visible before state, result, checkpoint, and published generation agree

## Risks

- Dynamic learner routing could accidentally weaken the strict three-voter authority cut
- A learner session could expose vote, client, endorsement, renewal, Shepherd, or Observatory RPCs
- Pending exclusion could be bypassed by a retained connection or stale retry namespace
- Recovery exception could silently restore ordinary voter authority
- Crash or rotation could publish an admission/exclusion without checkpoint parity
- Scope could drift into #199 membership coordination or #200 concrete stores

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/202/design.md

Digest: 991190e1de38edbc733c1af6fba81152162d22008fd60dbdc60bfd23fdc17832

## Diagram

.csdlc/prepared/issues/202/diagram.mmd

Digest: d8ba7967bc5778344da929c16608c0460e8f6be5d3555e72aa32c5dd69fdcf94

## Stop Conditions

- PR #197 or #201 is not externally reviewed, merged, and ancestral
- Learner admission would require caller-selected route authority or a parallel transport stack
- Existing voter cut or quorum semantics would be weakened
- The role-bound session cannot reject vote and all authority/client operations before dispatch
- Pending exclusion cannot be shared with #201 and ordinary transport admission without public self-attestation
- Implementation expands into #199, #200, Guardian/API/WSS, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
