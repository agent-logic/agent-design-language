# Required And Optional CI Lanes

## Operating Rule

ADL uses the configured 16-core GitHub-hosted runner for required heavy
validation. Cost control comes from preventing optional work from starting,
not from moving required work to a slower runner.

`.github/workflows/ci.yaml` is the only workflow that starts automatically for
a pull request. Its lightweight path-policy job classifies the exact changed
surface before any heavy job is eligible to acquire a runner.

## Lane Classes

| Class | Automatic PR behavior | Runner policy |
|---|---|---|
| Required light | Runs only when selected | Standard runner |
| Required heavy | Runs only when selected | `vars.ADL_HEAVY_RUNNER` (16-core) |
| Required focused coverage | Runs only when selected | 16-core runner |
| Native or retained proof | Deferred | Explicit dispatch only |
| Slow proof or soak | Deferred | Explicit dispatch only |
| Demo or provider canary | Deferred unless its own changed surface explicitly requires the bounded central demo lane | Explicit dispatch for standalone workflows |
| Nightly, release, or full ratchet coverage | Deferred | Explicit dispatch only |

Changing a shared manifest, crate root, or library module does not authorize
every native proof. The central classifier selects the smallest required lane;
standalone proofs remain available for an operator-requested exact-head proof.

## Duplicate And Superseded Revisions

The central workflow concurrency key uses the target repository and workflow
plus the source repository, source branch, and target base. Two pull request
objects for the same effective surface share one execution group, and a new
commit to that branch cancels its older run. Publication should reuse an open
pull request for the same source branch and base rather than creating another
one.

## Unknown Paths

Unknown or unclassifiable paths fail closed to the conservative required
baseline. They do not authorize native proofs, demos, providers, slow proofs,
soaks, or scheduled coverage.

## Machine-Readable Reasons

The path-policy step emits the selected validation reason and these stable
dispositions:

- `unselected_required_lanes_status=skipped`
- `optional_workflows_status=deferred`
- `soak_workflows_status=deferred`
- `duplicate_head_status=canceled`

Each disposition includes a reason. Skipped and deferred workflows do not run
merely to report that they were skipped.

## Change Procedure

1. Classify a proposed lane as required or explicit before adding a workflow.
2. Add required PR work to `ci.yaml` behind an existing or new path-policy
   output.
3. Keep standalone proof, soak, canary, and ratchet workflows on
   `workflow_dispatch` only.
4. Run `ruby adl/tools/validate_ci_workflow_policy.rb` and
   `bash adl/tools/test_ci_runtime_contracts.sh` locally.
5. Obtain bounded review, then publish one revision. Do not use repeated hosted
   runs as an editing loop.

The whole-workflow validator rejects a second automatic PR workflow, any
scheduled heavy workflow, missing source-branch/base concurrency, an ungated heavy job,
or an optional workflow that is no longer explicitly dispatchable.
