# Issue 84 Design: Unity Observatory Runtime v3 Consumer

## Outcome And Boundary

Issue 84 adds a native Unity adapter that makes the existing Unity Observatory
a real consumer of the same versioned Runtime v3 HTTPS and WSS contract used by
the HTML client. The Unity application renders live state, performs only
authorized actions, reconnects without duplicate event application, and
exposes trust, stale, unavailable, denied, and version-mismatch states.

This issue does not change the approved Unity visual design, the HTML client,
Runtime API/WSS behavior, or the shared Guardian restart coordinator.

## Source Baseline

- `demos/v0.91.6/unity-observatory/` contains the approved Unity shell and
  existing validation surfaces.
- `docs/api/runtime-v3/v1/observatory.openapi.json` and Runtime v3 architecture
  projections are read-only schema inputs.
- `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md`
  requires live native reads and writes, redaction and refusal, reconnect, and
  proof without fixture substitution.
- Issue #5837 owns shared Runtime/WSS integration and cross-client restart
  reconciliation. Issue #83 owns the HTML browser consumer.

## Design

`RuntimeV3Client.cs` is the single Unity transport adapter for discovery,
snapshot retrieval, authenticated WSS events and commands, reconnect cursor,
freshness, and visible connection state. Unity view components consume typed
adapter state; they do not implement transport, authentication, or a parallel
Runtime schema.

`runtime-v3-contract.json` is a versioned compatibility projection derived from
the shared Runtime contract. It records only the fields Unity needs to verify
API/catalog version, projection audience, stable IDs, event ordering,
correlation, reconnect metadata, authentication mode, and backpressure limits.
It is not an independently authored Unity protocol.

Read-only projection access does not imply write authority. Commands require
the approved authenticated session and surface accepted, denied, expired, and
unavailable outcomes. Tokens, private keys, and signing material never enter
Unity assets, scenes, logs, screenshots, player builds, or repository files.

Reconnect uses bounded exponential backoff with jitter and the last accepted
cursor. Retained state is visibly stale while disconnected. Live status is
restored only after a fresh Runtime correlation. Duplicate and out-of-order
events are rejected or ignored according to the shared Runtime contract.

## Owned Paths

- `demos/v0.91.6/unity-observatory/Assets/Scripts/RuntimeV3Client.cs`
- `demos/v0.91.6/unity-observatory/Assets/Resources/runtime-v3-contract.json`
- `demos/v0.91.6/unity-observatory/Assets/Tests/RuntimeV3ClientTests.cs`
- `adl/tools/validate_v092_unity_observatory_live.sh`

## Read-Only Inputs

- Runtime v3 HTTP/WSS implementation, schemas, authentication, certificates,
  and launch behavior.
- Existing Unity scenes, prefabs, view scripts, and visual assets.
- Issue #5837 restart coordinator and issue #83 HTML outputs.
- All sibling and dependency records.

## Invariants And Failure Semantics

- Unity uses the shared Runtime contract and cannot create a schema fork.
- No fixture, static packet, or cached snapshot is labeled live.
- Public reads never widen command authority.
- TLS trust failure, API/WSS version mismatch, stale data, backpressure,
  authorization refusal, and Runtime unavailability are visible.
- Reconnect cannot duplicate events, replay commands, or escalate authority.
- No private citizen state, key, token, or sealed checkpoint enters Unity.
- The approved Unity design and Runtime/application boundary remain intact.

## Dependencies And Execution Gate

Issues #5820 and #5832 must remain terminal and supply stable Runtime behavior
and the versioned ACIP/WSS contract. Issue #5836 must be terminal before final
implementation credit or live acceptance is claimed. Live native proof also
requires the approved Unity Editor/player environment. Preparation may complete
while those execution gates are open; execution must report them truthfully.

## Validation Boundary

`RuntimeV3ClientTests.cs` provides deterministic contract, ordering, cursor,
redaction, refusal, and failure-state coverage. The live entrypoint
`adl/tools/validate_v092_unity_observatory_live.sh` launches the approved Unity
Editor/player against the real Runtime revision and retains native interaction
evidence for fresh reads, authenticated writes, denial, stale/unavailable
states, disconnect, and reconnect. Fixture-only edit-mode tests, static
screenshots, and Runtime-only tests cannot satisfy live native acceptance.

Shared Guardian-owned restart coordination remains in #5837; this issue only
provides Unity-side hooks and assertions consumed by that coordinator.

## Rollback

Rollback removes or disables the native adapter, logs out write sessions, and
returns Unity to an explicit read-only or unavailable state. It does not add a
Unity-only protocol, replace live state with fixtures, or change Runtime.

## Non-Goals

- HTML implementation or proof.
- Runtime API, WSS, TLS, launch, or authentication changes.
- Cross-client restart orchestration.
- Unity visual redesign or unrelated scene/prefab changes.
- Provider or AWS work.
