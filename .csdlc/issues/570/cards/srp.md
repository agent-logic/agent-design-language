# Structured Review Prompt

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
docs/onboarding.md
csdlc-v3/README.md
csdlc-v3/AGENTS.md
csdlc-v2/AGENTS.md
csdlc-v2/operator/SKILLS.md
csdlc-v2/operator/skills
.csdlc/issues/570
.csdlc/prepared/issues/570
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

- Publication was not present during review, so the PR body closing keyword remains a publication-time check; the PR must include Closes #570.

## Review Result

Revision: Some("git-blake3:88a7e2f86589020ef6f753377941c96a908da94c:372c1ad4a0a6b8709ffd240c1245bd5e626c1ac1a27b441a092e12ca2f7905db")

Reviewer: Some("subagent:issue_570_final_review_3")

Result: pass
