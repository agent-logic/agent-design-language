# Structured Task Prompt

Template: 1.0.0

Issue: 285

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded #285 evidence reconciliation for ADR 0068 only.

## Deliverables

- .csdlc/evidence/285/evidence-manifest.json
- .csdlc/evidence/285/live-observations.json
- .csdlc/evidence/285/adr0068-birthday-governance-handoff-reconciliation.md
- .csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh
- Typed SOR/SRP truth for #285

## Acceptance

1. AC1: Retain issue-local evidence showing whether terminal birthday-to-governance handoff proof exists from WP-18 and WP-19 owners at exact revisions.
2. AC2: Machine-check #5839 terminal handoff evidence, including PR #289, merge SHA, and current derived-terminal cache identity.
3. AC3: Machine-check #5836 retained local lifecycle state and record the absence of current terminal authority as a residual WP-18 evidence gap rather than a completed proof.
4. AC4: Record machine-readable outcomes, human review boundary, residual gaps, and non-claims for #207 without updating shared ADR docs, the ADR index, final plan, or evidence manifest.
5. AC5: Publish #285 only after focused validation, typed lifecycle validation, and fresh bounded exact review pass.

## Dependencies

- #207 parent ADR proof gate coordination
- #5839 / PR #289 birthday-to-governance handoff terminal evidence
- #5836 retained WP-18 first birthday demo lifecycle state

## Inputs

- GitHub issue #285
- GitHub issue #207
- .git/csdlc-v2/derived-terminal/5839.json
- .csdlc/issues/5836/index.json
- GitHub PR #289
- GitHub issue/PR live observations for #5836 where available

## Non Goals

- Governance implementation.
- ADR acceptance or moving ADR 0068 to Accepted.
- Rewriting WP-18 or WP-19 acceptance criteria.
- Updating shared ADR docs, index, plan, or evidence manifest.
