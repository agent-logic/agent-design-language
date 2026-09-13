# QUAL-PROVIDER #901 evidence

This packet qualifies provider loss, timeout, interruption, and recovery through
the production `adl-provider-adapter` command. The provider is a task-owned,
CPU-only `llama-server` process using an already present local model artifact.
The runner uses a transparent loopback proxy to record when each inference
request is forwarded; the proxy does not generate responses or replace the
provider.

The four live scenarios are serialized. Loss kills the owned provider process
after forwarding an inference request. Timeout uses the adapter's real 150 ms
deadline. Interruption sends `SIGTERM` to the owned adapter only after the
provider request is forwarded. Recovery starts a fresh provider incarnation and
requires a successful, distinct request. Every process is started in a new
process group and reaped by the runner.

`qualification-report.json` is the portable receipt. It retains source, binary,
model, request, process, timestamp, timeout, failure, and recovery identity while
omitting prompts and generated output. The raw run remains private in the issue
worktree. Earlier failed runs are retained there and do not support acceptance.

PVF classification: required provider integration qualification; bounded local
CPU, loopback, process, and disk resources; live timing and PIDs vary across
runs. Deterministic validator tests reject caller-supplied failure flags, static
reference traces, missing execution logs, wrong request or process identity,
timeouts without elapsed/deadline evidence, duplicate work, and stale provider
identity.
