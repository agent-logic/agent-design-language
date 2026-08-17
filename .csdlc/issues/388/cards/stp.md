# Structured Task Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the narrow C-SDLC v2 owner-tool routes needed to repair implemented-phase SPP/VPP/SOR card truth after review recovery.

## Deliverables

- New or widened semantic operations for implemented SPP summary, VPP summary/failure-policy, and SOR follow-up replacement/removal repair
- CAS/recovery/downstream-truth guards
- Audit provenance containing previous value, new value, actor/reason, and recovery sequence
- Focused regression tests for #114-like sequencing, empty-vector SOR follow-up removal, blank-entry refusal, and other refusal cases
- .csdlc/prepared/issues/388/validate_preparation_bundle.py
- Fresh exact-head review and publishable PR

## Acceptance

1. AC-1: Implemented-phase SPP summary repair works after current recover_review with no active downstream truth, even without prior reviewed/published transition.
2. AC-2: Implemented-phase VPP summary and failure-policy repair works only after current recover_review with exact CAS and actor/reason.
3. AC-3: Implemented-phase SOR follow-up replacement works only after current recover_review with exact CAS and actor/reason, including replacing follow_ups with [] to remove all follow-ups while rejecting blank entries in non-empty replacements.
4. AC-4: Repairs mutate only the intended card fields and derived card/index/audit projections.
5. AC-5: Stale CAS, active review assignment/review/publication/readiness/terminal, wrong card/field, empty required text, blank SOR follow-up entries, and unrelated recovery epoch fail closed.
6. AC-6: Existing #363 immediate/sequenced SPP summary recovery remains compatible.
7. AC-7: Focused tests, strict relevant Clippy, validation, fresh review, publication, CI, and finish pass before #388 terminal closeout.

## Dependencies

- Blocks #114 WP-18C parent publication/finish
- #363 terminal behavior remains compatibility baseline

## Inputs

- agent-logic/agent-design-language#388
- agent-logic/agent-design-language#114
- agent-logic/agent-design-language#363
- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests

## Non Goals

- No #114 product/proof implementation changes
- No raw Markdown/card edits
- No generic implemented-phase set_field
- No review/publication/finish guard weakening
- No lifecycle reset or topology bypass
