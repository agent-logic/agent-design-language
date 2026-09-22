# Issue 1130 bounded repair proof

Scope: Quill plain empty-action greeting compatibility; nested/action validation
preserved; canonical resident/peer names in continuation context; Observatory
login audit and visible failure; displayed-message Copy buttons.

PVF classification: deterministic Runtime and UI contract regression proof,
local CPU/memory and loopback fixtures only, no external resources, no new release
gate. Operational deployment and hosted CI remain distinct proof surfaces.

- Runtime library `response_auth`: 4 passed, 0 failed. Covers empty-action
  compatibility, malformed real actions, nested protocol rejection through
  projection, safe authentication event fields and canonical continuation names.
- Runtime Observatory `observatory_websocket_rejects`: 3 passed, 0 failed.
- Observatory `node --test demos/html-observatory/tests/*.test.mjs`: 24 passed,
  0 failed. Copy success, denial/unavailable clipboard, safe text, login error.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --tests -- -D warnings`: passed.
- Rust formatting and diff hygiene: passed.
- Independent bounded review: initial nested-action finding fixed; subsequent
  review PASS with no actionable findings. Exact commit review recorded natively.

Live evidence before repair: Quill returned READY, but a welcome failed; direct
Bedrock reproduction with civic/action context returned an empty action envelope.
This reproduces formatting failure, not proof of the original provider's raw error.
Correct Observatory token authenticated successfully and operator confirmed login.
No live restart, model substitution or state-file edit during implementation.
