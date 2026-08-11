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

- Distributed and non-local roster projection remains outside #113 under #142.
- Long-running distributed and soak-style validation is out of band under #226 and is not a focused #113 gate.

## Review Result

Revision: Some("git-blake3:ba4b4bb5cdae10be896a33b21e53a05cd5763138:bd12f7e87a54b35d0a9d2574c5f064d5068068b8937ee1490e7664abf7cd0ebb")

Reviewer: Some("subagent:019fefbc-1619-7a21-b27c-8edb01692b23")

Result: pass
