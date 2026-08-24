# Structured Intent Prompt

Template: 1.0.0

Issue: 467

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Replace the vacuous #311 all-blocked quality-gate packet with a corrective v0.92 quality gate that deterministically hydrates all 13 feature and 20 critical-path rows from canonical evidence and classifies real blockers truthfully.

## Required Outcome

A reviewable PR for #467 that repairs the generator, validator, adversarial tests, matrix, gate record, blocker report, validation receipt, v0.92 quality/readiness docs, and #311 supersession note without rewriting #311/PR #466 history.

## Scope

- #467 corrective quality-gate generator and validator
- 33-row v0.92 quality-gate matrix and blocker report
- adversarial and positive control tests for canonical hydration
- v0.92 quality/readiness/proof documentation
- .csdlc/evidence/467 validation and review evidence

## Authority

- #467 supersedes #311 evidence semantically but does not rewrite #311 or PR #466 history
- #467 does not implement product features or waive real missing proof
- #312, administrative closeout, and unrelated release ceremony work are outside scope
- typed C-SDLC v2 is lifecycle authority; raw GitHub writes are not used for covered lifecycle state
- only merged predecessor work and authoritative evidence may grant release credit

## Assumptions

- none

## Operator Constraints

- Bind beneath /Volumes/FastWork/adl-worktrees before implementation edits
- Do not mutate #312
- Do not use administrative closeout as a dependency
- Do not use /private/tmp
- Do not edit main for implementation
- Preserve #311/PR #466 as historical provenance
- Publish only after one fresh exact-head review
