# #629 V3-H.3 GitHub, PR, and publication routes

## Intent

Implement the construction-only v3 GitHub/publication command slice under the single `csdlc` binary while v2 remains the live operational authority until #505.

## Scope

- Own the v3 routes `github`, `github-issue`, `github-pr`, `pr-state`, `publish`, and `review`.
- Preserve typed review assignment, review result, publication, and PR-state readback concepts.
- Make every route machine-readable, redacted, and non-authoritative before #505 cutover.
- Strengthen real issue canaries enough to prove closing linkage and caller-forged readback rejection.

## Non-goals

- No merge, finish, cleanup, install, proof, soak, or cutover authority.
- No v2 source edits.
- No raw `gh` lifecycle writes.

## Validation

- Issue-owned validator for manifest ownership, authority boundary, raw-gh denial, and v2-source hygiene.
- Focused v3 remote/publication tests.
- Real issue canary for PR/readback behavior.
- Full v3 regression, rustfmt, clippy, typed issue validation, and exact-range diff hygiene.
