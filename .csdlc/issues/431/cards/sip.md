# Structured Intent Prompt

Template: 1.0.0

Issue: 431

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Audit and refresh the existing v0.92.1 planning package now, producing a reviewed initial baseline and explicit handoff that WP-28 #316 can update later.

## Required Outcome

A truthful, internally consistent initial v0.92.1 package maps six execution lanes, the #432 repository prerequisite, carryovers, dependencies, proof surfaces, readiness gaps, Runtime v4 risk, and explicit handoffs to unchanged WP-28 and v0.92.2 CodeFriend Beta 1.

## Scope

- Existing docs/milestones/v0.92.1 planning package audit and refresh
- Live v0.92/v0.92.1 issue and dependency inventory
- Operator-authorized promotion of validated Axum configuration hot reload
- Podcast publication and Podcast Studio lane allocation
- Observatory redesign sprint allocation without Runtime backend expansion
- No-.adl repository authority prerequisite allocation
- v0.92.2 CodeFriend Beta 1 successor handoff
- Initial work-package, proof, readiness, and handoff reconciliation

## Authority

- This sidecar creates an initial planning baseline only
- WP-28 #316 remains unchanged and owns the later plan update
- No issue migration, product implementation, release approval, or milestone activation
- Runtime v4 remains a named risk and is not committed scope

## Assumptions

- none

## Operator Constraints

- Leave WP-28 #316 as-is
- Use typed C-SDLC v2 and a dedicated FastWork worktree
- Promote only the explicitly authorized Axum hot-reload item; do not promote other backlog items or move issues
- Do not create tracked dependencies on .adl paths
- Use focused docs/YAML/link/routing validation rather than broad Rust tests
