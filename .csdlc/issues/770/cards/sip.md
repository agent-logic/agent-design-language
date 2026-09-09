# Structured Intent Prompt

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Guarantee independent SSH recovery for the public Spot Runtime module.

## Required Outcome

Plan-time key and SSH-CIDR guards, recoverable example, negative Terraform tests, and explicitly approved bounded live proof with disposal.

## Scope

- infra/aws/modules/csm-runtime-spot
- infra/aws/csm-runtime-spot
- infra/aws/csm-runtime-spot/tests/run_contract.sh
- infra/aws/csm-runtime-spot/tests/run_live_proof.sh

## Authority

- Operator authorized typed v2 exception after native card-validation failure.
- Cloud apply requires separate explicit approval of concrete plan.

## Assumptions

- none

## Operator Constraints

- Use approved Agent Logic business AWS profile.
- Use one existing operator-approved key; never expose private key contents.
