# Structured Review Prompt

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/merge_coverage_summaries.py
adl/tools/test_merge_coverage_summaries.sh

## Prompts

- Does safe lexical normalization remain inside the owned root?
- Can any prefix or owned-root traversal normalize into accepted ownership?
- Are all existing merge and coverage gates unchanged?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Normalization intentionally remains lexical and does not resolve filesystem symlinks; llvm-cov filename ownership consumes lexical provenance only.

## Review Result

Revision: Some("git-blake3:1d37fd8aaf903d115f025511831882853194c3c3:d01383fc5d3c5a844a62b27cab55d0b313c2af1cc27cd6369247dacccf0f4f9c")

Reviewer: Some("subagent:review-5610-current-main")

Result: pass
