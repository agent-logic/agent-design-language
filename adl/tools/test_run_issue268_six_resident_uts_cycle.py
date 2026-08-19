#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import subprocess
import sys
import tempfile


ROOT = pathlib.Path(__file__).resolve().parents[2]
RUNNER = ROOT / "adl/tools/run_issue268_six_resident_uts_cycle.py"


def main() -> None:
    with tempfile.TemporaryDirectory(prefix="issue268-uts-cycle-") as temporary:
        root = pathlib.Path(temporary)
        fake = root / "fake_runner.py"
        fake.write_text(
            """#!/usr/bin/env python3
import json,pathlib,sys
args=sys.argv[1:]
assert __import__('os').environ['ADL_UTS_LOCAL_TEST_TIMEOUT_SECONDS']=='600'
assert __import__('os').environ['ADL_UTS_LOCAL_NUM_PREDICT']=='64'
assert __import__('os').environ['ADL_UTS_LOCAL_NUM_CTX']=='4096'
assert __import__('os').environ['ADL_UTS_OLLAMA_KEEP_ALIVE']=='30m'
models=pathlib.Path(args[1]).read_text().strip()
out=pathlib.Path(args[2])
task_panel=pathlib.Path(args[args.index('--task-panel-file')+1])
task=json.loads(task_panel.read_text())['tasks'][0]['id']
self_check_panel=pathlib.Path(args[args.index('--self-check-task-panel-file')+1])
assert len(json.loads(self_check_panel.read_text())['tasks']) == 11
lane=lambda: {'status':'evaluated','passed_count':1,'total_cases':1,'full_support':True,'cases':[{'task_id':task}]}
out.write_text(json.dumps({'schema_version':'uts_benchmark_runner.v1','deterministic_self_check':{'passed':True},'models':[{'candidate_id':models,'lanes':{'regular':lane(),'uts_only':lane(),'uts_acc':lane()}}]})+'\\n')
""",
            encoding="utf-8",
        )
        fake.chmod(0o755)
        state = root / "state.json"
        evidence = root / "evidence"
        common = [
            sys.executable,
            str(RUNNER),
            "--state",
            str(state),
            "--evidence-dir",
            str(evidence),
            "--runner",
            str(fake),
        ]
        subprocess.run(common + ["--phase", "pre"], cwd=ROOT, check=True)
        pre = json.loads(state.read_text())
        assert pre["phase"] == "pre_complete"
        assert len(pre["residents"]) == 6
        assert all(row["sequence"] == 1 for row in pre["residents"].values())
        assert all(len(row["completed_case_ids"]) == 1 for row in pre["residents"].values())
        assert all(len(row["pending_case_ids"]) == 1 for row in pre["residents"].values())
        assert len({row["role_digest"] for row in pre["residents"].values()}) == 6
        assert len({row["tool_authority_digest"] for row in pre["residents"].values()}) == 6
        assert all(len(row["checkpoint_lineage"]) == 1 for row in pre["residents"].values())
        replay = subprocess.run(common + ["--phase", "pre"], cwd=ROOT, capture_output=True, text=True)
        assert replay.returncode != 0 and "refusing completed-case replay" in replay.stderr
        subprocess.run(common + ["--phase", "post"], cwd=ROOT, check=True)
        post = json.loads(state.read_text())
        assert post["phase"] == "post_complete"
        assert post["all_pending_empty"] is True
        assert all(row["sequence"] == 2 for row in post["residents"].values())
        assert all(len(row["completed_case_ids"]) == 2 for row in post["residents"].values())
        assert all(not row["pending_case_ids"] for row in post["residents"].values())
        assert all(len(row["checkpoint_lineage"]) == 2 for row in post["residents"].values())
        assert len(list(evidence.glob("pre-*.json"))) == 6
        assert len(list(evidence.glob("post-*.json"))) == 6
    print("PASS: issue268 six-resident UTS cycle")


if __name__ == "__main__":
    main()
