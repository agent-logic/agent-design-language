# Structured Output Record

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Coalesce canonical coverage aliases only when their complete normalized records are identical, while failing closed on conflicts.

## Artifacts

- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh
- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh

## Execution

- Normalize slash-unified filenames with POSIX lexical semantics
- Permit bounded parent traversal only beneath the owned source root
- Add exact safe-path and repository/owned-root escape regressions
- Deduplicate identical complete records after ownership canonicalization
- Reject conflicting records for the same canonical owned filename
- Add exact identical-alias acceptance and conflicting-alias rejection regressions

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "purpose": "Prove exact safe lexical normalization, repository and owned-root escape rejection, unchanged merge semantics, and coupled authoritative coverage contracts.",
    "outcome": "passed",
    "evidence_ref": "FastWork: test_merge_coverage_summaries, test_ci_runtime_contracts, and test_run_authoritative_coverage_lane all passed; py_compile and git diff --check passed."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_merge_coverage_summaries.sh"
    ],
    "purpose": "Prove complete-record equality is required before canonical alias coalescing and conflicts remain fail-closed.",
    "outcome": "passed",
    "evidence_ref": "FastWork: py_compile, test_merge_coverage_summaries, test_ci_runtime_contracts, test_run_authoritative_coverage_lane, and git diff --check all passed."
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
