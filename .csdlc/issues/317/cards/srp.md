# Structured Review Prompt

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_317.md
.csdlc/evidence/317
.csdlc/prepared/issues/317/validate-closeout-plan.rb
.csdlc/issues/317

## Prompts

- Is the canonical v0.92 issue denominator complete and duplicate-free?
- Does every row bind exact immutable GitHub and typed truth without self-attestation?
- Does the DAG gate only on reviewed green merge ancestry while leaving finish and cleanup asynchronous?
- Do the negative cases fail closed for every declared risk?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live GitHub observations can drift after the retained snapshot; publication and later merge checks must re-read exact PR state.
- Typed finish, cleanup, umbrella bookkeeping, and handoff reconciliation remain asynchronous and are not successor gates.

## Review Result

Revision: Some("git-blake3:0fae275ca2b7dc82701058ef71da3d09b9cdbb62:ebc42ee5123dbb1fe2b9b3b53d6547f2c9e812d48a78e33eadf77b6107aad5dd")

Reviewer: Some("fresh-session:04ea0b23-0b9c-4e54-872c-42f4262bb6f1")

Result: pass
