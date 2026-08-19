# Structured Review Prompt

Template: 1.0.0

Issue: 421

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/cards.rs
csdlc-v2/tests/gate2.rs

## Prompts

- Does the implementation require an explicit typed deletion marker rather than accepting arbitrary missing files?
- Does readiness prove base existence and candidate deletion for the claimed path?
- Do ordinary validator deliverables still fail when missing?
- Are #414, #268, #269, AWS, and unrelated lifecycle state untouched?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was bounded to the implementation source/test scope; lifecycle metadata and hosted CI publication checks remain separate typed publication evidence.

## Review Result

Revision: Some("git-blake3:7861debb3ac22eb1b33db9df02fc23479d768699:0c23e657acc003695d075cdf17565f5d4e541a299e43a012e5a206d6bfbb5566")

Reviewer: Some("fresh-session:6c9f3f89-4ce4-4f1a-b816-0df6ad1e4a90")

Result: pass
