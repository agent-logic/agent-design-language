# Structured Task Prompt

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded #284 evidence reconciliation for ADR 0066 only.

## Deliverables

- .csdlc/evidence/284/evidence-manifest.json
- .csdlc/evidence/284/live-observations.json
- .csdlc/evidence/284/adr0066-guardian-authority-reconciliation.md
- .csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh
- Typed SOR/SRP truth for #284

## Acceptance

1. AC1: Retain an issue-local evidence packet that identifies exact #142-graph proof sources for ADR 0066 Guardian membership, authority, fencing, operational, migration, recovery, and shutdown claims.
2. AC2: Machine-check the retained #5878 WP-04.16 terminal proof, including PR #140 identity, execution-proof schema, command success, negative rejection cases, and referenced artifact hashes.
3. AC3: Machine-check the retained #194 private Wuji-AWS qualification evidence while preserving its partial-proof status, quota limitations, and explicit non-claim that it does not complete #142.
4. AC4: Record residual gaps, stale local-card caveats, and non-goals truthfully for #207 without updating shared ADR docs, the ADR index, the final ADR plan, or the milestone manifest.
5. AC5: Publish #284 only after focused validation, card validation/doctor, and fresh bounded review pass at the immutable implementation revision.

## Dependencies

- #207 parent ADR proof gate coordination
- #142 implementation graph evidence
- #5878 / PR #140 terminal WP-04.16 distributed integration proof
- #194 / PR #397 private Wuji-AWS qualification evidence

## Inputs

- GitHub issue #284
- GitHub issue #142
- GitHub issue #194 and PR #397
- GitHub PR #140
- .git/csdlc-v2/derived-terminal/5878.json
- .csdlc/evidence/5878/execution-proof.json
- .csdlc/evidence/194/private-wuji-aws-recovery-live-summary.redacted.json
- .csdlc/evidence/194/live-preflight/live-private-network-preflight.redacted.json

## Non Goals

- Runtime implementation.
- Cloud run or public deployment.
- Rewriting #142 graph acceptance criteria.
- ADR acceptance or moving ADR 0066 to Accepted.
- Updating shared ADR docs, index, plan, or evidence manifest.
