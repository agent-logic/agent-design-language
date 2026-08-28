# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/control/feeds.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/observatory.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/support/runtime_init.rs
docs/api/runtime-v3/v1/observatory.openapi.json
infra/runtime-v3/runtime-init.toml
demos/html-observatory/index.html
demos/html-observatory/app.js
demos/html-observatory/tests/polis_identity.test.mjs
.csdlc/prepared/issues/551
.csdlc/evidence/551
.csdlc/issues/551

## Prompts

- Does one validated reload atomically replace every Polis parameter and Runtime presentation consumer without restart?
- Do invalid reloads preserve the complete last-known-good snapshot with bounded redacted diagnostics?
- Does HTML use only the feed identity?
- Is Unity absent from the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- Hot reload changes Runtime presentation and origin policy but does not mutate external DNS, certificates, or ingress infrastructure.

## Review Result

Revision: Some("git-blake3:4bca700fd57370926e21874a6c8eeb47a3ea15a3:de3fd81b597ca0d9df83f6e40e4e929d22138bfe4084f92f89d7e5de69bcc20e")

Reviewer: Some("fresh-session:c7bd5e7f-878a-427e-8cd2-266522f83ee5")

Result: pass
