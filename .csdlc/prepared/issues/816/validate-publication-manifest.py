#!/usr/bin/env python3
"""Fail closed unless the OBS-B manifest classifies every declared artifact."""

from __future__ import annotations

import pathlib
import subprocess
import sys


def fail(message: str) -> None:
    raise SystemExit(f"manifest_integrity:{message}")


def markdown_paths(path: pathlib.Path, heading: str, bullets: bool) -> set[str]:
    lines = path.read_text(encoding="utf-8").splitlines()
    try:
        start = lines.index(heading) + 1
    except ValueError:
        fail(f"missing_heading:{path}:{heading}")
    values: set[str] = set()
    for line in lines[start:]:
        if line.startswith("## "):
            break
        value = line.strip()
        if not value:
            continue
        if bullets:
            if value.startswith("- "):
                values.add(value[2:])
        else:
            values.add(value)
    return values


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage:<repo-root> <manifest>")
    root = pathlib.Path(sys.argv[1]).resolve()
    manifest = pathlib.Path(sys.argv[2]).resolve()
    tracked = subprocess.run(
        ["git", "ls-files", ".csdlc/evidence/512"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.splitlines()
    expected = set(tracked)
    expected |= markdown_paths(root / ".csdlc/issues/512/cards/srp.md", "## Scope", False)
    expected |= markdown_paths(root / ".csdlc/issues/512/cards/sor.md", "## Artifacts", True)

    classified: dict[str, str] = {}
    for number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), 1):
        if not line or line.startswith("#"):
            continue
        fields = line.split("|", 2)
        if len(fields) < 2:
            fail(f"malformed_line:{number}")
        role, path = fields[0], fields[1]
        reason = fields[2].strip() if len(fields) == 3 else ""
        if role not in {"ui", "evidence", "excluded"}:
            fail(f"unknown_role:{role}")
        if path in classified:
            fail(f"duplicate_path:{path}")
        if role == "excluded" and (len(reason) < 20 or "non-publication" not in reason):
            fail(f"unreviewed_exclusion:{path}")
        if role != "excluded" and reason:
            fail(f"unexpected_classification:{path}")
        classified[path] = role

    missing = sorted(expected - classified.keys())
    extra = sorted(classified.keys() - expected)
    if missing:
        fail(f"missing_declared_artifact:{missing[0]}")
    if extra:
        fail(f"undeclared_manifest_artifact:{extra[0]}")
    print(f"manifest_integrity:pass:{len(expected)}")


if __name__ == "__main__":
    main()
