# Issue 680 design

Status: design prepared for execution.

## Goal

Add first-class ADL provider support for Moonshot/Kimi K3 so operators can
select it through the normal provider setup/profile surfaces.

## Current evidence

- `adl/src/provider_adapter.rs` already contains hosted Kimi/Moonshot chat
  completions support for `provider = "kimi"` and `provider = "moonshot"`.
- The current hosted adapter uses `MOONSHOT_API_KEY` and
  `https://api.moonshot.ai/v1/chat/completions`.
- `adl/src/provider/profiles.rs` currently exposes `kimi:k2.5` as
  `kimi-k2.5`; there is no `kimi:k3` profile yet.
- `adl/src/provider_substrate.rs` already preserves `kimi` as a vendor for
  `kimi:*` profiles.
- `adl/src/cli/provider_cmd.rs` does not expose a Moonshot/Kimi setup family.
- Official Kimi platform documentation provides the list-models endpoint at
  `https://api.moonshot.ai/v1/models` and documents use of
  `MOONSHOT_API_KEY`. Current browsed public sources for Kimi K3 identify
  `kimi-k3` as the platform-facing API model name, while the general Kimi model
  list may lag or emphasize the latest K2.x hosted models. The implementation
  must record this as catalog truth rather than silently inventing availability.

## Implementation map

1. Add a stable ADL-facing `kimi:k3` profile mapped to Moonshot's current
   provider-native model id `kimi-k3`.
2. Add Moonshot/Kimi to the provider setup/help surface with endpoint and
   `MOONSHOT_API_KEY` guidance.
3. Ensure the provider selection/help path treats `kimi`/`moonshot` as
   first-class alongside the already implemented hosted adapter route.
4. Add an issue-owned deterministic Rust integration test target
   `adl/tests/provider_moonshot_kimi_k3.rs` covering profile lookup, setup
   visibility, request/auth behavior, and fail-closed credential/transport
   classification without live provider calls.
5. Record SOR/evidence truth distinguishing offline deterministic proof from
   any optional live Moonshot proof.

## Boundaries

- No live paid/provider call is authorized by this issue.
- No Moonshot credentials may be added, printed, or committed.
- Existing `kimi:k2.5` behavior and OpenRouter Kimi model-id handling must
  remain compatible.
- Provider architecture changes must stay limited to the minimum first-class
  Moonshot/Kimi K3 path.

## Review focus

- Does the profile/setup/provider selection path make Moonshot/Kimi first-class
  without breaking existing Kimi routes?
- Does the test target prove deterministic offline behavior without treating
  provider availability as local implementation proof?
- Is the K3 model-id/catalog truth represented carefully enough for future
  drift?
