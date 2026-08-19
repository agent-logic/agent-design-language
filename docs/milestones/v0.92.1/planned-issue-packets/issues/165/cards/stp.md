# Structured Task Prompt

Template: 1.0.0

Issue: 165

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-04 within its exact owned paths and authority boundary.

## Deliverables

- Narrow traits including `ReviewerIdentityResolver`, an independently reviewed adapter-interface checkpoint, production and fake constructors, typed config schema, error taxonomy, cancellation policy, tracing contract, and redaction fixtures.

## Acceptance

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

## Dependencies

- V3-03: issue #164

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-04
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Domain lifecycle behavior, concrete GitHub endpoints, state transactions, detached telemetry, update checks, or background services.
