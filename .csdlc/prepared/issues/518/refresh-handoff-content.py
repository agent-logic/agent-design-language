#!/usr/bin/env python3
"""Refresh only the documentation content inventory; never grants review approval."""
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[4]
packet = Path("docs/milestones/v0.92.1/evidence/release/tail-02")
tracked = subprocess.check_output(["git", "ls-files", "-z"], cwd=root).decode().split("\0")
paths = {r["path"] for r in json.loads((root / packet / "document-inventory.json").read_text())["documents"]}
paths.update(p for p in tracked if p and (Path(p).name.startswith("README") or Path(p).name in ("AGENTS.md", "REVIEW.md", "Cargo.toml") or (p.startswith("docs/milestones/v0.92.1/") and Path(p).suffix in (".md", ".yaml", ".yml"))))
paths.update(str(p.relative_to(root)) for p in (root / "docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation").glob("*.json"))
paths.update(str(p.relative_to(root)) for p in (root / packet).iterdir() if p.is_file())
paths.update(str(p.relative_to(root)) for p in (root / ".csdlc/prepared/issues/518").iterdir() if p.suffix in (".py", ".rb"))
paths.difference_update((str(packet / "handoff-content.json"), str(packet / "final-validation.json")))
result = {"schema": "adl.tail02.handoff_content.v1", "merged_baseline": "7b66e5159c0bbccac3725832a8f876f5b820152d", "scope": "Original audit union all tracked README-prefixed files, AGENTS.md, REVIEW.md, Cargo.toml, v0.92.1 milestone Markdown/YAML, packet inputs and validators. Excludes this manifest and derived validation results to prevent hash cycles. Typed review separately binds the exact commit.", "documents": [{"path": p, "sha256": hashlib.sha256((root / p).read_bytes()).hexdigest()} for p in sorted(paths)]}
(root / packet / "handoff-content.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({"documents": len(paths)}))
