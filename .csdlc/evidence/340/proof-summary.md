# Issue #340 proof summary

Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-340-html-observatory-runtime-restart-integration`

Scope: HTML Observatory Runtime v3 local launch/start-stop-restart integration only. Unity, AWS/public hosting, provider credentials, #341, #343, #84, #122, and #251 remain out of scope.

## Typed finalize proof

The typed `csdlc-validate finalize` request at generation 11 re-ran and recorded these required lanes:

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
  - Command: `git diff --check -- CSMctl adl/tools/test_html_observatory.sh demos/html-observatory/app.js adl/tools/validate_v092_observatory_restart_reconnect.sh adl-runtime/tests/runtime_api_wss.rs .csdlc/issues/340 .csdlc/prepared/issues/340`
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

## Exposed-route coverage

The issue-owned live validator probes the documented non-mutating Runtime v3/OpenAPI surface used by the HTML Observatory:

- GET/static/read routes: `/v1/health`, `/v1/ready`, `/v1/metrics`, `/v1/openapi.json`, `/v1/docs/`, `/v1/observatory`, `/v1/observatory/openapi.json`, `/v1/observatory/docs/`, `/v1/agents`, and `/v1/agents/{agent_id}` when the live feed exposes an agent id.
- Preflight routes: `OPTIONS /v1/ready`, `OPTIONS /v1/control`, and `OPTIONS /v1/layer8/recipient-acknowledgement`.
- Fail-closed invalid-body routes: `POST /v1/control` and `POST /v1/layer8/recipient-acknowledgement`.
- WebSocket handshakes: `/v1/acip/ws` and `/v1/observatory/ws`.
