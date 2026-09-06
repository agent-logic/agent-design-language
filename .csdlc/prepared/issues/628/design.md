# Issue #628 design — V3-H.2 local lifecycle routes

## Goal

Implement the local, non-GitHub C-SDLC v3 command routes under the single `csdlc` binary, using #627's command manifest as the route denominator and preserving v2 as live operational authority until #505 cutover.

## Route set

Issue #628 owns these one-binary routes:

- `csdlc issue`
- `csdlc bind`
- `csdlc edit`
- `csdlc validate`
- `csdlc doctor`
- `csdlc schedule`
- `csdlc shepherd`
- `csdlc eligibility`

## Design

1. Add typed request/response boundaries for local lifecycle commands in `csdlc-v3`, without delegating to v2 binaries or raw shell wrappers.
2. Store local lifecycle state under an explicit v3 construction-state root so missing state is diagnosed as `missing_local_lifecycle_state` with a next action, not treated as mysterious loss.
3. Enforce the six-card lifecycle order `SIP -> STP -> SPP -> VPP -> SRP -> SOR` for any issue-local operation that reads or advances lifecycle state.
4. Preserve fail-closed behavior for routes that are not fully implemented in this issue.
5. Record a real issue canary that reaches ready-to-execute locally through v3 state in three minutes or less, with no v2 operational fallback.

## Boundaries

- No GitHub mutation, PR publication, review publication, finish, cleanup, or cutover.
- No `csdlc-v2/**` source edits.
- V3 output remains construction evidence only; v2 remains operational authority before #505.
- If a local route cannot be fully implemented, keep it explicitly fail-closed and record the defect for #632.

## Validation

- Focused local-route tests for positive preparation, stale digest, missing card, unsupported transition, unsafe primary checkout, and missing local state recovery.
- Issue-owned validator proving the eight route names have implemented or deliberately fail-closed behavior and do not call v2 or raw `gh`.
- Full `csdlc-v3` tests.
- Strict rustfmt/clippy.
- Typed v2 issue validation.
- Diff hygiene and no-v2-source-change checks.
