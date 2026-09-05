# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/505/publish-after-review-279342fea-pass.json
.csdlc/prepared/issues/505/update-pr591-after-defer-brief-reconciliation.json
.csdlc/prepared/issues/505/validate-authority-transition-prep.rb

## Prompts

- Verify #505 remains pre-bind preparation only until #504 is terminal, reconciled, and ancestral.
- Verify the packet preserves C-SDLC v2 live authority and rejects silent v2 retirement before explicit operator approval.
- Verify requirements #179 and #180 are named in the acceptance denominator and future proof plan.
- Verify the future PR body requirement visibly uses `Closes #505`.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Actual rollback-exercise evidence remains required before operator cutover approval.
- Terminal v3 finish/cleanup canary evidence remains required before operator cutover approval unless the operator records an explicit bounded waiver.
- C-SDLC v3 remains non-authoritative until explicit #505 operator approval, merge, typed finish, and cleanup reconciliation.

## Review Result

Revision: Some("git-blake3:f5621b164ef62c18885b5b57c4eff58a901fbbb2:3f678f7edd989c14b91c34fb740e491d4f7e3f1dcae851293f3bba9193861d66")

Reviewer: Some("subagent:/root/review_591_part_of_publication_fix")

Result: pass
