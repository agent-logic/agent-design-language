#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import pathlib
import subprocess
import sys
import tempfile


ROOT = pathlib.Path(__file__).resolve().parents[2]
ORCHESTRATOR = ROOT / "adl/tools/run_issue268_continuity_uts_qualification.py"


def main() -> None:
    assert "strict=True" not in ORCHESTRATOR.read_text(encoding="utf-8")
    for helper in ("uts_benchmark_panel.py", "uts_benchmark_tasks.py"):
        source = (ROOT / "adl/tools/benchmark" / helper).read_text(encoding="utf-8")
        assert "from __future__ import annotations" in source
    cycle_source = (ROOT / "adl/tools/run_issue268_six_resident_uts_cycle.py").read_text(encoding="utf-8")
    assert '"--self-check-task-panel-file"' in cycle_source
    os.environ["ADL_UTS_LOCAL_NUM_PREDICT"] = "128"
    os.environ["ADL_UTS_LOCAL_NUM_CTX"] = "32768"
    os.environ["ADL_UTS_OLLAMA_KEEP_ALIVE"] = "-1"
    sys.path.insert(0, str(ROOT / "adl/tools"))
    import uts_benchmark_runner as benchmark_runner
    adapter_request = benchmark_runner.adapter_request(
        {
            "id": "issue268-test",
            "provider_kind": "local",
            "provider": "ollama-local",
            "model_id": "llama3.1:8b",
        },
        "test prompt",
        "regular",
    )
    assert adapter_request["max_output_tokens"] == 128
    assert adapter_request["context_window_tokens"] == 32768
    assert adapter_request["local_keep_alive"] == "-1"
    with tempfile.TemporaryDirectory(prefix="issue268-continuity-uts-") as temporary:
        root = pathlib.Path(temporary)
        fake_uts = root / "fake_uts.py"
        fake_uts.write_text(
            """#!/usr/bin/env python3
import json,pathlib,sys
a=sys.argv; state=pathlib.Path(a[a.index('--state')+1]); evidence=pathlib.Path(a[a.index('--evidence-dir')+1]); plan=json.load(open(a[a.index('--plan')+1])); phase=a[a.index('--phase')+1]; evidence.mkdir(parents=True,exist_ok=True)
if phase=='pre':
 import hashlib
 digest=lambda x:hashlib.sha256(json.dumps(x,separators=(',',':'),sort_keys=True).encode()).hexdigest()
 r={x['agent_id']:{'role':x['role'],'model':x['model'],'role_digest':digest({'agent_id':x['agent_id'],'role':x['role']}),'tool_authority_digest':digest({'agent_id':x['agent_id'],'tool_authority':x['tool_authority']}),'sequence':1,'completed_case_ids':[x['pre_recovery_case']],'pending_case_ids':[x['post_recovery_case']],'uts_report_sha256':'a'*64,'continuation_request_sha256':'b'*64,'checkpoint_lineage':['f'*64]} for x in plan['residents']}; value={'schema':'adl.issue268.six_resident_uts_state.v1','phase':'pre_complete','residents':r}
else:
 value=json.load(open(state)); value['phase']='post_complete'; value['all_pending_empty']=True
 for x in value['residents'].values(): x['sequence']=2; x['completed_case_ids']+=x['pending_case_ids']; x['pending_case_ids']=[]; x['post_restore_uts_report_sha256']='c'*64; x['checkpoint_lineage'].append('1'*64)
state.write_text(json.dumps(value)+'\\n')
""",
            encoding="utf-8",
        )
        fake_uts.chmod(0o755)
        fake_continuity = root / "fake_continuity.py"
        fake_continuity.write_text(
            """#!/usr/bin/env python3
import json,pathlib,sys
a=sys.argv; command=a[1]; inp=json.load(open(a[a.index('--input')+1])); out=pathlib.Path(a[a.index('--output')+1]); residents=inp['residents']; assert len(residents)==6; assert len({x['agent_id'] for x in residents})==6
if command=='preflight': value={'status':'passed','resident_count':6}
elif command=='dehydrate': value={'generation':1,'population_sha256':'9'*64,'resident_count':6,'admission_open':False}
elif command=='restore': value={'generation':1,'population_sha256':'9'*64,'resident_count':6,'admission_open':True}
elif command=='complete':
 assert all(len(x['completed_task_sha256'])==64 and len(x['continuation_request_sha256'])==64 and x['next_task_sha256']=='c'*64 for x in residents); value={'generation':1,'population_sha256':'9'*64,'resident_count':6,'admission_open':True,'continuation_verified':True}
else: raise SystemExit(2)
out.write_text(json.dumps(value)+'\\n')
""",
            encoding="utf-8",
        )
        fake_continuity.chmod(0o755)
        plan = json.loads((ROOT / "adl/tools/issue268_six_resident_uts_plan.json").read_text())
        for resident in plan["residents"]:
            resident["model_ref_sha256"] = "d" * 64
            resident["quantization"] = "Q4_K_M"
            resident["configuration_sha256"] = "e" * 64
            spec = root / "agents" / resident["agent_id"] / "agent.yaml"
            spec.parent.mkdir(parents=True)
            canonical = lambda value: __import__("hashlib").sha256(json.dumps(value, separators=(",", ":"), sort_keys=True).encode()).hexdigest()
            spec.write_text(json.dumps({
                "schema": "adl.issue268.resident_agent_spec.v1",
                "agent_id": resident["agent_id"],
                "role": resident["role"],
                "role_digest": canonical({"agent_id": resident["agent_id"], "role": resident["role"]}),
                "tool_authority": resident["tool_authority"],
                "tool_authority_digest": canonical({"agent_id": resident["agent_id"], "tool_authority": resident["tool_authority"]}),
                "model": resident["model"],
                "model_ref_sha256": resident["model_ref_sha256"],
                "configuration_sha256": resident["configuration_sha256"],
            }) + "\n", encoding="utf-8")
        plan["materialization"] = {
            "schema": "adl.issue268.ollama_plan_materialization.v1",
            "template_sha256": __import__("hashlib").sha256(
                (ROOT / "adl/tools/issue268_six_resident_uts_plan.json").read_bytes()
            ).hexdigest(),
            "source": "ollama_api_tags",
        }
        plan_path = root / "plan.json"
        plan_path.write_text(json.dumps(plan) + "\n", encoding="utf-8")
        evidence = root / "evidence"
        command = [
            sys.executable, str(ORCHESTRATOR),
            "--continuity-bin", str(fake_continuity),
            "--runtime-root", str(root / "runtime"),
            "--build-cache-root", str(root / "build-cache"),
            "--agent-spec-dir", str(root / "agents"),
            "--runtime-volume-identity-sha256", "f" * 64,
            "--state", str(root / "state.json"),
            "--evidence-dir", str(evidence),
            "--plan", str(plan_path),
            "--uts-runner", str(fake_uts),
        ]
        subprocess.run(command, cwd=ROOT, check=True)
        receipt = json.loads((evidence / "qualification-receipt.json").read_text())
        assert receipt["status"] == "passed" and receipt["resident_count"] == 6
        assert json.loads((root / "state.json").read_text())["phase"] == "post_complete"
        assert (evidence / "dehydration-input.json").is_file()
        assert (evidence / "continuation-input.json").is_file()

        missing = root / "agents" / plan[0]["agent_id"] if False else root / "agents" / plan["residents"][0]["agent_id"] / "agent.yaml"
        missing.unlink()
        failed = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        assert failed.returncode != 0 and "six existing-agent specs are required" in failed.stderr
    print("PASS: issue268 continuity-coupled six-resident UTS qualification")


if __name__ == "__main__":
    main()
