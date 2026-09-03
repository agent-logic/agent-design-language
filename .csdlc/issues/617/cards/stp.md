# Structured Task Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One additive canonical-name field carried from authoritative agent state through AgentRosterEntry, roster/detail JSON, and OpenAPI.

## Deliverables

- Canonical name field in AgentRosterEntry
- Roster and detail API projection for dynamic agents and Shepherd
- Updated Observatory OpenAPI and checked inventory
- Focused serialization and compatibility tests

## Acceptance

1. AC-1: AgentRosterEntry and roster/detail JSON always emit a nonempty canonical two-part agent name in the additive wire field name.
2. AC-2: Operational id, canonical name, display label, and public role/office remain separate fields with unchanged existing meanings.
3. AC-3: Dynamic agents carry the validated AgentAdmissionRequest.name unchanged through AgentSample, AgentRuntimeEvidence, roster, and detail responses.
4. AC-4: Runtime init requires and validates resident_shepherd.name, and the production binary passes it to the Shepherd roster constructor without ID or label inference.
5. AC-5: Observatory OpenAPI keeps adl.runtime_v3.agent_roster_entry.v1, adds required outbound name, and its checked inventory matches serialized output.
6. AC-6: Focused tests prove current output always contains name, previously recorded v1 JSON without name remains readable through a deserialization-only default, existing fields retain exact serialization, and zero-test selection fails.

## Dependencies

- Issue #602 and PR #614 must be merged into the execution base

## Inputs

- agent-logic/agent-design-language#617
- agent-logic/agent-design-language#602
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- docs/api/runtime-v3/v1/observatory.openapi.json

## Non Goals

- Changing agent admission, checkpoint, migration, or rehydration semantics
- Renaming operational agent IDs
- Changing display-name or office semantics
- Amending the already merged #602 implementation outside a correctness defect required for projection
