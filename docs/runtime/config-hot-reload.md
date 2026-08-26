# Runtime Configuration Hot Reload

Issue #510 adds a runtime-owned hot-reload helper for Axum state. Callers store
`HotReloadHandle<T>` in application state and read complete `Arc` snapshots with
`current()`.

Runtime Observatory origin policy is the public operator-facing consumer for
this helper. A reload owner applies a validated config snapshot through
`ControlService::replace_observatory_allowed_origins` or
`ControlService::replace_observatory_allowed_origins_from_runtime_init`; the
next HTTP CORS or Observatory WSS request reads the atomically replaced policy
without rebuild or process restart. Invalid origin policy input is rejected and
the previous valid allowlist remains active.

The watcher debounces observed file changes before parsing. Change detection
hashes file contents so same-length rewrites and coarse timestamp resolution do
not hide a valid update. A valid update replaces the active snapshot as one
whole value; parse or validation failures are rejected and the last-known-good
snapshot remains visible. Shutdown is explicit through a cancellation token or
`ConfigReloadController::shutdown()`.

DEC-01 issue #513 depends on this HOT-01 surface and must not concurrently edit
`adl-runtime/src/config_reload.rs`, `adl-runtime/tests/config_reload.rs`,
`adl-runtime/src/lib.rs`, or this document.
