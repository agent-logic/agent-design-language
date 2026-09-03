# Structured Review Prompt

Template: 1.0.0

Issue: 662

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
.csdlc/prepared/issues/662/validate-focused.sh
.csdlc/evidence/662
.csdlc/prepared/issues/662/review-assign-red-janitor-head.json
.csdlc/prepared/issues/662/review-recover-after-red-janitor-fix.json
.csdlc/prepared/issues/662/review-recover-after-red-janitor-self-review.json
.csdlc/prepared/issues/662/review-assign-red-janitor-self-review.json
.csdlc/prepared/issues/662/review-record-red-janitor-self-review-pass.json

## Prompts

- Is agent-to-agent initiation distinct from user-facing replies?
- Are Beacon sender and Ember recipient identities canonical and non-confusable?
- Can duplicate or replayed initiation create duplicate work without an explicit rule?
- Do cancellation and provider/recipient failures produce truthful terminal state?
- Does activity projection expose authoritative initiation truth without inventing delivery?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Metadata-head review covers the committed red-check review request evidence at de44f0b4853d6cc3df33eacd86dc2520c7010234; subsequent record/publication commits are expected to be governed .csdlc/issues/662 metadata only.
- No additional runtime source changes were introduced after the locally and hosted-green Clippy fix f2d09fa64efed868b043809387efe573eee54941.
- No live Runtime mutation, provider call, AWS action, paid runner, merge, finish, or cleanup was performed during metadata-head review.

## Review Result

Revision: Some("git-blake3:de44f0b4853d6cc3df33eacd86dc2520c7010234:51ad76c9aae6b0723ddc456503b54bf3b5db4104e616a661a42ae6a01d547135")

Reviewer: Some("codex:/root:issue-662-red-janitor-metadata-head-review")

Result: pass
