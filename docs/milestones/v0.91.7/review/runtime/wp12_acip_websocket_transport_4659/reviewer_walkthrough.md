# Reviewer Walkthrough

1. Inspect `acip_websocket_transport_proof.json`.
2. Confirm `positive_case.status` is `delivered`.
3. Confirm failure cases include `malformed_message`, `auth_policy_denial`, `peer_close_before_response`, and `response_timeout` with `failed_closed` status.
4. Inspect `audit/artifact_safety_scan.json` for retained-artifact hygiene.
5. Re-run with `cargo run --manifest-path adl/Cargo.toml --bin run_wp12_acip_websocket_transport_proof -- --out docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659`.
