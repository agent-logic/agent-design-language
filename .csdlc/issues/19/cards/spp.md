# Structured Planning Prompt

Template: 1.0.0

Issue: 19

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate the repository preview source, discover and verify the approved existing website resources, upload the minimal object set with exact metadata, invalidate only affected paths, verify live behavior, retain redacted evidence, and publish the evidence change for review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Validate source references, replace the CSP-blocked design runtime with equivalent static HTML using native audio and FAQ controls, and identify the minimal local-only object set",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Verify the Agent Logic AWS profile and deploy the minimal object set to existing S3 and CloudFront resources without compute operations",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Verify live preview behavior, noindex, local-only assets, exact live digests, production non-mutation, and retain redacted evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve bounded exact-head review and publish the retained deployment proof",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Primary checkout remains clean on main
- Only the minimum preview and shared podcast objects are uploaded
- No EC2 or remote-build operation occurs
- Production podcast route and navigation remain unchanged
- Retained evidence is redacted and exact-digest bound

## Risks

- Incorrect S3 prefix or CloudFront origin mapping
- Relative preview references resolve to missing shared objects
- Stale CloudFront cache masks the deployed revision
- Sensitive infrastructure identifiers leak into retained evidence
- Production podcast objects are accidentally changed

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/19/design.md

Digest: 62a29ae4b32889420e852ec5c0b986edd36a74a41ed2feca63c667ff9cb15b1e

## Diagram

.csdlc/prepared/issues/19/diagram.mmd

Digest: e4d4212def0b1a4e0dbd622fd4b497ca1c9718492c28d5e3dbf1b8d77e143ec5

## Stop Conditions

- The agent-logic-admin profile does not resolve to the approved Agent Logic business account
- The target website resources cannot be identified without guessing
- Deployment would require EC2 or another compute service
- The minimal upload set would overwrite production podcast content
- Live readback differs from local source after bounded invalidation
- Evidence cannot be retained without exposing sensitive infrastructure identifiers

## Handoff

Proceed only after doctor readiness.
