# Structured Review Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/519/artifact-redaction.log
.csdlc/evidence/519/diff-hygiene.log
.csdlc/evidence/519/exact-head.log
.csdlc/evidence/519/negative-fixtures.log
.csdlc/evidence/519/publication-linkage.log
.csdlc/prepared/issues/519/design.md
.csdlc/prepared/issues/519/diagram.mmd
.csdlc/prepared/issues/519/validate-publication-candidate.rb
docs/milestones/v0.92.1/evidence/release/tail-03/README.md
docs/milestones/v0.92.1/evidence/release/tail-03/candidate.json
docs/milestones/v0.92.1/evidence/release/tail-03/source-pr.json

## Prompts

- Check all four acceptance criteria at the assigned commit. Cover code, security, evidence, exact source/review/merge identity, canonical origin, closing linkage, redaction, negative fixtures and release nonclaims. Findings first P0-P3 with file/line evidence. Read-only; no passing review with actionable findings.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- GitHub source observation is a snapshot; implementation session refreshed it after source merge.
- Redaction screens this publication packet, not every referenced historical artifact.
- Retained 791-document/14-artifact/nine-negative proof reused after confirming byte-identical inputs; no merge, tag, release or closure authorized.

## Review Result

Revision: Some("git-blake3:7dce40bb1d8119a5c59c5342308942233b067cc5:ab0221185a1fe9e7a91bdcffa08c2ffa2b1c8ae9cd39b962958afe1405b119c8")

Reviewer: Some("codex:/root/review_519_record_final")

Result: pass
