# Build Action Logs

ADL build and validation commands that participate in workflow truth can emit
durable build-action packets. The initial integrated producer is
`adl/tools/validation_manager.py --run`.

## Schema

Each packet uses `schema_version: adl.build_action_log.v1` and records:

- `runner`: command surface that created the packet, such as
  `validation_manager`
- `lane_id` and `reason`: validation-lane identity and selection reason
- `command` and `command_sha256`: command text plus a stable digest
- `cwd`, `binary_path`, and `cache_posture`: execution context
- `started_at`, `ended_at`, `elapsed_ms`, `exit_code`, and `status`
- `stdout_ref`, `stderr_ref`, and `packet_ref`: durable evidence refs
- `redaction_status` and `retention`: truth about publication posture

The manifest uses `schema_version: adl.build_action_log_manifest.v1` and lists
the packet refs emitted by one validation-manager run.

## Validation Manager

`validation_manager.py --run` writes build-action logs by default under:

```text
.adl/logs/build-actions/validation-manager/<timestamp>/
```

The default run directory includes a UTC timestamp and process id to avoid
overwriting packets from another validation-manager run started in the same
second. Use `--build-action-log-dir <path>` or
`ADL_BUILD_ACTION_LOG_DIR=<path>` for a bounded proof directory. When the
directory is inside the repository, refs are repo-relative. Explicit external
directories may produce absolute local refs and should stay out of tracked
artifacts.

The command replays captured stdout/stderr after each lane exits so existing
human-facing behavior remains available while durable logs are retained.

## Boundaries

This surface records private workflow evidence. It does not redact raw command
logs, upload logs, or claim hosted observability. CI log archive manifests remain
the CI raw-log evidence surface; build-action packets may cite or complement
those manifests as workflow consumers are wired in.
