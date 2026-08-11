# Structured Review Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/113
.csdlc/prepared/issues/113/publish-reviewable.json
.csdlc/prepared/issues/113/recover-publication-intent-review.json

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
    "id": "publication-request-stale-generation",
    "severity": "p1",
    "summary": "The publication request retained the pre-recovery generation and digest and was therefore non-executable.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "publication-body-stale-review-digest",
    "severity": "p2",
    "summary": "The publication body described the superseded issue digest as current retained review truth.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:b98cbe71c0ea66a5af6f93a8a61ba01d0464d7fe:fdc68d562472c4a7eaa8f73b4d850f3f83ad45047103a546a7a3028a0bb700ae")

Reviewer: Some("subagent:019fef34-1897-7353-96e7-49320ae0043a")

Result: changes_required
