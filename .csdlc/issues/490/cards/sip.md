# Structured Intent Prompt

Template: 1.0.0

Issue: 490

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one accepted GCP hierarchy and cost-envelope decision.

## Required Outcome

A reviewed decision register binding organization, folders, projects, billing, region, quota, naming, data residency, and first workload cost ceiling without mutating GCP.

## Scope

- docs/operations/cloud/gcp/decisions/**
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/**
- .csdlc/prepared/issues/490/**
- .csdlc/evidence/490/**

## Authority

- Issue #490 owns only read-only GCP hierarchy and cost decision truth.
- Issue #491 owns GCP Terraform bootstrap after #490.
- No GCP mutation, API enablement, project creation, or paid launch is authorized by #490.

## Assumptions

- none

## Operator Constraints

- Use the company account via gcloud auth login context; do not require a static key.
- Do not print, copy, commit, or expose GCP credential material.
- Use only read-only GCP discovery commands.
- Keep work in a bound FastWork issue worktree before tracked implementation edits.
- Obtain fresh exact-head review before publication.
