# ADL-Owned CI Step Logs

ADL keeps GitHub Actions logs as useful operator context, but important CI
steps should also produce small ADL-owned log artifacts. This gives reviewers a
durable fallback when a step fails late, times out, or GitHub's native log view
is hard to inspect.

## Current Coverage

The first covered critical step is the `adl-coverage` job's `Coverage run and
summary (json)` step.

The workflow runs it through:

```bash
bash tools/run_ci_step_with_log.sh --name "coverage-run-summary-json" --log-root ci-step-logs -- bash tools/run_authoritative_coverage_lane.sh --authority "adl_coverage_always_on" --event-name "$GITHUB_EVENT_NAME"
```

The wrapper preserves the wrapped command's exit code. It does not hide
coverage failures, skip policy gates, or replace GitHub's normal job status.

## Artifact Shape

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

## GitHub Artifact Fallback

The CI workflow uploads `adl/ci-step-logs/` with `if: always()` whenever the
coverage lane is required. That means the log artifact should still be
available when the wrapped coverage command fails.

Download path:

1. Open the failed workflow run.
2. Open `Artifacts`.
3. Download `adl-coverage-step-logs`.
4. Inspect `metadata.json`, then `stderr.log`, `stdout.log`, or `combined.log`.

## Boundaries

- These artifacts are review and debugging evidence, not publication-ready
  evidence.
- Do not paste secrets into wrapped command arguments.
- The wrapper performs basic command-token redaction for sensitive-looking
  argument names, but CI steps should still avoid putting credentials on the
  command line.
- The wrapper is not a performance fix. It makes slow or failing CI steps
  diagnosable and durable.
