# Structured Review Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/551
.csdlc/prepared/issues/551

## Prompts

- Does one validated reload atomically replace every Polis parameter and Runtime presentation consumer without restart?
- Do invalid reloads preserve the complete last-known-good snapshot with bounded redacted diagnostics?
- Does HTML use only the feed identity?
- Is Unity absent from the diff?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- Hot reload changes Runtime presentation and origin policy but does not mutate external DNS, certificates, or ingress infrastructure.

## Review Result

Revision: Some("git-blake3:9de9e110ee0ad1ace724cd1b140932ea7ba4bced:db065447bdb833e4c8ea5f07464ce2a933cb3354f4c2b5dcbd8ded73624414c1")

Reviewer: Some("subagent:/root/review_551_ci_fixture")

Result: pass
