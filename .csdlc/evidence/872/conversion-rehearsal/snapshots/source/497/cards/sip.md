# Structured Intent Prompt

Template: 1.0.0

Issue: 497

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare and execute the corporate operational-control transfer acceptance lane for v0.92.1 Sprint 4 after CORP-A, CORP-B, AWS-G, and GCP-D have landed.

## Required Outcome

A truthful corporate operational-control acceptance packet exists, distinguishes completed evidence from deferred/operator-only actions, and does not mutate production providers, billing, credentials, or legal/private records without explicit authorization.

## Scope

- docs/operations/corporate/control-transfer/**
- docs/milestones/v0.92.1/evidence/corporate/corp-c/**
- infra/** control-transfer references only when explicitly required by issue acceptance
- .github/workflows/** control-transfer references only when explicitly required by issue acceptance
- .csdlc/issues/497/**
- .csdlc/prepared/issues/497/**

## Authority

- Issue #497 may prepare corporate control-transfer acceptance evidence and issue-owned repository artifacts.
- Issue #497 does not authorize production/provider mutation, credential transfer, billing changes, GitHub lifecycle writes outside typed v2, or private legal/diligence disclosure.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 authority only.
- Do not work on main for tracked implementation.
- Use the Agent Logic business AWS profile for ADL AWS checks and verify account identity before relying on AWS state.
- Do not print, copy, commit, or expose credential material.
- Stop before external provider mutation unless the operator explicitly authorizes the exact action.
