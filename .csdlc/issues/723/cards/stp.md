# Structured Task Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Proof shadow install test restoration only.

## Deliverables

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/tests/proof_parity_install_commands.rs

## Acceptance

1. AC-1 all-target v3 suite green
2. AC-2 shadow emits or normalizes exactly one typed JSON document
3. AC-3 install blocked/nonzero behavior is intentional and gated
4. AC-4 generated fixtures are deterministic and clean

## Dependencies

- #505 authority contract
- unmerged #721 bounded proof/install patch as source evidence

## Inputs

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/proof_parity_install_commands.rs

## Non Goals

- weaken mismatch detection
- remove v2
- absorb unrelated #721 changes
