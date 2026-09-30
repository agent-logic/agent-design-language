#!/usr/bin/env python3
"""Build an offline consumer from the verified artifact, outside producer source."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess

import package


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    package.verify(package.HERE / package.MANIFEST["artifact"]["name"], output / "dependencies")
    consumer = output / "consumer"
    (consumer / "src").mkdir(parents=True)
    shutil.copyfile(package.HERE / "consumer.rs", consumer / "src/main.rs")
    (consumer / "Cargo.toml").write_text('''[package]
name = "remote-validation-artifact-consumer"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
adl-remote-validation = { version = "=0.92.1", path = "../dependencies/adl-remote-validation-0.92.1" }
''')
    env = dict(os.environ, CARGO_TARGET_DIR=str(output / "target"))
    shutil.copyfile(package.HERE / "consumer.Cargo.lock", consumer / "Cargo.lock")
    commands = [["cargo", "run", "--offline", "--locked"]]
    for index, argv in enumerate(commands):
        result = subprocess.run(argv, cwd=consumer, env=env, capture_output=True, text=True)
        (output / f"command-{index}.log").write_text(result.stdout + result.stderr)
        if result.returncode:
            raise SystemExit(f"consumer command {index} failed; see retained log")
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--offline", "--locked", "--format-version", "1"], cwd=consumer, env=env))
    local = [p for p in metadata["packages"] if p["source"] is None]
    if {p["name"] for p in local} != {"remote-validation-artifact-consumer", "adl-remote-validation"}:
        raise SystemExit("unexpected local package dependency")
    if any(not Path(p["manifest_path"]).is_relative_to(output) for p in local):
        raise SystemExit("consumer reached outside the extracted artifact")
    record = {"status": "passed", "artifact": package.MANIFEST["artifact"],
              "source_revision": package.MANIFEST["source_revision"],
              "consumer_lock_sha256": package.sha((consumer / "Cargo.lock").read_bytes()),
              "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "scope": "offline local artifact consumer; no OS sandbox or cloud/installed qualification claim"}
    (output / "result.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record))


if __name__ == "__main__":
    main()
