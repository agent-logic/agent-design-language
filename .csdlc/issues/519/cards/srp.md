# Structured Review Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

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

- Redaction is bounded pattern screening of the publication packet; historical source documents are hash-verified, not comprehensively rescanned.
- The merged source PR observation is a snapshot. No merge, tag, release or issue closure is authorized by this review.

## Review Result

Revision: Some("git-blake3:9d414f324b116e9a70777dc120bbf67a38eb70db:c48424c65980ca986f8f047cb6e0a6728a9c5199fff7d3543bba53ed0206cae3")

Reviewer: Some("codex:/root/review_519_final")

Result: pass
