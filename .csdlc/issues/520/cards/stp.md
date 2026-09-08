# Structured Task Prompt

Template: 1.0.0

Issue: 520

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one internal findings register; individual findings do not close independently.

## Deliverables

- Complete base-to-candidate path inventory
- Complete v0.92.1 issue/PR and acceptance-surface inventory
- Findings-first specialist reports
- Canonical findings register and synthesis
- Validated exact-head review packet

## Acceptance

1. AC-1: Every changed production file, test/proof file, canonical document, milestone issue/PR, and acceptance surface is inventoried and dispositioned
2. AC-2: Every finding binds exact evidence, severity, revision, impact, source lane, and owner
3. AC-3: The synthesis explicitly identifies partial, inert, unreachable, unproven, and documentation-only outcomes
4. AC-4: No empty, capped, sampled, zero-test, or CI-only lane is credited as a pass
5. AC-5: Packet manifests, counts, redaction, portability, and exact-head identity validate

## Dependencies

- TAIL-03/#519 reviewed green merge is live and supplies the immutable candidate

## Inputs

- agent-logic/agent-design-language#520
- agent-logic/agent-design-language#519
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#TAIL-04
- .csdlc/prepared/issues/520/internal-review-plan.md
- docs/tooling/OPUS_REVIEW_RUNBOOK.md

## Non Goals

- Fixing product findings
- External review
- Release approval
- Merge, deployment, Runtime restart, or paid execution
