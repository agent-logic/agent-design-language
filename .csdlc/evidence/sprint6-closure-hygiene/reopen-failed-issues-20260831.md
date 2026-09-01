# Sprint 5/6 closure hygiene sweep

Date: 2026-08-31
Actor: worker-6
Repository: agent-logic/agent-design-language

## Purpose

Ensure sprint issues are closed only when successful terminal evidence is present.
Issues with failed, stale, or not-yet-accepted review evidence were reopened so
they cannot be mistaken for completed work before V3-F cutover.

## Transport

All remote reads and updates used the typed C-SDLC v2 GitHub issue route:

- `.adl/bin/csdlc-v2/csdlc-github-issue run --request ...`

No raw `gh` transport was used for this sweep.

## Live issue disposition after sweep

| Issue | Live state | Disposition |
| --- | --- | --- |
| #500 | open | Reopened because the available review evidence for the delivered V3-A work was FAIL/unproven. |
| #501 | open | Reopened because the available review evidence for the delivered V3-B work was FAIL/unproven. |
| #502 | open | Reopened because the available review evidence for the delivered V3-C work was FAIL/unproven. |
| #503 | closed | Left closed because PR #581 merged, live GitHub reports the issue closed, and terminal truth was materialized locally from current main. |
| #504 | open | Reopened because the available review evidence for the delivered V3-E work was FAIL/unproven. |
| #505 | open | Left open because it is the explicit V3-F authority-transition decision gate and must not close before review/cutover approval. |
| #570 | open | Reopened because the available review evidence for the docs/skill cutover-readiness work was FAIL/unproven. |
| #571 | open | Reopened because the available review evidence for the V3-A corrective follow-up was FAIL/unproven. |
| #596 | open | Left open because PR #597 was changed to a non-closing relation and the remediation issue remains a live tracking gate. |

## Request evidence

Typed read/update request packets are retained under:

- `.csdlc/prepared/issues/503/closure-hygiene/`

The #503 terminal materialization request is retained at:

- `.csdlc/prepared/issues/503/materialize-terminal-main.json`

## Non-claims

This sweep does not assert that reopened issues are unfixed forever. It only
asserts they must not remain closed until successful lifecycle/review evidence is
current, explicit, and accepted.
