# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl/tools/test_v0917_html_observatory_integrated_proof.sh
adl/tools/validate_v0917_html_observatory.py
demos/v0.91.7/html-observatory/README.md
demos/v0.91.7/html-observatory/app.js
demos/v0.91.7/html-observatory/index.html
docs/api/runtime-v3/v1

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

Revision: Some("git-blake3:c22b25248166c277e7319d5e37f7e873fab560cd:b3bed829103c2845f83a7152268824f9a3faf936f0bca51b766cc74b93eda0f0")

Reviewer: Some("subagent:019fafbd-db23-7681-a979-4cc0bc68585d")

Result: pass
