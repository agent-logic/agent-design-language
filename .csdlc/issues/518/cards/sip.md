# Structured Intent Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one exact-revision documentation review and context-free external-review handoff for v0.92.1.

## Required Outcome

One exact-revision documentation-review packet with a context-free external-review handoff, grounded claims, and explicit residual risks.

## Scope

- docs/milestones/v0.92.1/**
- docs/planning/ADL_FEATURE_LIST.md
- .csdlc/issues/518/**
- .csdlc/prepared/issues/518/**

## Authority

- Issue authority is agent-logic/agent-design-language#518
- Execution starts only after #517 has a reviewed merge
- The issue reviews and repairs documentation truth but does not implement product behavior or publish
- Typed finish and cleanup of #517 are non-gating

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Bind every claim and review artifact to one exact candidate revision
- Use resolvable repository paths or canonical issue references
- Do not publish, merge, tag, or release
