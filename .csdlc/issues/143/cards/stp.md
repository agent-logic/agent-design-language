# Structured Task Prompt

Template: 1.0.0

Issue: 143

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Correct the stale v0.92 ADR plan, author and index source-grounded Proposed or Deferred ADR candidates 0059 through 0071, validate numbering and evidence links, obtain exact-head independent review, and publish a documentation-only PR.

## Deliverables

- Corrected canonical v0.92 ADR plan and accepted baseline
- Thirteen uniquely numbered Proposed or Deferred ADR candidate documents
- Reviewer-facing v0.92 ADR evidence and disposition index
- Focused numbering, status, link, evidence, and non-claim validator
- Exact-head architecture, security-boundary, and documentation review evidence

## Acceptance

1. AC-1: The accepted ADR baseline is derived from docs/adr/README.md and candidate numbers 0059 through 0071 do not collide
2. AC-2: Every number 0059 through 0071 has exactly one Proposed or Deferred candidate document and index row
3. AC-3: Every candidate contains status, context, decision, consequences, alternatives, source evidence, validation evidence, supersession relationships, non-claims, and approval boundary
4. AC-4: Existing ADRs are cited rather than duplicated and conflicts or supersessions are explicit
5. AC-5: ADR 0070 remains a non-production planning boundary proven by source consistency and copied-state negative cases, not an end-to-end production transfer claim
6. AC-6: ADR 0069 and ADR 0071 are drafted only if their named real executable proof is landed; otherwise they are explicitly Deferred
7. AC-7: Authority, security, replay, and mutation decisions cite focused positive and fail-closed negative proof when such proof has landed
8. AC-8: No candidate claims personhood, production citizenship, completed v0.93 governance, unrestricted adaptive learning, or production cross-polis migration
9. AC-9: Focused validation proves numbering uniqueness, status policy, required sections, repository-relative evidence links, index completeness, and diff hygiene
10. AC-10: One fresh independent exact-head review covers architecture, security boundaries, documentation truth, and all candidate dispositions with no unresolved actionable findings

## Dependencies

- Current accepted ADR registry through ADR 0058
- Landed v0.92 implementation and exact proof surfaces for each drafted candidate
- Canonical v0.92 feature contracts and issue wave
- Explicit human approval for any future ADR acceptance

## Inputs

- docs/adr/README.md
- docs/adr/0013-runtime-v2-citizen-state-continuity-substrate.md
- docs/adr/0016-moral-evidence-and-cognitive-being-substrate.md
- docs/adr/0017-secure-local-agent-comms-and-a2a-boundary.md
- docs/adr/0021-adl-capability-contract-runtime-authority-boundary.md
- docs/adr/0048-runtime-observability-and-otel-boundary.md
- docs/adr/0053-portable-signed-records-and-external-trust.md
- docs/adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md
- docs/adr/0055-runtime-v3-unified-redb-state.md
- docs/adr/0058-memory-palace-context-handoff-architecture.md
- docs/milestones/v0.92/ADR_PLAN_v0.92.md
- docs/milestones/v0.92/DECISIONS_v0.92.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/features

## Non Goals

- No Runtime or tooling implementation
- No accepted ADR promotion
- No reopening accepted ADRs without a documented conflict
- No production cross-polis migration or transfer proof
- No v0.93 citizenship or governance implementation
- No synthetic proof credit for planned but unexecuted WP-18A or WP-18B work
