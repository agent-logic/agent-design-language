# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

docs/api/runtime-v3/v1
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
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

Revision: Some("git-blake3:5459e860452afb19616af1df013d537f2915e18e:0ce676bb7be7d1a703e633e479808f55a21022c73912393f3cb0e815f257e1de")

Reviewer: Some("subagent:019faf8a-5313-7292-8ca5-57feb8f12d7b")

Result: pass
