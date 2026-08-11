# Structured Review Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/83/audit.jsonl
.csdlc/issues/83/cards/sip.values.json
.csdlc/issues/83/cards/sor.md
.csdlc/issues/83/cards/sor.values.json
.csdlc/issues/83/cards/spp.values.json
.csdlc/issues/83/cards/srp.md
.csdlc/issues/83/cards/srp.values.json
.csdlc/issues/83/cards/stp.values.json
.csdlc/issues/83/cards/vpp.values.json
.csdlc/issues/83/index.json
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs
adl-runtime-kernel/src/durable_state.rs
adl-runtime-kernel/tests/durable_state.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/production_acip_wss.rs
demos/html-observatory/app.js
adl/tools/validate_v092_html_observatory_live.mjs
docs/api/runtime-v3/v1/observatory.openapi.json

## Prompts

- Can any stale or fixture state be presented as live?
- Can reconnect duplicate an event, replay a command, or widen authority?
- Do all menus and controls have real behavior or an explicit unavailable state?
- Can tokens, keys, private state, or sealed data leak into browser evidence?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration confirmation after the refreshed branch is pushed.

## Review Result

Revision: Some("git-blake3:e1e27ef32d2eb992e69a27dbf5ce3137b4be04a2:a41c95264222d3121449424326e0cb137932793b13400a2a97603c63f0df9066")

Reviewer: Some("Arendt:019fedd2-cff2-70a2-89f0-64cd4217177c")

Result: pass
