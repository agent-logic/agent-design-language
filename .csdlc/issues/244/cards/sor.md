# Structured Output Record

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Stabilized the cleanup-race proof by queueing duplicate attachment behind re-authentication and proportionally widening only the synthetic test execution window and delays, preserving production behavior and timeout assertions.

## Artifacts

- adl-runtime-kernel/tests/conversation_sessions.rs
- .csdlc/evidence/244

## Execution

- Queued cleanup-race re-authentication and duplicate attachment frames back-to-back before awaiting the authentication response.
- Widened the test-only conversation execution window from 100 ms to 500 ms and proportionally scaled synthetic budget, disconnect, and cancellation delays so accepted, timed-out, and cancelled semantics remain exercised on shared CI runners.
- Documented the server frame-order proof and left Runtime production behavior, issue #237, PR #242, and issue #112 authority work unchanged.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions",
      "authenticated_selected_agent_conversation_uses_canonical_wss_ingress",
      "--",
      "--exact"
    ],
    "purpose": "Prove the exact cleanup-race sequence in 20 consecutive repetitions under the widened test-only window.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/244/conversation-cleanup-race-focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete Runtime kernel test target after the fixture timing repair.",
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
