# Structured Output Record

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded HTML Observatory Runtime v3 launch/start-stop-restart integration and stabilized an existing Runtime distributed learner coverage test that blocked PR #430. The CI-only fix keeps #340 product behavior unchanged and makes the learner replication test write through the currently writable Raft leader instead of assuming the initially observed leader remains current under coverage-mode timing.

## Artifacts

- CSMctl
- adl-runtime/src/bin/adl-observatory-static.rs
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- adl/tools/test_html_observatory.sh
- adl/tools/validate_v092_observatory_restart_reconnect.sh
- adl-runtime/tests/runtime_api_wss.rs
- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
- .csdlc/issues/340
- .csdlc/prepared/issues/340
- .csdlc/evidence/340

## Execution

- Retained the #340 CSMctl/HTML Observatory axum static-server implementation and documented Runtime/Observatory separation unchanged.
- Added a test-only helper in learner_transport tests that submits governed mutations to whichever node is currently writable, bounded by a 15-second election timeout.
- Updated the real_four_node_learner_replication test's leader-sensitive writes to use the helper so coverage-mode elections no longer fail with ForwardToLeader from a stale leader.
- Kept the change limited to the pre-existing distributed learner test harness failure observed in PR #430 coverage CI; no #341, #343, Unity, AWS/public hosting, provider credential, #84, #122, or #251 scope was added.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check after bounded learner transport test-harness stabilization.",
    "outcome": "passed",
    "evidence_ref": "local command exited 0 in /Volumes/FastWork/adl-worktrees/adl-issue-340-html-observatory-runtime-restart-integration"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Focused non-coverage proof for the exact distributed learner replication test that failed in PR #430 coverage CI.",
    "outcome": "passed",
    "evidence_ref": "1 passed, 0 failed, 324 filtered out; ADL_ISSUE_202_CASE_V1 real_four_node_learner_replication=passed"
  },
  {
    "command": [
      "cargo",
      "llvm-cov",
      "test",
      "distributed::transport::governed::learner_transport::tests::real_four_node_learner_replication",
      "--",
      "--exact",
      "--nocapture"
    ],
    "purpose": "Focused local coverage-mode repro for the exact GitHub failing lane.",
    "outcome": "passed",
    "evidence_ref": "1 passed, 0 failed, 324 filtered out under cargo llvm-cov in adl-runtime"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "--",
      "adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs"
    ],
    "purpose": "Whitespace and conflict-marker hygiene for the narrow CI-stabilization diff.",
    "outcome": "passed",
    "evidence_ref": "exit 0 with no output"
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
