# Structured Planning Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify copy-only organization gates, create exactly five destinations serially with Actions disabled before mirror push, prove Git/LFS parity and destination configuration, re-read every source for immutability, retain two negative controls, and hand website reference cleanup to #5888.

## Plan

Revision 20

## Steps

[
  {
    "id": "S1",
    "action": "Verify organization readiness and capture the exact seven-repository read-only preflight plus per-surface copy dispositions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Create and verify each destination serially with Actions disabled before mirror push and source-after immutability proof before continuing",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Record #5888 handoff, negative controls, final copy report, focused validators, and exact-revision review",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- All seven danielbaustin repositories remain present and unchanged
- Only five named agent-logic destinations may be created
- Four destinations are private and agent-design-language is public
- Destination Actions are disabled before mirror push
- One destination passes verification before the next starts
- Git and LFS parity are distinct from GitHub metadata reconstruction
- Secret values never enter retained evidence
- asksifu and Horust remain untouched

## Risks

- Mirror push triggers copied workflows before destination configuration is ready
- Git mirroring is falsely treated as issue, PR, release, settings, package, or integration parity
- Destination becomes an unintended divergent active repository
- Package, GitHub App, or organization Actions policy remains unaudited
- LFS objects are omitted despite ref parity
- A command targets the source push URL
- Concurrent source change invalidates the approved snapshot

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5819/design.md

Digest: 173fc8d7eb6d645d019cfe70be8610b8baf93f0f305b6c1a0ea73ce0a0d54b87

## Diagram

.csdlc/prepared/issues/5819/diagram.mmd

Digest: 683b91e14df1fe7f5e1c45427ae68b7610e33ab62d979d664fcfcbcb5042351e

## Stop Conditions

- WP-01B is not terminal before the ADL source snapshot
- Destination owner, billing, recovery, 2FA, security, or required capability gate is incomplete
- A destination name or required visibility conflicts
- Source-before drift is unexplained
- Destination Actions cannot be proven disabled before mirror push
- Push URL owner is not exactly agent-logic
- Git/LFS parity or destination configuration proof fails
- Source-after differs from source-before
- A secret value appears in retained evidence

## Handoff

Proceed only after doctor readiness.
