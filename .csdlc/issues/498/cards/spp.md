# Structured Planning Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize CORP-D, bind a FastWork execution lane, fail closed until CORP-A/B/C are live merged and ancestral, produce a redacted diligence index and acceptance record, validate the issue-owned evidence, obtain exact-head review, and publish one closing PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "STEP-498-001",
    "action": "Confirm #482, #483, and #497 live issue/PR merge state and ancestry against the #498 execution base.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "STEP-498-002",
    "action": "Build the exact diligence index and prerequisite blocker census without copying private diligence or legal advice.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "STEP-498-003",
    "action": "Record counsel-controlled judgments only as bounded public or redacted receipt references.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "STEP-498-004",
    "action": "Write the corporate diligence acceptance record only if every prerequisite blocker is dispositioned and live merged/ancestral.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "STEP-498-005",
    "action": "Run all planned PVF lanes, obtain exact-head review, publish a closing PR for #498, and update Sprint 4 child disposition truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Corporate diligence acceptance cannot be recorded while any CORP-A-C prerequisite is open, unmerged, non-ancestral, or has an unresolved blocker.
- Counsel-controlled judgment content is never committed; only bounded public or redacted receipt references are allowed.
- The diligence index is the denominator for the acceptance record.
- The issue does not mutate external control planes or absorb CORP-C/Sprint 4 work.
- All lifecycle state is generated through typed C-SDLC v2 tools.

## Risks

- CORP-C #497 may remain open, non-closing, or non-ancestral, which blocks CORP-D acceptance.
- A diligence record could overstate legal or corporate acceptance if counsel receipts are missing or private.
- A private diligence detail, account identifier, recovery factor, or credential could accidentally enter retained evidence.
- Sprint umbrella truth could drift if #498 readiness is confused with terminal child completion.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/498/design.md

Digest: 262827283b5eed453b3521ceac115d26850434611433ea64a006c762c6bcce59

## Diagram

.csdlc/prepared/issues/498/diagram.mmd

Digest: c9ce5694da948221c4c85497a9ee582dd0d5689343d4b1b15a8874d8ea900db3

## Stop Conditions

- An unresolved CORP-A-C blocker lacks disposition.
- CORP-C #497 is not live merged into main and ancestral to the #498 execution base.
- Private advice, private diligence material, account identifiers, recovery factors, credentials, or secrets would enter Git.
- A required action would mutate external provider, account, billing, credential, DNS, certificate, CI, Terraform, deployment, or production state without explicit operator authorization.
- Exact-head review finds unresolved actionable issues.

## Handoff

Proceed only after doctor readiness.
