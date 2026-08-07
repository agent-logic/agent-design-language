# Structured Task Prompt

Template: 1.0.0

Issue: 5901

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the claim-free Sprint 3 readiness mismatch proven at canonical revision 1757ea43e00f83447356b5749007f68a64144242.

## Deliverables

- Safe future-path readiness predicate and focused tests
- Typed #5865 planning normalization
- Claim-free WP-04 terminal validator
- Doctor and preflight proof for the exact sixteen-child wave

## Acceptance

1. Accept safe repository-relative future paths beneath a real canonical in-repository ancestor
2. Reject absolute, traversal, placeholder, unrooted, and metadata values
3. Reject inside-repository symlink ancestors, outside-repository symlink ancestors, symlink leaves, and existing-file intermediate prefixes
4. Permit exact-digest typed planning-collection repair while initialized or ready, then move #5865 serialization ordering to typed replan_triggers while keeping exactly four path-only affected_areas
5. Remove every claim read and claim-derived assertion from WP-04 preflight and terminal reconciliation; validate initialized unbound topology in preflight and csdlc.derived_terminal.v1 envelopes plus live closing linkage, exact heads, merge SHAs, digests, and candidate ancestry at terminal
6. Pass doctor for #5862 through #5878 without product file creation or child binding
7. Pass the exact sixteen-child, 38-path WP-04 preflight
8. Pass an exact-base changed-path allowlist that rejects every Sprint 3 product path and child topology mutation
9. Complete independent exact-head review and resolve every actionable finding

## Dependencies

- PR #5886
- Issue #5896 and PR #5897
- Sprint umbrella #5862

## Inputs

- AGENTS.md
- csdlc-v2/src/cards.rs
- .csdlc/prepared/issues/5862/validate-implementation-wave.rb
- .csdlc/issues/5863/cards/spp.values.json
- .csdlc/issues/5865/cards/spp.values.json

## Non Goals

- Guardian product implementation
- Child binding or publication
- Claim restoration
- DAG or owned-product-path changes
