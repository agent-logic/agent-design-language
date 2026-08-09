# Structured Task Prompt

Template: 1.0.0

Issue: 84

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove only the Unity Observatory consumer paths owned by issue #84.

## Deliverables

- Native RuntimeV3Client adapter for HTTPS/WSS snapshots, events, commands, and reconnect
- Shared-contract-derived runtime-v3-contract.json projection
- Focused Unity tests for schema, ordering, cursor, authority, and failure states
- Explicit native trust, stale, unavailable, backpressure, and version-mismatch states
- Live Unity Editor/player validation entrypoint and retained native evidence
- adl/tools/validate_v092_unity_observatory_live.sh

## Acceptance

1. Unity renders current Runtime v3 snapshots and WSS events through one native adapter with fresh correlation evidence
2. The compatibility resource is derived from and remains compatible with the shared Runtime contract without a Unity schema fork
3. Every exposed control performs real authorized behavior or shows an explicit unavailable or denied state
4. Writes require authenticated authority and refusal cases remain denied before and after reconnect
5. TLS trust, version mismatch, stale data, backpressure, and Runtime unavailability are visible and never presented as live success
6. Reconnect uses bounded backoff and cursor continuity without duplicate event application or command replay
7. Focused Unity tests and live Editor/player proof exercise reads, writes, redaction, refusal, disconnect, and reconnect without fixture substitution
8. No files outside the four declared owned paths change during implementation

## Dependencies

- #5820 stable Runtime launch and API behavior is terminal
- #5832 versioned ACIP/A2A and WSS contract is terminal
- #5836 first-birthday interaction surface is terminal before final implementation credit
- Approved Unity Editor/player environment is available
- #5837 supplies shared restart coordination for final integration

## Inputs

- AGENTS.md
- docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/v0.91.6/unity-observatory

## Non Goals

- HTML client implementation or proof
- Runtime API, WSS, TLS, launch, or authentication changes
- Cross-client restart coordination
- Unity visual redesign or unrelated scene and prefab changes
- AWS or provider work
