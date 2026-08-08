# Sprint Readiness Helper Typed-v2 Compatibility Gap

Observed during Sprint `#5862` startup on 2026-08-08.

## Result

`check_sprint_readiness.py` correctly reported:

- installed `sprint-conductor` parity matched the tracked bundle;
- the hybrid Sprint Execution Packet was present and structurally complete;
- the review and activity-log paths were declared and present.

It nevertheless classified every child `#5863` through `#5878` as blocked
with `No local task bundle found`, recommending `pr-init`.

## Cause Boundary

The helper's structured-prompt discovery searches the historical `.adl`
task-bundle layout. The current repository authority is typed C-SDLC v2, and
each child already has its six canonical cards and index under
`.csdlc/issues/<issue>/`. Independent typed `csdlc-doctor` checks for all
sixteen children passed before execution began, and the issue-wave validator
passed with sixteen approved unbound children and 38 exclusive owned paths.

## Disposition

- Treat the helper result as a sprint-conductor compatibility defect, not as
  authority to reinitialize or duplicate current typed-v2 issue records.
- Preserve the generated `sprint-state.json` as the exact failing evidence.
- Continue to gate child execution on typed-v2 doctor, live dependency truth,
  exact path ownership, and `validate-implementation-wave.rb`.
- Route the helper adaptation as `post_sprint_follow_on`; do not widen a
  Distributed Guardian product child into lifecycle-tooling repair.

