# Structured Review Prompt

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/validate_v0917_html_observatory.py
demos/v0.91.7/html-observatory/app.js
demos/v0.91.7/html-observatory/index.html

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

Revision: Some("git-blake3:1379d077a4497d6b86cf72d00303b5bfbced0b88:bdd39ffbcc0504c9c0ac4fe1b5e03474a23a0f611cb4bd868ec0a911b35aa4ee")

Reviewer: Some("subagent:019fafe0-c5a2-78b3-af0a-352df7715ee1")

Result: pass
