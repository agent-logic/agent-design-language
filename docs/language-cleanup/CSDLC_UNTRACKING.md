# Operator-requested C-SDLC untracking

The operator explicitly requested untracking all `.csdlc` content. Commit
`4787466c3c` removes all 25,524 tracked entries and adds `/.csdlc/` to
`.gitignore`. Local files are preserved; no historical Git objects are rewritten.
No exception was made to keep an authority receipt tracked under this directory.

Exact preservation anchor:

- Parent commit: `5dbcadc411b5f50928a54f396cda931395dbdbdb`
- Complete `.csdlc` tree: `cd0207ac5df53290061b272144cea42a331a64db`
- Recovery: `git show 5dbcadc411b5f50928a54f396cda931395dbdbdb:<original-path>` for an individual historical file.

## Integration dependency found after untracking

The current ADL selector points to
`csdlc-v3/operator/native-authority-receipt.json`. That receipt names
`.csdlc/evidence/505/terminal-receipt.json`.

Read-only inspection of Cognitive-SDLC source at
`f465963475670002cf27aed07fa3b7d18a3d2fe6`,
`csdlc-v3/src/authority.rs`, found two concrete requirements:

1. The terminal receipt path must equal that exact `.csdlc` path.
2. `canonical_tracked_bytes` reads the local file and compares its bytes with
   `git show refs/remotes/origin/main:<path>`.

A fresh checkout of the untracking candidate lacks the local receipt; after
merging it, canonical main also lacks the tracked receipt. Merely keeping this
worktree's ignored copy does not solve the post-merge authority dependency.
This is a source-verified compatibility dependency, not an established installed
binary regression or an authorization to weaken authority checks.

The owner needs a supported immutable Git-history receipt reference or a durable
receipt location outside local `.csdlc` state, with matching authenticated selector
and reconciliation handling. No owner implementation or binary replacement has
been performed. Cognitive-SDLC #330 remains untouched.

The public retirement guard also protects historical `.csdlc` identities. Its
final exact transition must account for this explicit removal and preservation
anchor rather than silently excluding the directory from checks. Existing #1347
preservation tests intentionally inspect their immutable merged snapshot and
must retain that historical assertion.

Status: untracking complete locally; integration dependency unresolved. Do not
claim safe post-merge native authority, final guard acceptance or complete #1351
delivery from the untracking commit alone.
