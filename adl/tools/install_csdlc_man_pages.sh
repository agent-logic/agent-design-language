#!/usr/bin/env bash
# Install reviewed generated pages without rebuilding or replacing the owner.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
if [[ $# != 2 || "$1" != "--stable-bin-dir" || -z "$2" ]]; then
  echo 'usage: install_csdlc_man_pages.sh --stable-bin-dir DIRECTORY' >&2
  exit 2
fi
MAN_ROOT="$2/share/man"
SOURCE="$ROOT_DIR/docs/csdlc-v3/man/man1"
[[ -f "$SOURCE/csdlc.1" && -f "$SOURCE/csdlc-requests.1" ]] || {
  echo 'csdlc manual source pages are missing' >&2
  exit 1
}
for directory in "$2" "$2/share" "$MAN_ROOT" "$MAN_ROOT/man1"; do
  [[ ! -L "$directory" ]] || { echo 'csdlc manual destination must not be a symlink' >&2; exit 1; }
done
mkdir -p "$MAN_ROOT/man1"
for source in "$SOURCE"/*.1; do
  destination="$MAN_ROOT/man1/$(basename "$source")"
  [[ ! -L "$destination" ]] || { echo 'csdlc manual page destination must not be a symlink' >&2; exit 1; }
  if [[ -f "$destination" ]] && cmp -s "$source" "$destination"; then
    continue
  fi
  stage="$(mktemp "$MAN_ROOT/man1/.csdlc-man.XXXXXX")"
  trap 'rm -f "$stage"' EXIT
  cp "$source" "$stage"
  chmod 0644 "$stage"
  mv "$stage" "$destination"
  trap - EXIT
done
printf 'csdlc manual installed: %s\n' "$MAN_ROOT"
printf 'Manual lookup: MANPATH="%s:${MANPATH:-}" man csdlc\n' "$MAN_ROOT"
