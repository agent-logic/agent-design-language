# Structured Output Record

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recorded the #114 durable-history coordination-parent integration proof after #276, #277, and #278 reached terminal ancestry. The proof remains parent-only and current authority is the repaired SIP scope/operator constraints, STP acceptance/deliverables, VPP, SRP, SOR, and validators. The issue-owned validator explicitly reports two typed-tool-limited stale STP fields (task_boundary and non_goals) that implemented-phase csdlc-edit cannot mutate; they are treated as historical residue, not current publication authority.

## Artifacts

- .csdlc/prepared/issues/114/validate_preparation_bundle.py
- adl/tools/validate_v092_durable_history_parent_integration.py
- adl-runtime-kernel/tests/durable_conversation_history_integration.rs

## Execution

- Materialized and ran the #114 issue-owned bound parent validator; it now rejects stale preparation-only markers outside the known typed-tool-limited STP task/non-goals fields and reports those fields explicitly.
- Added a focused durable conversation history parent integration regression covering restart, duplicate attempt admission, receipts, replay ownership, retention, deletion, and Observatory transcript restoration.
- Added the #114 integration proof runner that verifies terminal ancestry for #276, #277, and #278.
- Repaired implemented-phase SIP operator constraints and declared scope through typed owner routes so #114 remains parent-only and does not claim #276/#277/#278/#271/#115/#116/#117 scope.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/114/validate_preparation_bundle.py"
    ],
    "purpose": "Validate bound/implemented #114 identity, preserved design/diagram digests, terminal ancestry for #112/#265/#270/#271/#276/#277/#278, and explicit classification of typed-tool-limited stale STP task/non-goals fields.",
    "outcome": "passed",
    "evidence_ref": "local:114-issue-owned-bound-parent-validator"
  },
  {
    "command": [
      "python3",
      "adl/tools/validate_v092_durable_history_parent_integration.py"
    ],
    "purpose": "Validate #276/#277/#278 terminal caches, merged dispositions, merge-SHA ancestry, parent-only ownership, and preserved boundary truth.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-terminal-chain-validator"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "durable_conversation_history_integration",
      "--",
      "--nocapture"
    ],
    "purpose": "Run focused durable-history parent integration test across history, continuity, journal retention/deletion, restart, duplicate attempt admission, receipts, replay owner state, and Observatory transcript restoration.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-runtime-kernel-integration-test"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "durable_conversation_history_integration",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict clippy for the #114 focused Runtime kernel integration test target.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-hygiene"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Check whitespace/diff hygiene for the #114 parent proof surface.",
    "outcome": "passed",
    "evidence_ref": "local:114-diff-hygiene"
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

- If fresh design/card review passes and doctor is green, request explicit Planning authority before any bind or child implementation.
