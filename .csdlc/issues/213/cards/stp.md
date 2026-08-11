# Structured Task Prompt

Template: 1.0.0

Issue: 213

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Change only the initialized/ready authorization and review-invalidation behavior for the two existing semantic operations, plus focused tests and issue-local lifecycle evidence.

## Deliverables

- Initialized/ready STP acceptance-criteria repair through csdlc-edit apply
- Initialized/ready SPP plan-step repair through csdlc-edit apply
- Fresh design-review requirement after either repair
- Focused atomicity, CAS, ownership, coverage, regeneration, audit, and reapproval regression proof

## Acceptance

1. AC-1: An existing unbound initialized or ready issue can replace STP acceptance criteria through the existing typed semantic operation without deleting or rebootstrapping its record.
2. AC-2: An existing unbound initialized or ready issue can replace SPP plan steps through the existing typed semantic operation without deleting or rebootstrapping its record.
3. AC-3: Repository, issue, initialization lineage, lifecycle phase, topology, transition history, audit prefix, and every untouched semantic field remain preserved while all six projections regenerate atomically at one new generation.
4. AC-4: Either repair invalidates prior design approval and doctor blocks until a fresh exact design approval is recorded; reapproval restores readiness without resetting audit truth.
5. AC-5: Stale generation/digest, empty or malformed acceptance criteria, invalid/duplicate plan steps, missing acceptance coverage, wrong card ownership, unsupported phase, and interrupted writes fail closed without partial mutation.
6. AC-6: Existing bound and implemented operation behavior remains unchanged, and no execution, review, publication, merge, terminal, or Git-topology authority is added pre-bind.
7. AC-7: Focused Gate 2 tests prove the exact initialized and ready sequences, preservation, regeneration, audit, doctor, reapproval, rejection, and atomicity contracts.
8. AC-8: Formatting, the complete Gate 2 integration target, strict all-target C-SDLC v2 Clippy, diff hygiene, and fresh independent exact-head review pass before a ready unmerged PR opens.

## Dependencies

- Issue #213 is the tooling prerequisite for #205's final preparation repair
- No #205 dependency is required to implement this owner-level tooling fix

## Inputs

- AGENTS.md
- csdlc-v2/AGENTS.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- Issue #205 exact preparation head 2ffbaa3c9d59de4a23363c05a3290e3cb5942d26

## Non Goals

- Generic JSON Patch, raw values editing, or Markdown mutation
- Record deletion, rebootstrap, or audit reset as repair
- Any #205 product or lifecycle mutation
- Any weakening of dependency, bind, review, publication, merge, or closeout gates
