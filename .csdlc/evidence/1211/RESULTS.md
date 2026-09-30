# Issue 1211 validation and review

Implementation: shared evidence classification for health and roster, metadata-only readiness no longer treated as generated inference, retained/recovered incident wording, and raw orientation digest formatting.

## Local proof

- Kernel library: 279 passed, zero failed (`cargo test --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib`).
- Observatory JavaScript: 25 passed, zero failed (`node --test demos/html-observatory/tests/*.test.mjs`).
- Strict kernel clippy including tests: passed (`cargo clippy --offline --manifest-path adl-runtime-kernel/Cargo.toml --tests -- -D warnings`).
- Formatting and diff whitespace: passed.
- PVF runtime/local deterministic required regression: small CPU/files and existing loopback fixtures; no provider calls within automated tests.

## Independent review

Reviewer `/root/review1211` found two P2 issues: failed projection retained communication eligibility, and recovered incidents retained unverified wording. Both fixed with regressions. Final review found no actionable findings; reviewer independently ran three orientation/UI tests. Reviewed implementation diff SHA256: f5b30c02e78682c4a70b0dcc53b6359d433f48c5d5d792ce0932e7f2060f3f94.

## Separate installed Runtime qualification

At 2026-09-30 20:53–20:54 UTC, each of Beacon, Ember, Delta, Quill, Harbor and Nova received one explicit operator conversation and returned a delivered, nonempty reply. Read-only health checks then reported all six healthy/ready with a recorded successful response. This exercised deployed generation `issue-1209-789f30f370-approved`, not this candidate. The client received a transport error while closing after all six terminal successes; no request remained pending and no requests were replayed. An earlier authentication attempt used the wrong token type and dispatched no conversation; corrected to the configured Observatory token. No credentials or reply bodies are retained in this packet.

This proves bounded inference through the existing Runtime, not perpetual health. Evidence naturally ages after five minutes; this change makes that condition explicit rather than treating it as provider failure. No models, credentials, incident budgets or service generation changed.

## Delivery boundary

Candidate is not deployed or merged. Hosted CI and PR publication are recorded by native lifecycle receipts and GitHub; local pass does not imply deployment.
