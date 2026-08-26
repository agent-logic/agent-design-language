# Structured Review Prompt

Template: 1.0.0

Issue: 482

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/482/asset-denominator.log
.csdlc/evidence/482/diff-hygiene.log
.csdlc/evidence/482/provenance-and-license.log
.csdlc/evidence/482/redaction-and-custody.log
.csdlc/issues/482
.csdlc/prepared/issues/482
docs/milestones/v0.92.1/evidence/corporate/corp-a/custody-receipts.v1.json
docs/operations/corporate/asset-register/critical-asset-schedule.md
docs/operations/corporate/asset-register/critical-asset-schedule.v1.json

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not rerun the Ruby validators against a checked-out exact worktree because live HEAD contained later review-assignment metadata; reviewer inspected validator source/evidence logs and ran exact-revision Git diff hygiene directly.
- Reviewer could not independently verify the supplied git-blake3 digest because b3sum was unavailable.
- Private custody envelope, counsel/legal assertions, and provider account facts are intentionally represented only through redacted role attestations and were not independently verified.

## Review Result

Revision: Some("git-blake3:04b7db20f31c26a336b342ebd035869c290c6099:511751a29525368d6b8949f54598e57e9face82abdab1e59ea524b0952051334")

Reviewer: Some("fresh-session:codex-482-corp-a-review-r2")

Result: pass
