# Structured Review Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/282/production-polis-interface-qualification.md
.csdlc/evidence/282/validate_qualification_packet.py
.csdlc/prepared/issues/282/validate_preparation_bundle.py
.csdlc/prepared/issues/282/design.md
.csdlc/prepared/issues/282/diagram.mmd
.csdlc/issues/282

## Prompts

- Review the exact-revision qualification packet for stale evidence, overclaims, missing artifact links, and unclear residual risks.
- Review the operator runbook for local/read-only reproducibility without credentials or cloud deployment.
- Review product, architecture, and security synthesis for unsupported readiness claims.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This #282 PASS is bounded to exact-revision Observatory production Polis interface qualification evidence and does not claim public cloud deployment, Unity native live proof, provider credential proof, Runtime authority changes, API/storage/browser behavior changes, publication, merge, or terminal closeout.
- Reviewer reran local read-only validators and lifecycle validation; publication, CI, merge, and terminal finish remain pending as separate typed lifecycle steps.

## Review Result

Revision: Some("git-blake3:4e241f5dff406dc344f3ab5da8edbc9142847e1d:ad6b2612ad1d7f79c26641f7866520a95b08d362d964f74c9baad701399372d8")

Reviewer: Some("fresh-session:8397ad62-5e06-436a-855b-af7b3878fdbc")

Result: pass
