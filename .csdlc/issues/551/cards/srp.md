# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/551/design.md
.csdlc/prepared/issues/551/diagram.mmd
.csdlc/issues/551/cards/stp.values.json
.csdlc/issues/551/cards/spp.values.json
.csdlc/issues/551/cards/srp.values.json
.csdlc/issues/551/cards/sor.values.json
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

Revision: Some("git-blake3:dd01d24449fd8c84d8e5709808e1d19ae71948a9:439946158816b2e824ff00eaccc8366256fe8d7c4042ac6ee795b28990ac96e0")

Reviewer: Some("fresh-session:032c6dee-997d-4868-a0b2-ee0135d580f9")

Result: pass
