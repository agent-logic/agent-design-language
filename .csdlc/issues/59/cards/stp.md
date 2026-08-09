# Structured Task Prompt

Template: 1.0.0

Issue: 59

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Classify code authority and produce a truthful reviewed routing result; implement only if an actual repository-owned goal replacement seam exists.

## Deliverables

- .csdlc/prepared/issues/59/design.md source-grounded authority decision
- .csdlc/prepared/issues/59/diagram.mmd ownership-boundary diagram
- .csdlc/issues/59 typed lifecycle packet with explicit estimates
- .csdlc/issues/59/cards/stp.md upstream product contract
- .csdlc/issues/59/cards/srp.md retained readiness review truth

## Acceptance

1. AC-1: Repository evidence distinguishes ADL policy and telemetry consumers from the implementation that admits or rejects create_goal
2. AC-2: The routing result names the OpenAI Codex goal-tool service as external mutation authority unless a concrete repo-owned seam is found
3. AC-3: The historical blocked goal remains blocked with its status and accounting preserved
4. AC-4: The exact upstream contract covers explicit replacement or supersession, history retention, accounting retention, and fail-closed active-goal overwrite
5. AC-5: No ADL code, policy workaround, local goal store, or implementation PR is introduced without repository code authority
6. AC-6: Design, diagram, six typed cards, deterministic validation, and independent readiness review are current and truthful

## Dependencies

- agent-logic/agent-design-language#59
- Codex platform create_goal, get_goal, and update_goal contracts
- An upstream Codex product owner able to change thread-goal admission and persistence

## Inputs

- AGENTS.md
- adl/tools/skills/pr-run/SKILL.md
- adl/tools/skills/sprint-conductor/SKILL.md
- docs/milestones/v0.91.6/features/FIRST_CLASS_NESTED_GOAL_ACCOUNTING_v0.91.6.md
- Repository-wide source search for create_goal, update_goal, get_goal, and the exact failure message

## Non Goals

- Implementing a second goal database in ADL
- Mutating Codex session files
- Falsely completing the blocked goal
- Removing the issue-bound session-goal requirement
- Nested sprint-goal accounting
- Undocumented reverse engineering of Codex internals
