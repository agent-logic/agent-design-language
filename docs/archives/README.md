# Historical archives

The public repository was compacted to keep fresh clones small. Original Git
history and relocated files remain in verified private Agent Logic archives;
public builds need no archive access. Start with the
[full history and contributor continuity guide](PUBLIC_HISTORY_1382.md) or the
[milestone archive catalog and restore guide](HISTORICAL_FILES_1395.md).
Ask an Agent Logic maintainer for authenticated access to the exact archive
version. Preserve existing checkouts and restore into a new directory.

## Historical binary payloads

Completed Runtime #5820 and C-SDLC #872 binary payloads are preserved in the
archived ADL commit `2f26936b9c0ec582471a3f8fbebcce31b471512e`. Their logs,
receipts and manifests remain in the checkout. The original binaries are
historical evidence, not installed executables or current product qualification.

[The archive index](POST_SPLIT_PAYLOADS_1284.json) records the original paths,
Git blob identities, sizes and modes. All 14 files were restored from that Git
commit and compared byte-for-byte and by mode before removal from the checkout.
Four obsolete command stubs are indexed in the same archive after separate
restoration checks. Their only behavior was to refuse retired routes. The
binary relocation removes 112,224,007 bytes from a full checkout; it does not shrink Git
history. The original source commit is retained in the full-history archive.

To inspect or replay an old proof, restore its original environment in a separate
scratch directory on the build volume. First restore the [full-history bundle](PUBLIC_HISTORY_1382.md). Then, from
that restored Git repository containing the source commit:

```bash
mkdir -p /Volumes/FastWork/adl-historical-proof
git archive 2f26936b9c0ec582471a3f8fbebcce31b471512e \
  .csdlc/evidence/5820 .csdlc/evidence/872 |
  tar -x -C /Volumes/FastWork/adl-historical-proof
```

Do not rely on fetching an old commit from the compact public origin; use the
verified full-history archive. Historical receipt paths refer to this restored layout. Do not change
old receipts to claim that the archived executables are current installations.

Current authority selectors, admission receipts, active issue records and
v0.93.1 release evidence are not included in this archive removal.
