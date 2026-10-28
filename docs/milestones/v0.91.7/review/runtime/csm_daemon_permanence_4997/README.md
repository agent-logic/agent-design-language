# CSM Daemon Permanence Proof - Issue #4997

## Scope

Issue #4997 fixes the CSM daemon/service contract after the #4976 direct liveness rerun stopped with recoverable state but without a durable service-permanence classification.

This packet records the implementation proof for the local CSM contract only. It does not close #4976 and does not claim a 24-hour soak.

## Failure Input

Observed retained #4976 rerun evidence:

- Direct daemon PID became stale.
- Last successful local cycle was `cycle-002500`.
- Agent status was recoverable `idle`, with `last_error: null` and `stop_requested: false`.
- Safe-fail state classified the agent as recoverable sleeping.
- OTel retained event count was present.
- AWS heartbeat cursor reached `next_heartbeat_seq: 10000`.

The old evidence left the service lifetime boundary ambiguous enough that a finite heartbeat/cycle envelope could look like a daemon lifetime policy.

## Implemented Contract

- CSM daemon status now records `restart_policy: "always"`.
- CSM daemon status now records `service_mode: "permanent"` for service operation.
- Bounded test harness returns are explicitly marked with `service_mode: "bounded_test_only"` and `bounded_test_mode: true`.
- Daemon startup events record `agent_max_cycles_lifetime_boundary: "ignored_in_daemon_service_mode"`.
- Runtime capabilities expose supervisor metadata for launchd KeepAlive, systemd Restart=always-compatible service metadata, Rustysd service-manager compatibility, and rinit service-manager compatibility.
- CSM launchd service manifests and service status records retain `restart_policy: "always"` and `service_mode: "permanent"`.
- CSM local service manifests are classified as `restart_policy: "external_supervisor_required"` and `service_mode: "local_proof_only"`.
- CSM `--no-sleep` service manifests are classified as `restart_policy: "bounded_test_only"` and `service_mode: "bounded_test_only"`.
- CSM service status normalizes legacy manifests that are missing these fields instead of echoing stale permanence defaults.

## Non-Claims

- No 24-hour soak completion is claimed here.
- No OS reboot survival is claimed here.
- No kill -9 recovery is claimed here.
- No disk-full or host resource-exhaustion recovery is claimed here.
- No CloudFront, ACIP/SNS, or outbound shutdown notice delivery is claimed here.

Outbound shutdown/degradation notices are scheduled separately in #4998.

## Local Validation

Commands run from the #4997 worktree:

```sh
cargo fmt --manifest-path adl/Cargo.toml
cargo check --manifest-path adl/Cargo.toml
cargo test --manifest-path adl/Cargo.toml daemon_status_records_restart_always_permanent_service_contract --lib
cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_daemon_writes_status_checkpoints_and_otel_observability -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_service_install_writes_launchd_envelope_without_adl_runtime_owner -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_service_install_classifies_local_and_no_sleep_modes_truthfully -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent::csm_daemon_restart_budget_failure_leaves_recoverable_checkpoint -- --nocapture
cargo test --manifest-path adl/Cargo.toml --test cli_smoke agent::csm -- --nocapture
git diff --check
```

Observed result:

- `cargo check`: passed.
- Unit permanence contract test: passed.
- Focused daemon smoke: passed.
- Focused service install smoke: passed.
- Local/no-sleep service metadata regression: passed.
- Restart-budget bounded-test metadata regression: passed.
- Broader CSM smoke slice: 15 passed, 0 failed.
- `git diff --check`: passed.

## Follow-On

#4976 must remain open until a post-fix liveness run survives at least 24 hours.

#4998 owns governed shutdown/degradation outbound notices through CloudFront/control-plane hooks, ACIP/SNS, and other configured channels.
