# Structured Intent Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement WP-10 deterministic continuity across two or more bounded cycles without treating restart, wake, restore, or snapshot as sufficient identity continuity.

## Required Outcome

A versioned continuity record and validator linking identity root, predecessor and current cycles, ordered evidence, continuity-head derivation, witnesses, grade or stable rejection reason.

## Scope

- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/fixtures/birthday_continuity/authority_tests.rs
- docs/milestones/v0.92/features/IDENTITY_STABLE_NAME_AND_CONTINUITY_v0.92.md
- .csdlc/prepared/issues/5827/validate-native-receipts.rb
- .csdlc/prepared/issues/5827/produce-native-receipt.rb
- .csdlc/evidence/5827
- .github/workflows/wp10-native-birthday-continuity.yml

## Authority

- Issue 5827 owns bounded cycle linkage, not identity-root creation, memory retrieval, migration, or birthday approval.
- Prior lineage and wake evidence are inputs and never replacement authority.
- Continuity must not expose raw private state or infer metaphysical sameness.

## Assumptions

- Issue #5826 and PR #118 remain a serial execution gate: the repaired Birthday Identity implementation must be freshly independently reviewed, fully green, merged, terminally reconciled, and ancestral to the eventual #5827 execution base before binding or product edits.
- The authoritative Birthday Identity output and its verified identity-memory and governed private-state projection authorities are direct read-only inputs; #5827 must consume rather than recreate or weaken them.
- The future typed bind preserves the existing legacy issue identity danielbaustin/agent-design-language#5827 while declaring code_repository agent-logic/agent-design-language.
- The exact declared owned paths are complete for planning and must be collision-checked unchanged before implementation; widening requires explicit typed replan and reapproval.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations in an issue-bound worktree.
- Start product implementation only after a fresh exact claim and current dependency verification.
- Preserve deterministic output, repo-relative references, redaction, and stdout/stderr separation where applicable.
- Run one bounded exact-head review and publish only with the required closing keyword.
