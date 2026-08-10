# Structured Planning Prompt

Template: 1.0.0

Issue: 92

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Remove the custom local PKI, introduce one shared TLS policy/load layer, route Axum listeners and Quinn trust through it, align config/contracts/proofs, run focused security tests, obtain exact-head independent review, and publish one closing PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Remove production self-signed issuance, trust-store mutation, and obsolete local bootstrap surfaces.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement shared Rustls identity/trust policy and route all Axum HTTP/WSS listeners through it.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Reuse shared trust policy in Quinn and correct post-handshake mTLS identity semantics.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Reconcile init, OpenAPI, operational proof, browser, Unity, and architecture documentation.",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Add focused CA-chain, identity, mTLS, rotation, and negative regression coverage.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Obtain exact-head independent review and publish one ready PR closing issue 92.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Transport encryption and peer verification fail closed
- Guardian QUIC authorization remains stricter than transport liveness
- Browser and Unity remain public-server-TLS clients, not mTLS members
- Application authority is not conflated with X.509 transport identity
- Runtime never owns certificate issuance or host trust mutation

## Risks

- Deleting bootstrap behavior before every consumer is routed to external certificate material
- Treating server TLS and listener mTLS as interchangeable
- Changing Quinn identity semantics while consolidating policy
- Leaving stale proof or OpenAPI claims that overstate live transport security

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/92/design.md

Digest: 07c7b001c97e86d51837da6ec792c4dcea48e0fa95b9ccee2fe107881008980b

## Diagram

.csdlc/prepared/issues/92/diagram.mmd

Digest: b2c7062d0010a8ab635c076827fd71fee5b17f4d852d622d3f6e7d0181ac9844

## Stop Conditions

- A required change would replace Quinn or redesign Guardian consensus
- A proposed path requires Runtime-owned CA, ACME, trust-store mutation, or verification bypass
- An endpoint cannot be mapped to explicit public-TLS or private-mTLS policy
- Focused negative certificate validation does not fail closed

## Handoff

Proceed only after doctor readiness.
