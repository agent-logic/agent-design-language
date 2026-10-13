#!/usr/bin/env python3
"""Validate explicit Runtime issue268 source resources before caller effects.

Pins cover five original offline consumers plus the plan validator, not live
Runtime, provider, model, cloud or installed-package acceptance.
"""
import hashlib
import os
from pathlib import Path
import sys
import json
import base64
import zlib

EXPECTED = {'adl/tools/issue268_runtime_qualification.cloudformation.yaml': 'e6fa89e5f1d8a18ec22de86ae036835692578c9de7c5972ebdb169dc8185db23', 'adl/tools/issue268_runtime_uts_task_panel.json': 'd0959ef79168d6e8fc05ba02cab7d98b267280511eb5884684e14150ded3617e', 'adl/tools/issue268_six_resident_uts_plan.json': '7fa623819342d86bcfd5b0d16c5a502c5a7950141a7292e5689f8d257ea75f8b', 'adl/tools/materialize_issue268_ollama_plan.py': 'b12738bffa746b9b0b0c7a35215ddced7771b9d7adaf627523db1c0e3369a4f3', 'adl/tools/run_issue268_continuity_uts_qualification.py': '580027455b08064b591ea69077b97e56f27048a0b414e4240f64cf898e98bedf', 'adl/tools/run_issue268_six_resident_uts_cycle.py': 'd104c2a4c3f5294e83587c9e30c422f2dba9d19543bd8740f702714d75f375cd', 'adl/tools/test_issue268_runtime_qualification_cloudformation.py': '93576398e544942e95313bf30103cae0cac487ea8198a5960fc7c849fb1db4d6', 'adl/tools/test_materialize_issue268_ollama_plan.py': '5b677e6899be9ef1b31cf491d152aacddcee5af926291485159d605992b3a60e', 'adl/tools/test_run_issue268_continuity_uts_qualification.py': '29386d1cdc2a8e1faa28948bd2d6dfb06daa2f37c9f016c5240c4f2794f9fab5', 'adl/tools/test_run_issue268_six_resident_uts_cycle.py': 'e5369948db2acf1e6bce02cd6a31f85d93dc0bf0851773ae5baa18cb5f09beff', 'adl/tools/test_warm_issue268_ollama_models.py': '1f404c1960abfe635d5d65ef8d398a35322576d906a4640c2ccbe7c7bf1f4da8', 'adl/tools/validate_issue268_six_resident_uts_plan.py': '6383833aee37ff75ea239294e3d5b5c29e28e92730683bc3a3c8ca32850a23da', 'adl/tools/warm_issue268_ollama_models.py': 'fef747c8e948eae40acd6aee96eaf7240a91c39b16ba03352843c7b6668d3c76'}

def main():
    value = os.environ.get("ADL_ISSUE268_RUNTIME_SOURCE_ROOT", "")
    if not value or not Path(value).is_absolute():
        raise SystemExit("issue268: explicit absolute ADL_ISSUE268_RUNTIME_SOURCE_ROOT required")
    root = Path(value).resolve()
    for relative, digest in EXPECTED.items():
        source = root / relative
        if not source.is_file() or hashlib.sha256(source.read_bytes()).hexdigest() != digest:
            raise SystemExit(f"issue268: qualified Runtime resource missing or changed: {relative}")
    if sys.argv[1:] == ["--remote-profile"]:
        payload = {name: base64.b64encode((root / name).read_bytes()).decode() for name in EXPECTED}
        program = "import base64,hashlib,json,os,pathlib,subprocess,sys,tempfile,zlib\n"
        program += "expected=" + repr(EXPECTED) + "\n"
        program += "payload=json.loads(zlib.decompress(base64.b64decode(sys.argv[1],validate=True)))\n"
        program += "assert set(payload)==set(expected)\n"
        program += "data={name:base64.b64decode(value,validate=True) for name,value in payload.items()}\n"
        program += "assert all(hashlib.sha256(data[name]).hexdigest()==digest for name,digest in expected.items())\n"
        program += "with tempfile.TemporaryDirectory(prefix='issue268-owner-') as tmp:\n"
        program += " for name,value in data.items():\n  path=pathlib.Path(tmp)/name;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(value);path.chmod(0o644)\n"
        program += " env=dict(os.environ);env['ADL_ISSUE268_RUNTIME_SOURCE_ROOT']=tmp\n"
        program += " result=subprocess.run(['bash','adl/tools/run_issue268_remote_resident_qualification.sh'],env=env)\n"
        program += " sys.exit(result.returncode)\n"
        packed=base64.b64encode(zlib.compress(json.dumps(payload,sort_keys=True).encode())).decode()
        encoded_program=base64.b64encode(program.encode()).decode()
        print(json.dumps(["python3","-c","import base64,sys;exec(base64.b64decode(sys.argv.pop(1),validate=True))",encoded_program,packed]))
    elif not sys.argv[1:]:
        print(root)
    else:
        raise SystemExit("issue268: unsupported resource helper arguments")

if __name__ == "__main__":
    main()
