# Issue 340 design

## Status

Initialized design packet for #340: HTML Observatory Runtime restart integration.

## Intent

Issue #340 proves the local HTML/Polis Observatory can consume the real Runtime
v3 operator service through the merged CSMctl launch surface, survive a
Guardian-owned graceful stop and restart, and resume observation without
duplicate event application, stale correlation, authorization drift, or
projection leakage.

This issue deliberately no longer waits for Unity. #84, #122, and #251 are the
later Unity/TLS track. #341 and #343 remain parallel/downstream and are not
execution gates for this integration.

## Owned change surface

- `CSMctl`
- `start_CSM.sh`
- `docs/tooling/START_CSM_RUNBOOK.md`
- `docs/tooling/CSMctl.conf.example`
- `adl-runtime/tests/runtime_api_wss.rs`
- `adl/tools/validate_v092_observatory_restart_reconnect.sh`
- `.csdlc/issues/340`
- `.csdlc/prepared/issues/340`
- `.csdlc/evidence/340`

HTML/Polis product implementation paths are read-only inputs. The validator may
serve or inspect the existing HTML Observatory assets, but must not redesign or
rewrite them.

## Dependency and gate handling

- Treat terminal #110 HTML/Polis Observatory evidence as the UI input surface.
- Treat #256 terminal and ancestral truth as the local birthday interaction
  input surface.
- Treat #209 / PR #215 as the current Runtime v3 API/WSS authority.
- Treat #424 as merged CSMctl startup input.
- Reverify local browser/TLS trust at the exact #340 revision.

If any dependency evidence is contradictory, the #340 validator must fail closed
with a named blocker instead of manufacturing proof.

## Implementation plan

1. Bind a dedicated FastWork worktree from current `origin/main`.
2. Inspect the merged CSMctl behavior and Runtime API/WSS test surface.
3. Implement the smallest restart/reconnect proof path:
   - `./CSMctl start` only succeeds after `/v1/ready`,
     `/v1/observatory`, and `/v1/health` return HTTP 200.
   - `./CSMctl stop` performs graceful Guardian checkpoint/dehydration shutdown,
     confirms process termination, and removes script-owned PID and lease files.
   - A subsequent `./CSMctl start` restarts cleanly.
   - The Observatory reconnect proof records bounded replay, no duplicate
     application, fresh correlation, unchanged authorization, and redacted
     projection shape.
4. Add or repair `adl-runtime/tests/runtime_api_wss.rs` only for focused
   Runtime API/WSS restart/reconnect contract coverage.
5. Add `adl/tools/validate_v092_observatory_restart_reconnect.sh` as the
   issue-owned proof command, retaining evidence under `.csdlc/evidence/340`.
6. Run focused proof, typed validation, fresh exact-head review, publication,
   CI, and finish if gates stay green.

## Proof strategy

The proving denominator is intentionally live/local, not fixture-only:

- CSMctl must manage the real local Runtime process.
- Required endpoint probes must be HTTP 200.
- Stop must leave no script-owned PID or lease files behind.
- Restart must expose a new connection/correlation while retaining the same
  authority envelope.
- Replay evidence must include cursor or sequence accounting sufficient to deny
  duplicate application.
- Projection samples must be checked for audience-appropriate redaction.

Fixture/static checks may be retained as supporting evidence, but they cannot
replace the live/local restart proof.

## Non-goals

- Unity implementation or proof.
- TLS 1.2 work owned by #251.
- AWS/public launch.
- Provider credentials or multi-agent provider proof.
- HTML Observatory redesign.
- Runtime protocol redesign.
- #341 or #343 execution.

## Stop conditions

- CSMctl cannot produce a real local Runtime service with all required endpoint
  probes at HTTP 200.
- Graceful stop cannot prove checkpoint/dehydration and script-owned PID/lease
  cleanup.
- Restart/reconnect proof cannot distinguish live replay from fixture/static
  rendering.
- Work would require Unity, TLS backlog, AWS/public launch, provider
  credentials, HTML redesign, or Runtime protocol redesign.
- Fresh review returns actionable findings.
