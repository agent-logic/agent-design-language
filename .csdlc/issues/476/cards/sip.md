# Structured Intent Prompt

Template: 1.0.0

Issue: 476

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Integrate the three preserved post-merge review-truth repairs from ed454a246 without changing runtime behavior.

## Required Outcome

A reviewed, green, merged follow-on PR closing #476, followed by truthful #315 terminal reconciliation.

## Scope

- Typed #315 SPP and VPP truth repair
- WP-27 remediation validator claim correction
- WP-27 remediation README payload correction

## Authority

- Typed review and finish own exact-head and terminal truth
- No runtime behavior changes
- Do not inspect or execute #269

## Assumptions

- none

## Operator Constraints

- Use a bound FastWork worktree
- Apply only ed454a246
- Merge only after green CI and fresh exact-head review
- Do not inspect or execute #269
