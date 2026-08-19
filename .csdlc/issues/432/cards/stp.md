# Structured Task Prompt

Template: 1.0.0

Issue: 432

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remove tracked and authoritative repository dependence on .adl without redesigning worktree policy or moving .csdlc.

## Deliverables

- Pre-change migration inventory
- Relocated worktree policy and updated consumers
- Zero tracked .adl denominator
- Deterministic reintroduction guard
- Fresh-checkout bind proof

## Acceptance

1. AC-1: Exact pre-change tracked-path and active-reference inventories have one disposition per entry
2. AC-2: git ls-files .adl returns zero paths at the implementation head and in a fresh checkout
3. AC-3: Active source, build, test, documentation, CI, schema, manifest, policy, and fallback scans contain no authoritative .adl dependency
4. AC-4: One canonical policy outside .adl preserves allowed and rejected worktree behavior
5. AC-5: Git ignore and deterministic guards reject reintroduction
6. AC-6: No sensitive or local-only data is promoted
7. AC-7: Focused tests, fresh-checkout proof, diff hygiene, and exact-head review pass

## Dependencies

- None beyond current origin/main

## Inputs

- .adl/worktree-policy.json
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- AGENTS.md
- .gitignore
- git ls-files .adl

## Non Goals

- Deleting operator-local .adl content
- Moving .csdlc authority
- Redesigning worktree placement policy
- Broad unrelated documentation cleanup
