# Structured Review Prompt

Template: 1.0.0

Issue: 5789

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl/tools/test_html_observatory.sh
adl/tools/test_v0917_html_observatory_integrated_proof.sh
adl/tools/validate_v0917_html_observatory.py
demos/html-observatory/README.md
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/runtime-v3.config.json
docs/api/runtime-v3/v1/openapi.json
docs/api/runtime-v3/v1/observatory.openapi.json

## Prompts

- Does the default Observatory route now use live Runtime v3 truth without hidden query parameters?
- Are WebSocket, GET feed, retained fallback, and operator write states separated truthfully?
- Can the operator communicate with agents through governed, auditable, fail-closed controls?
- Do browser and CLI tests cover actual checked-in routes and negative cases?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The Observatory now sends signed Runtime v3 control-command envelopes to /v1/control; this proves operator command transport, not a free-form natural-language chat session with a specific shepherd model unless the runtime agent/action configuration exposes that target.
- The integrated localhost proof regenerates TLS certificate evidence files; they were restored after the passing run so the reviewed branch remained clean at the exact reviewed revision.

## Review Result

Revision: Some("git-blake3:c060ef20927081c2547f58a845c6b2ba50c66504:520da3a5a57833130cbb63dcb640e10d974698303ec10c87b971517af75371e2")

Reviewer: Some("subagent:019fc8ca-fab4-7df3-b05e-72e4554ff7e0:Sartre")

Result: pass
