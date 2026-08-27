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

Revision: Some("git-blake3:65631db732b1d8b8e7fbfc859163171b76051e10:c5772903ef566b0ef30f13037d580bbf761a4fe29e411afbb73128662a95187c")

Reviewer: Some("openai-responses:resp_05358d9aeb79ba51006a8f7e674e0487d09cc756cdcf1200d6:metadata-head-reconciliation")

Result: pass
