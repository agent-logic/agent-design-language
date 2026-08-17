# Structured Output Record

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic HTML Observatory large-Polis performance/recovery proof and remediated review findings by adding latency/resource metrics, committed JSON evidence, DOM transcript-window integration, recovery-to-healthy proof, and implementation-head ancestry validation.

## Artifacts

- demos/html-observatory/app.js
- demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
- .csdlc/evidence/280/large_polis_performance_recovery_metrics.json
- .csdlc/prepared/issues/280/validate_preparation_bundle.py
- .csdlc/issues/280
- .csdlc/evidence/280

## Execution

- Bound browser-facing roster and event projections and integrated transcript pruning into actual conversation and governed-room DOM append paths.
- Added deterministic resource and latency metrics for large roster, retained transcript, retained event tail, and recovery actions.
- Added degraded-to-healthy recovery sequence proof for reconnect, Runtime restart, backpressure, offline, and version mismatch without granting browser authority or hiding stale state.
- Committed the machine-readable #280 metrics JSON evidence and tied it to exact integrated candidate base 557dd28d85746a8dc5109dcc674f5a606b8c9890 plus the proof execution head.
- Repaired the preparation validator so it proves the integrated candidate base is ancestral to the implementation head instead of requiring the worktree to remain at the pre-implementation base.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/280/validate_preparation_bundle.py"
    ],
    "purpose": "Validate #280 issue identity, dependency ancestry, integrated candidate base ancestry, performance/recovery proof scope, and forbidden sibling/parent ownership.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/280/280-preparation-contract.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/large_polis_performance_recovery.test.mjs"
    ],
    "purpose": "Run deterministic large-roster, long-transcript, stream pressure, resource/latency, reconnect/restart/offline/version-mismatch recovery proof and emit JSON metrics.",
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
