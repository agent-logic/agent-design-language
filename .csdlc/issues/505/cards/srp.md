# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/commands/sprint.rs
.csdlc/prepared/issues/505/v3-sprint-readiness
docs/milestones/v0.92.1/evidence/csdlc-v3/v3-f

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

- C-SDLC v3 remains construction and cutover-readiness evidence only until explicit V3-F/#505 operator approval, merge, finish, and cleanup reconciliation.
- The v3 sprint readiness route is read-only and non-authoritative before cutover; typed v2 remains the only operational lifecycle/GitHub authority.
- V3-H children #629, #630, #631, and #632 remained open in the retained live readbacks, so the sprint readiness report proves the denominator is ready for execution planning rather than complete cutover.

## Review Result

Revision: Some("git-blake3:15cf11215b29959379ead339b1d3a4aabfcf01f4:12a0ed21c52f0029801429f7a508386692345fb933d02ec1184674a0bd7587c7")

Reviewer: Some("subagent:/root/review_505_full_current_diff_15cf1121")

Result: pass
