# Issue #254 design: eliminate repeated hosted workspace coverage compiles

## Problem

The full PR coverage route currently compiles the workspace in the hosted workspace producer shards, then the `adl-coverage-hosted` aggregate job installs the Rust coverage toolchain and runs the workspace coverage lane again in report mode before enforcing the changed-source gate.

For the Azure-backed heavy runner this wastes time and money, and it also makes unrelated runtime PRs wait on an aggregate job that does not add independent behavioral proof.

## Decision

Make the full workspace hosted producer emit the authoritative workspace summary artifact directly. The aggregate job becomes a light verification/merge job:

- download runtime and workspace evidence;
- verify provenance and required summaries;
- copy the workspace summary into the expected merge location;
- merge isolated runtime/workspace summaries and run the existing policy gates.

The aggregate job must not install Rust, configure coverage acceleration, download profraw shard profiles, or invoke `run_authoritative_coverage_lane.sh`.

## Scope

- `.github/workflows/ci.yaml`
- `adl/tools/test_ci_runtime_contracts.sh`
- `adl/tools/test_ci_path_policy.sh`
- `adl/tools/validate_ci_workflow_policy.rb`

## Non-goals

- No optional, cloud, paid, native, slow, soak, or spot job dispatch.
- No change to #199 runtime behavior.
- No weakening of required `adl-coverage` semantics.
