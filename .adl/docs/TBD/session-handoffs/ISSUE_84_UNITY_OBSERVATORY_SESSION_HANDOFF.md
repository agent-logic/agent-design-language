# Issue #84 Unity Observatory Session Handoff

Last updated: 2026-08-11

## Objective

Complete and prove the native Unity Observatory Runtime v3 consumer for
`agent-logic/agent-design-language#84`. The finished surface must consume the
real Runtime v3 projection, preserve read/write authority boundaries, expose
truthful failure states, reconnect without cursor or authority regression, and
retain native Unity evidence. A player binary must not be built.

## Bound Execution Context

- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-84-unity-observatory-readiness`
- Branch: `codex/84-unity-observatory-readiness`
- Current base commit: `2ca74474eedb80bedb194b2ebbdd888745d4d777`
- Remote relation at handoff: ahead 32, behind 1
- C-SDLC phase: `bound`
- C-SDLC generation: 14
- C-SDLC digest: `935820143ec18006b12f5d495aed1b6b694c6ba8a882a79541ba29d8ff814767`
- Session goal: blocked on governed UnityTLS trust, not complete

The worktree is intentionally dirty with the issue implementation and typed
records. Do not discard, reset, clean, or replace it. Never move this work to
`main`.

## Current Implementation

- `RuntimeV3Client.cs` implements public HTTPS snapshot reads, WSS event and
  control-result handling, authenticated writes, signed snapshot submission,
  bounded reconnect, local cursor deduplication, explicit connection states,
  and certificate pinning.
- `UnityObservatoryShellController.cs` binds the existing observatory shell to
  `RuntimeV3Client` as its single Runtime v3 transport.
- `runtime-v3-contract.json` is a digest-bound projection of the shared Runtime
  v3 OpenAPI contract.
- `RuntimeV3ClientTests.cs` contains focused contract checks and a Play Mode
  live driver that exercises the product component rather than a parallel test
  socket.
- `UnityObservatoryBatchValidator.cs` validates the flagship scene and shell.
- `validate_v092_unity_observatory_live.sh` stages proof under
  `/Volumes/FastWork/u84`, imports operator-owned Asset Store packages only into
  that disposable project, and never builds a player binary.

The licensed packs remain outside Git in Unity's local Asset Store cache. Do
not commit imported pack contents.

## Proof Status

| Surface | Status | Evidence |
| --- | --- | --- |
| Contract projection and focused adapter behavior | Pass | `.adl/evidence/84/unity-runtime-v3/20260809T091147Z-contract-tests-only.log` |
| Flagship licensed environment and shell | Pass | `.adl/evidence/84/unity-runtime-v3/20260809T090921Z-shell-tests.log` |
| Diff hygiene | Pass | `git diff --check` on 2026-08-11 |
| Real Play Mode HTTPS/WSS, auth, reconnect, signed snapshot | Failed closed | `.adl/evidence/84/unity-runtime-v3/20260809T091629Z-live.log` |
| Player binary | Not built | All retained validator result payloads report `player_binary_built:false` |

The live Runtime was observed at `https://localhost:20997`. The real Play Mode
adapter loaded the shell but did not claim live/authenticated state because
UnityTLS refused the self-signed WSS certificate. Supplying the exact
certificate through process-scoped `SSL_CERT_FILE` plus product pinning did not
resolve the UnityTLS handshake.

## External Dependency

Tooling issue `agent-logic/agent-design-language#92` owns the governed fix:

`[v0.92][runtime-v3][tooling] Support governed trust install for managed-external localhost certificates`

Issue #92 contains both the original `trust-install` policy refusal and the
later real Play Mode `SSL_CERT_FILE` failure. Resume the live lane only after
one of these is true:

1. #92 provides an explicit-consent, receipt-backed trust install/verify/remove
   path for the exact managed-external localhost certificate; or
2. the operator explicitly authorizes installation of that exact certificate
   into the selected login keychain and the trust is independently verified.

Do not silently mutate keychain trust, disable TLS verification, use `curl -k`
as proof, launch a proxy, substitute a fixture, or claim WSS success from HTTPS
alone.

## Independent Review Findings To Fix

The latest findings-first review was performed against the dirty snapshot based
on `2ca74474eedb80bedb194b2ebbdd888745d4d777`. These findings are actionable
and must be fixed before publication:

1. Require Runtime readiness/health before transitioning a structurally valid
   feed to `Live`; unhealthy or not-ready feeds must remain explicit degraded
   states.
2. Add a configuration generation token, suppress callbacks from cancelled
   generations, and reset feed, identity, cursor, freshness, queued actions,
   and authority on endpoint reconfiguration.
3. Strengthen live reconnect proof so it observes authentication loss and a
   newly authenticated connection generation. Add denied-before-reconnect,
   denied-after-reconnect, and command-replay refusal proof.
4. Correlate accepted command results to the submitted `command_id` and
   `correlation_id`.
5. Replace raw substring redaction checks with structural JSON key validation
   or strict schema parsing so whitespace and escaped-key variants cannot
   bypass private-field refusal.
6. Make the product client and shell share one origin-only endpoint validator;
   reject credentials, paths, queries, and fragments before auto-attachment.

The certificate blocker does not excuse these code findings. They can be fixed
and contract-tested while #92 is in progress.

## Safe Resumption Sequence

1. Confirm the primary checkout is clean on `main`; perform all edits in the
   bound worktree above.
2. Read the live issue and all six cards, then run `csdlc-validate` for issue
   84 using the installed v2 owner binary.
3. Recheck #92 through `csdlc-github-issue`; do not use raw `gh` or the GitHub
   connector for covered lifecycle writes.
4. Fix the six review findings in the declared paths. Add focused tests for
   every changed state boundary.
5. Run `--contract-tests-only` and `--shell-tests`; retain only focused proof.
6. Once trust is governed and verified, confirm the Runtime process using
   `adl process status --port <port> --json` and confirm the signed command
   targets the current `runtime_instance_id` without printing credentials.
7. Run `--live`. Require the product Play Mode adapter to prove public read,
   WSS authentication, forced disconnect, new connection generation,
   cursor-safe recovery, refusal behavior, replay refusal, and a correlated
   accepted signed snapshot result.
8. Run a fresh exact-head subagent review and fix all actionable findings.
9. Reconcile SPP/VPP/SRP/SOR only through typed editor operations, then run
   `csdlc-review` before `csdlc-publish`.
10. Publish with `Closes #84`. Do not merge without explicit operator authority.

## Secret And Evidence Boundaries

- Runtime token file and signing material live outside the repository. Never
  print, copy, commit, or embed their contents.
- Temporary private signing keys created during the prior session were deleted.
- The retained signed snapshot envelope is not signing authority, but it must
  be revalidated against the current Runtime identity before use.
- Logs must remain redacted and must not claim successful live operation when
  UnityTLS, authentication, reconnect, or command correlation is unproved.

## Completion Conditions

Issue #84 is not complete until all six review findings are resolved, the real
Play Mode live lane passes against the current Runtime, exact-head review is
current and clean, typed review/output truth matches the retained evidence, and
the implementation is published through the v2 lifecycle. Contract and shell
passes alone are necessary but insufficient.
