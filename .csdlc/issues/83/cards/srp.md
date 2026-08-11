# Structured Review Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/83/audit.jsonl
.csdlc/issues/83/cards/sip.values.json
.csdlc/issues/83/cards/sor.values.json
.csdlc/issues/83/cards/spp.md
.csdlc/issues/83/cards/spp.values.json
.csdlc/issues/83/cards/srp.values.json
.csdlc/issues/83/cards/stp.values.json
.csdlc/issues/83/cards/vpp.md
.csdlc/issues/83/cards/vpp.values.json
.csdlc/issues/83/index.json
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs

## Prompts

- Can any stale or fixture state be presented as live?
- Can reconnect duplicate an event, replay a command, or widen authority?
- Do all menus and controls have real behavior or an explicit unavailable state?
- Can tokens, keys, private state, or sealed data leak into browser evidence?

## Findings

[
  {
    "id": "83-review-p1-zero-test-vpp-filter",
    "severity": "p1",
    "summary": "The acknowledgement continuity VPP lane named a nonexistent test and could report a zero-test Cargo success; the lane now names the implemented restart-continuity test and requires exact selection.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9053fdf26aa24603f643bdfa997b575d7a821787:1d9a893ecbc0ad4f6587ac9006b568675547b2f5d8fd7b1da1c37928eb7b05af",
    "route": null
  },
  {
    "id": "83-review-p2-cross-carrier-reservation-race",
    "severity": "p2",
    "summary": "ControlService inferred reservation ownership from shared snapshots and could claim another same-sender carrier's watermark; canonical ingress now returns attempt-local disposition with a deterministic cross-carrier regression.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:9053fdf26aa24603f643bdfa997b575d7a821787:1d9a893ecbc0ad4f6587ac9006b568675547b2f5d8fd7b1da1c37928eb7b05af",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The exact live browser evidence is retained outside Git at the recorded FastWork path; hosted CI remains the final integration confirmation after publication.

## Review Result

Revision: Some("git-blake3:9053fdf26aa24603f643bdfa997b575d7a821787:1d9a893ecbc0ad4f6587ac9006b568675547b2f5d8fd7b1da1c37928eb7b05af")

Reviewer: Some("Arendt:019fedd2-cff2-70a2-89f0-64cd4217177c")

Result: pass
