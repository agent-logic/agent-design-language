# Structured Output Record

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement deterministic HTML Observatory large-Polis performance/recovery proof with bounded roster, transcript, event, and recovery-state projections.

## Artifacts

- demos/html-observatory/app.js
- demos/html-observatory/tests/large_polis_performance_recovery.test.mjs
- .csdlc/evidence/280/large_polis_performance_recovery_metrics.json
- .csdlc/issues/280
- .csdlc/prepared/issues/280

## Execution

- Add deterministic large-Polis fixture/proof helpers to the HTML Observatory model without changing Runtime authority or protocol semantics.
- Bound browser-facing large roster and event-tail projections while preserving exact total counts for proof and operator truth.
- Add a focused large-Polis performance/recovery Node proof that emits public-safe metrics for roster growth, long transcripts, stream pressure, reconnect, restart, offline, and version-mismatch recovery.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff hygiene check.",
    "outcome": "passed",
    "evidence_ref": "280-diff-hygiene.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "purpose": "Run focused conversation-session regression.",
    "outcome": "passed",
    "evidence_ref": "280-observatory-conversation-regression.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/large_polis_performance_recovery.test.mjs"
    ],
    "purpose": "Run the #280 large-Polis deterministic Node proof.",
    "outcome": "passed",
    "evidence_ref": "280-observatory-large-polis-performance-recovery.log"
  },
  {
    "command": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "purpose": "Run focused operator-attention regression.",
    "outcome": "passed",
    "evidence_ref": "280-observatory-operator-attention-regression.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/280/validate_preparation_bundle.py"
    ],
    "purpose": "Run the issue-owned #280 preparation validator.",
    "outcome": "passed",
    "evidence_ref": "280-preparation-contract.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-280-large-polis-performance-recovery",
      "issue",
      "--issue",
      "280"
    ],
    "purpose": "Run C-SDLC v2 typed issue validation for #280.",
    "outcome": "passed",
    "evidence_ref": "280-typed-validate.log"
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
