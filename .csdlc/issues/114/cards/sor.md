# Structured Output Record

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recorded the #114 durable-history coordination-parent integration proof after #276, #277, and #278 reached terminal ancestry. Review remediation added persisted retention-marker restart assertions and narrowed the terminal-chain validator claim to its actual proof boundary; lifecycle/card parent ownership remains enforced by the issue-owned preparation bundle validator. Publication, merge, and finish remain pending.

## Artifacts

- .csdlc/prepared/issues/114/validate_preparation_bundle.py
- adl/tools/validate_v092_durable_history_parent_integration.py
- adl-runtime-kernel/tests/durable_conversation_history_integration.rs

## Execution

- Added explicit retained ConversationJournal restart assertions before deletion so retention marker fields, total journal records, committed event continuity, and Observatory transcript visibility are proved before deletion is recorded.
- Narrowed adl/tools/validate_v092_durable_history_parent_integration.py to claim terminal child cache ancestry and focused test-surface presence only, while pointing lifecycle/card ownership proof to the issue-owned #114 preparation validator.
- Preserved #114 parent-only scope and did not absorb #276, #277, #278, #271, #115, #116, or #117 product behavior.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/114/validate_preparation_bundle.py"
    ],
    "purpose": "Validate bound/implemented #114 identity, preserved design/diagram digests, terminal ancestry for #112/#265/#270/#271/#276/#277/#278, and absence of stale preparation-only markers in current publication-relevant card truth.",
    "outcome": "passed",
    "evidence_ref": "local:114-issue-owned-bound-parent-validator-gen86"
  },
  {
    "command": [
      "python3",
      "adl/tools/validate_v092_durable_history_parent_integration.py"
    ],
    "purpose": "Validate #276/#277/#278 terminal caches, merged dispositions, merge-SHA ancestry, and focused test-surface presence; parent ownership/lifecycle truth is separately validated by the issue-owned bundle validator.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-terminal-chain-validator-gen86"
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
    "purpose": "Run focused durable-history parent integration test across restart, duplicate attempt admission, receipts, replay owner state, retained marker persistence before deletion, deletion cleanup, and Observatory transcript restoration.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-runtime-kernel-integration-test-reviewfix"
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
    "purpose": "Run strict Clippy for the #114 focused Runtime kernel integration test target.",
    "outcome": "passed",
    "evidence_ref": "local:114-parent-hygiene-reviewfix"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Check whitespace/diff hygiene for the #114 parent proof surface.",
    "outcome": "passed",
    "evidence_ref": "local:114-diff-hygiene-reviewfix"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-114-durable-history-parent-integration-proof",
      "issue",
      "--issue",
      "114"
    ],
    "purpose": "Validate canonical #114 lifecycle truth after review recovery and remediation.",
    "outcome": "passed",
    "evidence_ref": "local:114-csdlc-validate-gen86"
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

- Obtain fresh no-context exact-head review over the current #114 immutable head and repaired lifecycle truth.
- Publish through typed csdlc-publish with correct Closes #114 linkage only after review PASS and base/head ancestry are current.
- Shepherd required CI and use typed csdlc-finish only after exact green authority.
- After #114 terminal cache validates canonical and ancestral, refresh #115/#116/#117 readiness without absorbing their child scope.
