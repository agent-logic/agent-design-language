# Structured Task Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Preparation only: complete the #4762 six-card planning package, source-grounded design/diagram, exact dependency/path/budget/PVF criteria, and one bounded preparation review for later birth-witness and receipt-package execution.

## Deliverables

- Complete SIP, STP, SPP, VPP, SRP, and SOR preparation cards under `.csdlc/issues/4762/cards/`.
- Source-grounded preparation design at `.csdlc/prepared/issues/4762/design.md`.
- Mermaid diagram at `.csdlc/prepared/issues/4762/diagram.mmd`.
- Preparation validation evidence under `.csdlc/evidence/4762/preparation-validation/`.
- Bounded `openai:gpt-5.5` preparation review evidence under `.csdlc/evidence/4762/gpt-5.5-review/`, with preparation-scope fixes applied or explicitly recorded.
- Clean commit and push of the reviewed preparation branch.

## Acceptance

1. AC1: All six #4762 cards are issue-specific and state preparation-only truth.
2. AC2: The design and diagram name exact current source dependencies and distinguish execution-time witness/receipt creation from this preparation branch.
3. AC3: Intended later execution paths are exact, issue-local, and milestone-consumable.
4. AC4: COTS/external-service posture, LoC/time/token budgets, PVF lanes, rollback criteria, and no-deferral criteria are explicit.
5. AC5: Claim reacquisition, terminal receipts, PR publication, merge, and closeout are deferred to execution/finish and do not block preparation.
6. AC6: One bounded preparation review is retained and all actionable preparation-scope findings are fixed or recorded as blockers.
7. AC7: Local preparation validation records diff hygiene, card-surface integrity, and the expected `claim_not_live` doctor blocker without reacquiring the claim.

## Dependencies

- `origin/main` at `51bc5ae51b57c19dbab693af1c5a45142995f4e5`, integrated into this branch by merge commit `def3d8c34d5f98ff53f3d6ddd2d09c55a1ffa187`.
- GitHub issue `#4762` body and routing correction comment.
- Parent WP-21 issue `#5362`.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`.
- `docs/milestones/v0.91.8/WBS_v0.91.8.md`.
- `docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md`.
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md`.
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md`.
- `docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md`.
- `docs/milestones/v0.92/features/MEMORY_GROUNDING_CAPABILITY_AND_WITNESSES_v0.92.md`.
- `docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`.
- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`.

## Inputs

- `.csdlc/prepared/issues/4762/design.md`
- `.csdlc/prepared/issues/4762/diagram.mmd`
- `.csdlc/issues/4762/cards/*.md`
- `.csdlc/issues/4762/cards/*.values.json`
- `.csdlc/issues/4762/index.json`

## Non Goals

- Do not implement the birth witness register or receipt package in this branch.
- Do not add production code, validators, scripts, schemas, dependencies, package-locks, or runtime surfaces.
- Do not reacquire the expired #4762 claim.
- Do not publish a PR, merge, close an issue, or perform terminal closeout.
- Do not claim v0.92 activation, birthday readiness, public-launch readiness, legal personhood, production citizenship, or v0.93 governance completion.
