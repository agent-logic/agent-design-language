# Remaining utility migration (#1351)

Status: **in progress; not accepted or ready for publication**. Native issue
preparation and FastWork binding succeeded. The issue remains responsible for all
60 baseline Python paths, the later #1275 adapter checker, and executable inline
payloads. No exception or completion-by-omission is accepted.

## Completed source slice: adapter report consistency

`tools/public_contracts/src/adapter_evidence.rs` replaces the #1275 Python report
checker. Run:

```sh
cargo run --locked --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts -- adapter-evidence
cargo test --locked --manifest-path tools/public_contracts/Cargo.toml --test adapter_evidence
```

This is offline report consistency, not authenticated lifecycle or product
acceptance. The original checker passed before removal. Native tests cover the
current report, truthful incomplete reporting, mismatched identity/package/stage
claims, false aggregate success, missing evidence and duplicate repositories.
Missing identity fields cannot accidentally compare equal. Package-binding
ambiguity is rejected rather than silently overwritten.

`utility-dispositions.json` retains exact Git mode/blob/SHA256 custody for each
completed removal. No source payload is archived or embedded. Original #1275
reports remain unchanged. Remaining utilities, callers, final guard overlays,
independent review and CI are still required before this issue is delivered.

## Completed source slice: dependency cache warming

The Rust cache helper replaces the implementation and five Python regression
cases. The original tests passed before removal. Ten native tests preserve
artifact selection, multiline workspace manifests, dep-info/build exclusions,
explicit replacement, rollback/restoration, wrapper defaults, JSON/output-file
behavior and the optional cross-device skip. Direct CLI tests run with empty
`PATH` and verify successful JSON, runtime failure exit 2 and setup failure exit 1.
The cross-device case ran on this host using the available second filesystem.

The same-filesystem wrapper and direct dry-run interface remain available at
`adl/tools/rust_validation_warm_cache.sh` and
`adl/tools/warm_rust_dependency_cache.sh`. They invoke Rust through Cargo; the
first use may compile the helper. Warmup remains acceleration, never proof.
Current lane selectors and documentation point to the native implementation.

Safety corrections are explicit: source artifact/profile symlinks and destination
parent escapes are refused, wildcard workspace membership requires explicit
classification, and a failed rollback retains its original backup instead of
silently deleting it. Full TOML parsing replaces the old Python-version fallback.
