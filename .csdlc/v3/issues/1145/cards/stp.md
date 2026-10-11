---
issue_card_schema: adl.issue.v1
wp: "<wp>"
slug: "<slug>"
title: "Observatory: add Everyone selection to the multi-agent room"
labels:
  - "track:roadmap"
issue_number: 1145
generated_at: "<timestamp>"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.5"
required_outcome_type:
  - "<required_outcome_type>"
repo_inputs:
  - "<source_issue_prompt>"
canonical_files: []
demo_required: <demo_required>
demo_names: []
issue_graph_notes:
  - "<issue_graph_note>"
pr_start:
  enabled: true
  slug: "<slug>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: <timestamp>

# Structured Task Prompt

## Summary

<summary>

## Goal

<goal>

## Required Outcome

Add accessible Everyone selection for explicit eligible recipients in the selected Runtime polis.

## Deliverables

Everyone and Clear controls; visible names/count; individual deselection; stable selection across roster refresh; unavailable and over-limit feedback.

## Acceptance Criteria

Select all eligible recipients without wildcard, silent truncation or implicit sends; keep eight-recipient cap and write authorization; never silently add roster newcomers; show stale selections and per-agent outcomes.

## Repo Inputs

<repo_inputs>

## Dependencies

Existing governed-room UI and Runtime explicit-recipient protocol. User instruction on 2026-10-06 opens this bounded issue for implementation.

## Target Files / Surfaces

demos/html-observatory/app.js, index.html, styles.css; focused UI regression checks.

## Validation Plan

Focused deterministic Node tests and governed-room validator; desktop/mobile keyboard and responsive inspection with fixture data. No live sends or paid provider calls.

## Demo Expectations

<demo_proof_requirements>

## Non-goals

<non_goals>

## Issue-Graph Notes

<issue_graph_notes>

## Notes

Previously backlog-only; user now requests work on #1145. Preserve selection intent across roster changes; block sends containing unavailable recipients or more than eight IDs.

## Tooling Notes

<tooling_notes>
