# Structured Task Prompt

Template: 1.0.0

Issue: 287

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded #287 evidence reconciliation for ADR 0071 only.

## Deliverables

- .csdlc/evidence/287/evidence-manifest.json
- .csdlc/evidence/287/live-observations.json
- .csdlc/evidence/287/adr0071-provider-neutral-multi-agent-reconciliation.md
- .csdlc/evidence/287/validate_adr0071_provider_neutral_multi_agent_evidence.sh
- Typed SOR/SRP truth for #287

## Acceptance

1. AC1: Retain issue-local evidence showing whether terminal provider-neutral multi-agent proof exists from WP-18B owners at exact revisions.
2. AC2: Machine-check the current WP-18B umbrella #341 state and derived-terminal cache identity, recording absence of terminal umbrella authority as a residual evidence gap when applicable.
3. AC3: Reconcile exact terminal issue-local evidence that exists for related provider/multi-agent slices without upgrading supporting child evidence into terminal #341/#287 proof.
4. AC4: Record machine-readable outcomes, human review boundary, residual gaps, and non-claims for #207 without updating shared ADR docs, the ADR index, final plan, or evidence manifest.
5. AC5: Publish #287 only after focused validation, typed lifecycle validation, and fresh bounded exact review pass.

## Dependencies

- #207 parent ADR proof gate coordination
- #341 WP-18B provider-neutral multi-agent proof umbrella
- Terminal or retained evidence from WP-18B owners where present

## Inputs

- GitHub issue #287
- GitHub issue #207
- GitHub issue #341
- .git/csdlc-v2/derived-terminal/*.json for related WP-18B evidence where present
- GitHub issue/PR live observations for #341 where available

## Non Goals

- Provider execution.
- Credential access, setup, printing, copying, or mutation.
- ADR acceptance or moving ADR 0071 to Accepted.
- Rewriting WP-18B acceptance criteria.
- Updating shared ADR docs, index, plan, or evidence manifest.
