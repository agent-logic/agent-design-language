# Structured Intent Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the merged CSMctl launcher work safely on Linux while preserving Darwin behavior.

## Required Outcome

start_CSM.sh and CSMctl provide tested start, stop, restart, and status control on native Linux without invoking launchctl, while issue #268 retains authority for the native x86 Amazon Linux AWS qualification.

## Scope

- CSMctl
- start_CSM.sh compatibility
- focused launcher tests
- docs/tooling/START_CSM_RUNBOOK.md

## Authority

- Issue #426 owns only launcher portability
- Issue #268 owns the paid AWS qualification
- Issue #269 must never execute

## Assumptions

- none

## Operator Constraints

- Gemini must review the exact implementation head before merge
- Preserve existing Darwin behavior
