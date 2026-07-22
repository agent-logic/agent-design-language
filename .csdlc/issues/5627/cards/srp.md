# Structured Review Prompt

Template: 1.0.0

Issue: 5627

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/pvf.rs
csdlc-v2/src/store.rs
csdlc-v2/src/review.rs
csdlc-v2/src/publication.rs
csdlc-v2/src/doctor.rs
csdlc-v2/src/cards.rs
csdlc-v2/src/bin/csdlc-validate.rs
csdlc-v2/src/bin/csdlc-review.rs
csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/tests/gate4.rs
csdlc-v2/tests/gate5.rs
csdlc-v2/tests/gate6.rs
csdlc-v2/tests/gate7_lifecycle.rs
csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
.csdlc/issues/5627
.csdlc/prepared/issues/5627

## Prompts

- Can validation failure alter index, cards, audit, or generation?
- Can review record accept stale or out-of-scope evidence without assignment?
- Can direct publication record a different repository, branch, SHA, or draft state?
- Does active-draft compatibility remain bounded to existing draft records?
- Do command and artifact measurements match the executable four-command proof?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
