# Structured Planning Prompt

Template: 1.0.0

Issue: 674

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Draft one source-grounded versioned Welcome Package, validate its required sections and safety language offline, obtain exact-head documentation review, and publish without merging.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Draft the versioned Welcome Package from current resident-agent, Shepherd, Layer 8, and governed communication source evidence.",
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
    "action": "Run the issue-owned documentation validator and Markdown/diff hygiene checks.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Obtain independent exact-head documentation review and publish the PR without merging.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- Documentation claims only current governed capabilities
- The package grants no authority
- No credentials, private state, or machine-local paths appear
- No live Runtime or provider proof is implied

## Risks

- Friendly language could overstate capability or personhood
- Static documentation could drift from Runtime policy
- Readers could mistake future onboarding automation for delivered behavior

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/674/design.md

Digest: f023d034a3e65718f81897b43967afad4ad0300936d8a79350f63fb675e83a92

## Diagram

.csdlc/prepared/issues/674/diagram.mmd

Digest: 600ab04e631c91e18ff47616f5f1342ab94f4073e5318dcdf2c02118b02dbb50

## Stop Conditions

- A claim cannot be grounded in current repository evidence
- The document would imply new Runtime behavior
- Scope expands beyond the one document and validator
- Exact-head review finds unresolved overclaims

## Handoff

Proceed only after doctor readiness.
