# Structured Review Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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

Revision: Some("git-blake3:363925241c1e0743355feaabc46a0cf47f5e9598:77dbc68992d8f7c4936574692c2f81d31b809bf8df7357163d81042b7a2b5fb2")

Reviewer: Some("review_665_prepr_r1")

Result: pass
