# Structured Task Prompt

Template: 1.0.0

Issue: 96

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair only the WP-04 implementation-wave validator and its focused regression test.

## Deliverables

- .csdlc/prepared/issues/5862/validate-implementation-wave.rb
- .csdlc/prepared/issues/5862/test-validate-implementation-wave.rb
- V3 S-to-E-to-H topology validation with product and evidence immutability
- Exact PR head, merge, ancestry, terminal-envelope, denominator, DAG, and #5878 native proof preservation

## Acceptance

1. Valid S not equal to E not equal to H topology passes
2. Protected product paths are unchanged across S through E through H
3. Evidence is introduced once at E and immutable through H
4. Exact PR head H, merge, ancestry, terminal envelope, and unique mapping are checked
5. Product drift, evidence drift, wrong head or merge or ancestry, ambiguous or missing mapping, and fake self-reference are rejected
6. Sixteen-child, path, DAG, terminality, and #5878 integrated/native bindings remain mandatory
7. Focused proof, independent review, and green CI pass

## Dependencies

- Sprint #5862 implementation-wave contract
- Current C-SDLC v3 two-revision evidence model tracked by #53

## Inputs

- .csdlc/prepared/issues/5862/validate-implementation-wave.rb
- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/evidence/5863/execution-proof.json
- .csdlc/evidence/5866/replay-window/execution-proof.json
- .csdlc/evidence/5872/execution-proof.json

## Non Goals

- Closeout rewrite
- Runtime or distributed product changes
- Accepting unmerged or non-terminal children
- Weakening denominator, DAG, path, ancestry, evidence, or native receipt checks
