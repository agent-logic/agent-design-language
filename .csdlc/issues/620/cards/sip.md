# Structured Intent Prompt

Template: 1.0.0

Issue: 620

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce a coherent, reviewed, number-free first-pass v0.92.2 planning package and explicitly disposition relevant unplanned or unscheduled TBD material.

## Required Outcome

The canonical planning package, feature set, issue wave, execution specifications, sprint structure, release tail, and TBD scheduling reconciliation agree without opening the milestone or creating implementation issues.

## Scope

- docs/milestones/v0.92.2/**
- .csdlc/prepared/issues/620/**
- .csdlc/issues/620/**

## Authority

- #620 prepares documentation but does not open v0.92.2
- The existing v0.92.2 package is the baseline to audit rather than recreate
- .adl/docs/TBD is read-only source evidence
- Unresolved scheduling gaps require operator judgment and are not silently scheduled
- Existing GitHub issue and completion truth prevents duplicate planning

## Assumptions

- none

## Operator Constraints

- Do not create WP-01, sprint umbrellas, or child implementation issues
- Do not create the milestone or version label
- Do not implement product, Runtime, provider, cloud, or Unity features
- Do not edit or delete TBD sources merely to clean the inventory
- Do not broaden v0.92.2 with explicitly deferred work
