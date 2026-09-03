# Structured Review Prompt

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/src/publication.rs
csdlc-v2/tests/gate6.rs
csdlc-v2/tests/publication_ready.rs
csdlc-v2/tests/publication_tail.rs

## Prompts

- Can csdlc-publish still record a terminal closing publication when GitHub closingIssuesReferences is absent?
- Does the regression test cover the PR #644 stacked-base shape rather than only body parsing?
- Do publish and PR-state outputs agree on linked_issue and linkage_source?
- Is non-closing checkpoint publication clearly non-terminal?
- Does the failure message give the operator actionable stack/default/checkpoint choices?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live publication mutation was performed during pre-PR validation; the implementation was validated with read-only PR #644 shape inspection plus local regression and publication suite coverage.

## Review Result

Revision: Some("git-blake3:6918a84ae35a8d16fe8028354dbe35690cb1254e:0c43ff38f3de3bcc5b74836e805540517f93cd897b7ee56496de61e18f607fed")

Reviewer: Some("subagent:/root/review_645_stacked_closing_relation_pre_pr")

Result: pass
