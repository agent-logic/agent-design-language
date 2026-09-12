---
issue_card_schema: adl.issue.v1
wp: "Sprint 8"
slug: "910-observatory-deploy"
title: "[v0.92.2][OBS-S3] Deploy the existing Observatory S3 and CloudFront sidecar"
labels:
  - "track:roadmap"
issue_number: 910
generated_at: "2026-09-12T01:34:22.125782+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "deployment"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/910"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Sprint 8 #934; #720 accepted before preparation; #910 actual deployed acceptance gates TAIL-10."
pr_start:
  enabled: true
  slug: "910-observatory-deploy"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T01:34:22.125782+00:00

# Structured Task Prompt

## Summary

Deploy the exact reviewed static Observatory sidecar and prove authenticated infrastructure posture and browser HTTPS/Runtime WSS.

## Goal

Prepare exact reviewed Observatory assets/infrastructure deployment and approval packet; after explicit precise approval deploy and prove live HTTPS/WSS. This preparation phase performs no cloud writes.

## Required Outcome

Prepare exact reviewed Observatory assets/infrastructure deployment and approval packet; after explicit precise approval deploy and prove live HTTPS/WSS. This preparation phase performs no cloud writes.

## Deliverables

Exact sanitized plan and asset manifest, verified cloud preflight, rollback and cost/ownership approval packet; deployment remains gated.

## Acceptance Criteria

Business identity/DNS/certificate/current Runtime origins verified; exact merged #720 asset hashes; bounded Terraform plan from verified state; rollback/cache/cost/owners specified; independent review before precise apply/upload approval; later actual deployment/readback/browser evidence required for full #910 acceptance.

## Repo Inputs

infra/aws/observatory/README.md and readback.sh; #910 live issue; accepted #720 assets; company AWS readbacks.

## Dependencies

#864 accepted output; #720 PR #939 merged at 9c583cec78d396527798e592082595f316c67ce2; global 69-issue creation/review gate satisfied; umbrella #934.

## Target Files / Surfaces

infra/aws/observatory/ existing package; merged #720 static bundle; .csdlc/evidence/910/; .csdlc/issues/910/cards/

## Validation Plan

Existing Observatory Terraform/static validator; exact static asset hash/secret checks; explicit read-only business AWS/DNS/Runtime preflight; isolated backend-disabled Terraform fmt/validate/plan; independent plan/evidence review. No apply/upload/invalidation.

## Demo Expectations

Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute

## Non-goals

Runtime compute, architecture redesign, personal AWS, cloud writes before explicit approval, bulk resource changes.

## Issue-Graph Notes

Global all-69 creation/review gate; #864 accepted output for 908/909/910; #720 accepted output additionally for #910. No earlier sprint blanket gate.

## Notes

Stop on unavailable live Runtime origins, wrong identity, unknown state/custody, destructive plan or secret exposure. Preparation is not deployed completion.

## Tooling Notes

Native v3 lifecycle; agent-logic-admin read-only; precise operator approval required for apply, asset upload and invalidation.
