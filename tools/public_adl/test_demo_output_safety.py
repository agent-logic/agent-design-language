#!/usr/bin/env python3
"""PVF local_contract: deterministic temp-disk/process negatives; public safety gate.

This checks output ownership only. Demo content validity remains a separate gate.
"""
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
DEMOS = (
    "demo_v090_codebuddy_review_showcase.sh",
    "demo_v090_architecture_document_generation.sh",
    "demo_v0891_arxiv_manuscript_workflow.sh",
)


def main():
    with tempfile.TemporaryDirectory(prefix="adl-demo-safety-") as temp:
        parent = Path(temp)
        existing = parent / "existing"
        existing.mkdir()
        sentinel = existing / "sentinel"
        sentinel.write_bytes(b"must survive\x00\xff")
        file = parent / "file"
        file.write_text("keep")
        dangling = parent / "dangling"
        dangling.symlink_to(parent / "missing")
        linked = parent / "linked"
        linked.symlink_to(existing, target_is_directory=True)
        for demo in DEMOS:
            for target in (existing, file, dangling, linked, parent):
                result = subprocess.run(
                    ["bash", str(ROOT / "adl/tools" / demo), str(target)],
                    cwd=ROOT, capture_output=True, timeout=30,
                )
                assert result.returncode != 0, (demo, target)
                assert b"demo output must be a new directory" in result.stderr, result.stderr
                assert sentinel.read_bytes() == b"must survive\x00\xff"
                assert file.read_text() == "keep"
                assert dangling.is_symlink() and not dangling.exists()
                assert linked.is_symlink()
    print("PASS demo output preservation for all three generators")


if __name__ == "__main__":
    main()
