# Design: fail closed on stale C-SDLC owner binaries

## Problem

The primary-checkout bootstrap prohibition exists in current source, but an
older installed owner binary can execute without proving that it contains that
guard. This lets preparation recreate `.csdlc` issue state on `main`.

## Boundary

This issue adds one shared, pre-mutation owner-binary provenance gate and proves
that the existing primary-checkout guard remains independently effective. It
does not delete residue, change lifecycle semantics, revive v1 wrappers, or
make closeout an execution dependency.

## Authority model

Whole-repository HEAD equality is not a useful freshness test: documentation or
product commits may advance HEAD without changing C-SDLC. The single authority
is a new tracked `csdlc-v2/operator/owner-source-set.json`. It explicitly names
`csdlc-v2/Cargo.toml`, `csdlc-v2/Cargo.lock`, `csdlc-v2/src/**`,
`csdlc-v2/operator/**` (excluding the source-set file's own digest field),
`adl-resilience/Cargo.toml`, `adl-resilience/src/**`, and the embedded Gate
10B/10C evidence files. Directory entries expand only Git-tracked regular files;
symlinks, missing entries, duplicates, path escapes, non-files, and untracked
matches fail closed. Normalized repo-relative paths sort by raw UTF-8 bytes and
the digest frames each path, mode, byte length, and file bytes before BLAKE3.
The receipt stores this source-set schema/digest; its Git revision is diagnostic
only. Each mutating invocation recomputes the exact manifest digest and rejects
an unavailable or unequal result before mutation.

The receipt also binds the installed generation directory and every required
owner binary digest. Missing, extra operationally selected, partially replaced,
or digest-mismatched binaries are non-current.

## Execution design

1. Make `csdlc-v2/operator/skills.json` the complete binary/operation authority.
   Every required binary and every CLI subcommand is classified `mutating` or
   `read_only`; manifest validation rejects omissions, duplicates, and unknown
   commands. The initial mutating denominator includes `csdlc-issue create` and
   migrations/recovery, `csdlc-bind`, mutating `csdlc-edit`, `csdlc-validate
   finalize`, `csdlc-review`, `csdlc-publish`, `csdlc-finish`,
   mutating `csdlc-clean`, `csdlc-github run`, `csdlc-github-issue` writes,
   `csdlc-shadow`, `csdlc-soak`, `csdlc-proof`, and `csdlc-cutover`.
   `csdlc-schedule` and every current `csdlc-github-pr` command are explicitly
   `read_only`; other read-only commands remain explicitly classified. A parity
   test enumerates every installed Clap command and fails when any new binary or
   subcommand lacks classification. Every classified mutating route invokes one
   shared provenance preflight before its first repository-state mutation.
2. Resolve the repository root and installed receipt without creating locks,
   directories, temporary files, or request artifacts.
3. Validate receipt schema, selected generation, complete binary denominator,
   binary digests, source-set identity, and current owner-source bytes.
4. On failure, emit a credential-free diagnostic containing installed and
   expected source identities plus the repo-relative `csdlc-install` resolve and
   reinstall route; leave the target checkout unchanged.
5. Retain the independent `csdlc-issue` primary-checkout bootstrap rejection so
   a current binary still cannot initialize issue state on primary `main`.
6. Keep installation atomic: stage the complete selected generation and receipt,
   verify it, then exchange it into the stable directory as one recoverable
   operation. A partial generation is never selected.
7. Report pre-existing primary-checkout residue read-only. Never infer ownership
   and never delete or rewrite it.

Direct source/test binaries are not silently treated as installed operational
owners. Tests invoke library entrypoints or an explicit test-only harness; the
production operator route continues through `csdlc-install resolve` and the
stable generated directory.

## Proof contract

Focused tests use repository-local fixtures and snapshot a deterministic working
tree manifest before and after each rejected invocation. It excludes `.git` and
includes every other relative path, entry type, Unix mode, symlink target, and
regular-file byte digest, plus exact Git status and HEAD. They cover stale,
missing, malformed, partial, and digest-mismatched installations; the complete
topology matrix; and concurrent pre-existing residue that remains byte-identical. The
topology matrix expects: primary checkout rejected; linked worktree beneath the
tracked FastWork parent allowed; linked worktree outside that parent rejected;
standalone clone or ambiguous topology rejected for ADL issue bootstrap. A
source-set test proves unrelated repo commits do not create false staleness
while owner-source drift does.

## Stop conditions

- Any mutating installed owner can reach lock or filesystem creation before the
  shared gate.
- Freshness depends on whole-repository HEAD equality.
- A rejected invocation changes the target checkout.
- Installation can expose a mixed generation.
- The solution deletes or claims ownership of existing residue.
- The primary-checkout prohibition can be bypassed by a current owner.
