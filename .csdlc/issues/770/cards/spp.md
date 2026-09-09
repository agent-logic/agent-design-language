# Structured Planning Prompt

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add plan-time recovery guards to public module; preserve isolated ingress and hardening; prove mocked negative/positive cases; prepare exact cloud plan, obtain approval, prove SSH/isolation, dispose, independently review and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement and test Terraform recovery guards",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Prepare approved live proof and retain disposal results",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Independent security review and publish",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- SSH and Runtime ingress remain separate.
- IMDSv2 and encrypted root storage remain enabled.
- Private-only roots remain unchanged.

## Risks

- Live proof cannot complete without existing key selection and explicit cloud apply approval.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/770/design.md

Digest: 63e11968d11e443f3236358abb34789b2e0cf7afc0b8372534462e30c8f209dd

## Diagram

.csdlc/prepared/issues/770/diagram.mmd

Digest: 23da22f79df6ac8cbdb6d42373b200b6d8887050c3453b36bca11b43c35e9c4b

## Stop Conditions

- Failed recovery contract or security review
- No explicit authorization for live AWS apply

## Handoff

Proceed only after doctor readiness.
