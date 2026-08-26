# Runtime Configuration Hot Reload

Issue #510 adds a runtime-owned hot-reload helper for Axum state. Callers store
`HotReloadHandle<T>` in application state and read complete `Arc` snapshots with
`current()`.

The watcher debounces observed file changes before parsing. Change detection
hashes file contents so same-length rewrites and coarse timestamp resolution do
not hide a valid update. A valid update replaces the active snapshot as one
whole value; parse or validation failures are rejected and the last-known-good
snapshot remains visible. Shutdown is explicit through a cancellation token or
`ConfigReloadController::shutdown()`.

DEC-01 issue #513 depends on this HOT-01 surface and must not concurrently edit
`adl-runtime/src/config_reload.rs`, `adl-runtime/tests/config_reload.rs`,
`adl-runtime/src/lib.rs`, or this document.
