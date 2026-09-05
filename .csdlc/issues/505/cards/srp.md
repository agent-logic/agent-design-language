# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
adl/tools/test_install_adl_pr_cycle_skill.sh
csdlc-v3
docs

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

Revision: Some("git-blake3:69612ff17f9aad93449e4a35bac94eef62fcc1d0:960c95088d0605dd077e322ca080364a5ac2295f925c9f6a326a186a33749a7e")

Reviewer: Some("subagent:/root/review_591_69612ff17_terminal_sprint_fix")

Result: pass
