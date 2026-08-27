# Structured Intent Prompt

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one accepted AWS access and billing control baseline for the approved Agent Logic business AWS account.

## Required Outcome

A reviewed access and billing baseline with independently recoverable corporate administration, governed Agent Toolkit access, read-only default agent posture, CloudWatch and CloudTrail attribution, visible billing ownership, budgets, anomaly handling, export, and cost attribution without removing the proven administrator path.

## Scope

- infra/aws/account-foundation/**
- docs/operations/cloud/aws/access-billing/**
- docs/milestones/v0.92.1/evidence/cloud/aws-b/**
- .csdlc/prepared/issues/485/**
- .csdlc/evidence/485/**

## Authority

- Issue #485 owns only the AWS access and billing baseline.
- Issue #484 owns the accepted AWS resource ownership inventory predecessor.
- Issue #486 owns AWS Terraform bootstrap after this baseline is accepted.
- Issue #122 continues to own public WSS, Route53, ACM, and public exposure topology.
- No AWS Organizations rollout, workload deployment, administrator removal, Terraform apply, resource cleanup, unrestricted agent mutation, or production traffic change is authorized by #485.
- AWS mutation is not performed unless a typed issue lane explicitly records the exact approved operation and operator approval boundary.

## Assumptions

- none

## Operator Constraints

- Use the Agent Logic business AWS profile agent-logic-admin, not a personal/default profile.
- Do not print, copy, commit, or expose AWS credential material.
- Default AWS evidence collection to read-only commands.
- Preserve existing administrator access until replacement is independently proven.
- Keep work in the existing FastWork issue worktree and preserve #122, #554, and #490 lanes.
- Obtain fresh exact-head review before publication.
