# Structured Output Record

Template: 1.0.0

Issue: 137

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Registered a dispatch-only, exact-SHA, three-platform native receipt workflow with pinned actions, unique fragments, live hosted attestation, and fail-closed Ubuntu aggregation.

## Artifacts

- .github/workflows/wp04-native-distributed.yml
- .csdlc/prepared/issues/137/validate-workflow.rb
- .csdlc/issues/137

## Execution

- Added a required lowercase 40-hex source_sha input with pre-checkout validation and post-checkout exact revision verification.
- Added bounded Linux, macOS, and Windows producer jobs using the existing #5878 producer and unique run-attempt-platform receipt identities.
- Added all-three artifact aggregation and the existing live GitHub run/job attestation validator on Ubuntu.
- Added an issue-owned static contract validator that also executes the repository path-policy suite.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run Git diff hygiene at the exact committed source revision.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/137/validate-workflow.rb"
    ],
    "purpose": "Execute the issue-owned focused workflow validator and repository path-policy suite.",
    "outcome": "passed",
    "evidence_ref": "wp04-native-workflow-contract.log"
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
