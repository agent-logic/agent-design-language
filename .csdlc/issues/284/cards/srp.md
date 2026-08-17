# Structured Review Prompt

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/284
.csdlc/issues/284
.csdlc/prepared/issues/284

## Prompts

- Does #284 avoid claiming #142 completion from partial #194 evidence or stale local cards?
- Does the validator prove exact retained #5878/#194 evidence and hashes rather than relying on prose?
- Are shared ADR docs/index/plan/manifest untouched as required for #288?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was bounded to #284 issue-local ADR 0066 Guardian authority evidence reconciliation and did not claim #207 closeout, #288 ADR serialization, #142 completion, ADR acceptance, runtime implementation, cloud rerun, or WP-18C terminal proof.
- Typed validation uses the stable generated owner binary with worktree-relative --root . because the FastWork issue worktree does not carry its own .adl/bin owner installation.

## Review Result

Revision: Some("git-blake3:95587b9c75177ab1970368a26f35f334f7fa420f:b5f09ed4498272d90b074ea617697d5a70b7b1999c059bb5837ddf4aded9a6e8")

Reviewer: Some("fresh-session:3eac0ef5-62a6-464d-ae62-afe89ceb74cd")

Result: pass
