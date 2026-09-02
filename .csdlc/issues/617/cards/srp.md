# Structured Review Prompt

Template: 1.0.0

Issue: 617

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/617
.csdlc/prepared/issues/617
adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/conversation_sessions_tests.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime-kernel/tests/support/runtime_init.rs
docs/api/runtime-v3/v1/observatory.openapi.json
infra/runtime-v3/runtime-init.toml

## Prompts

- Does every canonical name come from authoritative configuration or admitted state?
- Are operational ID, canonical name, display name, and office still distinct?
- Do roster, detail, JSON serialization, and OpenAPI agree?
- Is Shepherd naming stable without changing lifecycle semantics?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:ad1eb44991f7cd686f514b2aea6a77d348b73949:4d337a1b4c98622c87ca5b8d47ba4b46a0f6d821a56b0d020bc022f9962514fb")

Reviewer: Some("fresh-session:cf241732-769d-423f-ae39-0d40bc44a966")

Result: pass
