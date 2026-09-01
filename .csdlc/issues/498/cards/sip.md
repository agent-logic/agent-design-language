# Structured Intent Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the corporate diligence acceptance lane for Sprint 4 so it can execute after CORP-C lands.

## Required Outcome

A truthful corporate diligence acceptance packet records diligence disposition, blockers, residual risks, private-evidence boundaries, and validation evidence without exposing private legal or diligence material.

## Scope

- docs/operations/corporate/diligence/**
- docs/milestones/v0.92.1/evidence/corporate/corp-d/**
- .csdlc/issues/498/**
- .csdlc/prepared/issues/498/**

## Authority

- Issue #498 may prepare corporate diligence acceptance evidence and repository-safe diligence summaries.
- Issue #498 does not authorize disclosure of private legal advice, private diligence material, credentials, tokens, or account secrets.
- Issue #498 must not execute until CORP-C is closed, merged, and ancestral.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 authority only.
- Do not work on main for tracked implementation.
- Do not commit private legal advice, private diligence material, credentials, tokens, or account secrets.
- Stop if a blocker lacks a disposition or if CORP-C is not terminal.
