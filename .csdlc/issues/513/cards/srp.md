# Structured Review Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/513
.csdlc/prepared/issues/513
.csdlc/evidence/513
docs/runtime/runtime-v2-v3-authority-topology.md
docs/milestones/v0.92.1/evidence/runtime-decoupling

## Prompts

- Does the manifest assign every declared Runtime v2/v3 source root one owner and disposition?
- Does the reverse-reference census cover the current repo references without leaving supported consumers unclassified?
- Are migration and rollback executable dry-run contracts rather than prose-only claims?
- Does the implementation avoid Runtime v4 scope and sibling Sprint 1 work?
- Are the validation lanes sufficient for DEC-01's bounded authority-separation result?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Origin/main or required CI movement before merge can still require ancestry refresh and re-observation.
- Untracked .csdlc/tmp request files are operational inputs only and must not be staged into the publication branch.

## Review Result

Revision: Some("git-blake3:6060943e7875378c4eab0e7c5f7a534e5db4273e:4700b5dd14ab9770e69da6e75ea7461aa2c958f6542b94c2ba5736439b661b19")

Reviewer: Some("gpt-5.5:thread-01a03fad-ed50-7141-ac1b-510bc6620305")

Result: pass
