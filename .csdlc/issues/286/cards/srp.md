# Structured Review Prompt

Template: 1.0.0

Issue: 286

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/286/adr0069-evidence-reconciliation.md
.csdlc/evidence/286/issue84-live-state.json
.csdlc/evidence/286/validate_adr0069_evidence_reconciliation.py
.csdlc/prepared/issues/286/validate_preparation_bundle.py
.csdlc/issues/286

## Prompts

- Does #286 stay limited to issue-local ADR 0069 evidence reconciliation?
- Are residual gaps explicitly allowed and truthfully separated from terminal-proving evidence?
- Does the plan avoid Runtime/UI/Unity/provider/cloud implementation and credential-bound proof?
- Are #207 and #288 boundaries preserved?
- Are validation lanes deterministic and issue-owned?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- #84 state is a timestamped retained observation; subsequent external issue-state changes require refreshing the packet.
- #117/#271/#282 validation depends on retained terminal caches and configured local owner/worktree paths, which were present and canonical during review.

## Review Result

Revision: Some("git-blake3:aea47db612ef10b76dc8b8212d4a50bd9349962f:c5f8cfee27329e8f09d718be04618b834b0d086e961db04b1cb2f47e39e3fa8b")

Reviewer: Some("fresh-session:22bfcbde-29b5-43d9-be7b-1ba533f3ebd6")

Result: pass
