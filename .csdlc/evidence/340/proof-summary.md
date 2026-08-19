# Issue #340 proof summary

Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-340-html-observatory-runtime-restart-integration`

Scope: HTML Observatory Runtime v3 local launch/start-stop-restart integration only. Unity, AWS/public hosting, provider credentials, #341, #343, #84, #122, and #251 remain out of scope.

## Current typed proof

The retained evidence logs were refreshed after the second exact-head review finding at the current #340 worktree state. The current focused proof is:

- `340-html-observatory-contract`
  - Command: `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --contract`
  - Result: PASS
  - Log: `.csdlc/evidence/340/340-html-observatory-contract.log`
- `340-runtime-api-wss-contract`
  - Command: `cargo test --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
  - Result: PASS, `3 passed; 0 failed`
  - Log: `.csdlc/evidence/340/340-runtime-api-wss-contract.log`
- `340-rust-format`
  - Command: `cargo fmt --manifest-path adl-runtime/Cargo.toml --check`
  - Result: PASS
  - Log: `.csdlc/evidence/340/340-rust-format.log`
- `340-diff-hygiene`
  - Command: `git diff --check -- CSMctl adl/tools/test_html_observatory.sh demos/html-observatory/app.js demos/html-observatory/index.html adl/tools/validate_v092_observatory_restart_reconnect.sh adl-runtime/tests/runtime_api_wss.rs .csdlc/issues/340 .csdlc/prepared/issues/340`
  - Result: PASS
  - Log: `.csdlc/evidence/340/340-diff-hygiene.log`
- `340-live-start-stop-restart`
  - Command: `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --live`
  - Result: PASS
  - Log: `.csdlc/evidence/340/340-live-start-stop-restart.log`
- `340-typed-validate`
  - Command: `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root /Volumes/FastWork/adl-worktrees/adl-issue-340-html-observatory-runtime-restart-integration issue --issue 340`
  - Result: PASS
  - Log: `.csdlc/evidence/340/340-typed-validate.log`

## Post-review repair proof

Fresh reviewer `fresh-session:07a4715e-f5e9-496c-a48e-ef3d04488ac8` returned two actionable findings against commit `593507f943111b6bacfba4edfb4dc5ee7aeb2f1b`:

- P2: HTML live mode accepted a non-200 `/v1/observatory` response because it checked `Response.ok` instead of exact `status === 200`.
- P3: `CSMctl urls/start` could print a stale default Observatory URL after the Observatory server fell back from port 8765 to 8766.

Repairs:

- `demos/html-observatory/app.js` now requires `/v1/observatory` to return exact HTTP 200 before live snapshot creation.
- `adl/tools/test_html_observatory.sh` now rejects a 204 `/v1/observatory` response even when `/v1/ready` and `/v1/health` return 200.
- `CSMctl urls_cmd` reloads persisted Observatory state before printing URLs.
- `adl/tools/validate_v092_observatory_restart_reconnect.sh` asserts `CSMctl urls` reports the persisted fallback Observatory URL and Runtime API base.
- `adl-runtime/tests/runtime_api_wss.rs` pins the CSMctl persisted-state reload contract.

Fresh reviewer `fresh-session:e7044db6-30ab-465c-b65b-5e31f0e2e8b5` then returned two evidence/proof findings against commit `cbb4762f50f1e68100b551cc82a30c2a772f539f`:

- P2: retained live evidence was stale and contradicted the proof summary.
- P3: the validator did not assert the second post-restart `CSMctl start` output.

Repairs:

- `adl/tools/validate_v092_observatory_restart_reconnect.sh` now captures the post-restart `CSMctl start` output and asserts it reports the current persisted Observatory URL plus Runtime API base.
- `CSMctl` now separates the static Observatory document root from the launchd working directory through `ADL_CSM_OBSERVATORY_LAUNCH_WORKING_DIR`, keeping the Apache-style document-root configuration simple while avoiding cwd access assumptions.
- `demos/html-observatory/index.html` and `demos/html-observatory/app.js` now distinguish live Runtime data from retained/published mirror evidence in visible labels. Live mode renders `Live Runtime API`; retained/offline paths remain explicitly retained evidence.
- `adl/tools/test_html_observatory.sh` and `adl-runtime/tests/runtime_api_wss.rs` pin the live-label and post-restart validator contracts.
- Retained logs in `.csdlc/evidence/340/` were refreshed after the repair instead of reusing the stale pre-fix live log.

Current validation:

- `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --contract`
  - Result: PASS
- `cargo test --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
  - Result: PASS, `3 passed; 0 failed`
- `cargo fmt --manifest-path adl-runtime/Cargo.toml --check`
  - Result: PASS
- `git diff --check -- CSMctl adl/tools/test_html_observatory.sh demos/html-observatory/app.js demos/html-observatory/index.html adl/tools/validate_v092_observatory_restart_reconnect.sh adl-runtime/tests/runtime_api_wss.rs .csdlc/issues/340 .csdlc/prepared/issues/340`
  - Result: PASS
- `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --live`
  - Result: PASS
  - Evidence: the refreshed live log proves Runtime read probes reach HTTP 200, Observatory root/index serve HTTP 200, graceful stop leaves no Runtime PID/lease files, and restart reaches PASS. The validator also captures `$STATE_DIR/CSMctl.restart.out` during the live run and fails closed if the post-restart `CSMctl start` output reports a stale Observatory URL or stale Runtime API base.

## Exposed-route coverage

The issue-owned live validator probes the documented non-mutating Runtime v3/OpenAPI surface used by the HTML Observatory:

- GET/static/read routes: `/v1/health`, `/v1/ready`, `/v1/metrics`, `/v1/openapi.json`, `/v1/docs/`, `/v1/observatory`, `/v1/observatory/openapi.json`, `/v1/observatory/docs/`, `/v1/agents`, and `/v1/agents/{agent_id}` when the live feed exposes an agent id.
- Preflight routes: `OPTIONS /v1/ready`, `OPTIONS /v1/control`, and `OPTIONS /v1/layer8/recipient-acknowledgement`.
- Fail-closed invalid-body routes: `POST /v1/control` and `POST /v1/layer8/recipient-acknowledgement`.
- WebSocket handshakes: `/v1/acip/ws` and `/v1/observatory/ws`.
