# Structured Intent Prompt

Template: 1.0.0

Issue: 312

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one evidence-bound canonical documentation and release-truth pass for v0.92 from merged producer work without serializing on administrative closeout.

## Required Outcome

Every current root, milestone, feature, release, ADR, skill, guidance, and external-launch claim is inventoried and reconciled to landed reviewed evidence; stale claims are corrected, blockers and non-claims remain explicit, and the packet receives exact-head review.

## Scope

- Canonical root documentation and agent guidance
- v0.92 milestone, feature, ADR, release, demo, launch, and handoff documentation
- C-SDLC v2 operator skill documentation
- Complete documentation-truth inventory and evidence map
- Markdown, JSON/YAML, links, commands, redaction, ownership, and exact-scope validation
- Findings-first documentation review and optional ADR candidate routing

## Authority

- Canonical #312 is current WP-23 authority; legacy danielbaustin/agent-design-language#5843 is immutable provenance
- Canonical #311's merged PR is the WP-22 dependency authority even when its gate result is blocked and administrative closeout is still asynchronous
- The blocked #311 result is documentation truth that #312 must preserve; it does not prohibit the documentation pass
- #312 execution depends on #311 merge only; terminal reconciliation, derived terminal records, closeout receipts, and worktree cleanup are never execution gates
- Documentation may report landed evidence and blockers but cannot create implementation, review, release, governance, or platform authority
- Historical evidence remains immutable; current canonical documentation is corrected without rewriting provenance

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and an issue-bound FastWork worktree
- Prepare in parallel while #311 is open; bind execution after #311 merges and never wait for administrative closeout
- Do not use or introduce tracked .adl paths or dependencies
- Do not mutate sibling issue state, historical evidence, product code, cloud resources, or release authority
- Use focused documentation validation; do not run broad Rust or paid runner lanes
