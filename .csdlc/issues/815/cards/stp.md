---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.10"
slug: "cloud-authorization-authenticity"
title: "[v0.92.1][TAIL-06.10][security] Authenticate AWS and GCP mutation authorization"
labels:
  - "track:roadmap"
issue_number: 815
generated_at: "2026-09-09T20:50:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "security_control_and_local_negative_proof"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/815"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522 resolving #520 findings D520-SEC-001 and D520-SEC-002."
pr_start:
  enabled: true
  slug: "cloud-authorization-authenticity"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-09T20:50:00Z

# Structured Task Prompt

## Summary

Replace self-asserted cloud approval with externally anchored operator signatures and exact Terraform plan-byte identity.

## Goal

Make both cloud mutation gates independently authentic and resistant to packet, plan, sidecar, expiry, and account/project replay tampering.

## Required Outcome

All mutation entrypoints fail before provider calls unless authentic operator approval covers the exact repository, issue, provider identity, bounded resources, expiry, and saved plan bytes.

## Deliverables

- Shared detached-signature verifier
- Hardened GCP authorization validator and callers
- Hardened AWS plan-byte/account validator
- Updated templates/runbooks
- Full negative fixture matrix

## Acceptance Criteria

- No self-authored approval can pass.
- Actual tfplan bytes and derived projection are bound.
- Cross-account/project replay and expiry fail closed.
- Read-only paths require no mutation authority.
- No paid cloud mutation occurs.

## Repo Inputs

- #815 issue contract
- D520-SEC-001/002 evidence
- #727/#731 validators, scripts, templates, and plan fixtures

## Dependencies

- Parent #522 is open.
- Current v0.92.1 main is the exact implementation base.
- ssh-keygen and Python standard library are local proof dependencies.

## Target Files / Surfaces

- .csdlc/prepared/issues/727/**
- .csdlc/prepared/issues/731/**
- .csdlc/prepared/issues/815/**
- tightly coupled cloud authorization docs

## Validation Plan

Deterministic local negative fixtures plus syntax/diff checks and exact-head review; no live provider mutation.

## Demo Expectations

Not required; local negative proof is authoritative for this repair.

## Non-goals

- Provider execution
- Credential storage redesign
- General-purpose PKI
- Terraform architecture changes

## Issue-Graph Notes

- Parent remediation #522.
- Source review #520 at fb6cbc7f619daa54f901fd2d12f480add682ace3.
- No cloud mutation is authorized.

## Notes

The test signer must be ephemeral and explicitly test-only; production validation must use the external trusted signer file.

## Tooling Notes

Use native C-SDLC v3 and keep all generated proof inside the bound worktree.
