# Structured Review Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/632/canary-evidence-index.md
.csdlc/prepared/issues/632/command-route-coverage.json
.csdlc/prepared/issues/632/validate-v3-canary-readiness.sh
.csdlc/prepared/issues/632/validate-sprint-review-readiness.sh

## Prompts

- Does the coverage matrix account for every v3 command-equivalent route without hidden v2 fallback?
- Do docs and skills warn operators before cutover while avoiding premature v3 authority claims?
- Are canary defects fixed or clearly owned as cutover blockers/follow-ons?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains construction/canary evidence only until explicit V3-F/#505 cutover; current v3 routes must not become live lifecycle authority before #505.
- Terminal finish and cleanup canary proof still requires an authorized canary merge or an explicit #505 blocker disposition.
- #505 must consume current #629 through #632 dependency evidence through its own exact-head review, publication, operator approval, and terminal reconciliation before cutover.

## Review Result

Revision: Some("git-blake3:04f155ed61eec699cdb45bc02647214262c7db1f:ffad1d035035996a12f7c84158b34be3de6b872c381b6b05ee57aa86d05a6525")

Reviewer: Some("subagent:/root/review_632_head_refresh")

Result: pass
