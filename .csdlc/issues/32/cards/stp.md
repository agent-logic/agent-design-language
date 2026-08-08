# Structured Task Prompt

Template: 1.0.0

Issue: 32

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Fix issue #32 only: branch-independent larger-runner eligibility diagnostics and focused CI integration proof.

## Deliverables

- Typed csdlc-runner-preflight request and result contracts, including bounded live job observation.
- Read-only Octocrab inspection of hosted runner, runner group, repository access, workflow restrictions, and stale refs.
- Focused unit and loopback integration tests.
- Live preflight and PR canary evidence.

## Acceptance

1. Normal PR branches can acquire the configured larger runner without branch allowlist changes.
2. Runner group access remains explicitly limited to agent-logic/agent-design-language.
3. Preflight reports label, group visibility, repository access, workflow restriction state, and stale workflow refs without secrets.
4. Diagnostic classification distinguishes capacity unavailable, policy ineligible, and configuration-eligible but non-dispatching jobs.
5. A live PR canary assigns a runner and reaches a terminal result.
6. No AWS is used.

## Dependencies

- GitHub Actions organization APIs
- Octocrab
- C-SDLC v2 typed JSON conventions

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/32
- https://github.com/agent-logic/agent-design-language/actions/runs/31236518300
- .github/workflows/ci.yaml
- csdlc-v2/src/github.rs

## Non Goals

- Changing Rust test content
- AWS or self-hosted runner infrastructure
- Broadening runner-group repository access
- Mutating organization settings from the preflight
