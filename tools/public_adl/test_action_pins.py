#!/usr/bin/env python3
# PVF deterministic repository contract; inherited canonical action pins and quoted negatives.
from pathlib import Path
import re
workflow_root = Path(__file__).resolve().parents[2] / ".github/workflows"
canonical_actions = {
    "actions/checkout": "9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
    "actions/download-artifact": "37930b1c2abaa49bbe596cd826c3c89aef350131",
    "actions/upload-artifact": "043fb46d1a93c77aae656e7c1c64a875d1fc6a0a",
    "Swatinem/rust-cache": "c19371144df3bb44fab255c43d04cbc2ab54d1c4",
}
deprecated_shas = {
    "34e114876b0b11c390a56381ad16ebd13914f8d5",
    "ea165f8d65b6e75b540449e92b4886f43607fa02",
    "779680da715d629ac1d338a641029a2f4372abb5",
}
seen = {action: 0 for action in canonical_actions}

def parse_uses(line: str) -> str | None:
    match = re.match(r"^(?:-\s+)?uses:\s+(.+?)\s*(?:#.*)?$", line.strip())
    if not match:
        return None
    value = match.group(1).strip()
    if value[:1] in {"'", '"'}:
        if len(value) < 2 or value[-1] != value[0]:
            raise SystemExit(f"invalid quoted workflow uses scalar: {line.strip()!r}")
        value = value[1:-1]
    return value

def require_canonical_action(uses: str, source: str) -> None:
    for action, sha in canonical_actions.items():
        if uses.startswith(f"{action}@"):
            expected = f"{action}@{sha}"
            if uses != expected:
                raise SystemExit(
                    f"workflow must pin {action} to the canonical Node 24 SHA; "
                    f"found {uses!r} in {source}"
                )
            seen[action] += 1

for candidate in sorted(workflow_root.glob("*.y*ml")):
    text = candidate.read_text()
    for deprecated_sha in deprecated_shas:
        if deprecated_sha in text:
            raise SystemExit(
                f"workflow retains deprecated Node 20 action SHA {deprecated_sha} "
                f"in {candidate.name}"
            )
    for line in text.splitlines():
        uses = parse_uses(line)
        if uses is None:
            continue
        require_canonical_action(uses, candidate.name)

for fixture in (
    'uses: "actions/checkout@v7"',
    "- uses: 'actions/upload-artifact@v7'",
):
    try:
        require_canonical_action(parse_uses(fixture), "quoted negative fixture")
    except SystemExit as exc:
        if "canonical Node 24 SHA" not in str(exc):
            raise
    else:
        raise SystemExit(f"quoted floating action pin escaped enforcement: {fixture}")

print("PASS canonical action pins across every workflow and quoted negatives")
