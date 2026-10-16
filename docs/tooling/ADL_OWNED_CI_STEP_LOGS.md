# ADL CI evidence and historical step logs

ADL keeps GitHub Actions logs as useful operator context, but important CI
steps should also produce small ADL-owned log artifacts. This gives reviewers a
durable fallback when a step fails late, times out, or GitHub's native log view
is hard to inspect.

## Current public coverage

The `public_adl_coverage` job in `.github/workflows/ci.yaml` runs
`tools/public_adl/coverage.py`. Its always-run upload step retains the
`public-adl-coverage-<run-id>-<attempt>` artifact for seven days, including
`result.json`, `executions.json`, per-execution `summary.json`, `coverage.lcov`
and log files. The `adl-coverage` job aggregates the selected lane's result.

The former monorepo coverage wrapper and `adl-coverage-step-logs` artifact are
historical. The retired `run_authoritative_coverage_lane.sh` is no longer a
current command. The log-wrapper description below documents its retained
format; it does not describe the current coverage workflow.

The wrapper preserves the wrapped command's exit code. It does not hide
coverage failures, skip policy gates, or replace GitHub's normal job status.

## Historical wrapper artifact shape

Each wrapped step writes a timestamped directory under `adl/ci-step-logs/`:

- `stdout.log`
- `stderr.log`
- `combined.log`
- `metadata.json`

`metadata.json` uses schema `adl.ci.step_log.v1` and records:

- step name
- start and finish timestamps
- elapsed seconds
- exit code
- redacted command tokens
- repo-relative log paths when possible

## Current GitHub artifact download

The current coverage workflow attempts its artifact upload with `if: always()`.
If execution failed before producing evidence, there may be no files to upload;
an upload step is not proof that an artifact exists.

Download path:

1. Open the failed workflow run.
2. Open `Artifacts`.
3. Download `public-adl-coverage-<run-id>-<attempt>` if present.
4. Inspect `result.json`, `executions.json` and the per-execution logs.

## Boundaries

- These artifacts are review and debugging evidence, not publication-ready
  evidence.
- Do not paste secrets into wrapped command arguments.
- The wrapper performs basic command-token redaction for sensitive-looking
  argument names, but CI steps should still avoid putting credentials on the
  command line.
- The wrapper is not a performance fix. It makes slow or failing CI steps
  diagnosable and durable.
