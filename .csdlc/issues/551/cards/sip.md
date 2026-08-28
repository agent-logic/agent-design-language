# Structured Intent Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one validated Runtime-owned Polis identity projection rendered by the HTML Observatory.

## Required Outcome

Runtime configuration, feed projection, atomic last-known-good hot reload of every Polis parameter, and HTML rendering pass without restarting the Runtime or reactivating Unity.

## Scope

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- adl-runtime-kernel/tests/support/runtime_init.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- infra/runtime-v3/runtime-init.toml
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/tests/polis_identity.test.mjs
- .csdlc/prepared/issues/551
- .csdlc/evidence/551
- .csdlc/issues/551

## Authority

- Issue authority is agent-logic/agent-design-language#551
- Unity issue #84 is backlog and non-gating
- DNS TLS routing certificates and continuity-state mutation are outside authority

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- No external infrastructure or credential mutation
- No Unity paths
