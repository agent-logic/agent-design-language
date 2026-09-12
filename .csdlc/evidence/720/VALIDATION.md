# #720 Live-only Observatory proof

Scope: remove historical telemetry fallback from startup, mode navigation and connection failure. Historical documents are unchanged; explicitly labelled integration/report evidence remains separate from live telemetry. No cloud deployment or runtime API changes.

PVF: tooling lane; deterministic UI and contract regressions; local CPU and browser resource profile; required #720 acceptance proof. Browser test intercepts all network requests and simulates API/WebSocket failures, so it does not prove deployed connectivity or TLS.

- `node --test demos/html-observatory/tests/*.test.mjs`: 24 tests pass.
- `bash adl/tools/test_html_observatory.sh`: pass; exercises Runtime v3 read, signed-command and roster projection and live-only route regressions.
- `node demos/html-observatory/tests/live_only.browser.mjs`: pass in local headless Chrome with installed Playwright. Proves successful live startup, initial outage, disconnect preserving last live count with stale notice, navigation, absent retained controls, zero historical telemetry fetches and zero retained timers.
- Integrated Observatory shell proof is being completed before publication. Hosted CI is pending publication, not claimed here.

The source baseline already had no `setRuntimeTestStatus` function or first orphan assignment. The remaining published-status and live-status orphan constants are removed together with unused runtime-kind/config locals. This is the current-source reconciliation of the issue's older line references.

Native cards were semantically normalized through edit and validated before implementation; generation 5, bound branch codex/720-observatory-live. An issue goal was created before source edits. #934 is the umbrella. No cloud deployment or merge authorized.
