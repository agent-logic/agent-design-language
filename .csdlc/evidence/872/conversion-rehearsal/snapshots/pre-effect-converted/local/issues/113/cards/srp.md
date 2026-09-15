# Structured Review Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/openapi_contract.rs
adl-runtime-kernel/tests/parity_b_live_kernel.rs
adl-runtime-kernel/tests/guardian_soak.rs
adl-runtime-kernel/tests/shepherd.rs
docs/api/runtime-v3/v1/observatory.openapi.json
demos/html-observatory/app.js
adl/tools/test_html_observatory.sh
.csdlc/evidence/113/roster-live-proof-2118c05b3
.csdlc/issues/113

## Prompts

- Can any unauthorized agent or private field reach serialized JSON, WSS, logs, browser state, or retained evidence?
- Can pagination, policy changes, reconnect, restart, event gaps, duplicate updates, or equal sort keys silently omit, duplicate, reorder, or falsely complete the roster?
- Does stable identity survive relocation while stale owners, duplicate identities, and split authority fail closed?
- Are ready, busy, sleeping, degraded, unreachable, migrating, and unknown derived from explicit fresh Runtime evidence rather than UI heuristics?
- Are page size, response bytes, memory, latency, event queues, retries, replay, and browser DOM growth all bounded and proven at large-Polis scale?
- Does the implementation remain within #113 ownership after #83/#142 handoff and avoid every sibling WP-18C capability?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Distributed and non-local roster projection remains explicitly outside #113 and is owned by issue #142.
- Long-running Runtime and soak-style validation remains out of band under issue #226 and is not coupled to this focused product issue.

## Review Result

Revision: Some("git-blake3:05fa579228dc983ec9a91d8895f19dc8acfb78d9:e0d8a79ab0b8028983766c5b9a45733b337e5c7696c84e99466d845ce62502dd")

Reviewer: Some("openai-codex:gpt-5:wp18c-required-ci-review:2026-08-11")

Result: pass
