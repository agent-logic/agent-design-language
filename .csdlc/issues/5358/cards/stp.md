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

1. AC-1: All six #5358 cards are generated from typed values and pass structure, schema, digest, and doctor checks
2. AC-2: The design defines exact-revision acceptance for install/selector authority, all typed lifecycle stages, claims, validation, review, publication, shepherding, and closeout
3. AC-3: #5540 and #5541 are declared as closed evidence inputs without reopening or absorbing their scope
4. AC-4: #5548 and #5558 are declared as independently owned open acceptance blockers without editing their surfaces
5. AC-5: Preparation makes no acceptance, deployment, implementation, publication, merge, or closeout claim
6. AC-6: The typed review card contains a bounded preparation-only review scope and evidence-focused prompts

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
