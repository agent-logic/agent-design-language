# Issue 141 Design: PR #120 Terminal and Proof Repair

## Decision

Repair the merged WP-04.07 record without changing Runtime behavior. The shared
receipt contract must recognize strict Clippy only as an exact structured
command whose argv, successful exit, timing, runner identity, and output digests
are validated. Issue #5909 terminal records must be derived from live merged PR
#120 and closed issue truth.

## Scope

- `.csdlc/prepared/issues/5862/proof-receipt-contract.rb`
- `.csdlc/prepared/issues/5909/validate-proof-receipt.rb`
- `csdlc-v2/src/store.rs`
- `.csdlc/prepared/issues/141/test-strict-clippy-proof.rb`
- `.csdlc/issues/5909`
- `.csdlc/prepared/issues/141`
- `.csdlc/issues/141`

## Owned Paths

- `.csdlc/prepared/issues/5862/proof-receipt-contract.rb`
- `.csdlc/prepared/issues/5909/validate-proof-receipt.rb`
- `csdlc-v2/src/store.rs`
- `.csdlc/prepared/issues/141`
- `.csdlc/issues/141`
- `.csdlc/issues/5909`

## Invariants

1. An opaque Clippy log or manifest cannot satisfy strict-Clippy proof.
2. The exact Clippy argv is required and all normal command provenance checks apply.
3. PR #120 remains the product merge authority; this issue changes no Runtime code.
4. Terminal records reflect live GitHub merge and closure truth.
5. Terminal materialization completes the active plan step and SOR status atomically.

## Validation

Run the focused Ruby regression, validate issue cards, and check diff hygiene.
