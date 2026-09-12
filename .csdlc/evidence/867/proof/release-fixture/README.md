# Historical release fixture isolation

PR #948 run `34666179934` at `03c5aead6d1849e30f05e01c095232737999a66a`
passed the real-curl default-config regression and 86 library tests. Its release
matrix failed because the synthetic CI merge included `adl-uts/Cargo.toml`, a
later package absent from the historical v0.92.1 inventory. The failure was a
truthful production rejection of an incoherent healthy-test fixture.

The fixture now retains only Cargo inputs explicitly declared by that historical
inventory, including workspace and historical lockfile entries. A synthetic later
package and lockfile exercise this isolation on every host. After constructing
the healthy fixture, the matrix introduces an unlisted package and requires
`release_inventory_omits_manifest`, then removes it and verifies healthy behavior.
Production release validation and the tracked release inventory are unchanged;
this fixture does not approve today's repository as a v0.92.1 release candidate.

Focused release-matrix validation passed, including its linked-worktree and
nonmutation cases. Strict all-target Clippy, formatting and diff checks passed.
This test-only correction does not rerun or relabel the previous 248-test local
run; the curl-config supplement retains that source-bound result and candidate
corpus. Production source equality with `03c5aead...` is checked independently.

`index.json` binds the new source revision, changed-file hashes and retained logs.
The original CI failure excerpt is retained as failure evidence. Log display
prefixes and trailing blank lines are normalized with original hashes retained.
Fresh current-head CI and independent review are still required before treating
the updated PR as ready. No merge, issue closure or live activation is claimed.
