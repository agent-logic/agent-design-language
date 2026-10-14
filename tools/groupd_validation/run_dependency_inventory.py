#!/usr/bin/env python3
"""PVF: deterministic local contract routing; small CPU/disk, no network.
Runs all six original owner tests; not dependency-audit or live product proof.
"""
import hashlib
import os
from pathlib import Path
import subprocess
import sys

# Authenticated CodeFriend artifact11803531674, candidate e93a457ad6325017627a66a2abe31a3376fa28fd.
# Whole archive SHA256:4ae4847f60a4b2cfd0504deb30fa01ae31db781f6a29ebad2255bbcba81113c4.
RESOURCES = {'repo-dependency-review/SKILL.md': '575a532a05fed00f6d8f399ef0b369bae316b0ff768f78a3e3fabd3718ec7157', 'repo-dependency-review/adl-skill.yaml': 'f17ac9a943828f6ece3139fb3d41a4b4e33a485f56e3078bb2c9bdb89a7f0269', 'repo-dependency-review/agents/openai.yaml': '047e87eb6e867f45288ab2f83ec3dc4f1036a15de7d9dc6399345a4b351cf9ce', 'repo-dependency-review/references/dependency-playbook.md': '1ee5ed9f6ee506f6c7c8c8619cf25ac404952d63105579ae73bc640161df7348', 'repo-dependency-review/references/output-contract.md': '7148e4fb8dddffab6291de368dc3a2afe06165b5ea9ce3d3b7147abfa50ac03a', 'repo-dependency-review/scripts/prepare_dependency_review.py': '626cc3f275d7f0409014a9b6561d4ade236cf69e1ffcde899e5d6708e1545a18', 'repo-dependency-review/scripts/test_prepare_dependency_review.py': '15dd9382577b65df82c078dd43eec091eede972fa4ff51739e329107e599a4b1'}


def main():
    selected = os.environ.get("CODEFRIEND_REVIEW_SKILLS_ROOT", "")
    root = Path(selected)
    if not selected or not root.is_absolute() or not root.is_dir():
        print("dependency_inventory: set CODEFRIEND_REVIEW_SKILLS_ROOT to the authenticated review-skills resource directory", file=sys.stderr)
        return 2
    for relative, expected in RESOURCES.items():
        try:
            data = (root / relative).read_bytes()
        except OSError:
            print(f"dependency_inventory: missing owner resource: {relative}", file=sys.stderr)
            return 2
        if hashlib.sha256(data).hexdigest() != expected:
            print(f"dependency_inventory: owner resource identity mismatch: {relative}", file=sys.stderr)
            return 2
    script = root / "repo-dependency-review/scripts/test_prepare_dependency_review.py"
    # Preserve all original tests and their exit status; no skip or fallback route.
    return subprocess.run([sys.executable, str(script)], env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"}).returncode


if __name__ == "__main__":
    raise SystemExit(main())
