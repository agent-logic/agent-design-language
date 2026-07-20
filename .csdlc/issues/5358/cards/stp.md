# Structured Task Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Generate and validate issue-local acceptance cards, design, diagram, and readiness only; stop before acceptance execution or implementation.

## Deliverables

- Typed SIP, STP, SPP, VPP, SRP, and SOR projections for #5358
- Retained C-SDLC v2 acceptance design and dependency diagram
- Explicit evidence and defect inventory for #5540, #5541, #5548, and #5558
- Doctor and six-card validation evidence
- Bounded subagent review of preparation truth

## Acceptance

1. AC-1: The synced exact revision installs all typed v2 binaries only into .adl/bin/csdlc-v2, verifies provenance and v1_sunset inventory, and resolves generation v2.
2. AC-2: The complete typed lifecycle initializes, approves design, binds, edits, schedules, validates, reviews, publishes, shepherds, reconciles, merges, and closes without direct Markdown or state edits.
3. AC-3: All six generated cards remain schema-valid, structurally valid, digest-consistent, and doctor-clean through every lifecycle phase.
4. AC-4: Claim ownership, protected-path collision rejection, heartbeat and recovery, existing-worktree binding, common-Git terminal receipts, and safe release behavior fail closed.
5. AC-5: Publication is exact-revision-bound, review-current, issue-linked, base-correct, and unable to bypass required green validation, checks, review, or conflict truth.
6. AC-6: Stable deployed binaries and all nine operator skill contracts agree with selector authority and current root and nested AGENTS.md policy.
7. AC-7: Issue #5540, #5541, #5548, and #5558 outcomes are verified integrated from repository and terminal evidence, never waived or absorbed.
8. AC-8: Focused regression, all-target tests, cargo fmt --check, and strict all-target Clippy pass with no deferral.
9. AC-9: Merged issue #5597 and PR #5598 prove the normal-merge terminal reconciliation path and remain truthfully closed out.
10. AC-10: Merged issue #5600 and squash PR #5601 prove published-head and squash-merge reconciliation, including post-merge mergeable_state unknown handling, and remain truthfully closed out.
11. AC-11: Issue #5358 completes exact-revision subagent review, green typed publication, required-check monitoring, merge, retained terminal receipt, and tracked terminal reconciliation.

## Dependencies

- Sprint #5595 opening-wave slot 3 authorization
- Closed recovery evidence from #5540
- Closed Gate 10D2 authority evidence from #5541
- Independent resolution and proof for open defect #5548 before acceptance can pass
- Independent resolution and proof for open defect #5558 before acceptance can pass
- Later integrated exact-revision C-SDLC v2 acceptance execution

## Inputs

- AGENTS.md
- docs/templates/prompts/current.json
- csdlc-v2/operator/generation-selector.json
- csdlc-v2/operator/skills.json
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/features/CSDLC_V2_ACCEPTANCE_v0.91.8.md
- .csdlc/issues/5540
- .csdlc/issues/5541

## Non Goals

- Do not execute C-SDLC v2 acceptance or deployment
- Do not change C-SDLC v2 implementation, owner binaries, templates, or owner validation lanes
- Do not edit shared milestone documents
- Do not absorb, repair, reopen, or close #5540, #5541, #5548, or #5558
- Do not use raw gh or AWS
- Do not publish a PR, merge, or close #5358 in this preparation session
