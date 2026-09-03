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
docs/csdlc-v3/full-replacement-denominator.json
docs/csdlc-v3/v3-command-manifest.json
csdlc-v3/tests/real_issue_canary.rs

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
- #631/#669 remains a dependency gate until its current hosted checks and review decision settle.
- Terminal finish and cleanup canary proof still requires an authorized canary merge or explicit #505 blocker disposition.

## Review Result

Revision: Some("git-blake3:2bddd359d8141899efc01e0088c3a33006c50504:2bff921f0a575563754d1cf297c23c26dd06e260b19c1a8440c6aecef41d623c")

Reviewer: Some("subagent:/root/review_632_head_2bddd")

Result: pass
