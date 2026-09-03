# Structured Review Prompt

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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

- The current head differs from the original substantive review by typed lifecycle metadata only; the reviewed code/test scope is unchanged and excludes .csdlc projections.

## Review Result

Revision: Some("git-blake3:e6bfe4674cb4c48a6834d01c5e58cfbc6b75d1b7:65ddfa6734a5c1e1589b6b4abf1940be8f057a9743eac34e28525f2bd4d739e5")

Reviewer: Some("subagent:/root/review_645_stacked_closing_relation_pre_pr")

Result: pass
