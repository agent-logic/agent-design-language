# Structured Intent Prompt

Template: 1.0.0

Issue: 484

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one accepted all-region AWS resource ownership inventory for the approved Agent Logic business AWS account.

## Required Outcome

A reviewed inventory that records the exact approved account and inspected regions, classifies every discovered AWS resource with an owner or frozen-unknown disposition, separates website Terraform and issue evidence ownership, and does not mutate AWS resources.

## Scope

- docs/operations/cloud/aws/inventory/**
- docs/milestones/v0.92.1/evidence/cloud/aws-a/**
- .csdlc/prepared/issues/484/**
- .csdlc/evidence/484/**

## Authority

- Issue #484 owns only read-only AWS resource discovery and ownership inventory truth.
- Issue #485 owns AWS access and billing baseline after this inventory is accepted.
- Issue #486 owns AWS Terraform bootstrap after #485.
- No AWS resource mutation, import, deletion, cleanup, or Terraform apply is authorized by #484.

## Assumptions

- none

## Operator Constraints

- Use the Agent Logic business AWS profile, not a personal/default profile.
- Do not print, copy, commit, or expose AWS credential material.
- Use only read-only AWS discovery commands.
- Keep work in a bound FastWork issue worktree before tracked implementation edits.
- Obtain fresh exact-head review before publication.
