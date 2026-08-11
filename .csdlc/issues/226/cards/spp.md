# Structured Planning Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add three narrow path selectors, prove exact focused routing, retain unknown-path fail-closed behavior, review, and publish the small tooling repair.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Add narrow selectors for the two Observatory validators and design diagrams.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add focused selector and CI path-policy regressions for the exact mixed path set.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run only focused contract tests, obtain exact-head review, and publish the bounded repair.",
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

- Unknown paths continue to fail closed
- No selector maps Runtime source to docs-only proof
- No slow or coverage job is selected solely by lifecycle metadata or issue diagrams
- Existing lane commands and proof semantics remain unchanged

## Risks

- An overbroad selector could hide a real tooling change
- Path-policy post-processing could still escalate despite selector coverage

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/226/design.md

Digest: b53a4f2164b857afb5267dc031684d166b53ae735222244b23e9580703177f50

## Diagram

.csdlc/prepared/issues/226/diagram.mmd

Digest: 8c56bd5b5995ff707eb45e3fc1afc455d18137f95ac93d1b4aef9c0d5a5457ab

## Stop Conditions

- The repair requires weakening unknown-path escalation
- The exact path set still selects slow proof or authoritative full coverage
- Scope expands into Runtime product or soak implementation

## Handoff

Proceed only after doctor readiness.
