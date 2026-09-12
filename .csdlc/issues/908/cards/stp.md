---
issue_card_schema: adl.issue.v1
wp: "Sprint 8"
slug: "908-aws-inventory"
title: "[v0.92.2][OPS-AWS] Produce one current AWS inventory packet from the #484 baseline"
labels:
  - "track:roadmap"
issue_number: 908
generated_at: "2026-09-12T00:09:29.974020+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "evidence"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/908"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Sprint umbrella #934; independent of #720/#909, no deployment in #908."
pr_start:
  enabled: true
  slug: "908-aws-inventory"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:09:29.974020+00:00

# Structured Task Prompt

## Summary

Produce a current sanitized AWS ownership inventory delta against #484 and a complete maintenance record.

## Goal

Current sanitized read-only business AWS inventory delta against immutable #484, including SCR, S3, model artifacts and maintenance record.

## Required Outcome

Current sanitized read-only business AWS inventory delta against immutable #484, including SCR, S3, model artifacts and maintenance record.

## Deliverables

Dated sanitized readbacks, baseline delta, maintenance runbook and negative validation evidence.

## Acceptance Criteria

Business account verified before reads; all historical scoped surfaces and current enabled regions represented; per-resource delta and explicit failures/unknown ownership; maintenance and negative completeness/redaction/staleness proof; independent evidence review.

## Repo Inputs

Historical #484 inventory, readbacks and scripts; current #908 issue and Sprint 8 handoff.

## Dependencies

#864 accepted merged output at f1c4e2a915c215797f0d2708cb8b0568f2b80b32; all 69 creation identities and 11 reviews passed.

## Target Files / Surfaces

.csdlc/evidence/908/; docs/operations/cloud/aws/inventory/current index only; .csdlc/issues/908/cards/

## Validation Plan

python3 .csdlc/evidence/908/inventory.py capture; python3 .csdlc/evidence/908/inventory.py validate; python3 .csdlc/evidence/908/test_inventory.py; independent review of sanitized actual readbacks and delta

## Demo Expectations

Read-only AWS inventory plus deterministic completeness/redaction negatives

## Non-goals

Cloud mutation, migration, deletion, deployment, GCP, Observatory infrastructure and modifications to #484 baseline.

## Issue-Graph Notes

Global all-69 creation/review gate; #864 accepted output for 908/909/910; #720 accepted output additionally for #910. No earlier sprint blanket gate.

## Notes

Read failures are not absence. Unknown ownership remains frozen. Raw identities, credentials and object contents never enter new evidence.

## Tooling Notes

Native v3 lifecycle only; explicit agent-logic-admin for every read-only AWS invocation.
