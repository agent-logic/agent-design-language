# Structured Planning Prompt

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind issue #18, add the shared EPIPE-safe writer, adopt it in both split binaries, prove process behavior, document the contract, review, and publish one canonical PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add a shared JSON stdout writer that accepts BrokenPipe and propagates other output failures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Route split GitHub issue and PR schema, success, and typed error payloads through the shared writer.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused process-level regression coverage for early-closing schema readers and ordinary JSON output.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Document the output contract and prepare the clean exact-head review revision.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- Machine JSON remains on stdout
- Human diagnostics remain on stderr
- Only BrokenPipe is normalized to success
- GitHub action semantics and schemas remain unchanged

## Risks

- A helper swallows non-EPIPE I/O failures
- One split binary retains println-based panic behavior
- A process test passes without actually closing the reader early
- Error payload exit codes change accidentally

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/18/design.md

Digest: b20e704be6264e0d1e06a94ba31f374c5fa04d7e93a3c132b6032820cfcb78ff

## Diagram

.csdlc/prepared/issues/18/diagram.mmd

Digest: a5ca558be2b84900ebea96067d66e8d6eb37b4c9c79f5589e172bf4432f4d598

## Stop Conditions

- The change requires GitHub action or schema semantics to change
- The shared writer cannot distinguish BrokenPipe from other I/O failure
- Scope expands into unrelated command refactoring

## Handoff

Proceed only after doctor readiness.
