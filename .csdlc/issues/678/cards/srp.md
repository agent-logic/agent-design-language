# Structured Review Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Review issue #678 installer route, stable CSM launcher behavior, isolated regression tests, and operator documentation.

## Prompts

- Does .adl/bin/csm now route to .adl/runtime-v3/current/bin/csm without becoming a second binary source of truth?
- Do activation and rollback switch the stable route atomically through the current symlink?
- Do missing or incomplete active generations fail before service mutation?
- Do the tests prove stale stable binary repair without touching the live Runtime?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
