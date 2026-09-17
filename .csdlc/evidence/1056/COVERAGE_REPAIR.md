# PR 1063 executable coverage repair

At e842618fe0, all test lanes passed but aggregate coverage rejected the new server binary at 0/36 lines. The subprocess integration test killed it with SIGKILL, so its instrumented process could not flush its profile.

The server now supports graceful Ctrl-C shutdown, waits for Axum to stop, and aborts/awaits its periodic retention task. The Unix subprocess test sends SIGINT and requires successful process exit within five seconds; forced kill remains failure cleanup. This does not undo provider effects or authorize replay. Explicit focused coverage mappings select the existing server and review integration tests for the touched sources; coverage thresholds are unchanged.

Validation: seven server tests passed normally and with LLVM instrumentation; fourteen review tests passed under LLVM instrumentation. Targeted clippy and coverage-mapping regression suite passed. Combined LLVM line coverage: server executable 42/46 (91.30%), server module 422/465 (90.75%), review runner 385/412 (93.45%). Full changed-source coverage-impact preflight passed against the current PR base e9d2429bd20eedf33b5efacf1d0c0c5128993e3e with the unchanged 80% threshold.

Independent source review by review_1056_server found no actionable source findings. The review's PASS-marker placement request was applied. Current native proof/review and remote CI are separate records. This component repair does not establish hosted deployment, real-provider acceptance, merge readiness or Sprint 10 completion.
