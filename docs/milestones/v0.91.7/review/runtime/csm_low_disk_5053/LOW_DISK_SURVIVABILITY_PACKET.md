# CSM Low-Disk Survivability Packet (#5053)

## Scope

This packet records the bounded #5053 runtime change for CSM low-disk
backpressure and checkpoint survivability.

## Incident Reference

The source issue records an observed 2026-07-08 UTC main CSM failure where the
host returned ENOSPC while replacing `continuity_checkpoint.json` and writing
`rust_supervisor_status.json`. The runtime retained some cycle and safe-fail
evidence, but it discovered storage pressure too late to enter a managed
degraded state first.

## Implemented Contract

- Required runtime state writes call a disk preflight before JSON and JSONL
  persistence.
- Low disk records `csm_backpressure_state.json` and
  `csm_low_disk_recovery_manifest.json` in the state root.
- The recovery manifest preserves a minimal checkpoint pointer set:
  `status.json`, `daemon_status.json`, `continuity_checkpoint.json`,
  `continuity_replay_manifest.json`, `safe_fail_bundle.json`, and
  `operator_events.jsonl`.
- Safe-fail serialization suppresses new per-sequence artifacts under low disk
  and updates the latest safe-fail pointer instead of deleting retained
  evidence.
- Runtime API responses project low-disk pressure through `/status`, `/health`,
  `/ready`, and `/metrics`.

## Proof Boundary

The local proof uses injected disk-pressure environment variables
`ADL_CSM_DISK_FLOOR_BYTES` and `ADL_CSM_TEST_AVAILABLE_BYTES` to force the
preflight branch without filling the host disk. This is a negative proof for
the runtime decision boundary, not a claim that a real ENOSPC filesystem event
was reproduced in this worktree.

## Validation Status

- `cargo fmt --manifest-path adl/Cargo.toml --check` passed after formatting.
- `git diff --check` passed.
- `cargo test --manifest-path adl/Cargo.toml runtime_api_projects_low_disk_degraded_state --lib`
  passed.
- `cargo test --manifest-path adl/Cargo.toml low_disk --lib` passed and covered
  storage preflight, safe-fail low-disk suppression, and runtime API degraded
  projection.

## Non-Claims

- This packet does not claim host reboot recovery.
- This packet does not claim kill -9 recovery.
- This packet does not claim successful full artifact writes after a true
  ENOSPC condition.
- This packet does not claim retained evidence deletion or cleanup.
