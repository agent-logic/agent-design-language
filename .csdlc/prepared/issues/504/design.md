# Issue 504 design

Status: pre-bind preparation only. Implementation is blocked until #503 is
terminal, reconciled, and ancestral.

## Scope

Issue #504 is the V3-E C-SDLC v3 remote delivery workflow slice. It prepares
the construction-only v3 surfaces that will later model exact review,
publication, finish, and cleanup behavior for requirements #174 through #178.

This packet does not make C-SDLC v3 authoritative. C-SDLC v2 remains the live
lifecycle authority until V3-F/#505 explicitly performs and proves the
authority transition.

## Dependency Gate

#503 must be terminal, reconciled by the typed v2 lifecycle, and ancestral to
the #504 execution branch before implementation starts. Before that point #504
may only carry initialized cards, design evidence, and pre-bind validation.

## Planned Workflow

After the #503 dependency is true, #504 should bind an issue worktree and add
the remote delivery model in small, reviewable surfaces:

1. Review binds exact immutable scope and cannot self-authorize publication.
2. Publication modes are explicit, including refusal when closing linkage is
   missing or downgraded.
3. Finish derives terminal truth from governed publication state rather than
   local claims.
4. Cleanup remains a separate transition and is unavailable before terminal
   truth.

The future implementation PR must visibly include `Closes #504` so GitHub
records closing linkage at merge time.

## Proof Shape

The current executable lane is limited to
`.csdlc/prepared/issues/504/validate-remote-workflow.rb`. It proves that this
initialized packet preserves the #503 dependency, the v2 authority boundary, the
v3 construction-only boundary, the V3-E acceptance denominator, and the future
`Closes #504` publication-linkage requirement.

Post-bind implementation proof should add positive and refusal tests for
requirements #174 through #178 across review, publication, finish, cleanup, and
remote command surfaces. String-only assertions are not sufficient for those
future behavior claims.
