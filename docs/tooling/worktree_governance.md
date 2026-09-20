# Worktree Governance

This policy separates issue-worktree closeout from reclaiming build output.
Directory location, a familiar name, or a cleanup helper's recommendation alone
never authorizes deletion. Preserve active work, unpublished changes, lifecycle
evidence and recovery records.

## Canonical namespaces and authority

- **Primary checkout:** the normal repository root, kept clean on `main` for
  inspection and native preparation; no issue implementation artifacts there.
- **Managed issue worktrees:** new bindings use the `required_parent` in
  [`.adl/worktree-policy.json`](../../.adl/worktree-policy.json), currently
  `/Volumes/FastWork/adl-worktrees`. Native `csdlc bind ISSUE` creates or confirms
  the registered issue branch/worktree; current paths commonly use
  `adl-issue-<issue>-<slug>`. Resolve the binding rather than guessing a path.
- **Historical namespaces:** `.worktrees/adl-wp-*`, `.worktrees/adl-lane-*` and
  external `$HOME/git/adl-wp-*` / `adl-lane-*` remain retained history, not the
  default for new execution. A replacement directory does not prove that the
  older copy's commits, dirty files or evidence are dispensable.
- **Codex ephemeral worktrees and foreign directories:** preserve their ownership;
  neither is implicitly part of an ADL cleanup batch.

[AGENTS.md](../../AGENTS.md) and [current native v3 authority](../csdlc-v3/CURRENT_AUTHORITY.md)
govern binding and lifecycle operations. Typed v2 binding and the old `pr.sh`
routes are historical; they are not alternatives when a current guard fails.

## Classification and fate

The policy maps registered FastWork issue worktrees to the existing
`managed_registered` concept; it does not introduce a new executable class.
The shell helpers' actual labels are described separately below. Classification
must establish repository identity, exact registration/binding, branch and head,
active ownership, dirty state, and terminal/merge evidence.

| Surface / class | Required fate |
| --- | --- |
| `primary_checkout` | `keep_primary`; never a cleanup target. |
| Registered managed issue worktree, including FastWork | `keep_active` while owned or needed; `keep_dirty_active` and review unpublished state. Closed/merged and clean is only a candidate for native terminal reconciliation and cleanup. |
| Historical `managed_clone` or `legacy_external` / `legacy_external_registered` | Review ownership, unique commits and replacement evidence. Preserve meaningful state; replacement or a matching basename alone is insufficient. |
| `managed_scratch` | Review contents and active users. `remove_scratch_clean` is an advisory candidate, not proof of disposability. |
| `stale_registration` | Verify the exact path is genuinely absent, not an unavailable mount; preserve recovery information before any separately authorized metadata pruning. |
| `codex_ephemeral` | `ignore_ephemeral` for ordinary ADL milestone cleanup; separate explicit scope is needed for Codex cleanup. |
| `foreign_excluded` | `ignore_foreign`; do not delete another project's files. |
| `orphan_dir`, including an unregistered FastWork issue-looking directory | `review_orphan` / `review_orphan_clean`; inspect identity and recover or retain meaningful state. An unregistered path is not a native clean target. |
| `other_registered` / `temporary_registered` | Review exact ownership and purpose. A temporary path or `prune_now` hint does not authorize removal of a live checkout. |

For dirty merged worktrees, the old `backup_then_remove` recommendation means
**preserve and review first**, not automatic removal after making any backup.
Retain unpublished commits, patches and required artifacts in a durable location,
verify that preservation, and obtain the appropriate native disposition. A
closed issue or green PR alone does not establish cleanup eligibility. Ignored
build files can coexist with a Git-clean status, so inspect retained evidence and
active users as well as Git's dirty flag.

## Supported closeout route

Use the installed native v3 owner selected under the repository contract, normally
`.adl/bin/native-v3/csdlc`. See the [operator workflow](../csdlc-v3/man/man1/csdlc-workflow.1)
and [clean manual](../csdlc-v3/man/man1/csdlc-clean.1).

1. Reconcile the exact closed issue and merged delivery, or an explicitly
   authorized no-PR disposition, through `csdlc finish ISSUE`.
2. Run `csdlc clean ISSUE` to obtain a read-only preview. Retain and review its
   exact target, head, evidence/archive treatment and preview token before removal.
3. Only for that approved unchanged target, use
   `csdlc clean ISSUE --execute --preview TOKEN`. Cleanup is separate from finish.

Native guards and receipts determine eligibility. Dirty, active, unregistered,
identity-mismatched or protected paths require resolution; do not bypass a denial
with shell pruning. Never move an issue's native index out of its worktree before
finish to make it look clean. Native cleanup owns permitted archival of generated
issue records; preserve required evidence before any removal.

**Example:** a registered `adl-issue-1086-fastwork-worktree-governance` under the
approved FastWork parent is policy-managed. While #1086 is active or has meaningful
local changes, keep it. After actual terminal reconciliation and a successful
reviewed native cleanup preview, it can become removable. Its name alone proves
none of those conditions.

