# Historical milestone archive (#1395)

Most pre-v0.93.2 milestone records are archived rather than included in every
public clone. Current/future plans and the small historical fixtures still used
by public source and tests remain in the tree. Retained source/fixture bytes are
unchanged. This is documentation relocation, not a new product acceptance claim.

## Preservation and custody

Archive identity: `agent-design-language/1395/20261030`.
The exact public baseline is `28d5b6681b52f2d3d7c80847438cc3cdfaaa6724`.
Its complete advertised graph contains 12,945 reachable objects. A fresh mirror
restore matched all advertised refs and reachable objects and passed Git fsck.
A separate milestone archive restored all 6,660 files with matching Git blob
identities and executable modes. Neither operation overwrote an existing checkout.

- Complete public graph: `public-before.bundle`, 44,059,852 bytes,
  SHA256 `d7118960f2998a62250df4eab9194c5da3cd4c3a96751745fb237c85013d8741`.
- Complete baseline milestone tree: `milestones-before.tar.gz`, 32,695,794 bytes,
  SHA256 `fa94103e15d326dca8d40b4499eab10ce324b82c03050712d8194652f85a2e3e`.

Both files have checksum-verified exact versions in the encrypted,
versioned private Agent Logic backup bucket, with all public access blocked.
Maintainers hold the exact bucket/key/version receipt. Versioning is not WORM
retention; never replace an exact-version lookup with an unversioned latest key.
No prior archive, release asset, legacy repository or developer checkout was deleted.
The complete older pre-cutover history remains in the separate
[#1382 archive](PUBLIC_HISTORY_1382.md).

## Find and restore a historical path

The [compressed file catalog](HISTORICAL_FILES_1395.json.gz) lists each relocated
path with its original Git blob, mode and byte count. Original provenance-manifest
SHA256 values are included unchanged. For example, inspect it without private access:

```sh
python3 -c 'import gzip,json; d=json.load(gzip.open("docs/archives/HISTORICAL_FILES_1395.json.gz")); print("\n".join(d["files"]))'
```

Ask an Agent Logic maintainer for the exact archive version. Verify its SHA256,
then restore into a **new directory**. Do not reset or overwrite an existing
checkout:

```sh
git clone --mirror /path/to/public-before.bundle historical.git
git --git-dir=historical.git fsck --full
mkdir restored-files
git --git-dir=historical.git archive 28d5b6681b52f2d3d7c80847438cc3cdfaaa6724 docs/milestones/v0.93.1 | tar -x -C restored-files
```

This restores the original directory hierarchy, including relative links and
records that are no longer current documentation. Older original Git SHAs may
require the #1382 archive instead of the newer public-only bundle. Preserved
historical fixture citations retain their original context and can be resolved
in the restored tree; they do not require private access for public builds.

## Current validation versus historical audit

The public retirement boundary still rejects the same retired paths and prefixes
and requires the same public source files. It verifies retained current manifests
and a pinned catalog containing every original archived-manifest checksum.
It does **not** claim to reread archived bodies or rerun the original historical
retirement audit. That audit runs explicitly against its preserved checkout and
custody archives. Public build/CI needs no sibling repository or archive download.

The archive catalog does not authorize new source deletions or turn historical
approval records into current operational authority. Changes to catalog pins or
retirement boundaries require independent review and focused negative tests.

## Public history transition

Removing files in a commit reduces the checkout but does not remove old blobs
from reachable Git history. A separate, independently reviewed exact-ref cutover
is required before claiming the ordinary clone is smaller. Preserve release
identity/assets, record old/new ref mappings and rollback custody, reject changed
remote refs, and verify a fresh ordinary clone after the transition. Existing
checkouts retain their history; preserve local work and use a new clone after
announced history changes.
