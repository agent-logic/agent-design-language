# Structured Task Prompt

Template: 1.0.0

Issue: 541

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded docs-truth repair for onboarding and nearby workflow guidance.

## Deliverables

- Updated onboarding workflow guidance
- Corrected repository identity wording
- Focused validation evidence for retired-route removal and current-authority presence
- Truthful SOR describing actual docs changes and validation

## Acceptance

1. AC-1: `docs/onboarding.md` no longer presents `adl_pr_cycle`, `pr ready`, or `pr run` as the current default lifecycle route.
2. AC-2: `docs/onboarding.md` names Gate 10D2 typed C-SDLC v2 as current authority and points current lifecycle work at `.adl/bin/csdlc-v2/` plus `csdlc-v2/operator/skills/`.
3. AC-3: Documentation distinguishes canonical `agent-logic/agent-design-language` from legacy `danielbaustin/agent-design-language` and explains the latter is historical provenance unless a bounded legacy task explicitly says otherwise.
4. AC-4: Startup/root checkout guidance remains aligned with `AGENTS.md`: root should stay on clean `main`, issue work happens in a bound worktree, and new ADL issue worktrees belong under `/Volumes/FastWork/adl-worktrees`.
5. AC-5: Docs avoid claiming that an initialized issue, green CI, or publication alone proves terminal closeout; they preserve review, publication, finish, and cleanup boundaries.
6. AC-6: Focused validation checks edited docs for retired route strings and confirms current typed v2 authority references are present.

## Dependencies

- AGENTS.md Gate 10D2 authority
- csdlc-v2/AGENTS.md typed owner contract
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md

## Inputs

- agent-logic/agent-design-language#541
- docs/onboarding.md
- AGENTS.md
- csdlc-v2/AGENTS.md
- csdlc-v2/operator/SKILLS.md
- docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md
- adl/tools/README.md

## Non Goals

- Implementing the documentation fix during bootstrap
- Binding a worktree during bootstrap
- Changing runtime behavior
- Changing lifecycle binaries or skill contracts
- Changing GitHub remotes, labels, milestones, PR state, or release state
- Reviving v1 wrappers or the compatibility `adl_pr_cycle` route
