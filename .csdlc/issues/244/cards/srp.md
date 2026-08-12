# Structured Review Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/tests/conversation_sessions.rs
.csdlc/issues/244
.csdlc/prepared/issues/244
.csdlc/evidence/244

## Prompts

- Does queuing the duplicate behind re-authentication preserve server frame order and the authentication-generation transition?
- Does the test attach to the existing in-flight turn before its existing deadline without changing production behavior?
- Are duplicate attachment and exactly-once terminal semantics preserved, with no #112 authority changes?

## Findings

[
  {
    "id": "R244-P2-1",
    "severity": "p2",
    "summary": "Align lifecycle planning with the test-only cleanup-race sequencing change and unchanged production behavior.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:db5d47b1424c5d59fc3359fca7a75852911afc4b:12805ad5f56fb5743495a5578045567abaad76e2c3f754a9ac67095807b8d9bc",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:db5d47b1424c5d59fc3359fca7a75852911afc4b:12805ad5f56fb5743495a5578045567abaad76e2c3f754a9ac67095807b8d9bc")

Reviewer: Some("/root/review_244_cleanup_race")

Result: pass
