# Structured Review Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

AGENTS.md
csdlc-v2/AGENTS.md
csdlc-v2/operator/SKILLS.md
csdlc-v2/operator/skills
csdlc-v2/tests/projection_recovery_integration.rs
csdlc-v3/AGENTS.md
csdlc-v3/README.md
docs/csdlc-v3
docs/default_workflow.md
docs/onboarding.md
docs/architecture/ADL_ARCHITECTURE.md
docs/tooling/adl_pr_cycle_skill.md
docs/tooling/card-lifecycle.md
docs/tooling/structured-prompt-contracts.md
docs/tooling/editor
docs/templates

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

- This PASS covers the #505 preparation gate only; it does not approve cutover, merge, finish, cleanup, v2 retirement, or terminal #505 closeout.
- Hosted checks were still pending during review and must settle before merge readiness is claimed.

## Review Result

Revision: Some("git-blake3:a6749932e540a8adb7b2ccf29ca3bc808a24dec1:e3c056beadf1bbf578b73d95014656791809f6e9436b658e4eabd3ade94e24e8")

Reviewer: Some("review_591_head_a6749932e")

Result: pass
