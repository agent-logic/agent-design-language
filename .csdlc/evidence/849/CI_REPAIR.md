# PR #952 CI repair

Source candidate: `94a39db15fe6e4555c6cc3c201da5409665a3b86`. Accepted-main integration: `900f1bf10717842b20eeceee910e2d35943c0a82`.

Hosted run 34674259527, job 103501184932 at published 62434aac8f640617944a17d18946a03deba40f6b failed with E0063: two `TypedReviewReceipt` constructors lacked `publication_linkage`. The publication/readback-only fixtures in `tests/support/baseline_journeys.rs` and `tests/operational_cli_commands.rs` now explicitly use `None`, preserving the legacy publication-only digest. Merge admission still requires qualified linkage.

Accepted #948 changes are preserved, including pending mutation receipt checks and curl configuration isolation. Its release-preflight fixture normalization already resolved the prior historical-inventory drift; this repair changes no production inventory, release guard, independent adl-uts version policy, or historical receipt hash. Earlier failure packets remain historical evidence.

Local proof at source candidate:

- `cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1`: 262 passed; zero failed, ignored, measured or filtered, across 15 result groups.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`: passed (2.58 seconds).
- Separate release-preflight test: 1 passed (67.71 seconds); its log named `inventory-before.log` records a pass.

Private local logs are under `.adl/runs/849/ci-repair-all-targets.log`, `ci-repair-clippy.log`, and `inventory-before.log`. Existing deterministic tooling PVF classification applies: required native owner contract, local Git/filesystem and fake authenticated transport. No live merge or cloud proof is claimed. Hosted CI is a separate gate observed against the final pushed revision; this packet does not claim merge or terminal closeout.

## Linux parallel fixture follow-up

Hosted run 34675094214 at `aa7f806558a51b23ec89ad342abd56373c68ec71` compiled successfully but failed job 103503466106: `observational_curl_uses_stdin_and_minimal_environment_without_secret_arguments` could not execute its newly written fixture (`Text file busy`, Linux ETXTBSY); 91 other library tests passed. This is distinct from the repaired E0063 constructors.

Successor `a2a05c7c7` moves the two unchanged shell behaviors into tracked mode-100755 fixtures. The test no longer writes or replaces executable files while parallel tests launch child processes. The stdin, minimal environment, first-argument `-q`, secret-free argv, redaction and truncation assertions are retained. No production retry, test serialization, skipped checks or release guard change is introduced. Existing deterministic local subprocess PVF classification applies to both fixtures.

Successor validation: default-parallel `cargo test --manifest-path csdlc-v3/Cargo.toml --lib` passed all 92 tests, zero ignored/filtered (13.49 seconds); strict all-target Clippy passed (1.74 seconds); diff hygiene passed. The earlier 262-test full result remains attributed to its source candidate; unchanged integration tests were not rerun for this fixture-only delta. Final hosted successor CI is still required.
