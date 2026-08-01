# Structured Review Prompt

Template: 1.0.0

Issue: 5558

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/csdlc_prompt_editor
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_csdlc_prompt_editor.sh
.csdlc/issues/5558

## Prompts

- Does any changed active surface still expose an executable sunset v1 lifecycle route?
- Does the owner lane run the real Gate 10A final-authority test?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub CI must confirm that deleted orphan source is absent from the changed-source coverage gate on the refreshed PR head.

## Review Result

Revision: Some("git-blake3:85e506c4613502753b895bcace4fe535289138ee:e04e8dc30470f367dfb2288ae0660ef419ec5cf412b6bbaf72775d8141902ed4")

Reviewer: Some("subagent:019fbf2d-07af-7ee0-a7e1-ca480b4a5d0a")

Result: pass
