# Issue 209 Design: Production ACIP Authority Repair

## Outcome

Repair the merged WP-14 contract so authenticated ACIP traffic reaches a real
production Guardian/kernel dispatch boundary with bounded pressure and typed
errors, replay state cannot be poisoned across unrelated principals or
sessions, and the public OpenAPI contract matches runtime signature admission.

## Owned Paths

- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/acip.rs`
- `adl-runtime-kernel/tests/production_acip_wss.rs`
- `adl-runtime/src/runtime_api_auth.rs`
- `docs/api/runtime-v3/v1/openapi.json`
- `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md`
- `.csdlc/prepared/issues/209`
- `.csdlc/evidence/209`
- `.github/workflows/wp14-production-acip-repair.yml`

## Read-Only Inputs

- other `adl-runtime-kernel` Guardian/kernel operation and ACIP semantic-binding APIs
- merged issue #5832 / PR #76 and its retained native receipts
- issue #5834 Birthday review packet and its independent review findings

## Trust And State Contract

Authenticated principal identity plus an explicit replay domain/session owns
sequence state. A frame cannot advance another principal or session, cannot
use the maximum integer as an unbounded poison value, and cannot bypass
duplicate denial after reconnect or eviction. Production dispatch validates
admission before enqueuing one bounded operation and returns a typed success or
typed error without echo-only substitution.

The externally published schema requires every field runtime admission
requires. Negative-test-only unsigned shapes are not advertised as valid
dispatchable carrier frames.

## Failure And Transaction Boundary

Admission, replay reservation, bounded enqueue, kernel dispatch, and response
projection must fail closed. Rejected or pressured traffic does not advance an
unrelated replay domain. Typed error responses disclose no credentials,
machine-local paths, or private payload content.

## Validation

- focused production WSS success and typed-error integration
- bounded pressure saturation/recovery proof
- replay `u64::MAX`, cross-principal, cross-session, reconnect, duplicate,
  stale, and eviction adversarial cases
- OpenAPI/runtime admission parity
- strict Clippy, formatting, exact native Linux/macOS proof where required,
  and fresh independent exact-head review

## Rollback

Revert only issue-owned runtime/auth/test/schema/workflow/evidence changes. Do
not rewrite merged PR #76 or #5832 evidence. A rollback leaves #5834 blocked.

## Non-Goals

Broader transport redesign, cloud provisioning, public Birthday publication,
Sprint 3 work, or rewriting historical child records.
