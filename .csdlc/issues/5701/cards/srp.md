# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

docs/api/runtime-v3/v1
adl-runtime-kernel/Cargo.lock
adl-runtime-kernel/Cargo.toml
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime/Cargo.lock
adl-runtime/Cargo.toml
adl-runtime/src/acip.rs
adl-runtime/src/runtime_api.rs
adl-runtime/tests/runtime_api_wss.rs
infra/runtime-v3/runtime-api-5665.toml

## Prompts

- Do the OpenAPI contracts cover every real production Runtime v3 and Observatory endpoint without documenting unavailable behavior?
- Does route-parity validation prevent undocumented real routes and documented phantom routes?
- Are WSS authentication, inbound frames, outbound frames, close/error behavior, and correlation identifiers documented accurately?
- Are constants such as port and base URL represented through config/server variables rather than hard-coded runtime behavior?
- Does the implementation avoid #5344 protected paths unless there is explicit typed transfer or release?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:f6c9d63ecd62a837a951e4106f964f215e7b64e6:b454fb763c1cbf2a46796a1fae04dc1fa4a9c3a830ee8fd1bed847626cded0ac")

Reviewer: Some("subagent:019faf97-63e1-7a31-ab74-c8e905e82b30")

Result: pass
