# Structured Planning Prompt

Template: 1.0.0

Issue: 661

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Trace the Shepherd-only reply branch, route it through existing configured provider execution, add deterministic success and failure proof, then review and publish.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Trace Shepherd conversation dispatch and the existing provider-backed resident-agent path.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Remove synthetic success and invoke the configured Shepherd provider through the governed boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused provider invocation, generated reply, correlation, and failure tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and independent exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No synthetic success
- Provider identity comes from configuration
- Reply envelope and correlation preserved
- Live Runtime untouched

## Risks

- Accidentally bypassing governed provider execution
- Provider errors masked as replies
- Breaking non-Shepherd conversations
- Confusing reply with agent-to-agent initiation

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/661/design.md

Digest: 68723dcb68dd267e3c67f4af99d34e1084b46e2c077ca19616c4d0f1c996ea21

## Diagram

.csdlc/prepared/issues/661/diagram.mmd

Digest: ed1f345a4ccd7d1b416e6c3548b5b8c553ef63d70f9e42d50eaf1e6aaf63f24a

## Stop Conditions

- Live restart becomes necessary
- Configured provider cannot be resolved without redesign
- Scope widens into initiation
- Review has unresolved findings

## Handoff

Proceed only after doctor readiness.
