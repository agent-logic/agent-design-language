# ntpd-rs Chronosense Fit Note (#5041)

## Decision

CSM accepts `pendulum-project/ntpd-rs` as the Rust NTP substrate for
Chronosense time-sync evidence and integrates it as a CSM-owned in-process
runtime component.

The runtime path added by #5041 reads `ntp_daemon::ObservableState` through the
ntpd-rs Rust crate observation-socket contract. That keeps the implementation
inside the CSM runtime binary instead of requiring a separate `ntp-ctl` command
or a sidecar control-plane binary. The old `ntp-ctl status --format prometheus`
path remains an explicit compatibility fallback only when
`ADL_CSM_NTPD_RS_CTL_COMPAT=1` is set.

Default CSM startup does not bind UDP port `123` and does not replace the
host's time-sync policy. It does, however, always report the in-process
Chronosense NTP component state in CSM daemon status, daemon events, and the
runtime API. Missing observation-socket evidence is a degraded/not-ready time
sync state, not a CSM process crash.

## Source Evidence

- `https://github.com/pendulum-project/ntpd-rs` describes ntpd-rs as a Rust NTP
  and NTS implementation with client and server support.
- The project README documents daemon installation with `systemctl` and status
  inspection through `ntp-ctl status`.
- The ntpd-rs `ntp-ctl(8)` man page documents `ntp-ctl status` and its
  `--format` option, including the Prometheus/OpenMetrics output mode used by
  the integration.
- The ntpd-rs metrics docs define source offset, source uncertainty, and
  unanswered-poll metrics, which CSM maps into Chronosense health/confidence
  instead of controlling the host clock.
- The repository shows active maintenance and a latest GitHub release of
  `1.9.0` on 2026-06-12 during #5041 evaluation.

## CSM Integration Boundary

- CSM records a `chronosense_time_sync_status.v1` projection with substrate
  `ntpd-rs` from `ntp_daemon::ObservableState`.
- `/chronosense` reports the projected source, confidence, health, drift status,
  monotonic runtime frames, and failure state.
- Daemon events and daemon status retain the same time-sync projection under
  `runtime_capabilities.chronosense.time_sync`.
- `/ready` reports `not_ready` when time-sync evidence is unavailable,
  degraded, unknown, or missing. An explicit operator-disabled state
  (`ADL_CSM_NTPD_RS_STATUS=0`) remains non-blocking so an operator can recover
  the runtime while repairing host time infrastructure.
- `/health` remains a daemon liveness and continuity health surface, allowing
  CSM to stay live while time sync is unavailable.

## Port And Privilege Policy

The integration is CSM-owned and in-process, but CSM does not listen on UDP port
`123` and does not expose remote time-sync reconfiguration. Port ownership and
remote access remain with the dedicated WP-07 port registry and API Gateway
issues.

## Negative Case

If the ntpd-rs observation socket is missing, refused, times out, or returns
unreadable state, Chronosense records `health: unavailable` with a
machine-readable `failure_state`. The daemon does not crash and the API keeps
returning structured JSON. If `ADL_CSM_NTPD_RS_CTL_COMPAT=1` is set, CSM may
fall back to the older `ntp-ctl` status projection for compatibility evidence,
but that path is not the primary implementation.
