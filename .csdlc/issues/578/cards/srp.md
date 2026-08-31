# Structured Review Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/profiles.rs
adl/src/provider_adapter.rs
adl/tests/provider_tests/http_family.rs
adl/tests/provider_tests/profiles.rs
docs/provider/inference-profiles.md
docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash/README.md
.csdlc/prepared/issues/578/design.md
.csdlc/issues/578/index.json
.csdlc/issues/578/audit.jsonl
.csdlc/issues/578/cards/sip.values.json
.csdlc/issues/578/cards/stp.values.json
.csdlc/issues/578/cards/spp.md
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

- GitHub PR #582 was still at 5501e85a6d0217f2592a67df8d92fab253948c53 during review; publication must push and reconcile the reviewed head before claiming live PR currency.
- High and max GLM-5.3-Flash effort modes are callable with larger output budgets but have materially higher latency; low remains the safe runtime/reviewer default.
- Human-facing medium effort is not a Z.ai provider parameter and must be implemented only as an ADL preset mapped to documented provider values.

## Review Result

Revision: Some("git-blake3:6a1fc52d6c185793a92178a2090c0c3373a90227:2d252d0fd15a9653db863e4fb459c8ae2d5a74f4e31f2e7cf7752b8eba49945f")

Reviewer: Some("fresh-session:09da19f4-5b8d-437a-99ed-6b7ebe038dd3")

Result: pass
