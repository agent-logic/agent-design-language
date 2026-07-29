# Structured Review Prompt

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5715
.csdlc/issues/5717
.csdlc/prepared/issues/5717
.csdlc/evidence/5717
demos/podcast/studio-reference/REFERENCE_DIGESTS.txt
demos/podcast/studio-reference/podcast-studio.html
demos/podcast/studio-reference/uploads/agent-logic-logo.svg
demos/podcast/studio/REFERENCE_DIGESTS.txt
demos/podcast/studio/podcast-studio.html
demos/podcast/studio/reference.sha256
demos/podcast/studio/uploads/agent-logic-logo.svg

## Prompts

- Does the studio page satisfy each operator-requested copy/logo/layout fix?
- Are fake historical episode numbers removed in favor of proposed launch topics starting at 1?
- Do the studio route, assets, audio artifact, and RSS feed still validate?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The generated page intentionally uses relative feed and audio links; deployment must preserve the current demos/podcast/studio parent-child layout where ../feed.xml and ../audio/meet-the-ai-coworkers.wav resolve.

## Review Result

Revision: Some("git-blake3:64b747a9cd9601e2c6444930c38a5dc32f99397b:fe8ae0a0e65d286ccbee0e218380af44e4c43aae0caaeca0f374967c2e05f730")

Reviewer: Some("gemini-3.1-pro-preview")

Result: pass
