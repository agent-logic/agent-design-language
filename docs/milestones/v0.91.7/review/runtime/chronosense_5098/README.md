# Issue 5098 Chronosense Runtime Proof

This packet retains the issue-local CSM runtime proof for the in-process
Chronosense time observation path.

- `service_status.json`: repo-native `csm service status` showing the CSM-owned
  local supervisor and canonical runtime API listener on `127.0.0.1:19997`.
- `process_pid.json`: permission-safe pid-file liveness probe for the CSM
  service.
- `process_port_19997.json`: permission-safe exact loopback port probe for the
  embedded runtime API.
- `status.json`: `/status` response from the running CSM API.
- `ready.json`: `/ready` response from the running CSM API.
- `chronosense.json`: `/chronosense` response showing SNTP health from
  `rsntp::AsyncSntpClient` without a missing observation socket or shellout.
- `disk.txt`: local disk snapshot retained because prior WP-07 runtime runs hit
  ENOSPC.

Observed proof highlights:

- CSM runtime API is bound on the canonical local port `127.0.0.1:19997`.
- Chronosense reports `substrate=SNTP`, `mode=csm_in_process_async_sntp_client`,
  and `health=synced`.
- CSM does not advertise a Chronosense UDP/123 listener; UDP/123 is recorded
  only as an external NTP server boundary used by outbound SNTP sampling.
- The sampled peer was `time.cloudflare.com`.
- `/ready` reports `ready` with no blocking reasons.
