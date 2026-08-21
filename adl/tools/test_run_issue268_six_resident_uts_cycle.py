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
    with tempfile.TemporaryDirectory(prefix="issue268-runtime-cycle-") as temporary:
        root = pathlib.Path(temporary)
        fake = root / "fake_adl.py"
        fake.write_text(
            """#!/usr/bin/env python3
import json,pathlib,sys
args=sys.argv[1:]
assert args[:2] == ['agent','tick']
spec=json.loads(pathlib.Path(args[args.index('--spec')+1]).read_text())
state=pathlib.Path(spec['state_root'])
cycles=state/'cycles'
cycles.mkdir(parents=True,exist_ok=True)
number=len(list(cycles.glob('cycle-*')))+1
cycle=cycles/f'cycle-{number:06d}'
cycle.mkdir()
receipt={'schema':'adl.runtime.resident_tool_receipt.v1','resident_id':spec['agent_instance_id'],
 'authority_id':spec['tool_authority']['authority_id'],'authority_sha256':spec['tool_authority']['authority_sha256'],
 'cycle_id':f'cycle-{number:06d}','checkpoint_lineage':f'continuity_checkpoint.json#sha256:{number:064x}',
 'proposal_sha256':'a'*64,'proposal_id':'sha256:'+'b'*64,'acc_contract_id':'acc.runtime.observe',
 'gate_reason_code':'allowed','adapter_id':'adapter.runtime.observe.dry_run','decision':'executed',
 'reason_code':'governed_execution_completed'}
(cycle/'resident_tool_receipts.json').write_text(json.dumps([receipt])+'\\n')
print(json.dumps({'state':'idle','completed_cycle_count':number}))
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
            "--runtime-bin",
            str(fake),
            "--runtime-root",
            str(root / "runtime"),
        ]
        subprocess.run(common + ["--phase", "pre"], cwd=ROOT, check=True)
        pre = json.loads(state.read_text())
        assert pre["schema"] == "adl.issue268.six_resident_uts_state.v2"
        assert pre["phase"] == "pre_complete"
        assert len(pre["residents"]) == 6
        assert all(row["sequence"] == 1 for row in pre["residents"].values())
        assert all(len(row["completed_case_ids"]) == 1 for row in pre["residents"].values())
        assert all(len(row["pending_case_ids"]) == 1 for row in pre["residents"].values())
        assert len({row["role_digest"] for row in pre["residents"].values()}) == 6
        assert len({row["tool_authority_digest"] for row in pre["residents"].values()}) == 6
        assert len({row["runtime_authority_sha256"] for row in pre["residents"].values()}) == 6
        replay = subprocess.run(common + ["--phase", "pre"], cwd=ROOT, capture_output=True, text=True)
        assert replay.returncode != 0 and "refusing replay" in replay.stderr
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
    print("PASS: issue268 six real Runtime resident cycles")


if __name__ == "__main__":
    main()
