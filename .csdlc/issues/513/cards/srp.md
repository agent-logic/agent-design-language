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

- OpenAI Responses API review resp_082c6a71386fee33006a8f58dcfc3487d09c69e7d1c6b53d61 returned PASS and publication_safe: true for exact head ca83c7a5f96c7190696adb3ec06f4c14bf647848.
- Required GitHub CI remains the integration gate after publication.

## Review Result

Revision: Some("git-blake3:ca83c7a5f96c7190696adb3ec06f4c14bf647848:4e8d5509dd222671d43deeb9d5d52462b1513b1a4b703a42f431bafb51befda1")

Reviewer: Some("openai-responses:resp_082c6a71386fee33006a8f58dcfc3487d09c69e7d1c6b53d61:gpt-5.6-sol")

Result: pass
