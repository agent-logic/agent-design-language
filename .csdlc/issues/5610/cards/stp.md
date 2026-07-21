# Structured Task Prompt

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Apply the already-reviewed two-file lexical normalization correction and exact regressions only.

## Deliverables

- Platform-independent lexical filename normalization
- Exact safe-path and traversal-escape regressions
- Focused FastWork validation and exact-head review

## Acceptance

1. AC-1: /repo/adl/src/bin/../aws_remote_validation.rs canonicalizes to /adl/src/aws_remote_validation.rs
2. AC-2: /repo/adl/src/../../outside.rs and ../adl/src/x.rs fail closed
3. AC-3: prefix traversal into an owned marker fails closed
4. AC-4: normalized paths outside /adl/src/ and /adl-runtime/src/ are not accepted
5. AC-5: provenance, metric, duplicate, total, atomic-output, and coverage gates are unchanged

## Dependencies

- Issue #5602 terminal receipt
- PR #5607 failed hosted coverage job 88566164781

## Inputs

- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh
- reviewed commit 5c1bbe71f

## Non Goals

- No PR #5607 or PR #5608 branch edits
- No coverage threshold or producer changes
- No #5602 terminal-state rewrite
- No AWS work
