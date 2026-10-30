# Operator-requested C-SDLC untracking

The operator explicitly requested untracking all `.csdlc` content. Commit
`4787466c3c` removes all 25,524 tracked entries and adds `/.csdlc/` to
`.gitignore`. Local files are preserved; no historical Git objects are rewritten.
No exception was made to keep an authority receipt tracked under this directory.

Exact preservation anchor:

- Parent commit: `5dbcadc411b5f50928a54f396cda931395dbdbdb`
- Complete `.csdlc` tree: `cd0207ac5df53290061b272144cea42a331a64db`
- Recovery: `git show 5dbcadc411b5f50928a54f396cda931395dbdbdb:<original-path>` for an individual historical file.

## Corrected authority assessment

The initial assessment mistook a historical relocation reader for ordinary
operational authority. Inspection of the full call path at Cognitive-SDLC source
`f465963475670002cf27aed07fa3b7d18a3d2fe6` shows `canonical_v3_authority` and
`context_authority` use installed-tool admission. The hard-coded #505 receipt
reader is used by `historical_relocation_binding`; it is not the ordinary path.

The installed executable
`/Volumes/FastWork/agent-logic-tools/csdlc/11786371585/installed/bin/csdlc`
was tested with the local `.csdlc/evidence/505/terminal-receipt.json` temporarily
absent. `validate 1351 --json` completed, reported `operational_authority: true`,
and passed lifecycle digest and six-card validation. The local receipt was
restored immediately afterward. The exact output is retained in the issue's
local metadata as `validate-without-legacy-receipt.json`.

No owner code or binary replacement is needed for the observed ordinary route.
Explicit historical relocation retains its old requirements and is not claimed
as qualified without its historical inputs. Cognitive-SDLC #330 is untouched.

## Retirement guard

`csdlc-untracking.json` pins the identical complete directory tree in merged
commit `1da95b799d8bbbfbedab72bd6b6b2c786ea349f3`, so custody does not depend on
an unmerged implementation commit. The guard verifies the pinned manifest,
source ancestry, tree identity, entry count and absence of every tracked
`.csdlc` path, then restores historical identities only in its in-memory audit
view. No local file is rewritten. Unrelated changes remain visible to existing
checks; historical #1347 snapshot assertions remain intact.

Status: untracked locally; installed ordinary validation passed without the legacy
receipt. Full #1351 source retirement admission and delivery remain unfinished.
