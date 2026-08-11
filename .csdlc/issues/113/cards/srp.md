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

[
  {
    "id": "fresh-review-222-p1-policy-projection",
    "severity": "p1",
    "summary": "Production roster policy authorizes and discloses every configured entry without a caller-specific visibility decision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/control.rs:1231"
  },
  {
    "id": "fresh-review-222-p1-cursor-gap",
    "severity": "p1",
    "summary": "The roster feed and browser have no cursor gap contract and silently accept skipped revisions instead of resynchronizing.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "demos/html-observatory/app.js:105"
  },
  {
    "id": "fresh-review-222-p1-pagination-cost",
    "severity": "p1",
    "summary": "Roster pagination allocates and sorts the whole visible population for every page, so memory and latency are not bounded by page size.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": "adl-runtime-kernel/src/agent_roster.rs:185"
  },
  {
    "id": "fresh-review-222-p2-retained-live-evidence",
    "severity": "p2",
    "summary": "Live-browser and signed-restart claims have no retained inspectable logs, screenshots, traces, or restart receipts in the issue evidence bundle.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": ".csdlc/issues/113/cards/sor.md"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Distributed and non-local roster projection remains owned by issue #142.
- Durable conversation history and cross-restart chat restoration remain owned by issue #114.
- The fresh reviewer could not rerun the environment-dependent trusted-TLS Playwright and restart proof.

## Review Result

Revision: Some("git-blake3:fcbe7609f5a346154e5967e225a3ced05385f5ac:c759650f8a7c61aab545acf17fc1f0c3ee092296f3f32637c93135699040096e")

Reviewer: Some("subagent:019fef96-7412-7f72-b863-3ecd4bace544")

Result: changes_required
