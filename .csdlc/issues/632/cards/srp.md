# Structured Review Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/632/design.md
.csdlc/prepared/issues/632/diagram.mmd
.csdlc/prepared/issues/632/bootstrap.json
.csdlc/prepared/issues/632/command-route-coverage.json
.csdlc/prepared/issues/632/canary-evidence-index.md
.csdlc/prepared/issues/632/finalize-implementation.json
.csdlc/prepared/issues/632/replace-execution-after-review-fixes.json
.csdlc/prepared/issues/632/replace-execution-current-command-surface.json
.csdlc/prepared/issues/632/recover-review-after-main-merge.json
.csdlc/prepared/issues/632/validate-v3-canary-readiness.sh
.csdlc/prepared/issues/632/validate-v3-guidance.sh
.csdlc/prepared/issues/632/validate-sprint-review-readiness.sh
docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md
csdlc-v3/README.md
docs/architecture/ADL_ARCHITECTURE.md
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/full-replacement-denominator.json

## Prompts

- Does the coverage matrix account for every v3 command-equivalent route without hidden v2 fallback?
- Do docs and skills warn operators before cutover while avoiding premature v3 authority claims?
- Are canary defects fixed or clearly owned as cutover blockers/follow-ons?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains construction/canary evidence only until explicit V3-F/#505 cutover; v2 remains live lifecycle authority before that cutover.
- Merge readiness still depends on the exact live PR #647 checks finishing successfully after the final branch push.
- #505 must consume current #627-#632 dependency evidence through its own exact-head review, publication, operator approval, and terminal reconciliation before cutover.

## Review Result

Revision: Some("git-blake3:fd786459ea83c426f45356126836281ebbaf0519:543349d5c6b04d537bf5409baf158bb13f294bbc43ec42932e5e51c8709049a5")

Reviewer: Some("subagent:/root/review_632_after_main_merge_fd786459")

Result: pass
