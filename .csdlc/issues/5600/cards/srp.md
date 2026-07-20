# Structured Review Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2
.csdlc/issues/5600
.csdlc/prepared/issues/5600
.csdlc/evidence/5600

## Prompts

- Does every required planning collection have an explicit card-owned typed replacement operation?
- Can any failed operation change a card, generation, digest, projection, or audit event?
- Does acceptance coverage reject stale, missing, duplicate, and extra identifiers across STP, SPP, and VPP?
- Does the #5337 fixture prove a real preparation-to-implementation conversion without direct card edits?
- Are phase authorization and existing serialized operation compatibility preserved?

## Findings

[
  {
    "id": "F-5600-5",
    "severity": "p1",
    "summary": "The committed publication request has stale generation and digest values after review recovery.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5600-6",
    "severity": "p2",
    "summary": "The publication body contains an unsupported whole-lifecycle process claim.",
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

Revision: Some("git-blake3:61c748be30383baf4227073c125ad4fef582b1d8:463d7d650b161ed755fd39d3dac17ada1c08a7db4388b804d69feccb4f55b418")

Reviewer: Some("subagent:codex-exec-5600-final-head")

Result: changes_required
