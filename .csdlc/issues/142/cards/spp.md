# Structured Planning Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove real three-voter Runtime integration with configurable shepherd_agent_ref and bounded local models; run/show/tear down three Wuji voters first, then run one Wuji plus two AZ-separated AWS voters, commit and recover from a snapshot boundary, partition live Wuji, transfer fenced authority and the single Observatory to AWS, heal and demote Wuji, prove true one-of-three halt, then tear down all AWS and local resources before exact review and operator-gated merge.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Review the design and exact product ownership, dependencies, AWS boundary, and serial-state-machine contract.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement production Guardian/kernel distributed wiring, coherent API/WSS projection, and one polis Observatory.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement focused regressions and the durable serial runner with fail-closed teardown gates.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run and show Demo A on Wuji, then prove complete cleanup.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "After the cleanup gate only, run and show Demo B across Wuji and AWS, then prove full teardown.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Validate exact receipts, resolve fresh independent review, publish, shepherd CI, and merge only after operator demonstration approval.",
    "acceptance_ids": [
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly one Observatory per polis
- The two demos never overlap
- Existing distributed authority remains fail closed
- Node identity, ports, state, and credentials are never shared
- Remote transport and viewing are authenticated, encrypted, and private
- Evidence is exact-source, exact-argv, redacted, and nonzero-denominator bound

## Risks

- Production entrypoint wiring may expose missing authority adapters
- A false distributed demo could accidentally launch independent singletons
- Process, port, or durable-state residue could invalidate the serial boundary
- AWS connectivity could tempt public exposure or wrong-account use
- Observatory projection drift could show an incoherent polis cut
- Proof or reviewer identity could be under-qualified

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

- Exact production ownership cannot be made disjoint
- The launcher would bypass or weaken a merged authority
- Phase A cleanup cannot be proven
- agent-logic-admin does not resolve to the approved Agent Logic business account
- Private authenticated Wuji-AWS transport cannot be established
- One coherent polis Observatory cannot be produced
- A prerequisite defect requires separate scope

## Handoff

Proceed only after doctor readiness.
