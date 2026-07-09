# CI Contract Split Evidence (#5037)

## Scope

Issue #5037 changes PR CI routing for tools/control-plane changes by splitting
the previous all-or-nothing `ci_contracts_required` workflow gate into focused
machine-readable outputs from `adl/tools/ci_path_policy.sh`.

## Before

When `ci_contracts_required=true`, `adl-ci` installed CI contract toolchain
helpers and ran unrelated contract families under the same gate:

- PVF release policy contract.
- tracked proof-validation lane contract.
- PR-fast test lane contract.
- slow-proof lane contract.
- authoritative coverage lane contract.
- repo-code-review, test-generator, demo-operator, arxiv-paper-writer, and
  ANRM/Gemma trace dataset skill/tooling contracts.
- CI runtime, runtime budget report, and cache/linker contracts.

That made narrow CI/path-policy changes pay for unrelated skill-author and proof
families even when the path-policy output already identified a smaller proof
surface.

## After

`adl/tools/ci_path_policy.sh` now emits focused booleans:

- `ci_path_policy_contracts_required`
- `ci_contract_toolchain_required`
- `pvf_ci_release_contract_required`
- `v0913_proof_contract_required`
- `slow_proof_contract_required`
- `skill_author_contracts_required`

`.github/workflows/ci.yaml` uses those booleans to run only the relevant
contract family for narrow PR policy surfaces. Broad Rust, coverage-required,
full-validation, and fail-closed paths still set `ci_contract_toolchain_required=true`
through `rust_required`, `coverage_required`, or `full_coverage_required`, and
full/fail-closed paths keep the heavier historical contract-family coverage.

## Expected CI Impact

For a narrow CI/path-policy tools PR, the workflow now skips:

- the two CI-contract tool install actions unless Rust or coverage proof needs
  them;
- PVF, tracked-proof, slow-proof, and skill-author contract families unless the
  changed surface selects those families.

The remaining path-policy proof is still explicit: path-policy, PR-fast runner,
authoritative coverage runner, CI runtime, budget report, and cache/linker
contracts stay gated by `ci_path_policy_contracts_required`.

Actual GitHub Actions wall-clock improvement must be confirmed by the #5037 PR
checks because runner scheduling, cache restore, and action setup time are
GitHub-hosted runtime properties.

## Local Proof

- `bash -n adl/tools/ci_path_policy.sh adl/tools/test_ci_path_policy.sh adl/tools/test_ci_runtime_contracts.sh`
  - result: pass
  - measured locally with `/usr/bin/time -p`: `real 0.00s`
- `bash adl/tools/test_ci_runtime_contracts.sh`
  - result: pass
  - measured locally with `/usr/bin/time -p`: `real 0.03s`
- `bash adl/tools/test_ci_path_policy.sh`
  - result: pass
  - covered the existing fast-path, full-path, and fail-closed path-policy
    fixture matrix, plus the new granular workflow output assertions.
