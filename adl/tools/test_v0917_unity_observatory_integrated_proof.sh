#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PROOF_4689="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4689/4689-unity-observatory-integrated-proof.md"
PROOF_4703="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md"
PROOF_4704="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md"
PROOF_4704_MCP="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4704/4704-unity-mcp-proof-summary.md"
PROOF_4652="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md"
POLICY_4745="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md"
MANIFEST_4745="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-manifest.json"
README_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/README.md"
PACKET_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/PROOF_PACKET.md"
SCENE_PATH="${ROOT_DIR}/demos/v0.91.6/unity-observatory/Assets/Scenes/UnityObservatory.unity"
HERO_4703="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-investor-hero.png"
WIDE_4704="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4704/flagship-wide-observatory-camera-4704.png"
SHELL_4652="${ROOT_DIR}/docs/milestones/v0.91.7/review/unity_observatory_4652/flagship-shell-main-camera-4652.png"

for path in \
  "${PROOF_4689}" \
  "${PROOF_4703}" \
  "${PROOF_4704}" \
  "${PROOF_4704_MCP}" \
  "${PROOF_4652}" \
  "${POLICY_4745}" \
  "${MANIFEST_4745}" \
  "${README_PATH}" \
  "${PACKET_PATH}" \
  "${SCENE_PATH}" \
  "${HERO_4703}" \
  "${WIDE_4704}" \
  "${SHELL_4652}"
do
  if [[ ! -f "${path}" ]]; then
    echo "missing Unity Observatory integrated proof artifact: ${path}" >&2
    exit 1
  fi
done

require_contains() {
  local path="$1"
  local needle="$2"
  grep -Fq "${needle}" "${path}" || {
    echo "missing required content '${needle}' in ${path}" >&2
    exit 1
  }
}

for path in "${PROOF_4689}" "${PROOF_4703}" "${PROOF_4704}" "${PROOF_4704_MCP}" "${PROOF_4652}" "${POLICY_4745}"; do
  require_contains "${path}" "Unity"
  require_contains "${path}" "Non-Claims"
done

require_contains "${PROOF_4689}" "PASS: the v0.91.7 Unity Observatory proof chain is integrated"
require_contains "${PROOF_4689}" "docs/milestones/v0.91.7/review/unity_observatory_4703/4703-flagship-observatory-stage-proof.md"
require_contains "${PROOF_4689}" "docs/milestones/v0.91.7/review/unity_observatory_4704/4704-operator-walkthrough.md"
require_contains "${PROOF_4689}" "docs/milestones/v0.91.7/review/unity_observatory_4652/4652-unity-shell-proof-summary.md"
require_contains "${PROOF_4689}" "docs/milestones/v0.91.7/review/unity_observatory_4745/4745-asset-mcp-publication-policy.md"
require_contains "${PROOF_4689}" "operator-provisioned"
require_contains "${PROOF_4689}" "does not claim Unity player-build readiness"

require_contains "${PROOF_4703}" "PASS: the bound #4703 Unity project contains and validates a flagship"
require_contains "${PROOF_4703}" "Assets/Scenes/FlagshipObservatoryStage.unity"
require_contains "${PROOF_4703}" "4703-flagship-observatory-investor-hero.png"
require_contains "${PROOF_4704}" "flagship-wide-observatory-camera-4704.png"
require_contains "${PROOF_4704}" "No wrong-port Unity-MCP endpoint is serving the proof session"
require_contains "${PROOF_4704_MCP}" "Unity-MCP"
require_contains "${PROOF_4704_MCP}" "FlagshipObservatoryStage"
require_contains "${PROOF_4652}" "runtime polis shell"
require_contains "${PROOF_4652}" "flagship-shell-main-camera-4652.png"
require_contains "${POLICY_4745}" "External/operator-provisioned only"
require_contains "${POLICY_4745}" "Unity-MCP is accepted as local editor/proof tooling"
require_contains "${PACKET_PATH}" "not claimed as"
require_contains "${README_PATH}" "#4745"
require_contains "${SCENE_PATH}" "Unity Observatory Bootstrap"

python3 - <<'PY' "${MANIFEST_4745}" "${HERO_4703}" "${WIDE_4704}" "${SHELL_4652}"
import json
import struct
import sys
from pathlib import Path

manifest = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if manifest.get("issue") != 4745:
    raise SystemExit("manifest issue is not 4745")
if manifest.get("asset_publication_decision") != "external_operator_provisioned_until_license_storage_subset_approval":
    raise SystemExit("manifest publication route does not preserve external asset boundary")
if manifest.get("unity_mcp_decision") != "editor_proof_tooling_not_runtime_demo_state":
    raise SystemExit("manifest Unity-MCP decision does not preserve editor proof tooling boundary")

expected_dimensions = {
    sys.argv[2]: (1920, 1080),
    sys.argv[3]: (1920, 1080),
    sys.argv[4]: (1600, 900),
}

for path_text, expected in expected_dimensions.items():
    path = Path(path_text)
    data = path.read_bytes()
    if len(data) < 33:
        raise SystemExit(f"PNG too small: {path}")
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit(f"not a PNG: {path}")
    width, height = struct.unpack(">II", data[16:24])
    if (width, height) != expected:
        raise SystemExit(f"unexpected dimensions for {path}: {(width, height)} != {expected}")
PY

echo "PASS: v0.91.7 Unity Observatory integrated proof chain is present and bounded"
