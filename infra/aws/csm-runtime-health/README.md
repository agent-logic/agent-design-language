# Runtime v3 health and bounded recovery

This template owns one Polis health loop in one AWS region:

1. Runtime v3 emits a non-sensitive `runtime_health_heartbeat` every 30 seconds.
2. Runtime Vector sends the redacted event stream to the per-Polis CloudWatch log group.
3. A metric filter counts only heartbeats where `ready=true` and `live=true`; numeric
   `ready_metric=1` and `live_metric=1` mirrors provide CloudWatch Logs-compatible matching.
4. Missing or unhealthy heartbeats put the alarm into `ALARM` after three one-minute periods.
5. EventBridge invokes one SSM document against instances tagged with the exact `AgentLogicPolisId`.
6. The document uses `csm runtime-v3 status` and the idempotent `start` command; it never deletes Runtime state.
7. Alarm transitions and terminal SSM command outcomes publish to the SNS topic.

The Runtime instance must already be an SSM managed node and carry both
`AgentLogicPolisId=<polis_id>` and `Environment=<environment>` tags. The instance role supplied to
this template receives only the CloudWatch Logs permissions needed by Vector. AWS credentials are
obtained from instance metadata; they are never written into Runtime configuration.

For a macOS hybrid managed node, set `runtime_plist_path` to the repo-owned launchd plist. Recovery
then supplies that plist to `csm runtime-v3 start`, allowing the same command to repair both a
loaded-but-unhealthy service and an unloaded service. All command paths are restricted to absolute,
shell-safe paths before Terraform renders the SSM document.
Set `runtime_run_as_user` when the service belongs to a user launchd domain; the SSM document then
executes CSM as that account instead of trying to control the service from SSM Agent's root domain.

Apply this separately from the shared edge and Runtime-instance templates so alarms and recovery
can evolve without replacing nodes or public routing.
