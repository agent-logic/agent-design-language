# Structured Review Prompt

Template: 1.0.0

Issue: 5695

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/github.rs
csdlc-v2/src/merge.rs

## Prompts

- Does every supported octocrab MergeableState map explicitly?
- Can blocked or unstable ever become stale_base?
- Does csdlc-merge remain fail-closed while checks or ancestry are pending?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:9325a45f7906d50cf2d280c9211470fa10b69f67:69ac1b5709559a69a6af7acc19651d13e7933f01160cde6a8cdba607a2832b77")

Reviewer: Some("codex-subagent:review-5695")

Result: pass
