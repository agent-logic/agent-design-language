#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]

required = [
    ROOT / ".csdlc/issues/330/index.json",
    ROOT / ".csdlc/issues/330/cards/sip.md",
    ROOT / ".csdlc/issues/330/cards/stp.md",
    ROOT / ".csdlc/issues/330/cards/spp.md",
    ROOT / ".csdlc/issues/330/cards/vpp.md",
    ROOT / ".csdlc/issues/330/cards/srp.md",
    ROOT / ".csdlc/issues/330/cards/sor.md",
]

missing = [str(path.relative_to(ROOT)) for path in required if not path.exists()]
if missing:
    raise SystemExit(f"missing #330 preparation artifacts: {missing}")

print("#330 preparation bundle present")
