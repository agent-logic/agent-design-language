# Issue 5911 Design: FastWork Storage Recovery And Worktree Placement

## Outcome

Archive the material local Codex transcript store directly to FastWork with a
checksum manifest and no deletion, and enforce the canonical ADL issue-worktree
parent `/Volumes/FastWork/adl-worktrees` in typed C-SDLC binding.

## Implementation

1. Add one canonical FastWork worktree-root policy to the v2 binding path.
2. Resolve and canonicalize requested worktree paths before topology mutation.
3. Refuse paths outside `/Volumes/FastWork/adl-worktrees` with a typed error.
4. Add focused positive and negative binding tests.
5. Align root operator policy and binding guidance with the enforced path.
6. Diagnose local transcript usage without inspecting `/private/tmp`, archive
   the selected transcript store directly to FastWork, and verify a manifest.

## Safety

- Do not delete transcripts or existing worktrees.
- Do not inspect or use `/private/tmp`.
- Do not expose transcript contents or credentials.
- Archive evidence records paths, sizes, timestamps, and digests only.

