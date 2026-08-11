# Structured Task Prompt

Template: 1.0.0

Issue: 141

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair only the two unresolved PR #120 findings; retain the already-fixed epoch overflow as verified current-main truth.

## Deliverables

- Strict structured Clippy receipt contract
- Focused negative and positive regression
- Terminal #5909 lifecycle records
- Reviewed corrective PR

## Acceptance

1. AC-1: #5909 committed index, SPP, and SOR reflect merged PR #120 and closed issue truth
2. AC-2: strict Clippy requires the exact structured argv and validated successful command provenance
3. AC-3: digest-only Clippy artifacts fail the focused regression
4. AC-4: exact structured successful Clippy proof passes
5. AC-5: focused validation and diff hygiene pass
6. AC-6: independent exact-head review has no unresolved findings

## Dependencies

- Merged PR #120
- Closed legacy issue #5909
- Current shared WP-04 proof receipt contract

## Inputs

- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/issues/5909
- PR #120 live state

## Non Goals

- Runtime behavior changes
- Broad lifecycle tooling redesign
- Unrelated WP-04 record cleanup
