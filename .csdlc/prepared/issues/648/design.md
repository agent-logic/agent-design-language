# Issue #648 Design: Post-merge provider reload ownership correction

## Purpose

PR #646 merged at `4c442cef90b06c4a491860ce1e9d9053dfed26eb` after exact-head review identified a P1 provider reload ownership defect. This corrective issue exists to land the already-developed run-scoped provider reload ownership repair against current `main` through a fresh typed issue path.

## Boundary

The correction is source/test-only and offline. It must not restart, stop, replace, or mutate the live Runtime or its active #640 configuration. It must not perform credential-backed provider inference, paid-provider runs, AWS operations, or live cutover work.

## Required behavior

- Production CSM `adl_workflow` execution owns a run-scoped `ProviderReloadHandle`.
- Sequential execution, deterministic concurrent execution, step retry execution, and called-workflow recursion receive that scoped handle.
- The compatibility process-global reload slot remains identity-aware: an older guard can only clear the registration it owns.
- Regression coverage proves overlapping workflows cannot consume or clear each other's provider snapshot.
- Regression coverage directly proves global guard old-drop/new-registration behavior.

## Existing local repair evidence

The #622 issue worktree contains a reviewed local correction packet ahead of the merged PR branch:

- `309de1037c7b604455691332de51044ebfca16ae` implements run-scoped handle propagation and overlap/shutdown regression coverage.
- `0b1bc46ca5f137871d870557c9334ff4b7d5a7a6` adds direct compatibility global-guard regression coverage.
- Focused local validation passed for the production lane, safety lane, fmt, and clippy.
- Independent exact-head review passed for `0b1bc46ca5f137871d870557c9334ff4b7d5a7a6`, but it could not be published through PR #646 because #646 had already merged at the older head.

## Review questions

- Does the corrective PR contain the run-scoped ownership fix relative to current `main`?
- Do the tests fail against the old process-global ownership behavior?
- Does the compatibility global fallback remain safe without becoming the production ownership model?
- Are the live Runtime and provider credential boundaries preserved?
