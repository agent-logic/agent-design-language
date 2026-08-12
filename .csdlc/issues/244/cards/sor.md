# Structured Output Record

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Made cleanup-race ordering deterministic with cfg(test)-only server instrumentation: observe the duplicate, hold its arbitration and the matching timeout branch, install and observe the new-generation attachment, then release barrier-held execution and prove one terminal result.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- adl-runtime-kernel/src/lib.rs
- .csdlc/evidence/244

## Execution

- Moved the conversation-session fixture into the crate unit-test boundary so every hook type, field, branch, installer, and test module registration is compiled only under cfg(test).
- Added a turn-scoped test hook that observes the duplicate before acceptance arbitration, holds the matching timeout branch, permits duplicate arbitration explicitly, and signals attachment insertion before response serialization.
- Added a drop guard that releases duplicate arbitration, timeout arbitration, and barrier-held execution on panic while avoiding extra permits on normal completion.
- Removed paused-time control and all widened constants; original 100/70/250/150 ms behavior and timeout, cancellation, capacity, and token-rotation assertions remain unchanged.
- Kept issue #237, PR #242, and issue #112 authority work unchanged.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation_sessions_tests::authenticated_selected_agent_conversation_uses_canonical_wss_ingress",
      "--",
      "--exact"
    ],
    "purpose": "Run the exact cleanup-race proof thirty consecutive times with built-in bounded scheduler pressure and no clock control.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/conversation-cleanup-race-focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation_sessions_tests::cleanup_race_guard_releases_every_barrier_during_unwind",
      "--",
      "--exact"
    ],
    "purpose": "Prove unwind cleanup releases duplicate, timeout, and execution barriers without leaking permits into normal completion.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/conversation-cleanup-race-failsafe.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete Runtime kernel test target, including real timeout, cancellation, capacity, and token-rotation behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/runtime-v3-fast-tests.log"
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
    "evidence_ref": ".csdlc/evidence/244/runtime-v3-fast-clippy.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "purpose": "Run the integrated Observatory proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/runtime-v3-fast-observatory.log"
  },
  {
    "command": [
      "cargo",
      "check",
      "--release",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Compile the non-test optimized Runtime path with all hook surfaces eliminated by cfg(test).",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/runtime-release-production-parity.log"
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
