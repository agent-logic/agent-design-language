# Structured Review Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/308
.csdlc/issues/308
.csdlc/prepared/issues/308
.csdlc/locks/308.lock
adl/tools/validate_v092_demo_proof_coverage.py
adl/tools/test_v092_demo_proof_coverage.sh
docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md

## Prompts

- Do all accepted rows agree across matrix, coverage, activation, and artifact index at one exact revision?
- Does every accepted claim retain required positive and negative proof plus platform, credential, review, and non-claim truth?
- Does the validator reject every declared invalid class without manufacturing proof?
- Does the patch preserve WP-21/WP-21A and child ownership boundaries?
- Is the predecessor gate terminal, reconciled, and ancestral at the execution base?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only and bounded to exact commit dae062e1267de2a783aa20e739b8d1caca19e78e; reviewer ignored post-HEAD review-assignment metadata dirt.
- Reviewer did not rerun the mutating shell negative suite in read-only review mode; checked-in validator logs and local pre-review validation record the passing run.

## Review Result

Revision: Some("git-blake3:dae062e1267de2a783aa20e739b8d1caca19e78e:0a56c91f01b013f01046e5beaf0ddd30647ff32713b09ad13637374b456a7626")

Reviewer: Some("fresh-session:codex-cli-issue-308-review-dae062e1")

Result: pass
