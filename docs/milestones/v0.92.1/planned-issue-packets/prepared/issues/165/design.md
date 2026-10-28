# V3-04 Design

Issue: #165

## Objective

Implement the invocation-scoped dependency container and common I/O, configuration, error, cancellation, and observability services.

## Scope

`App`, lazy sync/async initialization, streams, TTY and prompting, configuration precedence, credential references, cancellation token, tracing, redaction, operation IDs, OS signal handling, error-to-exit mapping, and test constructors.

## Dependencies

- V3-03: issue #164

## Architecture Decisions

- `V3-D04`

## Deliverables

- Narrow traits including `ReviewerIdentityResolver`, an independently reviewed adapter-interface checkpoint, production and fake constructors, typed config schema, error taxonomy, cancellation policy, tracing contract, and redaction fixtures.

## Owned Paths

- `csdlc-v3/src/app/**`
- `csdlc-v3/src/error/**`
- `csdlc-v3/src/cancel/**`
- `csdlc-v3/tests/app/**`
- `.csdlc/issues/165/**`
- `.csdlc/prepared/issues/165/**`
- `.csdlc/prepared/issues/165/validate-outcome.rb`
- `.csdlc/evidence/165/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. One `App` exists per invocation and no mutable global service locator exists.
2. `Git`, `FileSystem`, and `ProcessRunner` signatures are reviewed and frozen at an explicit checkpoint before parallel V3-05 or V3-09 implementation begins.
3. Expensive or credential-bearing services initialize only on demand.
4. Sync lazy accessors initialize once without panic and propagate one cached typed result to concurrent callers.
5. Async lazy accessors cache completed success/error results while cancelled initialization remains uninitialized and retryable.
6. Cancelled async initialization remains single-flight on retry, applies the configured cooldown for localized cancellation/timeouts, and never retries after root cancellation.
7. The selected Tokio release is exact-version pinned, and deterministic leader drop tests prove state reset, waiter notification, exactly one cooldown-governed retry, and absence of deadlock, leaked waiter, or retained initializer future.
8. Sync initialization tests prove that one terminal error is cached for the invocation and is not changed by later filesystem mutation.
9. Async adapter traits remain object-safe without infecting pure domain APIs.
10. Supported OS and console interruption signals drive root cancellation and bounded child/task teardown before exit code 130.
11. Machine output is stdout-only and diagnostics/tracing are stderr-only by default.
12. Secrets and machine-local paths are absent from durable output.

## PVF Lanes

- `v3-04-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/165/validate-outcome.rb`.
- `v3-04-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-04-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Constructor call-count tests, config precedence tables, TTY/non-TTY tests, cancellation tests, error/exit snapshots, tracing channel tests, and redaction corpus tests.

## Authority Boundary

- Issue V3-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Domain lifecycle behavior, concrete GitHub endpoints, state transactions, detached telemetry, update checks, or background services.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A service requires global mutation, credentials enter state/config output, a detached task survives command completion, or local commands initialize network clients.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-04`
