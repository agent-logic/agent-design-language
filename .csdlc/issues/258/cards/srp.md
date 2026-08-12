# Structured Review Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed
adl-runtime/tests
.csdlc/issues/258
.csdlc/prepared/issues/258
.csdlc/locks/258.lock
.csdlc/evidence/258

## Prompts

- Review whether raw store access is sealed and whether published receipt view is sufficient for the authority-serving boundary.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not fetch current GitHub/CI state or mutate remote state; remote PR freshness remains outside this read-only review.
- Reviewer distinguished assignment metadata commit d85f5f4936819e4aa845dadc36e7e75f65dddb38 from assigned substantive revision aff77bce21433f2a49c6c624850ede7689903ab5.

## Review Result

Revision: Some("git-blake3:aff77bce21433f2a49c6c624850ede7689903ab5:b9e8f5e13bf820be00665eadfaaa73836434156084f7722d547810d9cdb1b7fa")

Reviewer: Some("fresh-session:dcd53a3f-2f2e-43ea-a6ca-bd1ef46a4c5c")

Result: pass
