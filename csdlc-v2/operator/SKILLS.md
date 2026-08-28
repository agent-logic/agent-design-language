# C-SDLC v2 operator skills

The eleven skills in `skills.json` are thin typed routes. Skills select a
binary/subcommand, collect typed input, and display typed output. They never
edit Markdown, mutate canonical state directly, invoke shell/Python lifecycle
logic, or infer success from prose.

The tracked `generation-selector.json` is the sole default authority. Gate 10C
cutover is complete; Gate 10D2 records exact parity approval and the reviewed
final `v1_sunset` inventory. Install only into `.adl/bin/csdlc-v2/`, never
shared `.adl/bin/`. `csdlc-install verify` fails unless the regular executable
v2 binary set and provenance are complete, and the v1-sunset inventory remains
exactly matched.

The read-only doctor route also installs `csdlc-eligibility` as an auxiliary
operator binary. It may write only its requested decision output; it has no
authority to mutate candidate v1 paths or execute an eligible manifest.

The legacy AST importer remains only as an internal, one-way parity fixture for
Gate 8 tests. The `csdlc-import` executable and operator route are sunset; no
new lifecycle operation may invoke the importer after Gate 10D2.

For pull requests created before the v1 sunset, follow
`docs/tooling/C_SDLC_V2_V1_ORIGIN_PR_TAIL_PLAYBOOK.md`; do not revive the
historical v1 command surface.

## V3 replacement posture

These operator skills are the current lifecycle authority only until an
explicit operator-reviewed v3 cutover replaces them. During v3 construction,
keep routing live lifecycle writes through the typed v2 binaries and use
`csdlc-v3/**` only as a non-authoritative implementation and proof surface.

The intended cutover is a clean replacement, not a permanent compatibility
layer. When V3-F or its successor proves parity/import, rollback, publication,
finish, cleanup, documentation, and operator-start ergonomics, retire these
active v2 skill routes as one coordinated, approval-gated update while
preserving Gate 10A-D evidence as history.
