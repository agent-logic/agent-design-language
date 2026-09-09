# Structured Task Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one exact-revision documentation-review and external-handoff packet; document checks are evidence inputs.

## Deliverables

- docs/milestones/v0.92.1/evidence/release/tail-02
- docs/planning/ADL_FEATURE_LIST.md
- .csdlc/prepared/issues/518/validate-documentation-handoff.rb

## Acceptance

1. AC-1: Every canonical v0.92.1 document is inventoried and agrees with current authority
2. AC-2: Links, paths, issue references, and claims are resolvable and source-grounded
3. AC-3: Residual risks, deferred scope, and non-claims are explicit
4. AC-4: The review and handoff bind one exact candidate revision

## Dependencies

- Issue 517 passing reviewed merge gates final acceptance and handoff; operator authorized independent corrections now.

## Inputs

- agent-logic/agent-design-language#518
- agent-logic/agent-design-language#517
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-02
- docs/milestones/v0.92.1/**
- docs/planning/ADL_FEATURE_LIST.md

## Non Goals

- Product implementation
- Publication
- Release ceremony
- Inventing evidence for unresolved claims
