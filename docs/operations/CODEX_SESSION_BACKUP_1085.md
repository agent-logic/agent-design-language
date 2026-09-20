# Codex session backup #1085

Status: captured backup completed and independently checked on 2026-09-20.
This document records the completed one-time backup. It does not configure a
recurring service or authorize local deletion.

## Verified result

| Captured set | Files | Archive objects | Source bytes | Compressed bytes |
|---|---:|---:|---:|---:|
| Primary session history and metadata | 17,541 | 571 | 563,756,858,030 | 37,090,743,489 |
| Supplemental recovery copies | 3 | 2 | 2,222,655,678 | 1,135,159,019 |
| Additional recovered and normalized copies | 981 | 5 | 4,572,069,264 | 431,777,642 |
| **Total** | **18,525** | **578** | **570,551,582,972** | **38,657,680,150** |

The private company S3 bucket has all public-access blocks enabled,
bucket-owner-enforced ownership, AES256 encryption and versioning. Its identity,
region and exact object versions are retained in private operational evidence,
not in this repository.

All 578 archive objects passed exact-version S3 SHA-256 and size verification.
Downloaded manifests matched their upload receipts, and their file mappings
exactly covered all three captured inventories. Representative archives from
every source role were downloaded and decompressed; all 1,131 included files
matched their recorded hashes. The restored SQLite database independently
returned `ok` from `PRAGMA integrity_check`.

No original session files were removed, deduplicated or overwritten. Independent
audit confirmed that every recorded original and snapshot path remained present.
No expiration, cold-storage transition or recurring backup was configured.

## Capture boundary

The captured material includes current and archived sessions, recovered history,
the historical archive, recovery-test and normalized variants, the session index
and SQLite database. Authentication stores and unrelated configuration were
excluded. Same-named recovered versions remain distinct.

Files were captured individually with copy-on-write snapshots over recorded
capture windows. The primary window was 2026-09-19 20:32:25–20:32:36 UTC;
the supplemental manifests retain their own windows and per-file timestamps.
SQLite was captured through its online backup API. This was not a globally
atomic snapshot of the running application. Activity after each file's capture
is outside this backup.

Object integrity and manifest coverage were checked in full. Downloaded-file
restore verification was representative, not an extraction of every archive.
The result establishes recoverability of the captured data, not a tested
replacement of the running application's state.

## Private recovery handoff

The authorized backup custodian retains the bucket locator, `RESTORE.md`,
`verify_backup.py`, `completion.json`, its exact upload receipt,
`verification.json`, bucket-security readbacks, and each set's `snapshot.json`,
`manifest.json` and `manifest-upload.json`. These records contain private paths
and session filenames and must remain outside Git. The complete metadata and
restore instructions are also stored with the backup in S3.

To check recovery independently, obtain the private bucket, prefix and approved
AWS profile from that handoff. The retained verifier currently requires an
Apple Silicon macOS host with AWS CLI, Python 3 and `zstd` installed at
`/opt/homebrew/bin/zstd`; it has not been qualified as a portable Linux or
Intel-macOS tool. Use a new private directory on that supported host. Set the
three variables below to the supplied values; no credentials belong in the
command or repository.

```sh
export CODEX_BACKUP_BUCKET='REPLACE_WITH_PRIVATE_BUCKET'
export CODEX_BACKUP_PREFIX='REPLACE_WITH_BACKUP_PREFIX'
export CODEX_BACKUP_PROFILE='REPLACE_WITH_APPROVED_PROFILE'
export CODEX_BACKUP_REGION='REPLACE_WITH_PRIVATE_REGION'
export CODEX_BACKUP_VERIFIER_VERSION_ID='REPLACE_WITH_EXACT_VERSION_ID'
export CODEX_BACKUP_VERIFIER_SHA256='REPLACE_WITH_RETAINED_SHA256'
umask 077
mkdir codex-backup-restore
cd codex-backup-restore
aws s3 sync "s3://${CODEX_BACKUP_BUCKET}/${CODEX_BACKUP_PREFIX}/" . \
  --exclude '*' --include '*.json' --include 'RESTORE.md' \
  --profile "$CODEX_BACKUP_PROFILE" --region "$CODEX_BACKUP_REGION"
aws s3api get-object --bucket "$CODEX_BACKUP_BUCKET" \
  --key "${CODEX_BACKUP_PREFIX}/verify_backup.py" \
  --version-id "$CODEX_BACKUP_VERIFIER_VERSION_ID" \
  --profile "$CODEX_BACKUP_PROFILE" --region "$CODEX_BACKUP_REGION" \
  verify_backup.py
printf '%s  verify_backup.py\n' "$CODEX_BACKUP_VERIFIER_SHA256" | \
  shasum -a 256 -c -
test -f completion.json
CODEX_BACKUP_RESTORE_TMP="$PWD/restore-check" python3 verify_backup.py
```

Obtain the exact verifier object version and SHA-256 from the private handoff;
do not run an unversioned or mismatched verifier. Before running it, confirm
that `/opt/homebrew/bin/zstd` exists. The retained verifier uses the original
approved profile name recorded in the private instructions. Configure that
profile before running it; changing the download profile variable alone does
not alter the verifier's configuration.
Keep both supplemental subdirectories. The verifier checks all archive checksums
and manifest coverage and repeats representative restores without relying on the
original disks. Allow several GiB of temporary space for that check; full
extraction requires space for the complete uncompressed inventory.

For a full restore, follow the private guide: download exact object versions,
verify compressed hashes and sizes, reject unexpected or unsafe archive members,
extract into an empty directory, and check every restored file against its
manifest. Preserve source-role prefixes. Do not overwrite a running Codex
database; stop the application and preserve its current state before any
separately authorized application restore.

## Publication boundary

This report contains aggregate completion facts and recovery instructions only.
Session bodies, filenames, manifests, host paths, account identifiers and
credentials are not publication payload. Backup completion does not establish
native lifecycle reconciliation: that remains a separate tracked operation.
