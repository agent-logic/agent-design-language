# Structured Task Prompt

Template: 1.0.0

Issue: 323

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue 323 only; implement a bounded lifecycle owner recovery operation for active bound issue identity/repository migration. Do not recover #5913 in this issue and do not merge PR #320 here.

## Deliverables

- Typed request/report schema for bound issue identity migration
- csdlc-issue owner subcommand for the migration
- Fail-closed store implementation that migrates record/cards namespace and preserves provenance
- Focused regression tests covering #5913 -> #322 invariants
- Skill/schema documentation sufficient for operator use

## Acceptance

1. AC-1: A typed owner operation can migrate an active nonterminal issue record from source issue identity to target issue identity without hand edits
2. AC-2: The operation fails closed on stale digest/generation, terminal records, conflicting target namespaces, unsafe topology, or missing provenance
3. AC-3: Migrated record/cards validate under csdlc-validate and preserve source issue/repository provenance in audit/report evidence
4. AC-4: Publication/review truth is handled explicitly so subsequent csdlc-publish can relink the PR to the canonical target issue before csdlc-finish
5. AC-5: Regression tests cover the #5913 -> #322 published-PR shape and existing finish invariants remain strict

## Dependencies

- Current-repo issue #322 created from legacy #5913
- Green PR #320 preserved at 15bcf79f6c80e18c340db2f3d4de9c43099a5046

## Inputs

- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/src/migration.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md

## Non Goals

- Recovering #5913 itself inside this issue
- Merging or closing PR #320
- Generic lifecycle state rewriting
- Raw GitHub lifecycle mutation
- Changes to #112, #298, projection_recovery.rs, store.rs, or gate5.rs
