# Structured Intent Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare issue-specific C-SDLC v2 planning state for later execution of the #4762 birth witnesses and receipt package.

## Required Outcome

A later execution session can acquire a live #4762 claim and produce an auditable witness/receipt package, or fail closed with an operator-approved blocker naming the missing witness, receipt, redaction, or handoff dependency.

## Scope

- Complete six issue-specific #4762 cards for preparation only.
- Preserve current v0.91.8 routing: #4762 is WP-21 under parent #5362, not WP-14A platform acceptance.
- Plan the birth witness register and receipt package without creating those execution artifacts.
- Name exact intended issue-local paths, COTS posture, budgets, PVF lanes, rollback criteria, and no-deferral criteria.
- Retain one bounded preparation review and apply preparation-scope fixes.

## Authority

- The GitHub issue body is the implementation closure authority: planning-only work cannot close #4762.
- `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml` and `docs/milestones/v0.91.8/WBS_v0.91.8.md` are the current routing authority.
- `docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md` is the current v0.92 consumption row authority for `Birth witnesses/receipt | #4762 | Auditable receipt package`.
- `docs/milestones/v0.92/IDENTITY_CONTINUITY_AND_BIRTHDAY_PLAN_v0.92.md` and related feature docs define the witness/receipt semantics consumed by later execution.
- Execution-time claim acquisition, terminal receipts, PR publication, merge, and closeout are deferred and must not block this preparation branch.

## Assumptions

- `origin/main` at `51bc5ae51b57c19dbab693af1c5a45142995f4e5` is the current integrated baseline for this preparation pass.
- The existing #4762 claim is expired; this is recorded as an execution-time gate rather than reacquired during preparation.
- No implementation source path is required unless a later execution SPP/VPP revision explicitly authorizes it.

## Operator Constraints

- Use only `/Volumes/FastWork/adl-wp-4762` on branch `codex/4762-v0918-wp14-preparation`.
- Never use `/private/tmp`.
- Preparation only: no implementation, PR, publication, merge, or closeout.
- Do not let claim reacquisition, receipts, or closeout block preparation.
- Do not start v0.92 implementation or claim birthday readiness.
