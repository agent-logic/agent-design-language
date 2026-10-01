#!/usr/bin/env python3
"""Build a pinned installed demo package from an already-built executable."""
import argparse, hashlib, io, json, pathlib, subprocess, tarfile
ROOT = pathlib.Path(__file__).resolve().parents[2]
DATA = ["demos/rust-transpiler/workflow/rust_transpiler_demo.yaml", "demos/rust-transpiler/output/workflow_runtime.rs", "LICENSE"]
def build(binary, output):
    revision = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", "HEAD"], text=True).strip()
    payload = {p: (ROOT / p).read_bytes() for p in DATA}
    payload["bin/transpiler_demo"] = binary.read_bytes()
    manifest = {"schema": "adl.transpiler_demo.installed.v1", "source_revision": revision, "classification": "bounded_demo_scaffold", "members": [{"path": p, "sha256": hashlib.sha256(b).hexdigest(), "bytes": len(b), "mode": 493 if p.startswith("bin/") else 420} for p,b in sorted(payload.items())]}
    payload["manifest.json"] = (json.dumps(manifest, sort_keys=True, indent=2)+"\n").encode()
    with output.open("xb") as raw:
        with tarfile.open(fileobj=raw, mode="w") as tar:
            for p,b in sorted(payload.items()):
                info=tarfile.TarInfo(p); info.size=len(b); info.mode=493 if p.startswith("bin/") else 420
                tar.addfile(info, io.BytesIO(b))
    return manifest
if __name__ == "__main__":
    parser=argparse.ArgumentParser(); parser.add_argument("--binary",type=pathlib.Path,required=True); parser.add_argument("--output",type=pathlib.Path,required=True); args=parser.parse_args(); build(args.binary,args.output)
