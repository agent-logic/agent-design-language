# Structured Review Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/tests/projection_recovery_integration.rs
.csdlc/evidence/300/bridge-fed-r11
.csdlc/evidence/300/bridge-fed-r9/projection-recovery-integration.log
.csdlc/evidence/300/bridge-fed-r10/projection-recovery-integration.log
.csdlc/requests/300/replace-sor-bridge-fed-r11.json

## Prompts

- Are both prerequisite terminal and ancestry gates exact and fail closed before bind?
- Does every production mutation and durability boundary have before/after restart proof?
- Can any mock, constant, path, or self-authored receipt become authority?
- Are symlink, repeated-inode, ancestor-swap, destination-race, recovery/cleanup, ordinary-commit, and sentinel cases explicit?
- Does scope remain one new integration test target plus issue-local records?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not mutate, publish, merge, record review, or rerun Cargo. PASS is inspection-only against exact scoped source/evidence, commit relationship checks, hash checks of logged artifacts, and current typed terminal cache inspection.
- r8, r9, and r10 are retained as non-proving evidence history only; final proving validation is r11 at immutable source head 84156f343a047c7c9207193c21515c3dccbe2ead.

## Review Result

Revision: Some("git-blake3:d1ba6ce6e90ca5660d85bca6887475fdb453f563:0276331fce2eaba9d330e7f2cbf29cfede28b79c76a3015fbce94c10f4857627")

Reviewer: Some("fresh-session:franklin-300-bridge-fed-r11")

Result: pass
