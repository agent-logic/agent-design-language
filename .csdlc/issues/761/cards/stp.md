# Structured Task Prompt

Template: 1.0.0

Issue: 761

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair packet routing and denominator integrity for #761.

## Deliverables

- adl/tools/skills/repo-packet-builder/tests/test_denominators.py
- Builder, validator, exact fixture and coverage docs

## Acceptance

1. AC-1: Classify complete inventory before sampling.
2. AC-2: Record lane denominator, selected paths, exclusions, selection rule and digest.
3. AC-3: Reject empty or category-invalid materially present mandatory lanes.
4. AC-4: Distinguish complete deterministic routing scans from sampled manual review.
5. AC-5: Exact 5481-path #520 fixture populates all six mandatory lanes.
6. AC-6: Empty, invalid-category and digest-mismatch negatives fail deterministically.

## Dependencies

- #520 source candidate c24f8fa65ce445b03ce6cd69007307291d78b60c

## Inputs

- adl/tools/skills/repo-packet-builder
- adl/tools/skills/repo-review-synthesis/references/output-contract.md

## Non Goals

- Semantic review proof
- Unrelated lifecycle tooling repair
