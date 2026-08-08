# Structured Task Prompt

Template: 1.0.0

Issue: 53

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add an explicit two-revision v3 proof contract and focused regression without migrating retained receipts or changing Runtime product code.

## Deliverables

- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/53/test-proof-receipt-contract.rb
- .csdlc/prepared/issues/53/design.md
- .csdlc/prepared/issues/53/diagram.mmd
- Exact-head review and qualified issue-closing implementation PR

## Acceptance

1. AC-1: A tracked receipt for substantive commit A committed in evidence-only commit B validates without recursive HEAD rewriting
2. AC-2: The validator distinguishes and resolves source_revision and evidence_revision
3. AC-3: Source is an ancestor of evidence and evidence is an ancestor of current HEAD
4. AC-4: Every A..B changed path is confined to declared issue evidence and receipt paths
5. AC-5: Source, command, logs, negatives, native receipts, and artifact tampering still fail closed
6. AC-6: Focused regression proves A then B plus later metadata C and all named rejection cases
7. AC-7: Retained v2 receipts are not reinterpreted or migrated
8. AC-8: Exact-head review has no unresolved actionable findings

## Dependencies

- agent-logic/agent-design-language#53
- Existing WP-04 proof receipt v2 contract
- Git ancestry and diff pathspec behavior

## Inputs

- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/5863/validate-proof-receipt.rb
- .csdlc/issues/5863
- Git merge-base and diff semantics

## Non Goals

- Runtime or distributed Guardian product changes
- Migration or rewriting of retained receipts
- Weakening any artifact or runner provenance digest
- General redesign of all ADL evidence formats
