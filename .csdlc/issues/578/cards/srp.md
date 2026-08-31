# Structured Review Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
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
.csdlc/prepared/issues/578/record-live-zai-smoke-validation.json
.csdlc/prepared/issues/578/record-ox-alpha-reviewer-proof-validation.json
.csdlc/prepared/issues/578/record-open-pr-ox-alpha-review-smoke-validation.json
.csdlc/prepared/issues/578/record-ox-alpha-reviewer-effort-ceiling-validation.json
.csdlc/prepared/issues/578/recover-after-live-ox-alpha-proof.json
.csdlc/prepared/issues/578/assign-live-ox-alpha-proof-review.json

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

- Open-PR Ox Alpha / GLM-5.3-Flash review smoke used bounded diff excerpts, so it proves fast triage and limitation handling rather than exact-head approval.
- High and max reasoning_effort both returned HTTP 200 empty text for the #582 PR-review prompt shape; reviewer-trial defaults should stay at low until separately repaired or re-characterized.
- The live proof artifacts under .adl/provider-smoke are local redacted run artifacts and are intentionally not committed.

## Review Result

Revision: Some("git-blake3:af48fb7dac7d33f8612c94cc216a6afb6ac7cce6:21e0fe7d977b6572106a8ac4f351bd36800794ed089c4f03d981f7ef04f85c34")

Reviewer: Some("fresh-session:4771a65e-426b-4baf-b760-3a12aab2d8ed")

Result: pass
