# Structured Review Prompt

Template: 1.0.0

Issue: 5825

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5825
.csdlc/evidence/5825
.csdlc/prepared/issues/5825/validate-native-receipts.rb
adl-runtime-kernel/src/birthday.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/tests/birthday.rs
adl-runtime-kernel/tests/fixtures/birthday
docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md

## Prompts

- Does any startup, task, wake, restore, snapshot, admission, copied-state, migration, or incomplete packet incorrectly satisfy the birth contract?
- Are canonicalization, rejection reasons, fixtures, and retained reports deterministic and bound to the reviewed revision?
- Do all evidence references remain repo-relative and redaction-safe, with no personhood, consciousness, citizenship, governance, migration, or launch overclaim?
- Are #5818 and #5819 terminal proof and all claimed acceptance criteria evidenced at exact HEAD?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Native macOS and Linux exact-head portability receipts remain explicitly deferred to the issue-owned CI producer lanes and are not claimed by local proof.

## Review Result

Revision: Some("git-blake3:0cb12f10f87e39d8050aa1657d3d328fae1103b3:373ace0ed331ccd45506d18e7891ba8125fa531149e5f2dc64e6d8d9a1d9578b")

Reviewer: Some("codex:review_5825_exact_head")

Result: pass
