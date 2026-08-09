# Structured Intent Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make missing GitHub issue reads immediately actionable while preserving typed redacted failures and distinguishing other remote failure classes.

## Required Outcome

The real csdlc-github-issue CLI returns stable JSON and exit behavior for not-found, authentication, authorization, rate-limit, server, and transport observations without exposing secrets or changing successful reads.

## Scope

- csdlc-v2/src/error.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/bin/csdlc-github-issue.rs
- csdlc-v2/tests/gate_github_actions.rs
- .csdlc/prepared/issues/41
- .csdlc/issues/41

## Authority

- Issue and code authority are agent-logic/agent-design-language#41
- Only GithubAction::IssueRead receives the new contextual classifier
- The existing Octocrab client and csdlc.error.v1 CLI envelope remain authoritative
- GitHub 404 is reported as an observation and does not assert that a private repository exists

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use the typed C-SDLC v2 lifecycle and Rust-only control plane
- Do not emit tokens, token paths, authorization headers, raw response bodies, or Octocrab debug strings
- Use deterministic loopback or mocked proof rather than live GitHub failure injection
- Do not implement or publish during readiness preparation
