# C-SDLC v3 defect backlog candidates

Source context: operator request to make v3 issue creation easy, real, and aligned with the GitHub model while removing v2 code.

Existing backlog item retained:

- #721 — `[v0.92.1][tools][csdlc-v3] Fix terminal authority reporting and issue-create parity`
  - Covers terminal authority reporting and first-class v3 `issue_create` parity.

New candidates created from distinct defects:

1. #723 — `[backlog][csdlc-v3] Restore green proof, shadow, and install tests`
   - Evidence: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets` failed in `tests/proof_parity_install_commands.rs` with `shadow_output_not_json`, proof receipt failures, and `install_route_is_one_binary_plan_gated_by_505`.
2. #724 — `[backlog][csdlc-v3] Add a simple gh-style issue creation command`
   - Evidence: `csdlc github-issue --request <path> --execute` exists, but operators still need to hand-author a nested dispatch JSON envelope instead of using a direct `gh issue create` style command surface.
3. #725 — `[backlog][csdlc-v3] Remove remaining v2 authority and build dependencies after cutover`
   - Evidence: v3 operational authority and proof/install paths still reference the v2 selector, v2 exact-head approval, and compile the `csdlc-v2` crate during v3 validation.
