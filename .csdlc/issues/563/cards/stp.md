# Structured Task Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement one shared pre-mutation installed-owner provenance gate, atomic generation verification, and focused primary-checkout safety proof.

## Deliverables

- Shared installed-owner provenance preflight
- Complete-generation atomic install verification
- Independent primary-checkout bootstrap guard coverage
- Before/after no-mutation negative fixtures
- Focused owner/install validation evidence and exact-head review

## Acceptance

1. AC-1: Every installed owner capable of repository mutation invokes the shared provenance gate before its first mutation
2. AC-2: Stale, missing, malformed, partial, or digest-mismatched installations fail closed with no target-checkout change
3. AC-3: A current owner still rejects bootstrap on primary main and succeeds in an allowed isolated FastWork checkout
4. AC-4: Freshness detects owner-source drift without treating unrelated repository commits as stale
5. AC-5: Installation publishes only a complete verified generation and receipt atomically
6. AC-6: Existing residue is reported without deletion and diagnostics are portable and credential-free
7. AC-7: Focused tests, owner validation, install/resolve contracts, diff hygiene, and exact-head review pass

## Dependencies

- #548 primary-checkout bootstrap guard merged into current main

## Inputs

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/operator.rs
- csdlc-v2/src/bin
- csdlc-v2/src/bin/csdlc-install.rs
- csdlc-v2/operator/generation-selector.json
- csdlc-v2/operator/skills.json
- csdlc-v2/operator/owner-source-set.json
- csdlc-v2/Cargo.lock
- adl-resilience/Cargo.toml
- adl-resilience/src
- docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json
- docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md

## Non Goals

- Deleting existing primary-checkout residue
- Changing issue lifecycle semantics
- Making closeout or cleanup an execution dependency
- Reviving v1 wrappers
- Broad repository refactoring
