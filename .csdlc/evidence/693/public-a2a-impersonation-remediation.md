# Issue #693 public A2A impersonation remediation

## Review finding

Exact-head review at `84b58b581bbced3f3b5c169fec52ce3a5f6085bf`
reported that an authenticated public Observatory WebSocket client could submit
`adl.runtime_v3.observatory_agent_initiation_intent.v1` with the configured
resident `sender_id` and rely on the Runtime's process-global Layer8 signer
identity check. The payload did not prove that the resident originated the
action.

## Remediation

Public Observatory WebSocket agent-initiation payloads now fail closed with
`agent_initiation_requires_runtime_authority`, even after ordinary write bearer
authentication. The Runtime-internal/model-selected path remains separate
through `accept_runtime_agent_initiation_intent`, so resident-to-resident A2A
continues to work when Runtime-owned provider execution selects the action.

This intentionally defers external per-agent A2A initiation to a future
verifiable authority envelope instead of treating bearer-token possession plus a
caller-supplied `sender_id` string as resident proof.

## Focused validation

All commands used
`TMPDIR=/Volumes/FastWork/adl-worktrees/adl-issue-693-runtime-a2a-action-selection-reliability/.tmp`.

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_to_agent_ -- --nocapture`
  - result: passed, 7 tests.
  - covers Runtime-internal resident A2A dispatch, sender/signing mismatch
    refusal, and direct public configured-sender impersonation refusal.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test observatory observatory_websocket_rejects_public_agent_initiation_sender_impersonation -- --nocapture`
  - result: passed, 1/1.
  - covers the production Observatory WebSocket path with an authenticated
    bearer-token client attempting to impersonate `beacon`.
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
  - result: passed.
  - covers the full Runtime kernel lane after the public/raw A2A split.
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml`
  - result: passed.
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
  - result: passed.
- `git diff --check`
  - result: passed.
- `csdlc-validate --root . issue --issue 693`
  - result: passed at generation 31.
