# Structured Review Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/evidence/516
.csdlc/prepared/issues/516
docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md
docs/milestones/v0.92.1/WBS_v0.92.1.md
docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
docs/milestones/v0.92.1/evidence/integration

## Prompts

- Does the denominator include every planned issue, retained dependency, and acceptance row exactly once?
- Does observed evidence prove production behavior rather than test-only or string-presence claims?
- Are missing evidence and ambiguity reported rather than converted into success?
- Does every material gap have a truthful classification and existing owner?
- Would the admission decision fail closed for every unresolved P0/P1 gap?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The admission decision is intentionally blocked while its recorded P1 product gaps remain open.
- Hosted CI remains the final integration gate before merge.

## Review Result

Revision: Some("git-blake3:a5484bed3d9bd5fce18564fa8f980a3b8b46a280:54f0cbf23a4a4a8408c4580635dc760a6cc7d38fabba0cca7a38d02855927c01")

Reviewer: Some("fresh-session:66edcc5a-b723-4838-944d-5f9c86b5db8f")

Result: pass
