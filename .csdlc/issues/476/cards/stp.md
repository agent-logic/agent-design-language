# Structured Task Prompt

Template: 1.0.0

Issue: 476

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Apply and integrate only the post-merge WP-27 truth repair preserved in ed454a246.

## Deliverables

- Typed SPP/VPP repair
- Validator helper removal
- README payload correction
- Focused validation evidence
- Fresh exact-head review
- Merged PR closing #476

## Acceptance

1. AC-1: SPP records evidence-backed completed steps and keeps terminal reconciliation nonterminal until finish.
2. AC-2: VPP claims only local evidence and regression proof while typed review/finish retain exact-head and terminal authority.
3. AC-3: The validator has no unused GitHub helper and its declared regression lane passes.
4. AC-4: The README names CommittedWithCleanupPending(Box<ProductionBirthdayReceipt>).
5. AC-5: Diff hygiene and focused validation pass at the exact head.
6. AC-6: A fresh pre-assigned exact-head review has no actionable findings and the green PR merges closing #476.

## Dependencies

- #315 and PR #473 are terminal remote provenance
- ed454a246 is preserved as the exact repair source

## Inputs

- agent-logic/agent-design-language#315
- agent-logic/agent-design-language#476
- agent-logic/agent-design-language/pull/473
- ed454a2461daccf95f75191ccea69d7df9ae06df

## Non Goals

- No runtime behavior change
- No reopening PR #473
- No unrelated cleanup
- No #269 inspection or execution
