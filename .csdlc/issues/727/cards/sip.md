# Structured Intent Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Apply and prove the merged AWS account audit and security foundation in the approved business account.

## Required Outcome

An exact reviewed saved plan is authorized, applied, and followed by redacted readbacks proving every declared control.

## Scope

- infra/aws/account-foundation/**
- docs/operations/cloud/aws/audit-security/**
- docs/milestones/v0.92.1/evidence/cloud/aws-d/**
- issue-local C-SDLC v2 records

## Authority

- typed C-SDLC v2 lifecycle
- agent-logic-admin business account only
- us-west-2
- operator authorization required before apply

## Assumptions

- none

## Operator Constraints

- never expose account ids, ARNs, or credentials
- saved-plan digest must match apply
- no website Runtime DDNS or unrelated resources
