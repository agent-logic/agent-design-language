# Issue 627 design — V3-H.1 command denominator and one-binary CLI shell

Status: design ready for typed C-SDLC v2 bootstrap.

## Purpose

Issue #627 is the serial gate for sprint #625. It defines the exact C-SDLC v3 replacement denominator and introduces the final one-binary command shell so later sprint children can implement behavior without arguing about names, missing routes, or command ownership.

## Authority boundary

- C-SDLC v2 remains the only live operational lifecycle authority until #505 is explicitly approved, merged, and terminally reconciled.
- C-SDLC v3 may add construction code, command contracts, tests, and fail-closed stubs in this issue.
- This issue must not perform GitHub lifecycle writes through v3, publish a PR as v3 authority, finish issues, clean worktrees, retire v2, or merge #505.
- No v2 source file may be changed by this issue.

## Denominator

The denominator is the 21 installed v2 binaries, with the operator-confirmed sprint target of 19 remaining command-equivalent routes because two v3 construction surfaces existed at sprint setup time: `foundation` and `local`.

The one-binary shell must expose or reserve all replacement routes through `csdlc` and record each route's status as implemented, partial, fail-closed, or deferred-with-operator-approval.

## Deliverables

- A machine-readable v3 command manifest under `docs/csdlc-v3/`.
- A one-binary CLI shell in `csdlc-v3/src/main.rs` and owned modules as needed.
- Help/manifest tests proving all denominator routes are visible and stable.
- Tests proving unimplemented live-authority routes fail closed rather than silently falling back to v2, raw `gh`, or shell wrappers.
- Issue-local validation script and evidence for the command denominator.

## Non-goals

- Do not implement every command's full behavior in #627.
- Do not claim v3 operational authority.
- Do not close #505.
- Do not mutate v2 source.
- Do not hide missing behavior behind v2 fallback.

## Validation plan

- Run focused `csdlc-v3` command-manifest tests.
- Run help-output snapshot or semantic coverage tests for the one-binary shell.
- Run a no-v2-source-change guard.
- Run diff hygiene.
