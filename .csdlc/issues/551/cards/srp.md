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

Revision: Some("git-blake3:4d02f41a13438cb4a1253e7ea779c47481ea32cf:4d682ee6ae22caccda080b86e19ebbea7f52c305d5ef5fca1004c077680e579c")

Reviewer: Some("fresh-session:1fa7b2f0-4c59-4343-ae7b-afdce3064aa9")

Result: pass
