# Public-readiness retention decisions

The [#1325 inventory](PUBLIC_READINESS_1325.json) pins the reviewed source revision,
logical checkout sizes and all 44 original unbuilt v2-package file identities (30 retained, 14 archived).
The [current roadmap](../ROADMAP.md) provides the visitor-facing entrypoint;
historical records remain accessible as dated evidence.

## Keep and archive boundaries

- Keep current public source, schemas, licenses, selectors, receipts and active
  sprint work. The new sprint-2 preparation packet is active, not cleanup residue.
- Keep the CLI, engine and records trees while the characterization shadow
  manifest cites their source and proof identities. Archive the 14 adapter files separately:
  no current public-workspace consumer was found, and exact published Git
  preimages provide restoration custody. Their build status is now
  explicit in [adl-v2](../../adl-v2/README.md).
- Keep authenticated conversion/recovery evidence, including byte-identical
  snapshot copies. Copies represent separate checkpoints; deleting one solely
  because its contents match another would break its original evidence address.
- The [659 tracked lifecycle snapshots](LIFECYCLE_SNAPSHOT_CLASSIFICATION_1325.json)
  include 255 recorded `closed_out` states and 404 nonterminal recorded states.
  These are historical snapshots, not current authority. Treat these directories as candidates requiring individual
  native-state, consumer and custody checks, not blanket deletion authorization.
- The [#1284 payload archive](POST_SPLIT_PAYLOADS_1284.json) already records the
  retired large binaries. Podcast media has moved to its separate owner. This
  cleanup does not repeat either migration.

The original [#1223 guard](../../tools/source_retirement/README.md) rejects new
baseline deletions outside its reviewed execution set. Its original identity checks and approval packets remain unchanged. The new
[exact successor transition](../../tools/public_readiness_archive/transition.json)
validates the 14 adapter removals and one guard-call insertion before passing
restored historical identities to the existing guards. It admits no other deletion.
Future batches likewise need exact preimages, consumer disposition, custody,
restoration comparisons and a reviewed guard transition; an inventory alone is
not such approval. No history rewrite or mass removal is implied by repository size.

## Restoration and measurement

For a retained entry, retrieve its exact original bytes with
`git show <source_commit>:<path>` using the register's published source commit.
Compare the resulting SHA-256, Git blob identity and mode with the entry before
using it as restoration evidence. A shallow clone may need to fetch that source
commit first. Do not treat a missing object or a local path as durable custody.

The bounded batch removes **14 files / 91,054 logical bytes** of unselected
adapter source. The new inventory/guard documentation has its own size; the final
net checkout delta is measured separately in delivery evidence. Logical bytes
are not Git transfer size: deleting tracked files from the default tree
would not remove their objects from history. A removal decision must report both
what changed and what remains protected rather than claiming a percentage of
historical material is safe to delete.
