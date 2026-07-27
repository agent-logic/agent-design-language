# Structured Output Record

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #5692 closing-keyword policy and publication verifier enforcement.

## Artifacts

- implementation commit 89f447461
- cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6 => 8 passed
- cargo check --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-publish => passed
- cargo fmt --manifest-path csdlc-v2/Cargo.toml => passed
- git diff --check => passed

## Execution

- AGENTS.md now requires implementation PR bodies to include a GitHub closing keyword such as Closes #<issue>.
- csdlc-v2 publication request and remote PR validation now require a real GitHub closing keyword for the tracked issue.
- csdlc-publish existing-PR publication-mode guard now uses the same closing-keyword predicate.
- Focused gate6 tests cover accepted closing keywords and rejected bare issue mentions.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6"
    ],
    "purpose": "Focused publication closing-keyword verifier regression proof",
    "outcome": "passed",
    "evidence_ref": "local command output: gate6 8/8 passed at implementation commit 89f447461"
  },
  {
    "command": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-publish"
    ],
    "purpose": "Publish binary compile proof plus formatting/diff hygiene",
    "outcome": "passed",
    "evidence_ref": "local command output: csdlc-publish check passed; cargo fmt and git diff --check passed"
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
