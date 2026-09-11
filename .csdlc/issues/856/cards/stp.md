---
issue_card_schema: adl.issue.v1
wp: "release repair"
slug: "issue-856-native-release-preflight"
title: "[v0.92.1][release] Reconcile release versions and restore native v3 ceremony preflight"
labels:
  - "track:roadmap"
issue_number: 856
generated_at: "2026-09-11T18:46:11.480766+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "tooling_repair"
repo_inputs:
  - ".csdlc/evidence/856/source-issue.md"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Release blocker856 before526"
pr_start:
  enabled: true
  slug: "issue-856-native-release-preflight"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T18:46:11.480766+00:00

# Structured Task Prompt

## Summary

Reconcile release artifact manifest/lock identities and replace retired v2 ceremony fallback with native fail-closed exact-candidate preflight.

## Goal

Restore nonmutating native v3 v0.92.1 ceremony preflight and reconcile release versions

## Required Outcome

Reviewed green PR repairing preflight and version identities; no release publication.

## Deliverables

Explicit release artifact inventory, coordinated manifests/locks, native release-preflight, shell routing and stable installer, regression fixtures and operator documentation.

## Acceptance Criteria

All release package/lock identities agree; missing/stale native authority or candidate gate rejects; commit/notes/evidence hashes bind exact inputs; positive/negative fixtures are nonmutating; stable owners and channel contract proven; #526 receives repair handoff.

## Repo Inputs

Issue856, release_ceremony.sh, canonical v3 authority, tail-02 manifest inventory, current release plan/notes.

## Dependencies

Repair needed before #526 candidate approval; #522 remediation and #525 review remain semantic release gates; #833 untouched.

## Target Files / Surfaces

Release package manifests/active locks; csdlc-v3/src/commands/release.rs and CLI; release wrapper/owner installer; tests; release inventory and docs.

## Validation Plan

Native owner suite, focused real-CLI candidate matrix, shell routing/install tests, all package locked offline metadata, fmt/clippy and hosted CI.

## Demo Expectations

Nonmutating local candidate fixtures; no release publication

## Non-goals

No release/tag/push/merge, historical proof rewrites, release approval, v2 fallback or Runtime changes.

## Issue-Graph Notes

Required before #526 candidate approval; coordinate #522; preserve #833 and historical proof.

## Notes

Local gate hashes prove consistency, not semantic review/authentication; candidate release approval remains separate.

## Tooling Notes

Native v3 issue/doctor/bind/edit/validate; no v2.
