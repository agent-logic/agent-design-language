# #256 Design: Birthday demo after Observatory sprint

## Intent

Issue #256 is the current-repository successor for legacy #5836. It turns the preserved first-birthday packet and validation work into a terminal birthday demo only after the Observatory surface can visibly carry the demo truth.

The immediate purpose of this initialized package is preparation, not execution. It records the current Sprint 5 gate truth: legacy #5836 local/native packet work is an input, but #256 cannot claim terminal birthday-demo completion while the Observatory dependency and AWS/public runner prerequisite remain unsatisfied.

## Boundary

Owned current issue surfaces:

- `.csdlc/issues/256`
- `.csdlc/prepared/issues/256`
- `.csdlc/evidence/256`
- future #256-bound demo/publication surfaces only after bind

Read-only inputs:

- legacy #5836 worktree and retained evidence
- current #110/#84 Observatory state
- current #345 AWS GPU Shepherd proof-runner state
- current #341 and #343 Sprint 5 successors

Non-goals for this preparation slice:

- no #271 work
- no #84/#110 execution
- no #345/AWS spend or launch
- no #341 provider-neutral execution
- no source changes
- no terminal birthday-demo claim from packet evidence alone

## Gate semantics

The birthday demo has two tiers:

1. Local/governed packet tier: legacy #5836 evidence can seed exact packet, validator, local/native proof, and non-claim boundaries.
2. Terminal demo tier: #256 acceptance requires a working Observatory presentation surface and, for public/AWS variants, the #345 proof-runner lane.

This package fails closed on the common overclaim: local packet generation is not terminal birthday-demo completion.

## Execution plan

After this initialized package receives design review:

1. Reconcile the exact legacy #5836 evidence that remains valid on current main.
2. Verify Observatory dependency truth without touching #271 or sibling Observatory implementation work.
3. If gates are still blocked, keep #256 initialized/ready with an explicit stop condition rather than binding implementation.
4. Once Observatory/#345 gates are terminal as required by the live issue body, bind a dedicated FastWork #256 worktree and implement only the current issue’s demo/publication lane.

## Review focus

Review should verify that this preparation package:

- makes #256 the sole current authority for legacy #5836 successor work,
- preserves legacy #5836 as input evidence only,
- blocks terminal acceptance on Observatory/public proof gates,
- avoids #271 and other non-Sprint-5 implementation work,
- leaves #341/#343 serialized behind #256.
