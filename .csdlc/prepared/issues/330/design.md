# Design: #330 bridge-fed projection cleanup production defect

## Problem

#300 replaced its synthetic integration proof with real production recovery-to-cleanup authority from #297. The resulting focused command fails with two production invariants:

- Later ordinary typed commits reject a completed recovery after cleanup removes the retained rejected archive node, because recovery validation still requires the original archived bytes to match `003-rejected-archived`.
- A real `900-cleanup-complete.json` race appearing after the early shortcut and before pre-final validation is not fail-closed with byte-exact zero mutation in the integrated bridge-fed path.

## Approach

Keep #300 frozen and repair the production boundary in a separate issue:

1. Preserve recovery completed-attempt validation as strict by default.
2. Permit post-cleanup retained recovery validation only when exact cleanup authority proves the same completed recovery result, canonical archive manifest, archived root, and completed cleanup operation.
3. Preserve fail-closed semantics for missing, forged, stale, cross-operation, or mismatched cleanup authority.
4. Tighten cleanup pre-final/final receipt race handling so a preexisting real final receipt must validate against the complete current ledger authority before any mutation continues; unexpected or mismatched final receipts must reject without new ledger, namespace, receipt, or archive mutation.
5. Prove the fix with focused regressions and the parent #300 bridge-fed integration target.

## Owned paths

- `csdlc-v2/src/projection_recovery.rs`
- `csdlc-v2/src/projection_cleanup.rs`
- focused `csdlc-v2/tests/*` surfaces required for regressions
- `.csdlc/issues/330`
- `.csdlc/evidence/330`

## Dependency contract

- Requires #297 terminal/ancestral, already observed through PR #328 merge `5ebd2143e3f36638b16f6153446eff655116f76a`.
- Blocks #300 publication until #330 is terminal and #300 replays the bridge-fed integration proof.

## Validation

- Focused #330 regression(s) for cleaned recovery validation and final-receipt race zero mutation.
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test projection_recovery_integration`
- strict Clippy for `csdlc-v2/Cargo.toml`
- fmt/diff hygiene
- typed `csdlc-validate` / `csdlc-doctor`
