# Structured Planning Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind current main, transplant only the trusted-origin follow-up, harden exact-origin parsing, make the HTML Observatory trust the configured Wuji Runtime host, make the health endpoint obey the same configured-origin CORS policy as the other Observatory reads, add executable tests, retain live browser proof, obtain fresh review, and publish one closing PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue #550 from current main and transplant only the trusted-origin follow-up delta.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Harden exact public-origin validation and add executable valid and invalid generation tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Make the HTML Observatory trusted Runtime host config-owned and prove the configured Wuji host is accepted while arbitrary hosts fail.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Apply the configured-origin CORS policy to Runtime health and prove the real Observatory three-endpoint connection.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused local validation and retain bounded live Wuji trusted-TLS and CORS evidence.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Obtain fresh exact-head review, fix all findings, publish, shepherd green checks, merge, finish, and clean.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Only exact origins are emitted
- No self-signed certificate is introduced
- Invalid input cannot replace generated Runtime config
- The HTML Observatory does not trust arbitrary Runtime v3 hosts
- The merged #540 history remains immutable
- HOT-01 remains the sole dynamic-reload owner

## Risks

- Shell parsing could accept path or credential syntax
- A port outside the valid TCP range could pass shallow validation
- Static source assertions could falsely pass without executing generation
- A stale hard-coded HTML trusted host could reject the live Wuji runtime
- Transplanting the old branch could delete newer main work

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/550/design.md

Digest: ca3d5ecab77e251dfceec25366d2c2f2174b3e817337ed998b77c9026ecaad83

## Diagram

.csdlc/prepared/issues/550/diagram.mmd

Digest: a46fdc97ade937612faa0e9a6053b6e3b4472b17abf13b4968d6762c5ce8e1ce

## Stop Conditions

- The delta expands beyond the three declared source/docs/test files
- Current main cannot be preserved cleanly
- Executable invalid-input proof is absent
- Live 8765 TLS is not browser-trusted
- Fresh review finds unresolved actionable issues

## Handoff

Proceed only after doctor readiness.
