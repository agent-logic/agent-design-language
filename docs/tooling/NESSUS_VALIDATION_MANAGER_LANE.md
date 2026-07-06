# Nessus Validation Manager Lane

`adl/tools/run_validation_manager_nessus_lane.sh` is the repo-native wrapper for
consuming the validation-manager selected local lane on the Nessus remote
runner.

Use it when the changed surface is eligible for exactly one deterministic local
validation lane and the operator wants to offload that lane to the existing
`nessus.local` runner without hand-copying the selected command.

## Command

```bash
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --changed-files <changed-files.txt> \
  --remote-artifact-dir <artifact-dir> \
  --remote-git-ref <branch-or-ref> \
  --report-out <validation-manager-report.json> \
  --run \
  --json
```

The wrapper first asks `adl/tools/validation_manager.sh` for the selected local
profile. If the profile contains exactly one runnable local command, the wrapper
passes that command back through the validation manager with:

```text
--remote-runner nessus --remote-command '<selected local command>'
```

The validation manager remains authoritative for remote eligibility. Docs-only,
tiny, none, escalated, nondeterministic, multi-lane, or missing-command profiles
fail closed instead of being routed to Nessus.

## Inputs

- `--changed-files <path>` uses a precomputed changed-file list. When the
  wrapper derives the remote command from this mode, it recreates that manifest
  inside the remote checkout at
  `.adl/tmp/validation-manager-nessus-changed-files.txt` before running the
  consumed lane.
- `--include-working-tree` lets the validation manager derive changes from the
  current worktree.
- `--base <ref> --head <ref>` lets the validation manager derive changes from
  git refs when neither of the above is provided.
- `--remote-command <command>` bypasses command derivation but still keeps the
  validation-manager remote eligibility gates.
- `--remote-artifact-dir <dir>` fetches the Nessus summary and bounded log
  bundle into a local directory.
- `--remote-git-ref <ref>` sets `ADL_NESSUS_REMOTE_GIT_REF` for the underlying
  Nessus runner. Use the pushed issue branch or an explicit commit ref for
  branch-specific live proof.
- `--report-out <path>` writes the validation-manager JSON report.

## Evidence Contract

When `--run` is supplied, the report should preserve both layers:

- `run[0].lane_id=nessus_remote_validation`
- `run[0].local_run` containing the consumed local lane metadata
- `remote_runner.requested=nessus`
- `remote_runner.decision=selected`
- `run_status=passed` only after the remote command exits successfully

The fetched Nessus `summary.json` records the command handed to the remote
runner, the resolved git ref, elapsed time, cache roots, and retained remote log
paths. It does not expand nested shell scripts into every child command.

For live SSH proof, publish the target branch/ref before running and pass it via
`--remote-git-ref`; otherwise the underlying Nessus runner's default ref applies.

## Builder Image And `cargo-nextest`

PR-fast Rust validation uses `cargo nextest`. Do not run PR-fast validation on
the raw Nessus host unless `cargo nextest --version` succeeds there. The
underlying runner fails closed before validation when a nextest-backed command
is selected and no builder image is configured.

Prefer the ADL builder image for repeatable PR-fast lanes:

```bash
ADL_NESSUS_BUILDER_IMAGE=<pullable-image-uri> \
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --changed-files <changed-files.txt> \
  --remote-artifact-dir <artifact-dir> \
  --remote-git-ref <branch-or-ref> \
  --run \
  --json
```

For a local image that has already been loaded on Nessus, keep the image tag
local and disable pulls explicitly:

```bash
ADL_NESSUS_BUILDER_IMAGE=adl-builder:v0.91.7-fixed \
ADL_NESSUS_BUILDER_PULL_POLICY=never \
bash adl/tools/run_validation_manager_nessus_lane.sh \
  --changed-files <changed-files.txt> \
  --remote-artifact-dir <artifact-dir> \
  --remote-git-ref <branch-or-ref> \
  --run \
  --json
```

The summary records the builder image, runtime, pull policy, local-image
presence, and whether a pull was attempted. Do not treat a local-only tag as a
registry URI unless it is actually published.

## Non-Claims

This wrapper does not make Nessus the default lane for every issue. It also does
not copy provider, GitHub, AWS, or other operator credentials to Nessus. Remote
provider or network-bound validation remains out of scope unless a tracked issue
declares and proves that credential boundary explicitly.

## Related Evidence

- `docs/milestones/v0.91.6/review/build_throughput/NESSUS_REMOTE_VALIDATION_LANE_4553.md`
- `docs/milestones/v0.91.6/review/build_throughput/REMOTE_BUILD_LANES_4587.md`
- `docs/milestones/v0.91.7/review/build_throughput/NESSUS_VALIDATION_MANAGER_LANE_4678.md`
