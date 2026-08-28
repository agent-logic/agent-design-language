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

Revision: Some("git-blake3:8f71d16fd2bbcb3aaa28906aeefe9e1f4b942aa0:cd1bb15ad718032b8f8f758e8deabb9adc573c0039b27c3cc3e546b65e28decc")

Reviewer: Some("subagent:/root/review_551_ci_fixture")

Result: pass
