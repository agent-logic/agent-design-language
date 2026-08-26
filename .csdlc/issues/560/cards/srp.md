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
- Milestone detection remains document-pattern dependent and may need maintenance after future planning-format changes.

## Review Result

Revision: Some("git-blake3:82f7115bda4d3f0981717b634783c9ec553ed96a:3f42dd75d82806f4348909fa78fbf38425234aa8e18ce6f43d99b8660ac300fa")

Reviewer: Some("openai-responses:resp_0ffaf1b830609d2c006a8f7cb1c32087d085ec42651c870ea7")

Result: pass
