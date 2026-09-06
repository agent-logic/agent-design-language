# Structured Review Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/control.rs
adl-runtime-kernel/src/ingress.rs

## Prompts

- Can any admitted agent reach its first model turn without the active orientation snapshot?
- Does the recorded digest cover the exact delivered bytes rather than a mutable source or global resource?
- Can reload misreport the package delivered to an existing agent?
- Can invalid content replace the last valid active package?
- Does any wording or control path let orientation enlarge authority?
- Is the implementation smaller than a general prompt framework?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was focused on the multipart logical-part cap fix in assembly/control/ingress and did not run live provider inference, paid Runtime execution, external communications, or broad kernel validation.
- The prior exact-head reviewer found that scalar message/input plus 64 multipart entries exceeded the declared logical-part cap; this review verified the fix and focused multipart validation only.

## Review Result

Revision: Some("git-blake3:bd653fbed169d2072882900d85b50f2b81a33aae:c30cae2aebf0cad30e19b8305b775e28ac0de659de36cdd23c4fe89b533f87d4")

Reviewer: Some("codex:issue-708-logical-part-cap-review")

Result: pass
