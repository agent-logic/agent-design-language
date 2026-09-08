# Structured Task Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One CLI convenience surface and its focused tests and documentation; no remote mutation is needed for local proof.

## Deliverables

- Simple github-issue create argument surface
- Typed projection into the existing operational dispatch
- Focused positive and adversarial tests
- Operator documentation showing simple and advanced forms

## Acceptance

1. AC-1: Title and exactly one body source are validated
2. AC-2: Labels, assignees, and milestone project into the shared typed request
3. AC-3: Marker, durable intent, authenticated readback, assigned issue number, receipt, and idempotent reconciliation remain shared
4. AC-4: The command fails closed before v3 authority is active
5. AC-5: Documentation leads with the simple form and retains request-file usage as advanced/audit

## Dependencies

- #721 / PR #726 reviewed and merged before standalone integration into main

## Inputs

- agent-logic/agent-design-language#724
- agent-logic/agent-design-language#721
- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/tests/remote_publication_commands.rs
- docs/csdlc-v3/

## Non Goals

- Raw gh issue creation
- v2 fallback
- Alternate receipt or authentication path
- Generic CLI framework redesign
- C-SDLC cutover
