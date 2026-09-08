# Structured Task Prompt

Template: 1.0.0

Issue: 525

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Review and report on exactly one immutable successor planning revision; make no planning or product changes.

## Deliverables

- Exact-revision planning review report
- Complete reviewed-path denominator
- Evidence-backed finding and disposition register

## Acceptance

1. AC-1: Review binds the exact merged planning revision and records scope and dirt state
2. AC-2: The denominator covers all canonical v0.92.2 planning and feature-routing surfaces
3. AC-3: Review checks scope, dependencies, one-result units, deferrals, closeout order, operator gates, and non-claims
4. AC-4: Every finding has severity, evidence, owner, and disposition
5. AC-5: Any post-review planning change invalidates the result
6. AC-6: The final report distinguishes validator results from semantic review judgment

## Dependencies

- Reviewed merge for #524 / TAIL-08
- Merged #523 successor planning package

## Inputs

- docs/milestones/v0.92.2/**
- docs/planning/ADL_FEATURE_LIST.md
- docs/templates/RELEASE_PLAN_TEMPLATE.md
- agent-logic/agent-design-language#525

## Non Goals

- Remediating findings
- Creating v0.92.2 issues
- Approving or performing v0.92.1 release
- Product implementation
