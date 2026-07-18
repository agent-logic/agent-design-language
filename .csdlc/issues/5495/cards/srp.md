# Structured Review Prompt

Template: 1.0.0

Issue: 5495

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/git.rs
csdlc-v2/src/review.rs
csdlc-v2/src/doctor.rs
csdlc-v2/tests/gate5.rs
.csdlc/issues/5495/retained/design.md
.csdlc/issues/5495/retained/diagram.mmd

## Prompts

- Can a source or retained-design change bypass review?
- Are all normal typed publication metadata surfaces covered without allowing arbitrary files?
- Does merged reconciliation still require exact identity and final reviewed intent?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Automatic metadata proof relies on typed card projection pairing for each commit in the reviewed-to-current range; substantive source and standalone card-prose drift remain fail closed.

## Review Result

Revision: Some("git-blake3:c8189c643e780dae17f0300be04b756d8bc7fbc2:fab6846c0930a0019a7b8adedd1014bb5172804448ad1a53ec2e9ac0d2baeeaf")

Reviewer: Some("review_5495")

Result: pass
