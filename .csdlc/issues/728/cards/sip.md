# Structured Intent Prompt

Template: 1.0.0

Issue: 728

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Run one bounded non-production AWS-F disposable Runtime deployment proof, prove an external request reaches the exact private target through the approved ALB route, then destroy all issue-owned disposable resources and retain zero-residue evidence.

## Required Outcome

A disposable private Runtime node and ALB-origin proof run in the Agent Logic business AWS account demonstrates target health and an external health receipt tied to the exact instance, then tears down every issue-owned disposable resource with exact absence readbacks.

## Scope

- AWS-F disposable ALB-origin and private Runtime node live proof
- agent-logic-admin account and us-west-2 preflight
- saved Terraform plan identity and separate backend/workspace evidence
- external request receipt tied to the exact target
- reverse destroy and zero-residue readbacks
- issue-local lifecycle, evidence, runbook, and narrowly required disposable-proof safety fixes

## Authority

- Public edge authority remains #122-owned
- Terminal #489 and #579 truth is consumed read-only
- Release-tail admission #516 is updated only through its owner
- No production traffic or cutover
- No Route53 or ACM issuance ownership
- No GPU qualification or long-running Runtime operation

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle
- Use a dedicated FastWork worktree after readiness/bind
- Use AWS profile agent-logic-admin and verify business account identity before relying on AWS state
- Do not expose credentials or sensitive values in evidence
- Do not apply live Terraform until an explicit operator authorization packet names the exact mutable envelope
- Destroy issue-owned disposable resources in reverse order before claiming completion
