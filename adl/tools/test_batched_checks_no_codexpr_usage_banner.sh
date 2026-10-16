#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RETIRED_PR_NAME="codex_pr.sh"
RETIRED_CODEXW_NAME="codexw.sh"
RETIRED_PR_SH="$ROOT_DIR/adl/tools/$RETIRED_PR_NAME"
RETIRED_CODEXW_SH="$ROOT_DIR/adl/tools/$RETIRED_CODEXW_NAME"
BATCHED_SRC="$ROOT_DIR/adl/tools/batched_checks.sh"

if grep -Fq "$RETIRED_PR_NAME" "$ROOT_DIR/adl/tools/batched_checks.sh"; then
  echo "assertion failed: batched_checks.sh still references codex_pr.sh" >&2
  exit 1
fi

if grep -Fq "$RETIRED_CODEXW_NAME" "$ROOT_DIR/adl/tools/batched_checks.sh"; then
  echo "assertion failed: batched_checks.sh still references codexw.sh" >&2
  exit 1
fi

# The fail-closed migration wrappers are now removed. Their prior exit/guidance
# assertions remain in #1223 custody; public source/output non-use stays below.
for retired in "$RETIRED_PR_SH" "$RETIRED_CODEXW_SH"; do
  [[ ! -e "$retired" ]] || {
    echo "assertion failed: retired wrapper unexpectedly exists: $retired" >&2
    exit 1
  }
done

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT
repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools"
cp "$BATCHED_SRC" "$repo/adl/tools/batched_checks.sh"
cat > "$repo/adl/tools/pr.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--help" ]]; then
  echo "fixture pr help"
  exit 0
fi
exit 0
SH

python3 - "$BATCHED_SRC" "$repo" <<'PY'
from pathlib import Path
import re
import sys

source = Path(sys.argv[1]).read_text()
repo = Path(sys.argv[2])
for rel in sorted(set(re.findall(r'adl/tools/([A-Za-z0-9_.-]+\.sh)', source))):
    if rel in {"batched_checks.sh", "pr.sh"}:
        continue
    target = repo / "adl" / "tools" / rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text("#!/usr/bin/env bash\nexit 0\n")
    target.chmod(0o755)
PY
mkdir -p "$repo/tools/public_adl"
cp "$ROOT_DIR/tools/public_adl/run_package_checks.sh" "$repo/tools/public_adl/run_package_checks.sh"
cat > "$repo/tools/public_adl/run_cli_ci.sh" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$repo/adl/tools/batched_checks.sh" "$repo/adl/tools/pr.sh"

fakebin="$tmpdir/fakebin"
mkdir -p "$fakebin"
cat > "$fakebin/cargo" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$fakebin/cargo"

out="$tmpdir/out.log"
(
  cd "$repo"
  PATH="$fakebin:$PATH" bash adl/tools/batched_checks.sh >"$out" 2>&1
)

if grep -Fq "$RETIRED_PR_NAME" "$out"; then
  echo "assertion failed: batched checks output still mentions codex_pr.sh" >&2
  cat "$out" >&2
  exit 1
fi

if grep -Fq "$RETIRED_CODEXW_NAME" "$out"; then
  echo "assertion failed: batched checks output still mentions codexw.sh" >&2
  cat "$out" >&2
  exit 1
fi

echo "retired codex wrappers are absent and batched checks do not depend on them: ok"
