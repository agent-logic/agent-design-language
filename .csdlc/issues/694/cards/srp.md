# Structured Review Prompt

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
demos/html-observatory/app.js
adl/tools/validate_v092_observatory_transcript_history.mjs
adl/tools/test_issue694_conversation_history_reload.sh

## Prompts

- Does history contain both operator and agent halves from production authority?
- Can any replay or reconnect duplicate a turn?
- Are authorization redaction and page limits enforced at the source?
- Does fresh Observatory state invoke restoration deterministically?
- Does the end-to-end test use production code paths rather than parallel fixture logic?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:fdaf9f5ca6a62dd72832477e8a7287b84077ccd8:2f7a7f294d50d6c81c2abaf1fca428fbd4c37d40b1e673e893ace5873ce38e52")

Reviewer: Some("fresh-session:4b42960b-e80d-4b4b-8aa8-661c40266ed1")

Result: pass
