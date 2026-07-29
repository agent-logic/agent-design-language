# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope



## Prompts

- Does the tracked-path scan consume Git's NUL-delimited literal filenames without reintroducing line-oriented parsing?
- Do UTF-8 paths, spaces, and embedded newlines preserve complete path components during Windows portability validation?
- Do genuine Windows-illegal characters, trailing spaces or dots, backslashes, and reserved device names still fail closed?
- Does the focused regression mirror the PR failure by retaining a portable UTF-8 baseline path while proving ordinary and newline-hidden illegal components are rejected?
- Are the recovery, focused validation, and exact-head review records scoped truthfully to the two CI path-policy files?

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
