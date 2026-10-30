# Historical binary payloads

Completed Runtime #5820 and C-SDLC #872 binary payloads are preserved in the
published ADL commit `2f26936b9c0ec582471a3f8fbebcce31b471512e`. Their logs,
receipts and manifests remain in the checkout. The original binaries are
historical evidence, not installed executables or current product qualification.

[The archive index](POST_SPLIT_PAYLOADS_1284.json) records the original paths,
Git blob identities, sizes and modes. All 14 files were restored from that Git
commit and compared byte-for-byte and by mode before removal from the checkout.
Four obsolete command stubs are indexed in the same archive after separate
restoration checks. Their only behavior was to refuse retired routes. The
binary relocation removes 112,224,007 bytes from a full checkout; it does not shrink Git
history. Do not rewrite history or remove the retained source commit.

To inspect or replay an old proof, restore its original environment in a separate
scratch directory on the build volume. For example, from an ADL clone containing
the source commit:

```bash
mkdir -p /Volumes/FastWork/adl-historical-proof
git archive 2f26936b9c0ec582471a3f8fbebcce31b471512e \
  .csdlc/evidence/5820 .csdlc/evidence/872 |
  tar -x -C /Volumes/FastWork/adl-historical-proof
```

A shallow clone may first need to fetch that exact commit from the canonical
origin. Historical receipt paths refer to this restored layout. Do not change
old receipts to claim that the archived executables are current installations.

Current authority selectors, admission receipts, active issue records and
v0.93.1 release evidence are not included in this archive removal.
