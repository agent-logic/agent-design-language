# Issue #678 Design: Stable CSM route follows active Runtime generation

## Decision

Make `.adl/bin/csm` an installer-managed launcher that resolves and executes
`.adl/runtime-v3/current/bin/csm` at invocation time.

The stable operator path must not be an independently compiled Runtime-control
binary. The active generation remains the single source of truth for CSM,
Guardian, and kernel artifacts. Activation and rollback continue to move the
atomic `.adl/runtime-v3/current` pointer; the stable launcher follows that
pointer without a separate copy step.

## Behavior

- `adl/tools/install_runtime_v3_generation.sh` writes `.adl/bin/csm` as a small
  executable launcher.
- The launcher computes the repository-local Runtime v3 current path relative
  to itself, rejects missing or non-executable current-generation CSM, and then
  `exec`s `.adl/runtime-v3/current/bin/csm` with the original argv.
- The launcher refuses to mutate services when the active generation is missing
  or incomplete because it never reaches the generation-owned CSM binary.
- Existing generation activation and rollback semantics remain atomic because
  the stable command follows the `current` symlink.
- Non-CSM stable operator binaries are outside this issue.

## Validation

Focused shell tests use an isolated repository fixture:

- install a valid generation and prove `.adl/bin/csm` dispatches to
  `.adl/runtime-v3/current/bin/csm`;
- replace the active generation and prove the same stable path follows the new
  generation;
- roll back the current symlink and prove the same stable path follows the
  rollback generation;
- remove or break the active generation and prove the stable path fails before
  invoking stale service-control behavior;
- place a deliberately stale independent CSM beside a valid current generation
  and prove installer repair replaces it with the launcher.

No live Runtime start, stop, reload, or rollout is required for local
validation.
