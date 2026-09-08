# Structured Task Prompt

Template: 1.0.0

Issue: 522

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one complete finding-disposition ledger; individual remediations remain separately reviewable evidence.

## Deliverables

- Deduplicated but provenance-preserving finding census
- One disposition per accepted finding
- Exact-head fix and validation references
- Explicit owned deferrals
- Release-blocker summary

## Acceptance

1. AC-1: Every accepted #520/#521 finding appears exactly once in the census
2. AC-2: Every substantive fix has exact-head review and meaningful proving validation
3. AC-3: Every deferral names owner, rationale, target milestone, and release consequence
4. AC-4: No release-blocking finding remains unresolved
5. AC-5: Counts and provenance reconcile across source reports and the ledger

## Dependencies

- TAIL-05/#521 reviewed green merge is live

## Inputs

- agent-logic/agent-design-language#522
- agent-logic/agent-design-language#520
- agent-logic/agent-design-language#521
- docs/milestones/v0.92.1/evidence/release/tail-04/**
- docs/milestones/v0.92.1/evidence/release/tail-05/**
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-06

## Non Goals

- Silent deferral
- Treating issue closure as proof
- Release ceremony
- Combining unrelated substantive fixes
