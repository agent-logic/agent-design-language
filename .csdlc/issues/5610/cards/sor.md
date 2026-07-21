# Structured Output Record

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Normalize compiler-emitted coverage filenames lexically before ownership matching while failing closed on repository-prefix or owned-root traversal escapes.

## Artifacts

- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh

## Execution

- Normalize slash-unified filenames with POSIX lexical semantics
- Permit bounded parent traversal only beneath the owned source root
- Add exact safe-path and repository/owned-root escape regressions

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
