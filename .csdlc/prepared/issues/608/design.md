# #608 design: Vertex global Gemini endpoints and thinking config

## Goal

Make the native `vertex_ai_gemini` provider support current Vertex Gemini model-instance configuration without endpoint overrides:

- `location: global` must route to `https://aiplatform.googleapis.com/.../locations/global/...`.
- Regional locations keep the existing `<region>-aiplatform.googleapis.com` host.
- First-party Vertex endpoint trust accepts both global and regional Google hosts.
- Provider config can express Gemini thinking controls without Rust hard-coding:
  - `thinking_level`
  - `thinking_budget`
  - `include_thoughts`

## Minimal implementation

The change stays inside the existing Vertex provider implementation in `adl/src/provider/http_family.rs`.

1. Add optional parsed `thinking_config` state to `VertexAiGeminiProvider`.
2. Build `generationConfig.thinkingConfig` from provider config only when one of the thinking controls is present.
3. Reject simultaneous `thinking_level` and `thinking_budget`, because global Gemini 3.x thinking-level style and Gemini 2.5 thinking-budget style are different model-family contracts.
4. Normalize `thinking_level` to the documented uppercase enum values `MINIMAL`, `LOW`, `MEDIUM`, and `HIGH`.
5. Leave all existing regional endpoint behavior intact.

## Validation

Focused local proof:

- `cargo fmt --manifest-path adl/Cargo.toml`
- `cargo test --manifest-path adl/Cargo.toml vertex_ai_ -- --nocapture`
- `git diff --check -- adl/src/provider/http_family.rs adl/src/provider/http_family/tests.rs`
- `cargo check --manifest-path adl/Cargo.toml -p adl`

Live proof:

- Run ADL provider workflows against the approved company GCP project/key without printing or committing credentials.
- Prove regional Gemini 2.5 Flash, Flash-Lite, and Pro in `us-west1`.
- Prove global Gemini 3.x routes using native `location: global`, with no endpoint override.

## Non-goals

- No Polis integration; #592 owns that.
- No broad provider-system redesign.
- No new provider dependencies.
- No committed credentials, generated tokens, local gcloud cache, or secret output.

