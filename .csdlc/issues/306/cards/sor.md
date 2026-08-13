# Structured Output Record

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented ordered publication metadata tail handling: intent cache is stored under Git common-dir, record_publication metadata is committed as a governed metadata-only follow-up head, pushed, and reobserved before publication result.

## Artifacts

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/publication_tail.rs
- exact-head-review:43fc25089593cbcb84221ea3207ae7fd598a92ef

## Execution

- Moved publication intent persistence behind csdlc_v2::publication::persist_publication_intent using Git common-dir storage.
- Added commit_publication_metadata_tail to fail closed on pre-staged non-governed paths, commit only .csdlc/issues/<issue> metadata with an explicit pathspec, and verify the follow-up commit is metadata-only.
- Updated csdlc-publish to push the reviewed head, record publication metadata, commit/push the metadata-only follow-up head, and reobserve the PR at that exact head.
- Added publication_tail integration coverage for linked worktrees, create/update/noop action classes, interruption/retry cleanliness, finish-readiness, and non-governed pre-staged path rejection.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail"
    ],
    "purpose": "Focused Rust integration proof for #306 publication metadata tail ordering, retry determinism, and finish-readiness.",
    "outcome": "passed",
    "evidence_ref": "local-command:43fc25089593cbcb84221ea3207ae7fd598a92ef:cargo-test-publication-tail:4-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict warning-free proof for touched publication code and #306 publication_tail test target.",
    "outcome": "passed",
    "evidence_ref": "local-command:43fc25089593cbcb84221ea3207ae7fd598a92ef:cargo-clippy-publication-tail:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail"
    ],
    "purpose": "Focused Rust integration proof for #306 publication metadata tail ordering, interrupted-after-record retry recovery, and finish-readiness.",
    "outcome": "passed",
    "evidence_ref": "local-command:4a2c66086f883f6feb7ead1661fd1acad043acd4:cargo-test-publication-tail:5-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict warning-free proof for touched publication code and #306 publication_tail test target after retry recovery fix.",
    "outcome": "passed",
    "evidence_ref": "local-command:4a2c66086f883f6feb7ead1661fd1acad043acd4:cargo-clippy-publication-tail:passed"
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
