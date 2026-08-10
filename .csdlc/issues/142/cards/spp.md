# Structured Planning Prompt

Template: 1.0.0

Issue: 142

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review and bind exact production ownership, integrate real Guardian/kernel processes and one polis Observatory, prove Wuji-Wuji and clean it up, then prove Wuji-AWS and clean it up, followed by exact review, publication, and operator-gated merge.

## Plan

Revision 1

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

Digest: 0d7caadd58ae5112b92710a12ded77470d555c260609b1e2844d4f144ae93454

## Diagram

.csdlc/prepared/issues/142/diagram.mmd

Digest: d1f0e721fdaaf51515ac671b4ef380c4f71a237aa710dddcb2b0324687e70cef

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
