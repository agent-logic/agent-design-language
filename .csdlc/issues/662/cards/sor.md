# Structured Output Record

Template: 1.0.0

Issue: 662

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented governed agent-to-agent initiation for Runtime Observatory WSS so Beacon Axioma can initiate a bounded turn to Ember Axioma through the existing governed conversation ingress.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/telemetry.rs
- .csdlc/prepared/issues/662/validate-focused.sh
- .csdlc/prepared/issues/662/finalize-implementation.json
- .csdlc/issues/662

## Execution

- Added a versioned Observatory agent-initiation intent schema with authenticated WSS handling, sender and recipient identity validation, sender authorization, recipient eligibility checks, and sender-not-recipient fencing.
- Routed accepted initiations through the existing conversation session, replay, cancellation, and canonical ingress machinery while preserving the initiated work id instead of synthesizing ordinary conversation work.
- Extended recipient dispatch payloads and conversation results with sender_id and initiated_work_id so configured provider execution and terminal truth remain attributable.
- Emitted a correlated agent_to_agent_initiated runtime event for successful governed initiations and exposed it through the existing Observatory event feed.
- Added deterministic Runtime kernel tests for delivery through the configured recipient provider, exact replay, conflict detection, unauthorized sender refusal, stale recipient refusal, cancellation, and provider failure.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "outcome": "passed",
    "evidence_ref": "issue-diff-hygiene.log"
  },
  {
    "command": [
      ".csdlc/prepared/issues/662/validate-focused.sh"
    ],
    "purpose": "Run the issue-owned focused validator for successful Beacon-to-Ember initiation, configured provider dispatch, authoritative activity, exact replay, conflicts, cancellation, stale recipient, unauthorized sender, and provider failure.",
    "outcome": "passed",
    "evidence_ref": "runtime-agent-to-agent-initiation.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Reject Rust formatting drift in the changed Runtime kernel crate.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-fmt.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib"
    ],
    "purpose": "Confirm the changed Runtime kernel library compiles.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-lib-check.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
