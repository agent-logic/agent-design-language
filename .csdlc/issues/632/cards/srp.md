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
adl/src/cli/csmctl_cmd.rs

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
- #631/#644 stacked-publication topology remains a cutover blocker until typed retarget/supersede behavior or serial base closure is resolved.

## Review Result

Revision: Some("git-blake3:d9635ec4b63193e30321ab872e4636b5cfc19966:095512f74be8073d0a0813954a4c0d9efa74f350cce5720a26e954c993f0987c")

Reviewer: Some("subagent:/root/review_632_head_after_recover")

Result: pass
