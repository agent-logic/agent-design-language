# Structured Review Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
.csdlc/prepared/issues/578/record-glm-reviewer-quality-validation.json
.csdlc/prepared/issues/578/recover-after-reviewer-quality-probes.json
.csdlc/prepared/issues/578/recover-after-quality-evidence-binding-fix.json
.csdlc/prepared/issues/578/recover-after-unpublished-quality-binding-review.json
.csdlc/prepared/issues/578/reviewer-quality-binding-review-prompt.md
.csdlc/issues/578/index.json
.csdlc/issues/578/audit.jsonl
.csdlc/issues/578/cards/sip.values.json
.csdlc/issues/578/cards/stp.values.json
.csdlc/issues/578/cards/spp.values.json
.csdlc/issues/578/cards/vpp.values.json
.csdlc/issues/578/cards/srp.md
.csdlc/issues/578/cards/srp.values.json
.csdlc/issues/578/cards/sor.md
.csdlc/issues/578/cards/sor.values.json

## Prompts

- Does `z_ai:glm-5.3-flash` use the existing #514 profile machinery rather than ad hoc model routing?
- Are GLM-5.3-Flash parameters source-grounded and validated before network dispatch?
- Do focused tests prove exact profile/request behavior and redaction without credentials?
- Can reviewer selection name the new profile deterministically, and is live proof truthfully credential-gated?
- Did the patch avoid #446/#455 scope?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GLM-5.3-Flash reviewer-quality evidence is packet-bound and supports focused first-pass or second-opinion review only; it does not replace ADL exact-head approval review or CI inspection.
- The local smoke artifacts under .adl/provider-smoke are intentionally untracked; durable evidence records candidate commits plus request, result, and run-log digests instead.
- The initial open-PR GLM reviewer smoke remains limited by truncated/noisy PR packets and is not approval evidence.

## Review Result

Revision: Some("git-blake3:fdb60de325c63757eb6afb9021ccc24a6a89d321:0d708574d9bce26b7bb8917be08a9cd8a7f96afe1c4e6b7f6cfef95fe2753211")

Reviewer: Some("fresh-session:1af4bd6b-3806-4a70-acba-1998527ba0bc")

Result: pass
