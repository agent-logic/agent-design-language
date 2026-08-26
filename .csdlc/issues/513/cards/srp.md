# Structured Review Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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
- The DEC-01 contract proves authority topology and dry-run migration/rollback behavior; it does not implement Runtime v4 or move Runtime v2/v3 source ownership.

## Review Result

Revision: Some("git-blake3:0e6b6fd4ad73493ab5dd342f6c03db19d898fc87:c1800ff5f9cf8272bea63396f0eec358bfc37e2790fc5fa99d5bc0ce0120b288")

Reviewer: Some("gpt-5.5:thread-01a03fad-ed50-7141-ac1b-510bc6620305")

Result: pass
