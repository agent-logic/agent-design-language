# Structured Review Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Issue 41 design, typed cards, error taxonomy, issue-read mapper boundary, real CLI loopback proof plan, redaction, and no-widening constraints.

## Prompts

- Can a 401, ordinary 403, rate limit, 5xx, or connection failure be mislabeled as not-found?
- Can any token, token path, authorization header, raw response body, or Octocrab error text reach stdout or stderr?
- Does the 404 wording remain truthful for inaccessible private repositories?
- Do the tests invoke the real split CLI and assert exact JSON and exit behavior?
- Are successful issue reads byte-shape compatible?

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
