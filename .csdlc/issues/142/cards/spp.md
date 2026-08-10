# Structured Planning Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove real three-voter Runtime integration with configurable shepherd_agent_ref and bounded local models; run/show/tear down three Wuji voters first, then run one Wuji plus two AZ-separated AWS voters, commit and recover from a snapshot boundary, partition live Wuji, transfer fenced authority and the single Observatory to AWS, heal and demote Wuji, prove true one-of-three halt, then tear down all AWS and local resources before exact review and operator-gated merge.

## Plan

Revision 13

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the exact three-voter ownership, configurable shepherd, governed AWS standby admission, all-node local-model profiles, consensus snapshot, quorum, Observatory lease, two-AZ AWS, and teardown design; resolve every Gemini plan finding before binding.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-15",
      "AC-16"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Prove the supported local-model serving path on Wuji and ephemeral AWS, select bounded smaller Wuji models, pin artifacts/config identities for every Phase B voter, and retain private endpoint, resource-isolation, restart, and teardown contracts.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-8",
      "AC-14"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement real Guardian/kernel distributed launch, configured shepherd_agent_ref, one quorum-leased movable polis Observatory, durable serial-runner states, and fail-closed cleanup gates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-11",
      "AC-14"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement the governed AWS shepherd-admission mutation, durable inference/snapshot-boundary/materialization/compaction/restart/catch-up/leader-election/lease-expiry/fence/owner/shepherd-activation states, exact two-AWS catch-up and identical canonical snapshot proof, serial AWS snapshot-root restart proof, bounded local-model saturation with consensus timing assertions, bridge-free asymmetric Wuji partition, post-TTL stale-read denial, premature takeover denial, post-failure mutation, healed-partition Wuji demotion/synchronization, re-partition and opposite-AZ voter loss, and focused adversarial tests.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12",
      "AC-13",
      "AC-14",
      "AC-17"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run Phase A alone with three Wuji voters and smaller configured local models, show the single live polis Observatory, exercise one-voter loss and recovery, then prove complete teardown.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-15",
      "AC-16"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Only after Phase A cleanup, verify the AWS account and run Phase B with one Wuji and two AZ-separated AWS voters, each using a pinned bounded local model; commit AWS shepherd admission and snapshot boundary, prove exact AWS catch-up and identical local snapshot materialization plus inference isolation, asymmetrically partition live Wuji, prove old Observatory lease expiry and safe AWS authority/Observatory recovery, heal the partition and prove Wuji demotion/synchronization, then re-partition Wuji and stop the opposite-AZ AWS voter to prove one-of-three halt.",
    "acceptance_ids": [
      "AC-5",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12",
      "AC-13",
      "AC-15",
      "AC-16"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Tear down all hybrid compute, model, storage, and network resources; validate merge-compatible exact receipts and protected-source drift denial, resolve independent implementation review, publish, shepherd CI, and merge only after operator approval.",
    "acceptance_ids": [
      "AC-14",
      "AC-15",
      "AC-16",
      "AC-18",
      "AC-19"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly three authenticated voters form each test polis and exactly one quorum-leased Observatory represents it.
- The shepherd is selected by shepherd_agent_ref, launches with the polis, has a distinct governed identity, remains outside voter membership, and cannot mint distributed authority.
- Every configured model endpoint is private, pinned, bounded, health-checked, and isolated from consensus resources; model quality does not alter authority decisions.
- Phase A and Phase B never overlap, and Phase B cannot start without a complete Phase A cleanup receipt.
- Snapshot continuity uses one quorum-committed boundary and byte-identical canonical voter materializations; no manual transfer or uncommitted history is accepted.
- Wuji remains alive during asymmetric partition; AWS recovery requires two AWS votes, a newer term, safety-window completion, old Observatory lease expiry, fencing, owner activation, then shepherd activation.
- After healing, stale Wuji cannot mutate, renew authority, own the Observatory, or serve stale reads until governed demotion and synchronization complete.
- With Wuji re-partitioned and one AWS voter stopped, the remaining one-of-three voter halts mutation while the surviving shepherd reports the loss of quorum.
- Remote transport and viewing are authenticated, encrypted, private, redacted, exact-source bound, and fully torn down.

## Risks

- Production Guardian/kernel entrypoints may lack adapters needed to compose the already-merged distributed authorities without bypassing them.
- Three local model processes may exhaust Wuji memory or CPU and starve heartbeats unless per-node profiles and hard resource bounds are enforced.
- AWS model cold start, storage attachment, or inference saturation may exceed bounded election and recovery windows.
- A launcher could accidentally present independent singletons or multiple node-local views as one polis Observatory.
- A snapshot, stale local history, or same-epoch authority transition could be accepted without quorum-committed provenance.
- The failure harness could accidentally bridge the Wuji-AWS partition or restart Wuji, invalidating the continuity demonstration.
- Process, port, lease, lock, credential, volume, network, or model-cache residue could invalidate seriality or leave cost/security exposure.
- Evidence could omit exact model, shepherd, node, snapshot, AWS account/AZ, lease, cleanup, or live-Observatory provenance.

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/142/design.md

Digest: 3ef387ed9ba4563c10810ce21e9ea20c6eb72ccb9a9c09ba5e834e16533137ff

## Diagram

.csdlc/prepared/issues/142/diagram.mmd

Digest: 1513aa03f11436453510f0fa0edf7eed81c05436eafbbfa940dc36eae6afb5c6

## Stop Conditions

- The declared production ownership cannot be made disjoint or requires weakening a merged authority contract.
- shepherd_agent_ref cannot be configured and launched without making the shepherd a voter or granting authority-minting capability.
- Three bounded local model instances cannot coexist on Wuji without starving consensus, or AWS local models require public or hosted inference.
- Phase A cleanup cannot be proven exactly, so Phase B must not start.
- agent-logic-admin does not resolve to the approved Agent Logic business account, two distinct Availability Zones are unavailable, or private authenticated connectivity cannot be established.
- The hybrid snapshot is not quorum committed and independently materialized identically by all healthy voters, or either AWS voter cannot restart from it while quorum remains.
- The partition harness can bridge Wuji and AWS, or recovery requires stopping/restarting Wuji rather than keeping it alive and stale.
- Exactly one coherent quorum-leased Observatory cannot be shown before and after recovery.
- Any cleanup predicate remains false, exact proof cannot bind the live demonstrations, or a prerequisite defect requires separate scope.

## Handoff

Proceed only after doctor readiness.
