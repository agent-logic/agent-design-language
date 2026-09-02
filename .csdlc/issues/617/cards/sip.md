# Structured Intent Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Expose each agent's canonical two-part name directly in Runtime v3 roster and detail responses without changing lifecycle semantics or existing identifiers.

## Required Outcome

AgentRosterEntry, roster/detail JSON, OpenAPI, and focused tests expose stable truthful canonical names for dynamic agents and the Shepherd while preserving operational ID, display name, and public office as separate fields.

## Scope

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/runtime-v3/runtime-init.toml
- .csdlc/prepared/issues/617
- .csdlc/evidence/617
- .csdlc/issues/617

## Authority

- Issue authority is agent-logic/agent-design-language#617
- Merged issue #602 supplies canonical agent lifecycle state
- Existing operational IDs and lifecycle semantics remain unchanged
- Only Runtime v3 roster and detail projection plus its schema and tests are in scope

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not infer canonical identity from operational ID, display label, or office
- Keep the API change additive and compatibility-tested
- Use the current typed C-SDLC v2 route
