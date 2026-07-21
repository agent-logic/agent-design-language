# Structured Intent Prompt

Template: 1.0.0

Issue: 5610

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Accept safe lexical normalization inside owned coverage roots while rejecting every repository or owned-root escape.

## Required Outcome

Hosted coverage merge accepts safe in-repo bin/.. filenames and continues failing closed on traversal outside repository-owned source roots.

## Scope

- adl/tools/merge_coverage_summaries.py
- adl/tools/test_merge_coverage_summaries.sh

## Authority

- Issue #5610 owns only the two-file shared coverage merge correction
- Issue #5602 remains immutable closed-out evidence
- PR #5607 and PR #5608 remain owned by their existing branches

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 only
- Use /Volumes/FastWork for validation
- No raw gh or AWS
