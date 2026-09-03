# Structured Intent Prompt

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Land the post-merge corrective repair for provider reload ownership so overlapping workflows cannot consume or clear each other's provider snapshot.

## Required Outcome

Current main receives a reviewed corrective PR that replaces production process-global reload ownership with run-scoped ownership, retains identity-aware compatibility guard semantics, and proves overlap plus shutdown-order safety with focused offline tests.

## Scope

- adl/src/provider/reload.rs
- adl/src/execute/mod.rs
- adl/src/execute/runner.rs
- adl/src/execute/tests.rs
- adl/src/long_lived_agent.rs
- .csdlc/prepared/issues/648/**
- .csdlc/issues/648/**

## Authority

- Issue #648 is a corrective follow-up because #646 merged at stale head 4c442cef90b06c4a491860ce1e9d9053dfed26eb
- Live Runtime/#640 ownership remains with Planning #4
- All validation is offline and uses mock/local provider paths only
- Provider credentials remain references and must not be loaded, printed, or executed
- Typed C-SDLC v2 remains the lifecycle authority

## Assumptions

- none

## Operator Constraints

- Do not restart, stop, replace, or mutate live Runtime or active Runtime config
- Do not use AWS, paid provider APIs, big runners, or live provider inference
- Do not write tracked issue work on main
- Do not treat closed #622 as semantically complete until #648 lands and finishes
- Do not merge without explicit operator authorization
