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

Revision: Some("git-blake3:f42ac0d181a9dd58a1a6bdb0f5c6bebc87dca173:868f159cf7e16dcce28a79b59d0daf39a286738e051db87fa7c604b503c432fc")

Reviewer: Some("openai-responses:resp_0ffaf1b830609d2c006a8f7cb1c32087d085ec42651c870ea7:metadata-reconciliation")

Result: pass
