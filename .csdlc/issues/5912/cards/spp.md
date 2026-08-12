# Structured Planning Prompt

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add a narrow Runtime service around the existing opaque policy and builder/validator, prove it through an external integration test, then run focused review and publication.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Add Runtime-owned policy provisioning and invocation service.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add external production-path integration proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused tests, strict Clippy, and exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Policy internals remain opaque.
- Receipt emission occurs only after successful build and validation.
- Existing deterministic security and privacy semantics remain unchanged.

## Risks

- A convenience constructor could accidentally expose caller-forgeable authority.
- Emission could occur before validation if sequencing is not explicit.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5912/design.md

Digest: ef9a5c90f5bd38565ef962e36c0d863024be35c84af3da6fc575e6c3042f4a2b

## Diagram

.csdlc/prepared/issues/5912/diagram.mmd

Digest: cbf66d33c9338b8bbb53485c40f275668134d8f70a4fbcf32b39193a5dfa713c

## Stop Conditions

- Implementation requires exposing mutable or caller-forgeable policy internals.
- The existing #5833 contract must change rather than be integrated.
- The focused proof requires broad downstream Birthday orchestration.

## Handoff

Proceed only after doctor readiness.
