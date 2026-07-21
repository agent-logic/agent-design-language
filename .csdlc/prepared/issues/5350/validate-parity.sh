#!/usr/bin/env bash
set -euo pipefail

mode="${1:-}"
case "$mode" in
  subjects|compare|overlays|complete) ;;
  *) printf 'usage: %s subjects|compare|overlays|complete\n' "$0" >&2; exit 64 ;;
esac

inventory=".csdlc/prepared/issues/5350/source-inventory.json"
if ruby -rjson -e 'data=JSON.parse(File.read(ARGV.fetch(0))); exit(data.dig("adl_v2", "revision") && data.dig("adl_v2", "binary_sha256") ? 0 : 1)' "$inventory"; then
  printf 'WP-11 execution runner is intentionally unimplemented during preparation; amend through typed C-SDLC after all terminal gates pass\n' >&2
  exit 78
fi

printf 'WP-11 execution is blocked: exact ADL v2 identity and terminal dependency evidence are not populated\n' >&2
exit 78
