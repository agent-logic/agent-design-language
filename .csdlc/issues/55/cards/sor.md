# Structured Output Record

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Routed heavyweight hosted coverage aggregation through the established selected 16-core runner contract.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh

## Execution

- Changed only adl_coverage_hosted from ubuntu-latest to the centralized ADL_HEAVY_RUNNER expression.
- Added focused contract assertions rejecting standard-runner aggregation and updated the heavy-job count to eleven.
- Preserved stable adl-coverage, producers, Spot, artifacts, Codecov, and coverage policy semantics.

## Validation

[
  {
    "command": [
      "ruby",
      "-e",
      "require 'yaml'; YAML.load_file('.github/workflows/ci.yaml')"
    ],
    "purpose": "Parse the changed GitHub Actions workflow.",
    "outcome": "passed",
    "evidence_ref": "local:workflow-yaml:passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove selected heavy-runner routing and preserved coverage topology.",
    "outcome": "passed",
    "evidence_ref": "local:ci-runtime-contracts:passed"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove surrounding coverage route and policy semantics remain intact.",
    "outcome": "passed",
    "evidence_ref": "local:ci-path-policy:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_runner_preflight"
    ],
    "purpose": "Prove selected 16-core runner eligibility and redacted fail-closed preflight behavior.",
    "outcome": "passed",
    "evidence_ref": "local:gate-runner-preflight:1-passed"
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
