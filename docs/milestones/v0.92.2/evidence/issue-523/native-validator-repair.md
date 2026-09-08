# Native validator repair authorized under #523

The operator explicitly authorized fixing the publication blocker on 2026-09-08. This extends the original documentation scope only to the native card validator and its focused regression proof.

## Reproduced baseline defect

At baseline `bf159eb416950dfa3399933829726a7b7e71f897`, native validation rejects all six registry 1.0.3 cards with `card_structure_invalid`. The validator climbs two parents from a versioned template path and looks for schemas outside the version directory. It ignores each registry entry's `structure_schema_path`. It also requires all global scaffold vocabulary lines to appear in each card, although those lines are not mandatory per-card content.

Issue-local failures and typed edit/bind history remain in the primary checkout's native metadata and ignored review packet. The earlier bind succeeded despite failed validation; it never constituted proof of card validity.

## Repair contract

Use each active registry entry's declared schema. Validate schema identity and actual Markdown structure: ordered headings and fences, required locked lines in their declared sections, and frontmatter key paths. Missing or malformed schemas and structural damage must fail. Preserve rendered-value and lifecycle digest checks. Do not copy alias schemas or change the active schemas to accommodate the defective validator.

## Proof classification

The new regression surface is a deterministic local CPU contract test in `csdlc-v3`, with small repository-file reads and no network or paid resources. It gates this validator change. It proves active-template acceptance and rejection of structural damage; it does not prove Runtime behavior or release readiness. Native validation of the real #523 six-card bundle is required in addition to the template tests.

Validation and independent-review results are recorded in the accompanying handoff and native lifecycle evidence after execution.

## Validation-harness correction

The first complete native suite run reached the existing `cutover_and_rollback_share_one_mutation_lock` test and intermittently reported `cutover_mutation_locked` after `drop(holder)`. Its isolated rerun passed. Parallel Git subprocesses can briefly inherit the open descriptor, so dropping only the test process's copy does not reliably release the advisory lock. The test now explicitly unlocks its own holder before dropping it, matching the production `CutoverMutationLock::drop` contract. No production cutover behavior changes. This is a deterministic local CPU lock contract test, with temporary local Git fixtures and no network or release authorization.

## Final local results

- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml`: all 183 tests passed after the explicit test-lock release correction.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check`: passed.
- `cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`: passed.
- Planning self-test: 33 work packages, existing #717/#718/#720, all six negative fixtures rejected.
- Final stable native binary validated the real #523 six-card bundle; retained output is `native-validation-pass.json`.
- Independent `review_523` review found and verified fixes for dynamic SOR heading support and heading-name collisions; rechecked traversal refactor and lock-test correction with no remaining actionable findings. Exact committed-head receipt is retained separately by native publication.

The stable generated binary is installed outside Cargo build output. Its supplementary ignored provenance packet hashes the native Rust source files and binary, because the general install wrapper's source hash covers the ADL owner-binary source tree rather than this standalone crate.