## What the legacy shell helpers actually report

[`worktree_doctor.sh`](../../adl/tools/worktree_doctor.sh) observes registration,
local `main` ancestry and dirty state. It does not authenticate native terminal
receipts. Both it and [`worktree_prune.sh`](../../adl/tools/worktree_prune.sh)
accept `--managed-root`; their default remains `<repo>/.worktrees` and does not
read `.adl/worktree-policy.json`. Supply the **primary** repository explicitly:

```bash
# Read-only classification; the explicit root changes the scan, not authority.
bash adl/tools/worktree_doctor.sh --repo /path/to/primary \
  --managed-root /Volumes/FastWork/adl-worktrees --format tsv
```

The helpers do not implement the full current policy. Source inspection and a
read-only local fixture for #1086 establish these distinctions:

| Fixture path | Default doctor classification | With explicit FastWork managed root |
| --- | --- | --- |
| Registered `adl-issue-123-example` | `other_registered` / `review_other` | Still `other_registered` / `review_other` |
| Registered `adl-wp-124` | `other_registered` / `review_other` | `managed_registered`; clean local-main ancestry produces `remove_merged_clean` |
| Unregistered `adl-issue-125-unregistered` under that root | Not scanned by this fixture's default managed-root scan | `managed_scratch`, not automatically `orphan_dir` or `foreign_excluded`; fixture fate was `review_orphan` |

Only `adl-wp-*` and `adl-lane-*` match the registered managed-name branch.
Unregistered other names fall through to scratch handling; depending on the
observed dirty state this can recommend `remove_scratch_clean`. Git discovery in
an unregistered subdirectory can inherit an enclosing checkout's state. Do not
interpret either scratch classification as evidence that an issue directory is
safe to remove. Foreign-name exclusions are also limited heuristics, not verified
repository ownership.

`worktree_prune.sh` forwards `--managed-root` to the doctor. Its default mode is a
dry-run and `--report PATH` writes a report; `--apply` performs deletion/pruning.
Its selection uses helper fates, local merge checks and an issue-state lookup:
known open issues exclude ordinary merged-clean candidates, but an unknown issue
state is not a universal stop. `--include-scratch` and
`--include-legacy-external` broaden selection; these flags do not supply native
terminal authority. `--limit` is not an exact-target native preview token.

Use these scripts as legacy inventory aids, not as the supported native managed
issue cleanup executor. Keep report-before-delete and conservative dirty/orphan/
foreign handling. The namespace-classification gap is a bounded executable
follow-up: recognize policy-managed issue paths and fail closed on unregistered
issue-looking directories. #1086 documents the observed gap; it does not repair
the scripts or validate their apply mode. No pruning or deletion was run for this
change.

## Build targets, temporary data and caches

FastWork also holds Rust targets, `TMPDIR` contents, shared caches and Unity
staging. These are a separate reclamation surface, not issue worktrees merely
because they share a volume.

| Data classification | Fate |
| --- | --- |
| Disposable/rebuildable output | Candidate for separately scoped reclamation only after confirming ownership, no active users, reproducible inputs and no required evidence. |
| Active/in-use output or shared cache | Keep while builders, editors, runtimes or other sessions use it; coordinate with all affected owners. Do not purge by age or size alone. |
| Retained evidence or recovery data | Preserve validation logs/results, receipts, session archives, snapshots, patches and recovery records. They are not disposable build caches, even inside a target or temporary directory. |

For example, an idle issue-local `csdlc-v3/target` with reproducible source and
all required proof copied to durable evidence may be rebuildable. An in-use
shared target stays; a saved diagnostic or recovery packet stays regardless of
its directory name. Stable installed owner binaries and provenance must not be
removed as Cargo build output.

Account for shared caches and [hardlinked dependency artifacts](HARDLINKED_RUST_DEPENDENCY_CACHE.md):
logical directory sizes do not equal reclaimable physical bytes. Other hard links,
filesystem clones or retained snapshots can keep blocks allocated. Do not promise
reclaimed space from a sum of directory sizes, and do not modify a shared inode's
contents to reclaim another path.

The [developer fast lane](DEVELOPER_THROUGHPUT_FAST_LANE.md) describes FastWork-required
placement for worktrees, `TMPDIR` and build output; placement is not purge authority.
Its v2 lifecycle wording is historical and subordinate to current v3 policy.
Likewise the [historical ADL v2 installer](../../adl-v2/tools/install-adl-v2.sh)
uses `CARGO_TARGET_DIR` or `/Volumes/FastWork/adl-v2/target` but installs binaries
and receipts separately; it does not define current worktree governance.

[Unity editor/batch guidance](unity_observatory_editor_batch_proof.md) separates
project ownership and FastWork staging. The [ILPP diagnosis](unity_ilpp_getdomainname_diagnosis.md)
retains issue-local logs and distinguishes licensed imported assets from generated
`Library` state. Keep editor/runtime users, asset recovery sources and evidence
intact; neither all Unity staging nor an Asset Store cache is automatically
rebuildable or disposable.
