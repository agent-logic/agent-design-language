# ADL CI performance: issue 1270

Part of agent-logic/cognitive-sdlc#205.

Successful baseline run 38499336594 at b9099ab2f640f57f735cb3bf66a02f632d7e45c2 took 1,175 seconds wall time and 1523 summed runner seconds. The C-SDLC v3 job took 1,128 seconds; its test step took 1,079 seconds. Compilation took 48.83 seconds, while installed_intent_commands took 750.44 seconds.

The CI-only test profile now uses optimization level 1, with debug assertions and overflow checks explicitly enabled. Both dev and test debug symbols are omitted and incremental compilation is disabled in this ephemeral job. Installed fixtures copy the exact Cargo-built executable; reducing its size and optimizing repeated hashing and command execution targets the measured cost. This may increase initial compilation time; compare total job and runner time, not just a test duration. Debug-symbol diagnostics are reduced. Production manifests and shared installed binaries are unchanged.

All existing test, format, Clippy, selection and required aggregation commands remain unchanged. No test filtering, ignored-test changes, cross-repository builds, stale executable reuse or paid inference is introduced.

Local proof: eight deterministic workflow contract cases passed, including refusal of disabled assertions/overflow checks, skipped/filtered tests, failure suppression and step profile overrides. Existing Ruby workflow-policy validation passed. PVF: required tooling contract, deterministic local files and processes, low CPU/memory, no network or provider use. Full Rust integration is deferred to the unchanged hosted lane; no measured speedup or release claim until that run succeeds.
