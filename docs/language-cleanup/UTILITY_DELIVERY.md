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
