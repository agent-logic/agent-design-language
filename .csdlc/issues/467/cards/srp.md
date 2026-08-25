# Structured Review Prompt

Template: 1.0.0

Issue: 467

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/467
.csdlc/prepared/issues/467
.csdlc/evidence/467
docs/reviews/v0.92/quality-gate-467
docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
docs/milestones/v0.92/QUALITY_GATE_v0.92.md
docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md
docs/milestones/v0.92/WBS_v0.92.md
docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
docs/milestones/v0.92/features/README.md
docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md

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

- Unity and broader Observatory consumer work remains explicitly backlogged or scheduled later under #84, #122, and #251; #467 does not claim it complete.
- External publication remains separately operator-authorized and is not implied by this passing engineering quality gate.

## Review Result

Revision: Some("git-blake3:82e3d155784361593682edddc5382005f9e9c5db:391336b854fa4314ba432b0136c592502585310b225161fbad0e395ca0b96ee6")

Reviewer: Some("fresh-session:dcaa14a9-7136-4bee-ae5c-ddf9ede35073")

Result: pass
