# Structured Task Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Plan and prepare Runtime v3 functional parity only; do not execute runtime implementation, cutover, deletion, or v0.92 activation.

## Deliverables

- Runtime v3 functional-parity plan and machine-readable matrix
- Updated canonical v0.91.8 planning, proof, demo, review, release, and handoff surfaces
- Four bounded parallel implementation lane definitions under #5361
- Architecture diagram and explicit 12K source / fewer-than-1000-test budgets
- Typed lifecycle records, focused validation, and bounded review

## Acceptance

1. AC-1: The nine historical fixture groups plus Observatory have explicit live-process proof requirements
2. AC-2: Every v0.91.7 feature document and implemented feature-list row has a required pre-deletion disposition
3. AC-3: Four disjoint implementation lanes fit the global writable WIP cap and converge before WP-11
4. AC-4: #5361 acceptance precedes WP-12 cutover and WP-13 deletion
5. AC-5: The plan names one canonical Runtime v3 and enforces the 12K source and fewer-than-1000-test budgets
6. AC-6: Canonical docs, YAML, JSON, links, and diff hygiene validate without Runtime implementation or AWS

## Dependencies

- Closed v0.91.7 Runtime v3 implementation and review packets
- docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json
- Current Runtime v3 source and feature-list audit
- Closed v0.91.8 setup issue #5383

## Inputs

- docs/milestones/v0.91.8/README.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.91.7/features
- adl-runtime/src
- adl-runtime-kernel/src

## Non Goals

- Do not implement Runtime v3 behavior
- Do not switch the default runtime or delete Runtime v2
- Do not use AWS
- Do not add hard-coded hosts, IP addresses, credentials, or ports
- Do not expand the product feature list
