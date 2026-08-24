# Structured Review Prompt

Template: 1.0.0

Issue: 467

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/467
.csdlc/prepared/issues/467
.csdlc/evidence/467
docs/reviews/v0.92/quality-gate-467
docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
docs/milestones/v0.92/QUALITY_GATE_v0.92.md
docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md

## Prompts

- Does the generator deterministically discover and hydrate all 33 canonical rows without defaulting to packet-missing blockers?
- Do accepted rows carry exact repository, PR, merge, ancestry, check, proof, claim-boundary, and terminal authority evidence?
- Do non-accepted rows name concrete blockers rather than normalization gaps as product failures?
- Does the validator independently reject substitutions, missing/duplicate/extra/ambiguous/unclassified rows, stale authority, and vacuous all-blocked publication?
- Do corrected docs and evidence agree without rewriting #311 history or unlocking downstream work beyond concrete accepted rows?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The quality gate intentionally remains blocked: 30 rows lack canonical accepted release-credit evidence and downstream work must not treat #467 as a release unlock.

## Review Result

Revision: Some("git-blake3:3e03769a1c81539f8952dfa1bc3baab4b93e9964:46277af45d48ff6384e9db354f07118836eb807172da41df1322fd624ed8ac65")

Reviewer: Some("fresh-session:bc87db90-6f30-4e8e-a391-3a353eb9f95e")

Result: pass
