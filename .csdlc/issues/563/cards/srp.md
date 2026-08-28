# Structured Review Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/563/audit.jsonl
.csdlc/issues/563/cards/sip.values.json
.csdlc/issues/563/cards/sor.values.json
.csdlc/issues/563/cards/spp.values.json
.csdlc/issues/563/cards/srp.md
.csdlc/issues/563/cards/srp.values.json
.csdlc/issues/563/cards/stp.values.json
.csdlc/issues/563/cards/vpp.values.json
.csdlc/issues/563/index.json
.csdlc/prepared/issues/563/assign-simplified-final-review.json
.csdlc/prepared/issues/563/record-simplified-final-review.json

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

## Review Result

Revision: Some("git-blake3:8af0fc73f55ba855ab892464cc24831ad5aabb7d:42c2e6023d8364f99a2560efc81c90966125b503c8424d8e5217fd7b6be8de70")

Reviewer: Some("fresh-session:d831dd0b-3c3b-4726-b58e-f5242b1364f6")

Result: pass
