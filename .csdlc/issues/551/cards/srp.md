# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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

Revision: Some("git-blake3:151bf977d7c33357ba8f7f957a23ef12fec36b01:c0c87d8a0650b822a2afdbfaee100de79d097021ba552f999af003f4bed06412")

Reviewer: Some("fresh-session:abfe76c4-cf6d-4302-8b0f-ff98f7da7191")

Result: pass
