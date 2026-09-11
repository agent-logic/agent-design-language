#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# PVF: deterministic, local shell routing proof; small; required for #856.
FIXTURE="$(mktemp -d "$(git -C "$ROOT" rev-parse --absolute-git-dir)/release-wrapper.XXXXXX")"
trap 'rm -rf "$FIXTURE"' EXIT
mkdir -p "$FIXTURE/adl/tools" "$FIXTURE/.adl/bin/native-v3"
cp "$ROOT/adl/tools/release_ceremony.sh" "$FIXTURE/adl/tools/"
SCRIPT="$FIXTURE/adl/tools/release_ceremony.sh"
for flag in --allow-dirty --skip-sor-gate --create-tag --push-tag --draft-release --publish-release --version; do
  if bash "$SCRIPT" "$flag" value >"$FIXTURE/stdout" 2>"$FIXTURE/stderr"; then echo "unexpected bypass: $flag" >&2; exit 1; fi
  [[ ! -s "$FIXTURE/stdout" && -s "$FIXTURE/stderr" ]]
done
if bash "$SCRIPT" --request "$FIXTURE/request.json" >"$FIXTURE/stdout" 2>"$FIXTURE/stderr"; then exit 1; fi
grep -q 'stable native v3 owner missing' "$FIXTURE/stderr"
cat >"$FIXTURE/.adl/bin/native-v3/csdlc" <<'INNER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 3 && "$1" == release-preflight && "$2" == --request && -f "$3" ]]
printf '%s\n' '{"fixture_route_only":true,"mutation_allowed":false}'
INNER
chmod +x "$FIXTURE/.adl/bin/native-v3/csdlc"
echo '{}' >"$FIXTURE/request.json"
bash "$SCRIPT" --request "$FIXTURE/request.json" >"$FIXTURE/stdout" 2>"$FIXTURE/stderr"
grep -q 'fixture_route_only' "$FIXTURE/stdout"
[[ ! -s "$FIXTURE/stderr" ]]
echo 'PASS release wrapper routing; native semantic fixtures run separately'
