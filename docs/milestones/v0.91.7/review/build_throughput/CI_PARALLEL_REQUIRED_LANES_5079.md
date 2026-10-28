# CI Parallel Required Lanes Evidence (#5079)

Issue #5079 follows up #5037. #5037 split CI contract routing, but broad
Rust/tools PRs still waited on one serial `adl-ci` job. This change preserves
the stable required check name `adl-ci` as an aggregator and moves independent
work into parallel jobs.

## Workflow Change

- `adl_path_policy` classifies changed paths once and publishes the path-policy
  outputs used by the split lanes.
- `adl_tooling_contracts` runs shell/tooling/CI contract checks.
- `adl_rust_fmt_clippy` runs Rust formatter and clippy proof when Rust is
  required.
- `adl_rust_tests` runs PR-fast Rust tests and doc tests when Rust is required.
- `adl_demo_proof` runs or truthfully skips demo/proof validation lanes.
- `adl-ci` remains the stable required check and fails closed if any split lane
  fails or is cancelled. Skipped lanes are accepted only when path policy skipped
  them.

This changes wall-clock behavior for ordinary Rust/tools PRs from serial
execution to the max duration of the independent required lanes plus the small
aggregator step.

## Local Proof

- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_ci_path_policy.sh`
- `bash adl/tools/test_summarize_ci_runtime.sh`
- `ruby -e 'require "yaml"; YAML.load_file(".github/workflows/ci.yaml"); puts "YAML_OK"'`
- `git diff --check`

Local proof validates the workflow contract, path-policy behavior, runtime
budget summarizer, YAML parseability, and whitespace hygiene. Hosted GitHub
Actions wall-clock improvement cannot be proven locally; it must be observed on
the #5079 PR tail after publication.

## Non-Claims

- This does not weaken runtime, provider, security, release, or fail-closed
  escalation proof.
- This does not change `adl-coverage`; coverage speed remains governed by its
  existing split and path-policy behavior.
- This does not migrate branch protection away from the stable `adl-ci` check
  name.
