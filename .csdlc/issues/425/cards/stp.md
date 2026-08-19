# Structured Task Prompt

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only #425 C-SDLC v2 tooling recovery/classification for recordless already-merged closeout authority and focused tests; do not perform unrelated closeout/manual card synthesis.

## Deliverables

- csdlc-v2/src/finish.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/tests/gate_recordless_closeout.rs
- .csdlc/issues/425
- .csdlc/prepared/issues/425

## Acceptance

1. AC-1: Typed request requires issue, repository, merged PR, expected head SHA, merge SHA, actor, reason, token source, and recovery mode classify_only or recordless_terminal.
2. AC-2: Route proves live GitHub issue is closed by exact merged PR and PR head/merge/linkage/repository match the request.
3. AC-3: Route refuses open issue, open/unmerged PR, wrong repository, wrong issue linkage, wrong head SHA, wrong merge SHA, ambiguous closing PRs, missing token source, and contradictory retained evidence.
4. AC-4: Route does not mutate product files, edit GitHub state, rewrite historical issue/card records, or claim absent review/implementation evidence.
5. AC-5: #248-style contradictory precedence is classified fail-closed with no merged receipt.
6. AC-6: No-projection residuals produce safe recordless receipts or machine-readable fail-closed blockers.
7. AC-7: Positive tests cover recordless closed-by-merged issue with exact PR evidence and no issue index at PR head.
8. AC-8: Negative tests cover contradictory publication precedence, missing closing keyword, wrong head SHA, wrong merge SHA, wrong repository, unmerged PR, and ambiguous closure.
9. AC-9: v0.92 closeout sweep can rerun and leave either 92/92 receipts or only irreducible contradictory blockers.

## Dependencies

- #425 live GitHub issue body updated to current nine-residual truth
- retained blocker packet .git/csdlc-v2/closeout-blockers/v092-closed-merged-no-projection-residuals-20260818T2315Z.json
- current csdlc-v2 typed finish/clean/publication code on main

## Inputs

- agent-logic/agent-design-language#425
- csdlc-v2/src/finish.rs
- csdlc-v2/src/cleanup.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/tests
- retained v0.92 closeout blocker packets

## Non Goals

- Product Runtime, Observatory, provider, Unity, AWS, or documentation behavior changes
- Raw GitHub writes
- Manual lifecycle/card synthesis
- Weakening ordinary active issue publication requirements
- Resolving #248 precedence by assumption
