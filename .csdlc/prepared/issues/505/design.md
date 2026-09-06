# Issue 505 design

Status: pre-bind preparation only. Implementation is blocked until #504 is
terminal, reconciled, and ancestral.

## Scope

Issue #505 is the V3-F C-SDLC v3 authority-transition decision slice. It
prepares one operator-reviewed decision record for requirements #179 and #180:
v2-v3 parity, migration canary, rollback, observation evidence, and explicit
operator disposition.

This packet does not cut over authority. C-SDLC v2 remains the live lifecycle
authority unless and until #505 implementation records the required evidence
and the operator explicitly approves the transition.

## Dependency Gate

#504 must be terminal, reconciled by typed v2 closeout, and ancestral to the
#505 execution branch before implementation starts. #505 may not retire v2,
change live operator guidance, or claim v3 authority from planned evidence.

## Planned Decision Workflow

After the #504 dependency is true, #505 should bind an issue worktree and build
one reviewable authority-transition decision:

1. Map retained requirements #179 and #180 to explicit proof lanes.
2. Measure v2-v3 parity with a machine-readable parity matrix.
3. Exercise a migration canary and rollback path before any approval claim.
4. Record an observation window with non-claims and residual risks.
5. Require explicit operator approval before cutover or retirement.

Any future implementation PR must visibly include `Closes #505` so GitHub
records closing linkage at merge time.

## Proof Shape

The current executable lane is limited to
`.csdlc/prepared/issues/505/validate-authority-transition-prep.rb`. It proves
that this initialized packet preserves the #504 dependency, the v2-live
boundary, the no-silent-retirement rule, the explicit operator approval gate,
and the future `Closes #505` publication-linkage requirement.

Post-bind implementation proof should be behavioral. String-only checks are not
sufficient for parity, rollback, observation, or authority-transition claims.
