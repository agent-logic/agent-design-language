#!/usr/bin/env python3
"""Check tracked UTF-8 text for whitespace defects; no external dependencies.

PVF: deterministic repository hygiene proof, small local CPU/filesystem,
no network or provider access. Required bootstrap CI, not product qualification.
"""
from pathlib import Path
import subprocess
import sys


def main():
    root = Path(__file__).resolve().parents[1]
    result = subprocess.run(
        ["git", "ls-files", "-z"], cwd=root, check=True, stdout=subprocess.PIPE
    )
    failures = []
    checked = 0
    for raw in result.stdout.split(b"\0"):
        if not raw:
            continue
        relative = raw.decode("utf-8")
        path = root / relative
        if path.is_symlink() or not path.is_file():
            continue
        data = path.read_bytes()
        if b"\0" in data:
            continue
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            continue  # Binary/non-UTF-8 assets need their own future validators.
        checked += 1
        if data and not data.endswith(b"\n"):
            failures.append(f"{relative}: missing final newline")
        for number, line in enumerate(text.splitlines(keepends=True), 1):
            if "\r" in line:
                failures.append(f"{relative}:{number}: CR line ending")
            if line.rstrip("\r\n").endswith((" ", "\t")):
                failures.append(f"{relative}:{number}: trailing whitespace")
    if not checked:
        failures.append("No tracked UTF-8 text files checked; stage intended files first")
    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    print(f"Repository hygiene passed: {checked} tracked text files checked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
