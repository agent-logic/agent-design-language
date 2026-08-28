# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/551/design.md
.csdlc/prepared/issues/551/diagram.mmd
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
docs/api/runtime-v3/v1/observatory.openapi.json
demos/html-observatory/app.js
demos/html-observatory/runtime-v3.config.json
adl/tools/validate_v0917_html_observatory.py
.csdlc/evidence/551/html-polis-node.tap

## Prompts

- Does validation reject an advertised Observatory origin that the combined CORS and WSS policy would not accept?
- Do REST and WSS default to the existing v2 contract, explicitly project v1 and v3, and reject unsupported schema selectors?
- Does one validated reload atomically replace every Polis parameter and Runtime presentation consumer without restart?
- Do invalid reloads preserve the complete last-known-good snapshot with bounded redacted diagnostics?
- Does HTML explicitly request v3 and render only feed-owned identity values?
- Is Unity absent from the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- The exact-head review did not perform a live browser, external network, or deployed TLS exercise; local Runtime TLS/WSS and HTML proof remain the bounded pre-publication evidence.

## Review Result

Revision: Some("git-blake3:b79f38a5883b8dee48054bbe11207f194a559817:757f62bb76a0c8e1e846573569d1629442b1badd18267ff50acb2eaa71b66804")

Reviewer: Some("fresh-session:0be75823-7e07-43dc-aaa8-0d6c545831b4")

Result: pass
