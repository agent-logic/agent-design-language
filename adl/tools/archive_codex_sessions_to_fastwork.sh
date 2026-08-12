#!/usr/bin/env bash
set -euo pipefail
set -C
umask 077

usage() {
  echo "usage: $0 --source <codex-root> --destination <archive-dir> [--min-age-days <days>] | --verify-only <manifest>" >&2
}

verify_manifest() {
  local manifest="$1"
  local archive_root
  archive_root="$(cd "$(dirname "$manifest")" && pwd -P)"
  (cd "$archive_root/data" && shasum -a 256 -c "../$(basename "$manifest")" --quiet)
}

if [[ "${1:-}" == "--verify-only" ]]; then
  [[ $# -eq 2 ]] || { usage; exit 2; }
  verify_manifest "$2"
  exit 0
fi

source_root=""
destination=""
min_age_days=1
while [[ $# -gt 0 ]]; do
  case "$1" in
    --source) source_root="$2"; shift 2 ;;
    --destination) destination="$2"; shift 2 ;;
    --min-age-days) min_age_days="$2"; shift 2 ;;
    *) usage; exit 2 ;;
  esac
done

[[ -d "$source_root/sessions" && -d "$source_root/archived_sessions" ]] || {
  echo "source must contain sessions and archived_sessions" >&2
  exit 2
}
[[ "$destination" == /Volumes/FastWork/* ]] || {
  echo "destination must be beneath /Volumes/FastWork" >&2
  exit 2
}
[[ "$min_age_days" =~ ^[0-9]+$ ]] || {
  echo "min-age-days must be a non-negative integer" >&2
  exit 2
}

destination_parent="$(dirname "$destination")"
[[ -d "$destination_parent" ]] || {
  echo "destination parent must already exist" >&2
  exit 2
}
destination_parent="$(cd "$destination_parent" && pwd -P)"
[[ "$destination_parent" == /Volumes/FastWork/* ]] || {
  echo "canonical destination parent must be beneath /Volumes/FastWork" >&2
  exit 2
}
destination="$destination_parent/$(basename "$destination")"
if [[ -e "$destination" || -L "$destination" ]]; then
  echo "destination must not already exist" >&2
  exit 2
fi
mkdir "$destination"
mkdir "$destination/data"
destination="$(cd "$destination" && pwd -P)"
[[ "$destination" == /Volumes/FastWork/* ]] || {
  echo "canonical destination must be beneath /Volumes/FastWork" >&2
  exit 2
}
chmod 700 "$destination" "$destination/data"
file_list="$destination/files.null"
source_manifest="$destination/source.sha256"
archive_manifest="$destination/manifest.sha256"

(cd "$source_root" && find sessions archived_sessions -type f -mtime "+$min_age_days" -print0 | sort -z > "$file_list")
(cd "$source_root" && xargs -0 shasum -a 256 < "$file_list" > "$source_manifest")
rsync -a --from0 --files-from="$file_list" "$source_root/" "$destination/data/"
find "$destination/data" -type d -exec chmod 700 {} +
find "$destination/data" -type f -exec chmod 600 {} +
chmod 600 "$file_list" "$source_manifest"
(cd "$destination/data" && xargs -0 shasum -a 256 < "$file_list" > "$archive_manifest")
chmod 600 "$archive_manifest"
cmp "$source_manifest" "$archive_manifest"
verify_manifest "$archive_manifest"

source_bytes="$(cd "$source_root" && xargs -0 stat -f '%z' < "$file_list" | awk '{sum += $1} END {print sum + 0}')"
file_count="$(tr -cd '\0' < "$file_list" | wc -c | tr -d ' ')"
printf '{"schema":"adl.codex_session_archive.v1","source":"%s","archive":"%s/data","files":%s,"bytes":%s,"minimum_age_days":%s,"source_deleted":false}\n' \
  "$source_root" "$destination" "$file_count" "$source_bytes" "$min_age_days" > "$destination/summary.json"
chmod 600 "$destination/summary.json"
echo "archive verified: $file_count files, $source_bytes bytes"
