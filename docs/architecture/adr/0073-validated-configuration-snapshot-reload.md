# ADR 0073: Validated Configuration Snapshot Reload

## Status

Proposed. Tracked by issue #745. Documents the HOT-01 contract and its current implementation without widening reload scope.

## Context

Axum consumers need configuration changes without process restart. Mutating individual live fields can expose partial state, while restarting stateful services to apply a flag couples simple configuration changes to resource lifecycle management.

## Decision

Load and validate a complete candidate, then publish a complete immutable snapshot for subsequent reads. Existing readers may retain their prior snapshot. Debounce observed file changes; detect changed contents rather than relying only on timestamp or length. Invalid or incomplete candidates retain the last-known-good snapshot and expose bounded failure diagnostics. Cancellation and watcher shutdown are explicit.

The generic helper owns snapshot delivery. Consumers own semantic validation and any application callback. Do not interpret snapshot atomicity as a transaction across arbitrary external side effects. The current Runtime Observatory origin-policy consumer applies validated policy through the control service; malformed origin input retains the previous allowlist.

Keep routine reload limited to the declared strings, flags, limits, and templates. Database pools, credentials, listeners, and authority-bearing resources require separate lifecycle designs. The generic parser/applier API does not itself prove every caller respects these exclusions.

## Consequences

Readers receive whole values and failed candidates do not replace valid state. Watcher ownership, shutdown, validation, and consumer application remain explicit responsibilities. A stateful resource change must follow its own lifecycle rather than masquerading as ordinary config reload.

## Alternatives Considered

Update fields in place: rejected because readers could observe mixed generations. Restart the whole process for each stateless change: unnecessary for this bounded contract. Reload every serializable object: rejected because object deserialization does not manage resource identity, credentials, or authority.

## Supersession Relationships

Refines configuration consequences of [ADR 0054](../../adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md). Does not supersede runtime authority or storage decisions.

## Source Evidence

- [HOT-01 feature contract](../../milestones/v0.92.1/features/AXUM_CONFIGURATION_HOT_RELOAD_v0.92.1.md)
- [Milestone decisions](../../milestones/v0.92.1/DECISIONS_v0.92.1.md)
- [Operator documentation](../../runtime/config-hot-reload.md)
- [Current kernel implementation](../../../adl-runtime-kernel/src/config_reload.rs)
- [Retained reload tests](../../../adl-runtime/tests/config_reload.rs)

## Validation Notes

The helper currently lives in adl-runtime-kernel/src/config_reload.rs; the older operator document still names its original owner file. This ADR uses the inspected implementation location. Runtime tests are source evidence and were not rerun for this documentation packet. Validate concurrent snapshot reads, rejected candidates, burst handling, recovery, and shutdown when implementation changes.

## Approval Boundary

Normal architecture/documentation review is required. No stateful reload, credential rotation, or new authority-bearing policy lifecycle is approved here.
