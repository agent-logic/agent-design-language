# Structured Review Prompt

Template: 1.0.0

Issue: 5589

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/bin/adl-runtime-governed-operations.rs
adl-runtime-kernel/tests/governed_operations.rs
.csdlc/issues/5589/cards/sor.values.json
.csdlc/issues/5589/cards/srp.values.json
.csdlc/evidence/5589/implementation

## Prompts

- Does every capability require a production or maintained COTS-backed adapter invoked by a live initialized kernel, with explicit zero credit for degraded and fixture evidence?
- Is signed governance observably before every provider/tool actuation and recovery dispatch, including revocation, replay, expiry, appeal, and quarantine negatives?
- Are delegation attenuation, resource cleanup, cancellation precedence, retry/idempotency, and scheduler race behavior fully bounded?
- Do provider, scheduler, governed-tool, Agent, and Shepherd proofs execute live multi-agent work without treating provider output as authority?
- Are citizen identity, private state, qualified time, checkpoint authority, lifelog non-authority, redaction, restart, and no-duplicate continuity complete?
- Are #5591, #5592, #5590, ADL v2, C-SDLC v2, Runtime v2, AWS, cutover, and publication boundaries explicit and disjoint?
- Do SPP steps and VPP lanes cover AC-1 through AC-8 bidirectionally with no deferral and no nonexistent proof command at implementation time?
- Does the current preparation claim protect only #5589 lifecycle/evidence paths and preserve the #5591 review and collision blockers truthfully?

## Findings

[
  {
    "id": "replay-fingerprint-bypass",
    "severity": "p1",
    "summary": "Cached replay compares only request and citizen identity, allowing changed governed fields and a substituted unrevoked commitment to bypass gate validation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "provider-io-unbounded",
    "severity": "p1",
    "summary": "Blocking process wait cannot be preempted by the Tokio timeout and accumulates unbounded stdout before checking its limit.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "crash-stale-lock",
    "severity": "p1",
    "summary": "Directory locking removed only by Drop leaves state permanently busy after process or host crash.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "synthetic-per-request-services",
    "severity": "p1",
    "summary": "Fresh passthrough scheduler and shepherd adapters plus a global operation lock do not establish shared resident multi-agent scheduling, saturation, resource races, or cleanup.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "proof-name-overstatement",
    "severity": "p2",
    "summary": "Appeal, cancellation-race, scheduler-saturation, and append-only non-authority tests still do not exercise their named semantics.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "unbounded-authoritative-state",
    "severity": "p2",
    "summary": "Completed, request-id, and private-state checkpoint collections grow without a declared bound.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "budget-exception-still-unjustified",
    "severity": "p1",
    "summary": "The 13,496-line result is additive, duplicates existing assembly patterns, retains synthetic components, and does not justify the +1,287 baseline exception.",
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

Revision: Some("git-blake3:353459e57e5f9b3cd96a5372d927088f8c4f8e34:089d7652ef41a655f6879ea90345ddf0cf9e5d6f3b090ea45729306760f1ad20")

Reviewer: Some("subagent:/root/review_5589_exact")

Result: changes_required
