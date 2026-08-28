# Structured Review Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/563
.csdlc/prepared/issues/563/assign-metadata-reconciliation-review.json
.csdlc/prepared/issues/563/record-metadata-reconciliation-review.json
.csdlc/prepared/issues/563/recover-review-for-metadata-reconciliation.json
.csdlc/prepared/issues/563/recover-review-for-residual-risk-repair.json

## Prompts

- Does every repository-mutating installed owner reach one shared read-only gate before locks or filesystem writes?
- Does owner-source freshness avoid whole-repository HEAD false positives while detecting actual C-SDLC drift?
- Can any partial or mixed installation become selected?
- Are primary, linked, isolated, and pre-existing-residue cases proven with exact before/after state?
- Are diagnostics portable, actionable, and free of credential or host-path disclosure?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final pre-merge integration gate.
- The exhaustive preservation proof runs only in a tiny synthetic fixture; production freshness uses two bounded Git queries and installed executable digests.

## Review Result

Revision: Some("git-blake3:2772844ca07061ceb6f30319b92f79f9ef155a30:9718a15061b8fe1effe7d59faf08f6436410059630d96dba5b04fbe7c94476d1")

Reviewer: Some("fresh-session:d831dd0b-3c3b-4726-b58e-f5242b1364f6")

Result: pass
