# Structured Output Record

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Made central ci.yaml the sole automatic PR workflow, preserved selected heavy validation on the configured 16-core runner, isolated standalone proofs and scheduled coverage behind explicit dispatch, coalesced duplicate head revisions, and added deterministic whole-workflow and path-routing guards.

## Artifacts

- .github/workflows/ci.yaml
- .github/workflows/nightly-coverage-ratchet.yaml
- .github/workflows/wp08-native-birthday.yml
- .github/workflows/wp09-native-birthday-identity.yml
- .github/workflows/wp10-native-birthday-continuity.yml
- .github/workflows/wp11-native-memory-palace.yml
- .github/workflows/wp12-native-capability-envelope.yml
- .github/workflows/wp13-authority-repair.yml
- .github/workflows/wp13-native-cognitive-profile.yml
- .github/workflows/wp13a-native-adaptive-learning.yml
- .github/workflows/wp14-native-acip.yml
- .github/workflows/wp14-production-acip-repair.yml
- .github/workflows/wp14-retained-native-proof.yml
- .github/workflows/wp15-native-birth-witness.yml
- adl/tools/ci_path_policy.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/validate_ci_workflow_policy.rb
- docs/tooling/CI_REQUIRED_AND_OPTIONAL_LANES.md
- .csdlc/evidence/234/ci-workflow-inventory.json

## Execution

- Removed automatic pull-request triggers from twelve standalone native and retained proof workflows while preserving manual dispatch.
- Removed scheduled CI and coverage execution and made slow proof explicit-dispatch only.
- Changed central CI concurrency to repository plus head SHA and gated heavy coverage aggregation before runner allocation.
- Stopped ordinary Rust and fail-closed paths from selecting optional demo smoke while retaining explicit demo dispatch.
- Added machine-readable skipped, deferred, soak, and duplicate-head dispositions.
- Added a whole-workflow policy validator, representative path-policy regressions, an operating procedure, and a retained workflow inventory.

## Validation

[
  {
    "command": [
      "ruby",
      "adl/tools/validate_ci_workflow_policy.rb"
    ],
    "purpose": "Scan all 17 workflows and prove one automatic PR entrypoint, explicit-only standalone workflows, no schedules, head-SHA concurrency, gated heavy jobs, and manual-only slow proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/234/ci-workflow-inventory.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove required heavy runner routing, job-level gates, manual-only slow proof, conditional coverage aggregation, manual Codecov upload, and post-merge suppression.",
    "outcome": "passed",
    "evidence_ref": "local:issue-234-ci-runtime-contracts:passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove docs, ordinary Rust, runtime, mixed policy, fail-closed, schedule, and explicit-dispatch classifications while keeping optional demo and soak work deferred.",
    "outcome": "passed",
    "evidence_ref": "local:issue-234-ci-path-policy:passed"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/ci_path_policy.sh",
      "adl/tools/test_ci_runtime_contracts.sh",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove shell syntax, Ruby syntax, all workflow YAML parsing, and clean patch whitespace.",
    "outcome": "passed",
    "evidence_ref": "local:issue-234-syntax-yaml-diff-hygiene:passed"
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
