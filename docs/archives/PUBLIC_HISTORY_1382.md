# Public history archive and cutover contract (#1382)

The full original advertised Git graph is preserved outside the public repository.
Archive identity: `agent-design-language/1382/approved-cli-20261027-74976e8b`.
The original default commit is `21c8acd018e13dc2bc50b124210646e816c5190b`.
The bundle contains 1,862 advertised refs and 238,963 reachable object records.
It does not promise recovery of unadvertised/deleted refs, Git LFS payloads,
GitHub discussions or Actions artifacts. The inventoried release has no assets.

The private Agent Logic backup bucket has versioning, encryption and public
access blocks. Versioning is not WORM retention. Maintainers retain the exact
S3 version IDs in the delivery receipt; do not replace them with an unversioned
latest-object lookup. Request authenticated archive access from Agent Logic
maintainers when an old commit or custody manifest is needed. Public build and
validation do not require access to this archive or any private owner checkout.

The full bundle SHA-256 is
`cd8f17efb2329dded55f35407d7d28a51ba889c8e9582aebf6a4da6fa34917c5`.
Its 360,299,272 bytes were verified at exact S3 version
`ZRJOODwae.ADmAAHBk_b0RQteU0s01au`; S3 FULL_OBJECT CRC64NVME
`0dnfxWEAS30=` matches the locally computed checksum. Local bundle verification,
full fsck, mirror restore, ref equality and reachable-object equality passed.
Remote download/restore was deferred by the operator on the mobile link.

## Historical lookup and restore

When a download is appropriate on a suitable network, obtain the exact version
from the maintainer receipt and verify the full local SHA-256 above. Preserve
existing checkouts and restore into a new empty directory:

```sh
git clone --mirror /path/to/agent-design-language-precutover.bundle restored-history.git
git --git-dir=restored-history.git fsck --full
git --git-dir=restored-history.git cat-file -t ORIGINAL_COMMIT
git --git-dir=restored-history.git show ORIGINAL_COMMIT:path/to/original/file
```

Old source commit IDs in custody manifests resolve in that restored graph. The
separate audit custody archives must also be restored at their recorded Git
metadata-relative paths. Do not claim that the Git bundle includes untracked
custody tarballs. The archive audit runs against the pinned archived checkout,
not against an unrelated new public root commit.

## Contributor continuity and public cutover

A compact public candidate must preserve every retained file's bytes and modes,
except separately reviewed CI/provenance changes. Preserve README, license,
UTS, dispatch adapters and language/compiler packages. Qualify with public CI
and an ordinary Git transport clone; compressed tar size is not clone size.

Before changing any public ref, record exact old/new refs, release/tag treatment,
open-PR state, protection exceptions and restoration, measured clone target and
recovery actions. Existing branches and tags can keep old history reachable in
a normal clone. Provider-owned PR refs and caches require provider-supported
handling; changing refs does not prove old public objects have been erased.

Contributors should preserve their old checkout, feature branches and untracked
files. After an announced cutover, clone the new public repository into a new
folder and port reviewed changes as patches. Do not force-reset existing work.
A cutover fails acceptance until a fresh ordinary public clone meets the agreed
size target and required checks pass. Archive delivery alone does not close
#1382 or mean the public cutover has happened.
