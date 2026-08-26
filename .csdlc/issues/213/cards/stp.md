# Structured Task Prompt

Template: 1.0.0

Issue: 213

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Change only the initialized/ready authorization and review-invalidation behavior for the two existing semantic operations, plus focused tests and issue-local lifecycle evidence.

## Deliverables

- Initialized/ready STP ordinal acceptance-criteria repair through csdlc-edit apply
- Initialized/ready pending-only SPP plan-step repair through csdlc-edit apply
- Atomic current design/diagram binding refresh and fresh design-review requirement
- Focused before/after byte, path/identity drift, CAS, ownership, coverage, compatibility, regeneration, audit, reapproval, and base-to-source diff proof

## Acceptance

1. AC-1: An existing unbound initialized or ready issue can replace same-denominator STP criteria whose ordered entries use exact ordinal IDs AC-1 through AC-N, and the successful transaction atomically refreshes current canonical design/diagram refs and digests in both SPP and VPP without deleting or rebootstrapping the record.
2. AC-2: An existing unbound initialized or ready issue can replace SPP steps only when every step is pending and the exact ordinal acceptance-ID union equals the current STP denominator; the same atomic current-binding refresh applies.
3. AC-3: Repository, issue, initialization lineage, lifecycle phase, branch/worktree topology, transition history, audit prefix, later-phase evidence, design/diagram bytes, and every untouched semantic field remain byte-preserved while all six projections regenerate atomically at exactly one new generation.
4. AC-4: Either repair sets design review pending without changing initialized/ready phase or topology; ready repair additionally requires unbound topology, no later lifecycle evidence, and exact CAS, and doctor blocks until one fresh independent reapproval restores current design readiness.
5. AC-5: Stale CAS, malformed/reordered/renumbered or changed-denominator criteria, non-pending/invalid/duplicate steps, missing/extra acceptance coverage, wrong card ownership, exact design/diagram path or card identity drift, unsupported phase, bound topology, later evidence, and interrupted writes fail closed with byte-identical rollback.
6. AC-6: Explicit compatibility fixtures prove existing bound STP/SPP operations and implemented SPP plan-step behavior retain their prior statuses, outputs, phase, topology, and audit shape; no execution, review, publication, merge, terminal, or Git-topology authority is added pre-bind.
7. AC-7: Focused Gate 2 tests reproduce the literal #205 prior-planning-edit plus design-change sequence and prove both phases, both operations, before/after bytes, binding refresh, preservation, regeneration, audit, doctor, reapproval, drift rejection, atomicity, and compatibility contracts.
8. AC-8: Formatting, the complete Gate 2 integration target, strict all-target C-SDLC v2 Clippy, `git diff --check origin/main...HEAD`, and fresh independent exact-head review pass before a ready unmerged PR opens.

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
