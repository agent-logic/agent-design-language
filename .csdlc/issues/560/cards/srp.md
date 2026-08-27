# Structured Review Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/560
.csdlc/prepared/issues/560
.csdlc/evidence/560
adl/.config/nextest.toml
adl/src/adl_gws_context_mirror.rs

## Prompts

- Verify the change is an exact ci-coverage timeout/profile adjustment for only the three observed runtime_v2 tests.
- Verify Runtime v2 semantics and assertions are unchanged.
- Verify hosted coverage remains the final shared-gate proof.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted adl-coverage remains the required integration proof before merge.

## Review Result

Revision: Some("git-blake3:b877efc189b823e6f4b4bc145ea8bbe14c632618:9da95cc9e31763e36a8355db40b9e35b408aee43c0bee060509e391b72970fd2")

Reviewer: Some("openai-responses:resp_05358d9aeb79ba51006a8f7e674e0487d09cc756cdcf1200d6")

Result: pass
