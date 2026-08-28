# C-SDLC v3 Agent Guidance

This directory is a construction surface, not the active lifecycle authority.
Root `AGENTS.md` and `csdlc-v2/AGENTS.md` remain authoritative for C-SDLC
operations until a later operator-reviewed V3-F cutover explicitly changes
that.

## Current Boundary

- `csdlc-v3/**` may model lifecycle decisions, transaction storage, recovery,
  adapters, projections, and validation fixtures.
- The current V3-C slice may contain pure lifecycle transition decisions,
  deterministic transaction storage/recovery classification, typed adapter
  boundaries, and focused transaction tests for retained requirements #168
  through #170. Those surfaces are construction evidence only.
- `csdlc-v3/**` must not bind worktrees, mutate issue state, publish pull
  requests, finish issues, clean worktrees, retire v2, call GitHub, call v2
  owner binaries, or claim operational authority.
- Unsupported or not-yet-proven behavior fails closed. Do not fall back to v1
  wrappers, raw GitHub mutation, hand-edited cards, shell strings, or ambient
  local state.
- Branch or worktree observation is evidence only. It never authorizes lifecycle
  work by itself.

## Construction Style

- Keep v3 code deterministic, typed, and explicit.
- Prefer pure transition functions, in-memory fakes, and data-only recovery
  classifiers until an issue specifically authorizes production adapters.
- Preserve `argv`, status, stdout, stderr, timeout, cancellation, truncation,
  and redaction distinctions at adapter boundaries.
- Treat cards, evidence indexes, and rendered views as repairable projections;
  modeled `state.json` replacement is the only committed authority inside the
  transaction model.

## Operator Target

A prepared v3 issue should be inspectable, bindable, and startable in three
minutes or less once its dependencies are satisfied. Add automation only when it
removes real operator friction while preserving the v2 authority boundary.

## Cutover Rule

Do not change root C-SDLC authority from v2 to v3 in this directory. A clean v3
replacement requires a later explicit cutover issue with parity/import,
rollback, publication, finish, cleanup, documentation, and operator-start proof.
Until that V3-F decision lands, root `AGENTS.md`, onboarding, v2 operator
skills, and installed PR skills must point to v3 as non-authoritative
construction evidence rather than as an executable operator path.
