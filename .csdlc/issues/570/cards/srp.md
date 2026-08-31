# Structured Review Prompt

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
csdlc-v2/AGENTS.md
csdlc-v2/operator/SKILLS.md
csdlc-v2/operator/skills
csdlc-v3/README.md
docs/architecture/ADL_ARCHITECTURE.md
docs/onboarding.md
docs/tooling/adl_pr_cycle_skill.md
.csdlc/prepared/issues/570
.csdlc/issues/570
.csdlc/evidence/570

## Prompts

- Do the changed docs and skills consistently preserve v2 as live authority until explicit V3-F/#505 cutover?
- Do any updated surfaces accidentally claim v3 can mutate lifecycle state before cutover?
- Are stale routes such as adl_pr_cycle, pr.sh, pr ready, pr preflight, or raw GitHub fallback still presented as current ADL workflow guidance?
- Is the three-minute prepared-issue start target documented as simplification without bypassing typed v2 guards?
- Are local-only installed PR skill checks or updates disclosed truthfully?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- This docs/skill cutover-readiness issue does not cut over authority from C-SDLC v2 to v3; V3-F/#505 remains the explicit authority cutover gate.

## Review Result

Revision: Some("git-blake3:b347a0932446802dbfcf4f92e9a8693c9632c17f:6ddae2240c70c2c31f1ca2154ae12d926dd14a615f7bad73ccf434839694eb53")

Reviewer: Some("review_pr_584_postfix")

Result: pass
