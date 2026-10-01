#!/usr/bin/env python3
"""Install only an authenticated archive with the exact bounded member set."""
import argparse, hashlib, json, pathlib, tarfile
EXPECTED = {"bin/transpiler_demo", "LICENSE", "demos/rust-transpiler/workflow/rust_transpiler_demo.yaml", "demos/rust-transpiler/output/workflow_runtime.rs"}
def install(archive, expected_sha, destination):
    if hashlib.sha256(archive.read_bytes()).hexdigest() != expected_sha:
        raise ValueError("archive digest mismatch")
    with tarfile.open(archive) as tar:
        members=tar.getmembers()
        if len(members)!=5 or {m.name for m in members} != EXPECTED | {"manifest.json"} or any(not m.isfile() for m in members):
            raise ValueError("unexpected members or types")
        manifest=json.load(tar.extractfile("manifest.json"))
        if manifest.get("schema")!="adl.transpiler_demo.installed.v1" or len(manifest.get("members",[]))!=4 or {m["path"] for m in manifest["members"]}!=EXPECTED:
            raise ValueError("invalid manifest")
        payload={}
        for m in manifest["members"]:
            info=tar.getmember(m["path"]); raw=tar.extractfile(info).read()
            mode=0o755 if m["path"].startswith("bin/") else 0o644
            if hashlib.sha256(raw).hexdigest()!=m["sha256"] or len(raw)!=m["bytes"] or m["mode"]!=mode or info.mode!=mode:
                raise ValueError("member mismatch")
            payload[m["path"]]=(raw,mode)
        destination.mkdir()
        for name,(raw,mode) in payload.items():
            path=destination/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(raw);path.chmod(mode)
        (destination/"manifest.json").write_text(json.dumps(manifest,sort_keys=True,indent=2)+"\n")
    return manifest
if __name__=="__main__":
    p=argparse.ArgumentParser();p.add_argument("--archive",type=pathlib.Path,required=True);p.add_argument("--sha256",required=True);p.add_argument("--destination",type=pathlib.Path,required=True);a=p.parse_args();install(a.archive,a.sha256,a.destination)
