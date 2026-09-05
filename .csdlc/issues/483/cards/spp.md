# Structured Planning Prompt

Template: 1.0.0

Issue: 483

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #483, write the read-only custody register/action list and redacted receipts, validate denominator/redaction/domain receipt truth, obtain exact-head review, fix findings, and publish a PR closing #483.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind #483 and confirm the merged CORP-A denominator and domain receipt inputs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Write the custody register, redacted receipts, and concise action list without external mutations.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused validation, exact-head review, fixes, and PR publication.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- No external service mutation occurs in #483.
- Credentials and private recovery material remain outside repository custody.
- Follow-up-required rows are not represented as custody-complete.
- v-*.ai backlog domains are not scheduled or used as milestone gates.

## Risks

- The original issue ACs imply live recovery/break-glass proof, but the narrowed boundary forbids those operations.
- A read-only register can be mistaken for completed operational transfer unless statuses are explicit.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/483/design.md

Digest: 42a96e269c8502c070599bb4204a7774672065e435e83852c35b5682220b9d3b

## Diagram

.csdlc/prepared/issues/483/diagram.mmd

Digest: 2c9ec19c6c0f743bf503f146842feebefcf541165b176d9736786e12bbbc223c

## Stop Conditions

- Validation finds credential-like or PII-like material in tracked artifacts.
- The register cannot cover the CORP-A denominator.
- Publication would require live service mutation or overclaiming completed recovery/break-glass proof.

## Handoff

Proceed only after doctor readiness.
