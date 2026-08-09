# Structured Task Prompt

Template: 1.0.0

Issue: 79

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove the narrow initialized-phase admission exception required to bind Sprint #5862 children safely.

## Deliverables

- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate2.rs

## Acceptance

1. AC-1: Fixtures matching #5866, #5871, and #5872 pass doctor and bind before their exact declared source and test files exist
2. AC-2: Admission requires exact path ownership, exact deliverables, fail-closed policy, and non-placeholder lane deferrals
3. AC-3: An absent new Rust module is admitted only through an issue-owned temporary path test harness route
4. AC-4: Arbitrary unroutable modules, undeclared targets, zero tests, missing implemented targets, and absent proof still fail closed
5. AC-5: Prose deliverables are not misclassified as filesystem validator paths
6. AC-6: Existing false-readiness negatives, focused C-SDLC v2 tests, strict Clippy, and exact-head independent review pass

## Dependencies

- Current canonical Agent Logic main
- Sprint #5862 prepared child records for #5866, #5871, and #5872

## Inputs

- AGENTS.md
- csdlc-v2/src/cards.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/issues/5866/cards/stp.values.json
- .csdlc/issues/5866/cards/vpp.values.json
- .csdlc/issues/5871/cards/stp.values.json
- .csdlc/issues/5871/cards/vpp.values.json
- .csdlc/issues/5872/cards/stp.values.json
- .csdlc/issues/5872/cards/vpp.values.json

## Non Goals

- Distributed Guardian product implementation
- Shared production module registration owned by #5878
- General relaxation of Rust module routing or validation proof
- Lifecycle wrappers, claims, leases, or unrelated workflow repair
