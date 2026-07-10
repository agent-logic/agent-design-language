# ntpd-rs Chronosense Fit Note (#5041)

## Decision

CSM accepts `pendulum-project/ntpd-rs` as the Rust NTP substrate candidate for
Chronosense time-sync evidence, but integrates it as an external status
projection rather than an embedded host clock controller.

The runtime path added by #5041 reads the ntpd-rs management surface with
`ntp-ctl status --format prometheus` only when `ADL_CSM_NTPD_RS_STATUS=1` is
set. Default CSM startup does not require privileged host NTP control, does not
bind UDP port `123`, and does not replace the host's time-sync policy.

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
  `ntpd-rs` from the ntpd-rs status/metrics surface.
- `/chronosense` reports the projected source, confidence, health, drift status,
  monotonic runtime frames, and failure state.
- Daemon events and daemon status retain the same time-sync projection under
  `runtime_capabilities.chronosense.time_sync`.
- `/ready` reports `not_ready` when enabled time-sync evidence is unavailable,
  degraded, unknown, or missing. The explicit default-disabled probe state
  (`failure_state: ntpd_rs_probe_disabled`) remains non-blocking so ordinary CSM
  startup does not depend on host ntpd-rs control.
- `/health` remains a daemon liveness and continuity health surface, allowing
  CSM to stay live while time sync is unavailable.

## Port And Privilege Policy

The integration is status-only. CSM does not listen on UDP port `123`, does not
start or configure the ntpd-rs daemon, and does not expose remote time-sync
reconfiguration. Port ownership and remote access remain with the dedicated
WP-07 port registry and API Gateway issues.

## Negative Case

If `ADL_CSM_NTPD_RS_STATUS` is unset, `ntp-ctl` is unavailable, the command
fails, or output is unrecognized, Chronosense records `health: unavailable`
with a machine-readable `failure_state`. The daemon does not crash and the API
keeps returning structured JSON.
