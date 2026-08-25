# Structured Review Prompt

Template: 1.0.0

Issue: 471

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/471
.csdlc/prepared/issues/471
adl-runtime-kernel

## Prompts

- Can any component access a channel absent from its contract?
- Are startup, restart, degradation, and shutdown behaviors bounded and topology-correct?
- Does health match actual component and capability state?
- Can telemetry or contention poison the data path?
- Did the change stay within Runtime v3 kernel scope?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:01e4779f58c2ca3905f274df499e3db9aca8b498:15256f00c2243d20bd26d330b2bbde6e7c2f538bc16181b9e771059311d27d16")

Reviewer: Some("fresh-session:/root/review_471_r6")

Result: pass
