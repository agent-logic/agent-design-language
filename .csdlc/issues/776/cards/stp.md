---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.17"
slug: "versioned-template-structure-schemas"
title: "[v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates"
labels:
  - "track:roadmap"
issue_number: 776
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "defect_fix"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/776"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "Blocks #758; remediation lineage #520 and #522."
pr_start:
  enabled: true
  slug: "versioned-template-structure-schemas"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

Correct the one-directory-too-high structure-schema lookup in native C-SDLC v3.

## Goal

Make versioned templates resolve schemas from their own versioned template directory.

## Required Outcome

The current 1.0.4 SIP, STP, SPP, VPP, SRP, and SOR renders all pass structure validation.

## Deliverables

One narrow production fix; one focused all-six-card regression; focused local-command and diff-hygiene proof.

## Acceptance Criteria

Resolve schemas relative to each template parent; cover all six current cards; retain existing focused tests; unblock #758 without bypass.

## Repo Inputs

csdlc-v3/src/commands/local/mod.rs; csdlc-v3/tests/local_commands.rs; docs/templates/prompts/current.json and referenced 1.0.4 artifacts.

## Dependencies

Current native v3 authority; #758 remains blocked until this tested commit is available.

## Target Files / Surfaces

csdlc-v3/src/commands/local/mod.rs and csdlc-v3/tests/local_commands.rs.

## Validation Plan

Exact all-six-card regression; full local_commands target; git diff --check.

## Demo Expectations

Show the real current registry validates all six card structures.

## Non-goals

No template/schema edits, registry redesign, validation weakening, or Runtime changes.

## Issue-Graph Notes

Narrow tooling defect discovered while binding #758; parent remediation ledger #522.

## Notes

A synthetic-only fixture would not prove the real registry path; test current.json directly.

## Tooling Notes

Use native v3 for edit, validation, review, publish, and shepherding; do not merge without operator authorization.
