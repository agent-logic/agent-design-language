# C-SDLC v3 Package Boundary

`csdlc-v3/**` is the construction surface for the planned clean replacement of
C-SDLC v2. It is not the live lifecycle authority yet. Root `AGENTS.md`,
`csdlc-v2/AGENTS.md`, and the typed v2 operator skills remain authoritative
until the explicit V3-F/#505 cutover decision approves the transition.

## Current construction state

- V3-A/#500 established the v3 contract and predecessor map as construction
  evidence. Corrective follow-up #571 remains an explicit Sprint 6 gate for
  predecessor owner/proof-lane precision and lifecycle-gate consistency before
  V3-F can approve cutover.
- V3-B/#501 added foundation import/projection surfaces for v2 compatibility
  exploration. Those surfaces are read-only construction evidence, not live
  import or migration authority.
- V3-C/#502 added the lifecycle-kernel construction slice and
  `csdlc-v3/AGENTS.md` package-local guardrails. It does not replace v2
  lifecycle commands.
- V3-D/#503 adds the local preparation command model and `csdlc-v3-local`
  proof surface. It must remain non-authoritative until V3-F.

## Clean replacement target

The target is a clean v3 replacement line, not permanent v2/v3 coexistence.
V3 work should make issue start, review, publication, finish, and cleanup easier
to operate while preserving typed contracts, exact topology, exact-head review,
closing-linkage proof, terminal truth, and safe cleanup boundaries.

A prepared v3 issue should be inspectable, bindable, and ready for first useful
work in three minutes or less once dependencies are satisfied. That time target
removes ceremony and ambiguity; it does not remove typed authority, review, or
validation gates.

## Non-goals before V3-F/#505

Before V3-F/#505, v3 must not:

- bind worktrees for live issues;
- mutate `.csdlc/issues/**` as lifecycle authority;
- publish pull requests or mutate GitHub;
- finish issues, close issues, or derive terminal truth;
- clean worktrees or retire v2;
- claim compatibility, migration, rollback, or authority cutover as complete.

## Focused proof commands

Use the issue-owned focused proof for the slice you are working on. Current
local construction checks include:

```sh
cargo fmt --manifest-path csdlc-v3/Cargo.toml --check
cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands
cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings
```

For docs and cutover-readiness work, use the issue-owned validators declared by
the active issue, such as the #570 stale-route and skill-guidance scans. Passing
v3 construction checks is evidence for the v3 package only; live lifecycle work
still routes through typed C-SDLC v2 until V3-F/#505.
