# #608 Vertex provider proof summary

Date: 2026-09-01

Credential handling:

- Used approved service-account key by path through `ADL_VERTEX_GCP_KEY`.
- Did not print, copy, or commit key contents or generated access tokens.
- Used a worktree-local `CLOUDSDK_CONFIG` during the live proof and removed it at script exit.

Focused local validation:

- `bash .csdlc/prepared/issues/608/validate-provider.sh`
- Result: PASS
- Covered:
  - `cargo fmt --manifest-path adl/Cargo.toml --check`
  - `cargo test --manifest-path adl/Cargo.toml --lib vertex_ai_ -- --nocapture`
  - `git diff --check -- adl/src/provider/http_family.rs adl/src/provider/http_family/tests.rs`
  - `cargo check --manifest-path adl/Cargo.toml -p adl`

Live Vertex validation:

- `ADL_VERTEX_GCP_KEY=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json bash .csdlc/evidence/608/live-vertex/run-live-provider-proof.sh`
- Result: PASS
- Regional `us-west1` Gemini 2.5 native provider proof:
  - `gemini-2.5-flash`: success
  - `gemini-2.5-flash-lite`: success
  - `gemini-2.5-pro`: success
- Global Gemini 3.x native provider proof with `location: global` and no endpoint override:
  - `gemini-3.7-flash`: success
  - `gemini-3.6-flash`: success
  - `gemini-3.5-flash`: success
  - `gemini-3.5-flash-lite`: success
  - `gemini-3.1-pro-preview`: success
  - `gemini-3-flash-preview`: success

