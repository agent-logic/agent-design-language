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
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime/src/acip.rs
adl-runtime/src/runtime_api.rs
adl-runtime/tests/runtime_api_docs.rs
adl-runtime/tests/runtime_api_wss.rs
infra/runtime-v3/runtime-api-5665.toml

## Prompts

- Do the OpenAPI contracts cover every real production Runtime v3 and Observatory endpoint without documenting unavailable behavior?
- Does route-parity validation prevent undocumented real routes and documented phantom routes?
- Are WSS authentication, inbound frames, outbound frames, close/error behavior, and correlation identifiers documented accurately?
- Are constants such as port and base URL represented through config/server variables rather than hard-coded runtime behavior?
- Does the implementation avoid #5344 protected paths unless there is explicit typed transfer or release?

## Findings

[
  {
    "id": "R5701-01",
    "severity": "p1",
    "summary": "The secondary adl-runtime router served the same Core OpenAPI document while returning incompatible health and metrics schemas.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R5701-02",
    "severity": "p2",
    "summary": "ACIP monotonic sequence state was retained when canonical ingress dispatch failed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R5701-03",
    "severity": "p2",
    "summary": "The secondary adl-runtime route inventory test still asserted obsolete unversioned routes.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R5701-04",
    "severity": "p3",
    "summary": "The Observatory WebSocket contract declared HTTP bearer authentication although the real handler authenticates in the first WebSocket frame.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:86aab3d09800574a0aa95e270a6bbaa45cfb5b36:ddd2a916c2dfe7ad177c0f62e3242e19ed4b63c74d01f2fc0e93e7b8dcaa85e2")

Reviewer: Some("subagent:019faf71-f57d-7c01-8ccf-8ceec597f8e7")

Result: changes_required
