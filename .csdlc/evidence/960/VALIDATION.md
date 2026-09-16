# Shutdown barrier acknowledgment proof

Issue #960 is a separate Runtime repair discovered while diagnosing PR #957.
The original CI failure's daemon stderr was not retained, so its exact cause is
still unknown. The preserved diagnostic copy at `.adl/960-diagnosis/` reproduced
an independent failure using supported 1 ms observability heartbeats: the daemon
exited 1 with `configured ADL_OTEL_STATUS sink did not acknowledge shutdown_observability_drain`.
The barrier appeared in JSONL before a heartbeat replaced the status snapshot.

The repair records the synchronous result of this event's local sink writes.
The status write returns while protected by the existing writer lock; subsequent
writers cannot change the returned acknowledgment. Existing void event callers
retain their behavior. This does not prove remote OTLP delivery, filesystem
survival after power loss, or permanent retention of a mutable monitor snapshot.
Configured local sink errors still fail the shutdown barrier.

## Planned focused validation

| Surface | PVF lane / role | Determinism and resources | Release gate |
| --- | --- | --- | --- |
| `event_receipt_survives_a_concurrent_status_replacement` | runtime unit / positive interleaving regression | Explicit writer ordering; local files and one joined thread; no network | Required |
| `event_receipt_rejects_failed_required_sink_writes` | runtime unit / negative sink regression | Directory-at-file-path failure; local files; no network | Required |
| `shutdown_barrier_requires_its_own_status_write_acknowledgment` | runtime integration / negative and positive consumer proof | Actual barrier and local Runtime context; no provider calls | Required |
| Existing `csm_runtime_api_serves_status_health_ready_metrics_and_events` | runtime CLI / notice and disposition integration | Owned loopback services, bounded existing timeouts; run with normal and 1 ms heartbeat settings | Required |
| Existing observability tests and governed-shutdown failure test | runtime compatibility / sink, redaction, and failure semantics | Existing focused local fixtures; no paid providers | Required |

Local validation passed:

- 17 observability tests, including both new receipt cases.
- One actual shutdown-barrier consumer test: failed status publication rejects
  the barrier despite successful JSONL append; repairing the sink permits it.
- The existing status/health/ready/metrics/events CLI test passed normally
  (21.64 s) and under `ADL_OBSERVABILITY_HEARTBEAT_MS=1
  ADL_OBSERVABILITY_STDERR=0` (15.89 s), retaining all notice and disposition
  assertions. These commands execute this checkout's Cargo-built CLI binaries.
- The existing continuity/publication-failure shutdown CLI test passed (21.88 s).
- Scoped Clippy for the library, `adl`, `csm`, `csmctl`, and `cli_smoke` passed
  with `-D warnings`. No broad runtime suite or local coverage run is claimed.

The first unit build failed because the new test's explicit import list omitted
`emit_event_with_receipt`. The import was corrected before the passing runs;
production source was unchanged by that correction. Independent source review at
`91cf2c85e3c85051b4ebcdca2eed9d6c8cdf7909` found no actionable findings;
final metadata/import acknowledgement remains a separate publication gate.

Raw logs remain under `.adl/960-*.log`; hashes and bounded result excerpts are
retained in `LOCAL_PROOF.json`. Cache warmup linked 16,503 dependency artifacts
without errors; it is build acceleration only. Required GitHub integration and
coverage are deferred to the standard selected CI lanes after publication.

## Worker #10 handoff verification

The operator assigned issue #960 to Worker #10; the previous writer stopped.
Worker #10 created a separate issue-bound goal before further changes, verified
all seven inherited raw-log digests and result excerpts, and ran `cargo fmt
--manifest-path adl/Cargo.toml -- --check` successfully. No unchanged runtime
proof was rerun. Sanitized retained logs are in `logs/`; `LOCAL_PROOF.json`
binds their digests and the four validated source-file digests. The independent
reviewer `/root/review_960` found no actionable source defects. Final committed
head review is required before native publication.

`REPRODUCTION.json` retains the original sanitized failure observation, source
revision, barrier/heartbeat timestamps and explicit unknown original-CI cause.
The complete original diagnostic harness remains preserved locally under
`.adl/960-diagnosis/`.

### Focused reproduction commands

From the issue worktree, these are the corresponding focused test entrypoints:

```sh
cargo test --locked --manifest-path adl/Cargo.toml --lib observability::tests
cargo test --locked --manifest-path adl/Cargo.toml --lib long_lived_agent::tests::shutdown_barrier_requires_its_own_status_write_acknowledgment -- --exact
cargo test --locked --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_runtime_api_serves_status_health_ready_metrics_and_events -- --exact
ADL_OBSERVABILITY_HEARTBEAT_MS=1 ADL_OBSERVABILITY_STDERR=0 cargo test --locked --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_runtime_api_serves_status_health_ready_metrics_and_events -- --exact
cargo test --locked --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_governed_shutdown_retains_continuity_and_publish_failures_without_false_success -- --exact
```

The local CLI fixture owns loopback services and uses Cargo-built binaries.
Required GitHub Rust integration/coverage remains pending publication.
