# Structured Intent Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce a complete, independently reviewed Rust architecture and implementation issue plan for C-SDLC v3 without starting implementation.

## Required Outcome

A source-grounded Rust plan defines the complete v3 product architecture, quantified targets, migration safety, and an implementation-ready issue decomposition reviewed at the same exact revision by Claude and Gemini.

## Scope

- Rust C-SDLC v3 architecture modeled on the official GitHub CLI
- Measured v2 baseline and bounded effect estimates
- Eighteen implementation issue specifications plus one deferred v2-retirement issue
- Independent Claude and Gemini architecture review with finding dispositions

## Authority

- GitHub issue #73 owns planning only
- C-SDLC v2 remains the sole operational lifecycle authority
- The official cli/cli repository is an architectural source, not copied implementation authority
- Only later operator-authorized issues may create child issues or implement, migrate, cut over, or retire authority

## Assumptions

- none

## Operator Constraints

- Implement C-SDLC v3 in Rust
- Keep this issue planning-only
- Include the complete implementation issue breakdown
- Obtain independent Claude and Gemini reviews
- Do not create implementation child issues
- Do not change selectors, migrate records, cut over authority, or delete v2
- Do not write tracked changes on primary main
