# #256 readiness refresh — 2026-08-19

## Scope

This packet refreshes dependency/readiness truth for current-repository #256 after the local HTML Observatory startup gate landed. It records current input/gate truth for the implemented local birthday-demo-after-Observatory packet without claiming public/AWS launch, Unity proof, or terminal closeout.

## Current live dependency truth

- #110 is closed and no longer blocks #256 readiness as an Observatory coordination parent.
- #414 is closed via PR #422 merge `b7828c455234b42f83bf30c9ad2c6790c5cc635d`; that merge is ancestral to current `origin/main` as observed during the #256 refresh.
- #84 is backlog and depends on #122/#251; Unity/TLS live proof is not an active v0.92 #256 gate.
- #345 is backlog and owns public/AWS GPU Shepherd proof-runner prerequisite work for public/AWS demo variants only.
- #424 is merged/terminal/canonical for the local CSMctl/HTML Observatory startup surface, and #256 consumes that local Observatory startup surface as an input gate.
- #341 remains serialized behind terminal #256.
- #343 remains serialized behind terminal #256 and #341.

## Typed card repair attempt

The following earlier typed edit attempts were preserved under `.git/csdlc-v2/requests/` and rejected by the owner binary because #256 was already in `bound` phase:

- `issue256-edit-01-sip-required-outcome.json` -> `invalid_transition`: `sip mutation is not allowed during bound`
- `issue256-edit-02-stp-dependencies.json` -> `invalid_transition`: `stp mutation is not allowed during bound`
- `issue256-edit-03-spp-plan-summary.json` -> `invalid_transition`: `spp mutation is not allowed during bound`

That earlier rejection is historical evidence only. The implemented-phase recovery route later repaired SIP/STP/SPP truth through supported typed lifecycle operations; this evidence packet now records the current post-#424 dependency truth without bypassing typed cards.

## Non-claims

- No public/AWS birthday launch was performed.
- No #341 or #343 provider/release proof was performed.
- No AWS launch, spend, or credential use occurred.
- No #84 Unity/TLS evidence was synthesized or claimed.
- No HTML/Polis, Unity, or sibling Observatory implementation paths were absorbed.

## Local validation

- `python3 .csdlc/evidence/256/validate_preparation_gate.py` passed and reported `schema=adl.issue256.preparation_gate.v1`, `status=passed` after post-#424 readiness repair.
- `python3 adl/tools/validate_issue256_birthday_after_observatory.py --root /Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory` passed and reported `schema=adl.issue256.birthday_after_observatory.v1`, `status=passed`.
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root /Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory issue --issue 256` passed with implemented-phase issue truth during the post-#424 #256 proof lane.
- `git -C /Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory diff --check` exited 0 with no output.

## Remaining gate

#256 is implemented for the narrowed local HTML Observatory birthday packet. Remaining gates are fresh exact-head review, typed publication, CI, and finish. Public/AWS launch, Unity/#84, #341, and #343 remain out of scope for this packet.
