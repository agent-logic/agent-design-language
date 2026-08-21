# Structured Output Record

Template: 1.0.0

Issue: 449

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

#449 wires governed Adaptive Learning into a Runtime-owned resident-cycle path, retains durable resident terminal evidence for both binding and governed-executor rejection paths, and remediates PR #456 CI by aligning the integrated HTML Observatory proof fixture with the current Runtime v3 /v1/health fetch and Runtime Source label.

## Artifacts

- adl-runtime-kernel/src/adaptive_learning.rs
- adl-runtime-kernel/tests/adaptive_learning.rs
- adl-runtime-kernel/src/live_continuity.rs
- docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md
- adl/tools/validate_v0917_html_observatory.py
- .csdlc/issues/449

## Execution

- Added resident-cycle production entrypoints in adl-runtime-kernel/src/adaptive_learning.rs for execute and startup reconciliation.
- Added redacted ResidentAdaptiveLearningReceipt evidence that distinguishes accepted, rejected, cancelled, and restored outcomes without exposing private profile/provider content.
- Added redacted ResidentAdaptiveLearningTerminalEvidence retained in a separate governed durable domain for invalid resident binding/precondition failures, so rejected invalid inputs leave terminal proof without creating normal adaptive-learning history or mutating the graph/state.
- Remediated R5 P1 by retaining ResidentAdaptiveLearningTerminalEvidence when execute_governed_adaptive_learning returns executor-level errors such as malformed, stale, unbounded, unauthorized, invalid proposal, or mutation-preview failures.
- Added resident_cycle integration tests inside adl-runtime-kernel/tests/adaptive_learning.rs covering accepted MutationGate-only mutation, rejected non-mutation, invalid binding terminal-evidence retention, governed-executor failure terminal-evidence retention, restart restoration, and deterministic continuation.
- Updated docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md from library-only/local contract wording to resident-cycle integration proof truth.
- Fixed adl/tools/validate_v0917_html_observatory.py so the integrated Runtime v3 Observatory proof provides the current /v1/health mock payload and validates the current Runtime Source label instead of the stale Runtime Mirror label.
- Preserved #450 and sibling Runtime/Observatory work out of #449 scope; this remediation only restores #449 terminal-evidence truth and the required proof lane selected for PR #456.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Reject rustfmt drift in the touched Runtime kernel package after R5 P1 remediation.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-fmt-check"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "adaptive_learning",
      "resident_cycle"
    ],
    "purpose": "Prove #449 resident-cycle accepted mutation, rejected non-mutation, invalid binding terminal-evidence retention, governed-executor failure terminal-evidence retention, restart, and deterministic continuation behavior.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-resident-cycle-4-pass"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "adaptive_learning"
    ],
    "purpose": "Run the complete adaptive_learning integration target after resident terminal-evidence remediation.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-adaptive-learning-19-pass"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--tests",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict clippy for the touched Runtime kernel package/tests after R5 P1 remediation.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-strict-clippy-pass"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_v0917_html_observatory_integrated_proof.sh"
    ],
    "purpose": "Reproduce and fix the exact PR #456 failing Runtime v3 Observatory proof lane; proves /v1/health fixture and Runtime Source label truth.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-pr456-observatory-proof-pass"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and malformed diff defects after R5 P1 remediation.",
    "outcome": "passed",
    "evidence_ref": "root-turn-live-output:2026-08-20:r5-diff-hygiene-pass"
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
