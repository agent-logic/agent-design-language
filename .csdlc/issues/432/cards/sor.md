# Structured Output Record

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Make .adl local-only while preserving every local file and relocating tracked worktree-policy authority.

## Artifacts

- adl/config/worktree-policy.json
- adl/tools/check_no_tracked_adl.sh
- adl/tools/test_check_no_tracked_adl.sh
- .csdlc/evidence/432
- .csdlc/issues/432

## Execution

- Remove all 27 .adl paths from the Git index without deleting working-tree files.
- Relocate canonical worktree-policy authority to adl/config and update source, tests, and agent policy.
- Add a deterministic repository-boundary guard with positive and negative regression cases.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_check_no_tracked_adl.sh"
    ],
    "purpose": "Run the issue-owned repository-boundary validator.",
    "outcome": "passed",
    "evidence_ref": "432-adl-boundary.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run Git diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "432-diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "lifecycle::fastwork_policy_tests"
    ],
    "purpose": "Prove relocated worktree-policy behavior.",
    "outcome": "passed",
    "evidence_ref": "432-fastwork-policy.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-432-remove-tracked-adl-execution",
      "issue",
      "--issue",
      "432"
    ],
    "purpose": "Run C-SDLC v2 issue validation.",
    "outcome": "passed",
    "evidence_ref": "432-typed-issue.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
