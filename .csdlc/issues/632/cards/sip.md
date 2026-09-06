# Structured Intent Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove the v3 replacement workflow with real issue canaries and update operator-facing readiness guidance before #505 cutover.

## Required Outcome

A reviewable canary/readiness packet maps every v3 command-equivalent route to real issue proof, deterministic fixture proof, or an explicit cutover-blocking finding, and docs/skills/AGENTS guidance truthfully describe the pre-cutover and post-cutover authority boundary.

## Scope

- Real issue canary coverage for v3 issue/init through PR publication
- Terminal finish and cleanup canary planning with proof after an authorized merge
- Defect capture and disposition for every v3 function failure discovered
- Docs, skills, AGENTS, onboarding, and changeover notice updates
- Sprint Execution Packet and sprint-end evidence index updates for #625

## Authority

- v2 remains live lifecycle authority until explicit #505 cutover
- v3 is construction and canary evidence only before #505
- No raw gh lifecycle writes
- No hidden v2 operational fallback in v3 proof
- Do not merge or close #505 from this issue

## Assumptions

- none

## Operator Constraints

- Use typed v2 for live GitHub publication, finish, cleanup, and issue comments before #505
- Keep scratch request artifacts under .git/csdlc-v2/requests
- Do not use /private/tmp
- Do not touch green sibling PRs unless a fresh gate requires it
