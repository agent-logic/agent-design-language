# Structured Review Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/build.rs
adl-runtime-kernel/src/agent_roster.rs
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/telemetry.rs
adl-runtime-kernel/src/live_continuity.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/tests/agent_roster.rs
adl-runtime-kernel/tests/control.rs
adl-runtime-kernel/tests/openapi_contract.rs
docs/api/runtime-v3/v1/openapi.json
docs/api/runtime-v3/v1/observatory.openapi.json
demos/html-observatory/app.js
demos/html-observatory/index.html
demos/html-observatory/styles.css
adl/tools/test_html_observatory.sh
adl/tools/validate_v092_html_observatory_roster.mjs
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
- Durable conversation history and cross-restart chat restoration remain outside #113 and is owned by issue #114.

## Review Result

Revision: Some("git-blake3:52179dc4f49255c4ab3a29726afe7c1e95a22097:504659dac81c77b4a589e859151c81ccda3d9707ff465f8074202fe5d4406357")

Reviewer: Some("subagent:019fef34-1897-7353-96e7-49320ae0043a")

Result: pass
