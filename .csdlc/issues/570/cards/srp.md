# Structured Review Prompt

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
docs/onboarding.md
docs/architecture/ADL_ARCHITECTURE.md
docs/tooling/adl_pr_cycle_skill.md
csdlc-v3/README.md
csdlc-v3/AGENTS.md
csdlc-v2/AGENTS.md
csdlc-v2/operator/SKILLS.md
csdlc-v2/operator/skills
.csdlc/prepared/issues/570/validate-authority-boundary.sh
.csdlc/prepared/issues/570/validate-docs-routes.sh
.csdlc/prepared/issues/570/validate-skill-guidance.sh
.csdlc/prepared/issues/570/recover-review-after-pr584-findings.json
.csdlc/prepared/issues/570/repair-sor-status-ready.json
.csdlc/prepared/issues/570/normalize-sip-scope-local-skills.json
.csdlc/prepared/issues/570/normalize-stp-deliverables-local-skills.json
.csdlc/prepared/issues/570/normalize-stp-acceptance-local-skills.json
.csdlc/prepared/issues/570/replace-sor-execution-portable-evidence-refs.json
.csdlc/prepared/issues/570/replace-sor-execution-final-evidence-refs.json
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

- Immutable audit history still records superseded machine-local paths from prior generation payloads; live SIP/STP/SOR card truth and prepared replay files now use portable CODEX_SKILLS_ROOT/default-root wording.

## Review Result

Revision: Some("git-blake3:256a222edf9030877b42e31f2521a1ef43690fda:0f7f20a1a2cd61bf1d2fc1df80a6fa6a80721807853b3d1c6208de46ad01b77d")

Reviewer: Some("subagent:issue_570_pr584_repair_review")

Result: pass
