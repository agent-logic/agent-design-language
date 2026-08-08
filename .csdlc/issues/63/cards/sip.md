# Structured Intent Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide one audited typed route to correct SIP declared scope after implementation and before publication without weakening card or lifecycle authority.

## Required Outcome

A dedicated csdlc-edit operation corrects implemented-phase SIP declared_scope only when review/publication truth is clear, records the old and new scope plus reason, preserves typed card invariants, and remains unavailable after publication without typed recovery.

## Scope

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/prepared/issues/63
- .csdlc/issues/63

## Authority

- Issue and code authority are agent-logic/agent-design-language#63
- Only the typed csdlc-edit semantic operation may correct implemented SIP declared_scope
- Canonical values JSON and the markdown.rs-backed renderer remain card truth; Markdown is never mutated directly
- csdlc-review remains the sole authority for clearing review and publication truth during recovery

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use the independent Rust C-SDLC v2 binaries and typed editor route
- Require an actor and human reason and retain full previous and replacement scope arrays in audit truth
- Use focused deterministic repository-local proof; do not run broad test suites
- Do not mutate issue #53 or historical lifecycle evidence
