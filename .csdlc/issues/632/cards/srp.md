# Structured Review Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/632/command-route-coverage.json
.csdlc/prepared/issues/632/canary-evidence-index.md
.csdlc/prepared/issues/632/validate-v3-canary-readiness.sh
.csdlc/prepared/issues/632/validate-v3-guidance.sh
.csdlc/prepared/issues/632/validate-sprint-review-readiness.sh
docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md
csdlc-v3/README.md
docs/architecture/ADL_ARCHITECTURE.md
.git/csdlc-v2/requests/v0921-v3-full-command-sprint/632-publish-refresh-631-one-pending.json

## Prompts

- Does the coverage matrix account for every v3 command-equivalent route without hidden v2 fallback?
- Do docs and skills warn operators before cutover while avoiding premature v3 authority claims?
- Are canary defects fixed or clearly owned as cutover blockers/follow-ons?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains construction/canary evidence only until explicit V3-F/#505 cutover.
- #505 must consume #629 through #632 dependency evidence through its own exact-head review and publication before cutover.
- Terminal finish and cleanup canary proof still requires an authorized canary merge or explicit #505 blocker disposition.

## Review Result

Revision: Some("git-blake3:eb2cb2d161344b45f0dd3f3373626953ca3452b0:fad7db3e07f48aed1df4c4b43532612b11af4f0c2b50380ef6c536e9a8cb5fab")

Reviewer: Some("subagent:/root/review_632_head_eb2cb2d16")

Result: pass
