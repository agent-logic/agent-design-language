# Structured Intent Prompt

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Qualify the private Wuji-AWS recovery lane for WP-04/#142 with fail-closed AWS preflight, private networking, local model proof, cleanup, and truthful redacted receipts.

## Required Outcome

A bounded private qualification harness for one Wuji voter plus two AWS voters in separate AZs, with private local models, SSM maintenance, direct private agent/voter TCP adjacency, cleanup, redacted receipts, exact review, and publication-ready PR truth.

## Scope

- Issue-owned CloudFormation private AWS runner, runbook, cleanup/janitor contract, and focused harness tests
- Agent Logic business-account authority and private AWS topology validation
- Private subnets, SSM endpoints, S3 gateway endpoint, direct private voter mesh, and no public Runtime/model exposure
- Redacted evidence for private network, private model artifact delivery, local model health/restart, and zero cleanup
- Remaining serial hybrid recovery proof with Wuji partition/AWS continuity/heal/demotion/one-of-three halt before completion credit

## Authority

- AWS work must use profile agent-logic-admin and must not expose credentials or raw account identifiers in retained publishable evidence
- SSM is shepherd maintenance/recovery plane only; agent/voter peer traffic must use direct private TCP/IP suitable for ACIP routing
- Hosted/cloud model fallback is forbidden in the proof path
- Raw AWS/SSM IDs remain local-only; retained publication evidence must be redacted

## Assumptions

- none

## Operator Constraints

- Use the #194 FastWork worktree, not main
- Do not use /private/tmp for local artifacts
- Keep AWS machines running only for bounded live proof windows and verify zero after every run
- Use CloudFormation for the current private IaC path
- Do not claim production public stack completion
