# Structured Intent Prompt

Template: 1.0.0

Issue: 674

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Write one versioned, agent-readable Welcome Package that helps newly admitted residents understand Axioma Polis and its governed boundaries.

## Required Outcome

A concise source-grounded Welcome Package explains Polis context, resident identity, the Shepherd help path, Layer 8 communication, conditional agent-to-agent actions, forbidden actions, privacy, credentials, and safe escalation without claiming new Runtime behavior.

## Scope

- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
- .csdlc/prepared/issues/674/validate-welcome-package-docs.sh
- .csdlc/prepared/issues/674
- .csdlc/issues/674

## Authority

- The document describes current governed behavior and grants no authority
- Runtime admission, communication eligibility, Layer 8 authority, and provider availability remain authoritative
- Documentation cannot prove live delivery to an agent or implement onboarding automation

## Assumptions

- none

## Operator Constraints

- Issue #674 is docs-only
- Never write tracked work on main
- Use a bound FastWork issue worktree for the document
- Do not modify or restart the live Runtime
- Do not change Rust, APIs, OpenAPI, Observatory, providers, configuration, or cloud resources
