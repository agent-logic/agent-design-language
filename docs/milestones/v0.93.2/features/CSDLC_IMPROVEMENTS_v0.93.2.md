# C-SDLC improvements for v0.93.2

## Existing C-SDLC improvements

These four existing issues are included in v0.93.2 without duplication. The operator approved the plan and proceeding with C-SDLC #296–299 first. Other work remains unactivated.

- [C-SDLC #296](https://github.com/agent-logic/cognitive-sdlc/issues/296): Use one editable issue source and remove projection-drift gates — planned Sprint 1.
- [C-SDLC #297](https://github.com/agent-logic/cognitive-sdlc/issues/297): Preserve proof and review across metadata-only edits — planned Sprint 1.
- [C-SDLC #298](https://github.com/agent-logic/cognitive-sdlc/issues/298): Remove mandatory portable-review attestation and publication receipt gates — planned Sprint 1.
- [C-SDLC #299](https://github.com/agent-logic/cognitive-sdlc/issues/299): Simplify lifecycle commands with optional start and consolidated delivery — planned Sprint 1.

#299 depends on #296, #297 and #298. These run first so later work benefits from the simplification; CodeFriend assessment depends on #299. The milestone quality gate includes their completion. Existing #304 remains an additional separately tracked remount-reliability fix; it is not one of these four improvements. CI #205 is required independently; see [CI normalization](CI_NORMALIZATION_v0.93.2.md). The four issues have no GitHub milestone assigned at this planning snapshot; metadata reconciliation is outstanding.

## Acceptance authority

The linked existing issues retain their complete acceptance, compatibility and validation requirements. No shared binary replacement, Runtime restart, deployment, duplicate issue creation or additional state machinery is part of this planning change. CI distinguishes code from non-paper documentation and excludes papers.
