# Issue 526 design — operator-authorized release ceremony

## Result

One ceremony receipt proving that the v0.92.1 tag, release, and notes resolve to
the exact operator-approved candidate. This is the sole release mutation unit.

## Pre-mutation gate

After #525 has a reviewed merge, freeze the candidate and prove every preceding
tail merge is reviewed, green, and ancestral. Finalize truthful notes, then
obtain explicit operator authorization naming the exact candidate, tag, and
release operation. Any drift invalidates authorization and returns to preflight.

## Ceremony and readback

Use the typed release owner route only after the gate passes. Immediately read
back tag target, release identity, notes, timestamps, and hashes into an
immutable issue-owned receipt. Typed finish and worktree cleanup remain later
asynchronous bookkeeping.

## Guardrails

- No feature work or unreviewed merge.
- No tag or release mutation without exact authorization.
- Stop on missing ancestry, identity drift, or unavailable exact readback.
