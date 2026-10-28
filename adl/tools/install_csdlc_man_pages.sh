#!/usr/bin/env bash
# Read-only manual discovery; package admission belongs to native C-SDLC.
set -euo pipefail
if [[ $# != 2 || "$1" != "--stable-bin-dir" || -z "$2" ]]; then
  echo 'usage: install_csdlc_man_pages.sh --stable-bin-dir DIRECTORY_CONTAINING_SELECTED_CSDLC' >&2
  exit 2
fi
python3 - "$2/csdlc" <<'PYTHON'
from pathlib import Path
import os
import shlex
import sys

selected = Path(sys.argv[1])
try:
    executable = selected.resolve(strict=True)
    package = executable.parent.parent
    if executable.name != "csdlc" or executable.parent.name != "bin" or package.name != "installed" or not executable.is_file() or not os.access(executable, os.X_OK):
        raise ValueError("selected executable is not an installed owner-package entrypoint")
    manual = package / "resources/docs/csdlc-v3/man"
    for path in (package / "manifest.json", manual / "man1/csdlc.1", manual / "man1/csdlc-workflow.1", manual / "man1/csdlc-requests.1"):
        if path.resolve(strict=True) != path or not path.is_file():
            raise ValueError("package manual or manifest is missing or aliased")
except (OSError, RuntimeError, ValueError) as error:
    print(f"csdlc manual lookup refused: {error}", file=sys.stderr)
    print("Use native csdlc package-install --request REVIEWED_REQUEST.json with an authenticated exact owner package; then use its returned executable directory. ADL source builds and manual copies are not a substitute.", file=sys.stderr)
    sys.exit(2)
print(f"csdlc package manual directory: {manual}")
print(f'export MANPATH={shlex.quote(str(manual))}:"${{MANPATH:-}}"')
print("Run man csdlc-workflow. This lookup neither authenticates nor activates the package; native admission remains required.")
PYTHON
