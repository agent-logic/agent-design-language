# Structured Output Record

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic HTML Observatory large-Polis performance/recovery proof and remediated exact-review duplicate-action finding by suppressing already-pending recovery actions across consecutive degraded frames.

## Artifacts

- demos/html-observatory/app.js
- demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
- .csdlc/issues/280
- .csdlc/evidence/280

## Execution

- Suppressed recovery actions that are already pending before exposing the per-step large-Polis recovery view to consumers.
- Marked repeated already-pending recovery actions as duplicate_action_prevented so metrics truthfully reflect suppressed duplicates.
- Added a focused disconnected-to-disconnected-to-healthy regression proving a consumer receives only one schedule_single_reconnect action while recovery still resolves cleanly.
- Re-ran the issue-owned preparation validator, large-Polis proof, existing conversation regression, operator-attention regression, and diff hygiene.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/280/validate_preparation_bundle.py"
    ],
    "purpose": "Validate #280 issue identity, dependency ancestry, performance/recovery proof scope, and forbidden sibling/parent ownership.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-preparation-contract.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/large_polis_performance_recovery.test.mjs"
    ],
    "purpose": "Run deterministic large-roster, long-transcript, stream pressure, duplicate-action, reconnect/restart/offline/version-mismatch recovery proof and emit JSON metrics.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-observatory-large-polis-performance-recovery.log and .csdlc/evidence/280/large_polis_performance_recovery_metrics.json"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "purpose": "Prove existing Runtime conversation acceptance/delivery/reconnect behavior still passes after #280 projection-bounding changes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-observatory-conversation-regression.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "purpose": "Prove operator-attention Runtime-authority boundaries still pass after #280 projection-bounding changes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-observatory-operator-attention-regression.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed whitespace or patch artifacts before review.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-diff-hygiene.log"
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
