# Structured Review Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs
csdlc-v2/tests/card_identity.rs
csdlc-v2/tests/code_repository_migration.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate2.rs
csdlc-v2/tests/gate4.rs
csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
csdlc-v2/tests/projection_recovery_integration.rs
docs/tooling/EMERGENCY_BRANCH_ADOPTION.md
.csdlc/issues/665
.csdlc/prepared/issues/665

## Prompts

- Can the adoption route import an unrelated or stale branch/worktree?
- Does successful adoption advance only to bound and preserve all later lifecycle gates?
- Are dirty, ambiguous, unsafe-parent, main-branch, and missing-ancestry cases rejected?
- Does ordinary bind/create behavior remain unchanged?
- Does documentation make clear that emergency product actions do not grant lifecycle authority?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:76c27a7b6e2867f1da69edd290403c832b234ccc:fa8a77ef6c19a4dc78c0098900616c4ce6d46b3cc959a2241625c6096435ff2b")

Reviewer: Some("review_665_prepr_r1")

Result: pass
