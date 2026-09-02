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

1. AC-1: AgentRosterEntry and roster/detail JSON expose an explicit canonical two-part agent name
2. AC-2: Operational ID, canonical name, display name, and public office remain distinct and truthful
3. AC-3: Dynamic agents preserve their admitted canonical names across roster and detail responses
4. AC-4: The startup Shepherd exposes a configured stable canonical name without inference
5. AC-5: Observatory OpenAPI and checked API inventory require and describe the additive field
6. AC-6: Focused serialization and compatibility tests prove existing field meanings and clients are not unexpectedly broken

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
