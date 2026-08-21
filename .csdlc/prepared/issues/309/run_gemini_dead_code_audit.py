#!/usr/bin/env python3
"""Run a bounded Gemini dead-code audit for ADL issue #309."""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import urllib.request

ROOT = Path("/Volumes/FastWork/adl-worktrees/adl-issue-309-repository-wide-code-reduction")
PRIMARY = Path("/Users/daniel/git/agent-design-language")
BASE = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def git_show(path: str) -> str:
    return subprocess.check_output(
        ["git", "-C", str(ROOT), "show", f"{BASE}:{path}"], text=True
    )


edges_doc = json.loads(read(ROOT / ".csdlc/evidence/309/reference-edge-manifest.json"))
dispositions_doc = json.loads(read(ROOT / ".csdlc/evidence/309/disposition-manifest.json"))
incoming: dict[str, list[dict]] = {}
for edge in edges_doc["edges"]:
    incoming.setdefault(edge["target"], []).append(edge)

candidates: list[str] = []
for row in dispositions_doc["files"]:
    path = row["path"]
    substantive = []
    for edge in incoming.get(path, []):
        source = edge.get("source", {}).get("path", "")
        if source == "adl/src/lib.rs" and edge.get("reference_class") == "module":
            continue
        if source.endswith("/mod.rs") and edge.get("reference_class") == "module":
            continue
        if source == path:
            continue
        substantive.append(edge)
    if not substantive:
        candidates.append(path)

candidate_source = "\n\n".join(
    f"===== {path} =====\n{git_show(path)}" for path in candidates
)

instructions = f"""
You are an independent senior Rust repository-reduction reviewer working with
the ADL team on issue #309. This is a DEAD-CODE-ONLY deletion task. Do not
recommend deleting, disabling, replacing, or simplifying any capability that
has a real runtime, CLI, test-contract, artifact, workflow, documentation, or
declared external consumer. Do not count a `pub mod` declaration or public
re-export alone as proof of a real consumer; it proves build membership only.
When uncertain, classify NEEDS_CHARACTERIZATION rather than DELETE_NOW.

Facts to reconcile:
- historical #309 trend denominator: 355,675 physical Rust lines;
- pinned execution baseline {BASE}: 485 adl/src Rust files and 265,633 lines;
- therefore 90,042 lines disappeared before this execution baseline;
- current reviewed Band A deletes two superseded evaluation modules totaling
  1,165 lines;
- Runtime v2 and #414 continuity behavior are protected;
- later behavior-preserving refactoring belongs to #310, not this deletion.

Audit objectives:
1. Explain how much of the apparently missing reduction already occurred before
   the pinned baseline, versus what remains plausible as dead code.
2. Audit the supplied 49 low-incoming-edge candidates. For every candidate give
   DELETE_NOW, RETAIN_ACTIVE, or NEEDS_CHARACTERIZATION; cite exact source and
   manifest evidence, and name the cheapest decisive check.
3. Look across the full baseline/disposition/reference manifests for additional
   likely dead or superseded clusters that the low-edge filter missed. Prioritize
   old evaluation prototypes, retired demos, duplicate compatibility authorities,
   and implementation/test pairs with no authoritative entrypoint.
4. Identify false-liveness rules in the current census and propose a deterministic
   authority-rooted reachability algorithm suitable for a fail-closed validator.
5. Produce a conservative ordered deletion-wave proposal with estimated files
   and physical lines per wave. Separate DELETE_NOW from characterization work.
6. Explicitly list protected surfaces that must not be deleted.

Output Markdown with these exact sections:
- Executive finding
- Historical denominator reconciliation
- DELETE_NOW table
- NEEDS_CHARACTERIZATION table
- RETAIN_ACTIVE highlights
- Additional candidate clusters
- Corrected reachability algorithm
- Reversible wave proposal
- Protected surfaces and stop conditions

Do not claim a file dead solely because it has few text references. Do not
recommend a percentage target. Be findings-first and specific.
"""

parts = [
    instructions,
    "===== CURRENT WP-21 PLAN =====\n" + read(PRIMARY / ".adl/docs/TBD/WP_21_REPOSITORY_REDUCTION_PLAN.md"),
    "===== HISTORICAL REPOSITORY REDUCTION PLAN =====\n" + read(PRIMARY / ".adl/docs/TBD/rust_refactoring/ADL_REPOSITORY_CODE_REDUCTION_PLAN_v0.91.8.md"),
    "===== BASELINE MANIFEST =====\n" + read(ROOT / ".csdlc/evidence/309/baseline-manifest.json"),
    "===== DISPOSITION MANIFEST =====\n" + read(ROOT / ".csdlc/evidence/309/disposition-manifest.json"),
    "===== REFERENCE EDGE MANIFEST =====\n" + read(ROOT / ".csdlc/evidence/309/reference-edge-manifest.json"),
    "===== CURRENT REDUCTION REPORT =====\n" + read(ROOT / ".csdlc/evidence/309/reduction-report.json"),
    "===== LOW-INCOMING-EDGE CANDIDATE SOURCE =====\n" + candidate_source,
]

payload = {
    "contents": [{"role": "user", "parts": [{"text": part} for part in parts]}],
    "generationConfig": {"temperature": 0.1, "maxOutputTokens": 32768},
}
key = read(Path(os.environ["GEMINI_KEY_FILE"])).strip()
request = urllib.request.Request(
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-pro-preview:generateContent",
    data=json.dumps(payload).encode(),
    headers={"Content-Type": "application/json", "x-goog-api-key": key},
    method="POST",
)
with urllib.request.urlopen(request, timeout=600) as response:
    result = json.load(response)
text = "".join(
    part.get("text", "")
    for candidate in result.get("candidates", [])
    for part in candidate.get("content", {}).get("parts", [])
)
if not text.strip():
    raise SystemExit(json.dumps(result, sort_keys=True)[:2000])
output_path = Path(
    os.environ.get(
        "GEMINI_OUTPUT_FILE",
        str(ROOT / ".csdlc/evidence/309/gemini-dead-code-audit.md"),
    )
)
output_path.write_text(text.rstrip() + "\n", encoding="utf-8")
print(json.dumps({"status": "pass", "output": str(output_path), "characters": len(text)}, sort_keys=True))
