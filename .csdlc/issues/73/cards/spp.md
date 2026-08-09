# Structured Planning Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Complete the Rust architecture and detailed issue decomposition, obtain independent findings-first Claude and Gemini reviews over the same exact revision, incorporate or disposition findings, validate the final planning packet, and stop before implementation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete the Rust architecture, effect targets, diagram, migration boundaries, and full issue decomposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run independent findings-first Claude and Gemini reviews against the same exact plan revision.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Incorporate or explicitly disposition every actionable review finding and run exact-revision verification reviews.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate the complete planning packet and report readiness for later operator-authorized issue creation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- C-SDLC v2 remains sole operational authority during planning and construction
- No issue is writable by v2 and v3 simultaneously
- Review precedes publication and exact-head truth remains mandatory
- State authority, generated projections, and external GitHub truth remain distinct
- The issue plan does not hide implementation inside planning

## Risks

- A single binary could rebundle v2 complexity instead of simplifying it
- Issue boundaries could omit cross-cutting parity or recovery proof
- State and projection semantics could overstate multi-file atomicity
- Rust async and trait abstractions could spread into the pure domain kernel
- Migration planning could accidentally imply dual authority or reversible remote mutations

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md

Digest: 546a88f2ad71bb367629a9b986f8f805ca7cd5b41f47104207c8c317dd3f3b13

## Diagram

.adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd

Digest: f2a686c350e84e3fdf2e8b56249b571f8f8d33a09ddf9ebd7b3558dadc7e9b6a

## Stop Conditions

- The plan requires implementation to resolve an architectural decision
- Claude or Gemini identifies an undispositioned P1 finding
- The issue breakdown omits a retained v2 safety invariant or proof lane
- Any tracked edit appears on primary main
- Work expands into implementation, child creation, migration, cutover, or retirement

## Handoff

Proceed only after doctor readiness.
