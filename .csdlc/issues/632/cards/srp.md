# Structured Review Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/632/command-route-coverage.json
.csdlc/prepared/issues/632/canary-evidence-index.md
docs/csdlc-v3/CUTOVER_READINESS_NOTICE.md

## Prompts

- Does the coverage matrix account for every v3 command-equivalent route without hidden v2 fallback?
- Do docs and skills warn operators before cutover while avoiding premature v3 authority claims?
- Are canary defects fixed or clearly owned as cutover blockers/follow-ons?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains construction and canary evidence only until explicit V3-F/#505 cutover.
- #505 must consume current #629 through #632 dependency evidence through its own exact-head review, publication, operator approval, and terminal reconciliation before cutover.
- Terminal finish and cleanup canary proof still requires an authorized canary merge or explicit #505 blocker disposition.

## Review Result

Revision: Some("git-blake3:d4c637d56a7359832ab68c76f1761a27bac843d5:727cf625893d9f70263502b4b24ccff395cc1128b86858f0dece784e869912d0")

Reviewer: Some("subagent:/root/review_632_docs_d4c637")

Result: pass
