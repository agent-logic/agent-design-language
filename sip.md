# Structured Intent Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one production-ready Axum hot-reload implementation.

## Required Outcome

One production-ready Axum hot-reload implementation with last-known-good retention.

## Scope

- adl-runtime/src/config_reload.rs
- adl-runtime/tests/config_reload.rs
- docs/runtime/config-hot-reload.md
- adl-runtime/src/lib.rs
- .csdlc/prepared/issues/510

## Authority

- Issue #510 owns only the HOT-01 runtime hot-reload surface.
- Sprint umbrella #529 coordinates child status and must not absorb child implementation.
- DEC-01 issue #513 is gated behind #510 and must not concurrently edit adl-runtime/src/config_reload.rs, adl-runtime/tests/config_reload.rs, docs/runtime/config-hot-reload.md, or adl-runtime/src/lib.rs.
- Invalid reload input must not replace the last-known-good configuration.
- No cloud, provider, database-pool, HTML template, admin API, merge, closeout, or adjacent sprint lifecycle work is authorized.

## Assumptions

- none

## Operator Constraints

- Use the typed C-SDLC v2 lifecycle route.
- Bind execution to a FastWork worktree under /Volumes/FastWork/adl-worktrees before implementation.
- Do not edit tracked main.
- Preserve unrelated and untracked preparation residue.
- Run the prepared HOT-01 validators where applicable.
- Obtain independent exact-head review before publication.
- Publish a PR with the correct closing keyword and stop before merge.
