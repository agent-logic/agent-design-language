# Structured Output Record

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Combined deterministic cleanup-attachment synchronization with issue #248's reviewed production process-backend output-limit/timeout precedence fix so one explicit PR can close both issues without bypassing required CI.

## Artifacts

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- adl-runtime-kernel/src/lib.rs
- .csdlc/evidence/244
- adl-runtime-kernel/src/parity.rs
- adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
- adl-runtime-kernel/tests/parity.rs
- .csdlc/evidence/244/spp-integration-immutability.log
- .csdlc/evidence/244/combined-focused-proof.log
- .csdlc/evidence/244/combined-required-proof.log

## Execution

- Moved the conversation-session fixture into the crate unit-test boundary so every hook type, field, branch, installer, and test module registration is compiled only under cfg(test).
- Added a turn-scoped test hook that observes the duplicate before acceptance arbitration, holds the matching timeout branch, permits duplicate arbitration explicitly, and signals attachment insertion before response serialization.
- Added a drop guard that releases duplicate arbitration, timeout arbitration, and barrier-held execution on panic while avoiding extra permits on normal completion.
- Removed paused-time control and all widened constants; original 100/70/250/150 ms behavior and timeout, cancellation, capacity, and token-rotation assertions remain unchanged.
- Integrated issue #248's exact reviewed three-file substantive patch in adl-runtime-kernel/src/parity.rs, adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs, and adl-runtime-kernel/tests/parity.rs to break the cross-PR required-CI cycle.
- Replaced the prior one-second oversized-file test workaround with production-owned post-termination arbitration: observable file output at the enforced RLIMIT boundary reports output_limit; otherwise the generic deadline reports timeout.
- Added the deterministic output-limit-then-hang fixture while retaining ordinary timeout, legacy oversized output, process-tree termination, cancellation, and zero-artifact cleanup semantics.
- Preserved #244 SPP as immutable historical design-time truth after typed csdlc-edit rejected mutation in implemented phase; the combined final diff is authoritatively recorded in this SOR, the fresh SRP review, and dual-closing PR linkage.
- Kept issue #112 authority work and issue #237/PR #242 unchanged.

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
    "purpose": "Prove cleanup duplicate ingress, attachment installation, execution release ordering, and exactly one terminal result; repeated 30 times in the combined session.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/combined-focused-proof.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity",
      "process_backend_timeout_and_oversized_file_leave_no_artifacts",
      "--",
      "--exact"
    ],
    "purpose": "Prove deterministic output-limit precedence over generic timeout at the post-termination boundary, ordinary timeout, and zero artifacts; repeated 20 times.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/combined-focused-proof.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete combined Runtime kernel suite.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/combined-required-proof.log"
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
    "purpose": "Reject warnings across combined Runtime targets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/combined-required-proof.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "purpose": "Run the required integrated Observatory proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/combined-required-proof.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
