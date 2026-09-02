# Structured Review Prompt

Template: 1.0.0

Issue: 263

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92.1/review/podcast_directory_263

## Prompts

- Does the candidate satisfy every acceptance criterion on its real owned path?
- Does it preserve sibling ownership, operator authority, privacy, and rollback?
- Are all proof claims exact-revision and non-overstated?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Fresh reviewer could not execute the Ruby validator in read-only sandbox; implementation session retained executable validator proof at the same candidate head.

## Review Result

Revision: Some("git-blake3:ee61ef40d7e7862b172e848a4f89eca52977715c:3f23bf6d51e42dac3f924c10086c5deb6cd457b3b30f0694977a0f99e8080ae0")

Reviewer: Some("fresh-session:94d2a427-ed77-45b0-8994-5cac9c0b2cb2")

Result: pass
