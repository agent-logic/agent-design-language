# Structured Output Record

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Stabilized the cleanup-race proof by queueing the duplicate attachment immediately behind re-authentication, preserving server frame order and production deadlines while removing a client round-trip scheduling race.

## Artifacts

- adl-runtime-kernel/tests/conversation_sessions.rs
- .csdlc/evidence/244

## Execution

- Queued cleanup-race re-authentication and duplicate attachment frames back-to-back before awaiting the authentication response.
- Documented why the ordering preserves the authentication-generation transition and attaches before the barrier-held turn's execution deadline.
- Left Runtime production behavior, issue #237, PR #242, and issue #112 authority work unchanged.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions"
    ],
    "purpose": "Prove the cleanup-race sequence once under typed validation after 20 consecutive preflight passes.",
    "outcome": "passed",
    "evidence_ref": "conversation-cleanup-race-focused.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across Runtime kernel targets.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-fast-clippy.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "purpose": "Run the integrated Observatory proof.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-fast-observatory.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete Runtime kernel test target.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-fast-tests.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
