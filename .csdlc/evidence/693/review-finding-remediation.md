# Issue #693 review finding remediation

Exact-head review at `e0344304eb029142f4ae264844881d6c2f72e977`
reported that runtime-internal A2A accepted resident sender identifiers without
requiring the active Layer8 signing identity to match the selected sender.

The remediation makes runtime-internal A2A initiation use the same
sender-identity binding as externally submitted initiation intents. A resident
agent can initiate an internal A2A message only when the Runtime has an active
Layer8 signed exchange for that resident sender.

Focused regression coverage:

- `agent_to_agent_runtime_internal_initiation_rejects_sender_identity_mismatch`
  proves a `scribe -> ember` runtime-internal intent is refused when the loaded
  signed exchange belongs to `beacon`.
- `agent_to_agent_runtime_internal_initiation_allows_resident_agent_pairs` now
  provisions a matching Layer8 sender exchange for each ordered resident pair,
  so all resident-to-resident internal A2A paths remain supported without
  sharing one resident's signing identity across the roster.

Local validation after remediation, with `TMPDIR` inside the #693 worktree:

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_to_agent_ -- --nocapture`
  - result: passed, 6 tests.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
  - result: passed.
- `git diff --check`
  - result: passed.
