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

Validation is pending until recorded results replace this statement. Cache warmup
linked 16,503 dependency artifacts without errors; it is build acceleration only.
Local proof does not substitute for required GitHub integration checks.
