# Structured Review Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Approved two-file behavior diff
Typed metadata-only drift from prior reviewed implementation head

## Prompts

- Does only the heavyweight adl_coverage_hosted job move to the selected 16-core runner?
- Does focused proof fail if that job returns to ubuntu-latest?
- Are stable adl-coverage, producers, Spot, artifacts, Codecov, and coverage policy unchanged?
- Is the runner expression valid GitHub Actions syntax and consistent with existing heavy lanes?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:0cc4b6f24f6314609d49faabc0c4c9aa4a24967d:3d18904541c6fec9697296a554c8d2f9ab0c3fc2e9fe44be8fd548b97d0e3959")

Reviewer: Some("codex:rereview_55_publication_head")

Result: pass
