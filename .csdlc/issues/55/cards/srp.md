# Structured Review Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Heavyweight adl_coverage_hosted selected-runner route
Focused regression assertions and preserved surrounding CI semantics
Typed issue evidence at exact implementation revision

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

Revision: Some("git-blake3:79280aa3b1c5887876d5f4e9a4252d963c92828a:63eab5f08f70d6ad08aea6729d7ab5385ea9a62be74e93e1e6e4560ecc5b9994")

Reviewer: Some("codex:review_55_exact_head")

Result: pass
