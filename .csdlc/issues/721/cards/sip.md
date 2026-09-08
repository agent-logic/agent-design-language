# Structured Intent Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Fix C-SDLC v3 terminal authority reporting and add typed GitHub issue-create parity.

## Required Outcome

v3 reports operational terminal authority truthfully after cutover and can create GitHub issues through the v3 typed mutation path with durable intent, authenticated readback, and receipt binding.

## Scope

- csdlc-v3 terminal finish authority reporting
- csdlc-v3 GitHub mutation model
- csdlc-v3 GitHub adapters
- csdlc-v3 command documentation and tests

## Authority

- Do not weaken canonical selector checks or caller-authored authority rejection.
- Do not introduce raw gh issue create as a lifecycle route.
- Do not delete v2 code in this issue; v2 removal is tracked separately.

## Assumptions

- none

## Operator Constraints

- Never write on main.
- Follow the GitHub issue-create model for title/body/labels/assignees/milestone.
- No fake commands.
