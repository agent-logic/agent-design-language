# Structured Output Record

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Recovered PR #561 after hosted workspace coverage run 33021333783 and stabilized the shared coverage gate by granting only the exact runtime_v2 unified-kernel module prefix a bounded ci-coverage nextest timeout override while correcting current v0.92.1 context-mirror milestone truth.

## Artifacts

- adl/.config/nextest.toml
- adl/src/adl_gws_context_mirror.rs
- .csdlc/prepared/issues/560/design.md
- .csdlc/prepared/issues/560/diagram.mmd
- .csdlc/prepared/issues/560/validate-focused-proof.sh
- .csdlc/prepared/issues/560/validate-lifecycle-evidence.sh
- .csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log
- .csdlc/evidence/560/context-mirror-temp-repo-compat.log
- .csdlc/evidence/560/lifecycle-evidence-hygiene.log
- .csdlc/evidence/560/diff-hygiene.log

## Execution

- Changed the ci-coverage nextest override from three exact runtime_v2 unified-kernel test names to the exact fully qualified `runtime_v2::tests::unified_runtime_kernel::*` module prefix after run 33021333783 proved four sibling module tests still hit the generic 120s coverage ceiling.
- Kept the override ceiling at 240s with terminate-after = 1, leaving the profile-wide ci-coverage timeout at 120s for all unrelated tests.
- Updated the issue-owned focused proof to assert the selector denominator is exactly seven tests, then run those seven tests under cargo llvm-cov nextest ci-coverage.
- Updated the context-mirror milestone truth path to read the current ADL feature list in addition to README milestone markers, accept v0.92.1 as a current milestone, and expect v0.92 activation no longer blocked when v0.92.1 is current.
- Added temp-repo fixture coverage for the feature-list dependency so execute-mode context mirroring remains compatible outside the repository root.
- Recorded typed review/publication recovery, design-review recovery, refreshed authored design/diagram bindings, refreshed VPP validation lanes, and retained current local proof evidence.
- Kept Runtime v2 product semantics and unified-kernel test assertions unchanged; the change is instrumentation-aware coverage scheduling plus stale baseline/test truth repair.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/560/validate-focused-proof.sh"
    ],
    "purpose": "Prove the ci-coverage override selector matches exactly seven runtime_v2 unified-kernel module tests, run all seven under cargo llvm-cov nextest ci-coverage, run the current-repo v0.92.1 milestone truth test, and run diff hygiene.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "-p",
      "adl",
      "--lib",
      "adl_gws_context_mirror::tests::execute_mode_recursively_mirrors_markdown_with_verified_content"
    ],
    "purpose": "Prove the context-mirror execute-mode temp-repo fixture remains compatible with the feature-list milestone reader dependency.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/560/context-mirror-temp-repo-compat.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/560/validate-lifecycle-evidence.sh"
    ],
    "purpose": "Verify issue-local lifecycle state and evidence directory exist before review/publication.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/560/lifecycle-evidence-hygiene.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene drift.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/560/diff-hygiene.log"
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
