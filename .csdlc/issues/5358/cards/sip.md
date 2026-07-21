# Structured Intent Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the exact typed acceptance contract for proving C-SDLC v2 as the deployed lifecycle control plane without executing acceptance in this session.

## Required Outcome

All six issue-specific cards, retained design, declared evidence and defect inputs, validation lanes, non-claims, and stop conditions are ready for a later exact-revision acceptance execution decision.

## Scope

- .csdlc/issues/5358
- .csdlc/prepared/issues/5358
- .csdlc/evidence/5358
- C-SDLC v2 acceptance criteria and proof topology
- Read-only inventory of #5540, #5541, #5548, and #5558

## Authority

- #5358 owns only its acceptance contract and future acceptance synthesis
- #5540 and #5541 remain closed evidence owners
- #5548 independently owns the Gate 2 non-Git fixture repair
- #5558 independently owns final typed-v2 owner-guidance repair
- This preparation does not authorize acceptance execution, deployment, publication, merge, or closeout

## Assumptions

- none

## Operator Constraints

- Use only the installed typed C-SDLC v2 binaries and repository owner tools; do not use raw gh or AWS
- Work only on the existing bound #5358 branch and limit tracked changes to .csdlc/issues/5358, .csdlc/prepared/issues/5358, and .csdlc/evidence/5358
- Preserve every STP acceptance criterion and complete bidirectional SPP and VPP coverage; later acceptance execution must implement and prove every criterion without deferral, omission, or weakening
- This preparation may repair issue-local lifecycle truth but must not perform product implementation, deployment, publication, merge, closeout, or edits to independently owned issue surfaces
