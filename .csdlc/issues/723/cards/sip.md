# Structured Intent Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Restore the complete C-SDLC v3 proof, shadow, and install integration suite.

## Required Outcome

All csdlc-v3 all-target tests pass with typed output, fail-closed mismatch detection, and intentional install authority behavior.

## Scope

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- docs/milestones/v0.92.1/evidence/csdlc-v3/issue-723

## Authority

- v2 remains live authority
- do not weaken proof parity or #505 install gate
- no v2 removal

## Assumptions

- none

## Operator Constraints

- root main inspection only
- bounded issue 723 fixes
- deterministic fixtures must not dirty worktrees
