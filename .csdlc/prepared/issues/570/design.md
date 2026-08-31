# Issue 570 design

Status: design approved and implementation finalized.

Issue #570 is a documentation and operator-guidance readiness slice for the
C-SDLC v3 cutover. It does not move lifecycle authority to v3. The design keeps
C-SDLC v2 as the live bind, validation, review, publication, finish, cleanup,
and GitHub-mutation authority until explicit V3-F/#505 approval.

The implementation updates the root and onboarding guidance, the checked-in v2
operator guidance, the checked-in v2 operator skill documents, the v3 package
README, and the local installed PR-janitor guidance. The local installed skill
path is recorded as local-only evidence and is not part of the repository
commit.

Proof is intentionally narrow and deterministic:

- stale-route docs scan;
- skill guidance scan over checked-in and installed guidance surfaces;
- authority-boundary scan that rejects contradictory pre-cutover v3 authority
  claims;
- diff hygiene.
