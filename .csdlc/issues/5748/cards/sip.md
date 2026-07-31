# Structured Intent Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Recover truthful typed terminal state for every closed v0.91.8 issue excluded from the clean #5746 projection wave.

## Required Outcome

Every inventoried issue has remote disposition, retained receipt, tracked closed_out projection, released claim, and retained artifacts in exact agreement.

## Scope

- typed receipt recovery in issue-local worktrees
- receipt-backed terminal projection materialization in the dedicated #5748 worktree
- special disposition and retained-artifact repairs explicitly named by issue #5748
- issue-local evidence under .csdlc/evidence/5748

## Authority

- Issue-local worktrees own receipt creation and repair; #5748 materializes only validated retained authority.
- Generated cards, index records, and terminal receipts change only through typed C-SDLC v2 operations.
- Do not touch #5746 or write tracked changes on main.

## Assumptions

- none

## Operator Constraints

- never write tracked changes on main
- never touch #5746
- use typed C-SDLC v2 only
- preserve dirty worktrees
- no AWS or raw GitHub commands
