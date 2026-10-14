#!/usr/bin/env bash
set -euo pipefail
# PVF: local deterministic compatibility regression; no network/build/provider;
# required focused caller test. Proves refusal diagnostics, not CSM availability.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
python3 - "$ROOT_DIR/adl/tools/ensure_csm_binary.sh" <<'PY'
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

helper = Path(sys.argv[1])
with tempfile.TemporaryDirectory(prefix="csm-retired-diagnostic-") as directory:
    fixture = Path(directory)
    tools = fixture / "bin"
    tools.mkdir()
    marker = fixture / "unexpected-execution"
    trap = '#!/usr/bin/env bash\nprintf unexpected >>"$CSM_TEST_MARKER"\nexit 91\n'
    for name in ("cargo", "csm"):
        executable = tools / name
        executable.write_text(trap)
        executable.chmod(0o755)
    source = fixture / "source"
    (source / "adl/src/bin").mkdir(parents=True)
    (source / "adl/src/bin/csm.rs").write_text("fn main() {}\n")
    (source / "adl/Cargo.toml").write_text('[package]\nname="fixture"\nversion="0.0.0"\n')
    env = dict(os.environ, PATH=str(tools) + os.pathsep + os.environ["PATH"],
               ADL_TOOLING_MANIFEST_ROOT=str(source), ADL_CSM_SOURCE_ROOT=str(source),
               ADL_CSM_BIN=str(tools / "csm"), CSM_TEST_MARKER=str(marker))
    def run(*args):
        return subprocess.run(["bash", str(helper), *args], env=env,
                              text=True, capture_output=True, check=False)
    def failed_json(*args):
        result = run("--json", *args)
        assert result.returncode == 1, result
        assert not result.stderr, result.stderr
        payload = json.loads(result.stdout)
        assert payload["schema"] == "adl.csm.binary_availability.v1"
        assert payload["status"] == "failed"
        assert payload["runtime_owner"] == "csm"
        assert payload["action"] == "retired_source_route"
        assert payload["binary"] is None
        assert payload["source_presence"] is False
        assert payload["validation_proof"] is False
        assert str(fixture) not in result.stdout
        return payload
    expected = failed_json()
    assert failed_json("--check-only", "--csm-bin", str(tools / "csm")) == expected
    output = fixture / "diagnostic.json"
    assert failed_json("--out", str(output)) == expected
    assert json.loads(output.read_text()) == expected
    human = run("--out", str(output))
    assert human.returncode == 1 and "status=failed" in human.stdout
    assert json.loads(output.read_text()) == expected
    help_result = run("--help")
    assert help_result.returncode == 0
    assert "never builds, discovers, or executes" in help_result.stdout
    for arguments in (("--out",), ("--csm-bin",), ("--out", "--json"), ("--unknown",)):
        result = run(*arguments)
        assert result.returncode == 2 and result.stderr and not result.stdout, result
    assert not marker.exists(), "retired route invoked Cargo or a CSM candidate"
print("CSM retirement diagnostics: 9 cases passed; no Cargo or CSM execution")
PY
