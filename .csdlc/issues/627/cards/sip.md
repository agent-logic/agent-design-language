# Structured Intent Prompt

Template: 1.0.0

Issue: 627

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Define the exact C-SDLC v3 replacement denominator and implement the stable one-binary CLI shell for the remaining v2 command-equivalent surface.

## Required Outcome

A reviewed machine-readable command denominator and one `csdlc` command shell that exposes or reserves every v2 replacement route, while unimplemented live-authority behavior fails closed before #505 cutover.

## Scope

- docs/csdlc-v3/full-replacement-denominator.json
- docs/csdlc-v3/v3-command-manifest.json
- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/**
- csdlc-v3/tests/**
- .csdlc/prepared/issues/627/**
- .csdlc/issues/627/**

## Authority

- C-SDLC v2 remains the sole live lifecycle authority until #505 cutover is explicitly approved and merged.
- Issue #627 may define v3 command contracts and fail-closed shell routes only.
- Issue #627 must not perform v3 lifecycle writes, GitHub writes, publication, finish, cleanup, or v2 retirement.
- No C-SDLC v2 source files may be changed in this issue.

## Assumptions

- none

## Operator Constraints

- Do not merge #505.
- Do not use v3 as live C-SDLC authority.
- Do not use raw gh for lifecycle writes.
- Do not change v2 source.
- Keep the route contract simple enough for issue startup in three minutes or less.
