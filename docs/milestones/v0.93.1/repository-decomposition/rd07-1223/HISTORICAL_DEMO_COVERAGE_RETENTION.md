# Historical WP20 demo coverage contract

Retain RD07-2150 and RD07-2234 as the original validator/test pair. At revision `9f373f5f04b0f8c9dc6e3e6cbf348fddec98486c` the unchanged suite passed two positive validations and twelve expected refusals. The thirteen exact input paths and Git blobs are recorded in `HISTORICAL_DEMO_COVERAGE_RETENTION.json`. The other eleven input paths are outside the original retirement inventory. Their historical Git objects provide the reproduction closure; no copied fixture tree is added to current source.

The attempt against current documents failed before negative cases ran: the later corrective ledger uses `implemented_with_evidence`, which the original validator does not accept. Four milestone documents have changed since the pinned revision. This retained historical proof does not validate those current documents or current milestone acceptance. Neither the validator enum nor milestone records were changed to manufacture a pass.

To reproduce when necessary, run the following from the ADL issue checkout. It extracts only the thirteen authenticated Git inputs into a new FastWork directory and runs the unchanged historical script there. Its generated negative fixtures remain isolated from current evidence. Existing reviewed execution is sufficient for this retirement record; adoption did not rerun it.

```python
import json
import pathlib
import subprocess
import tempfile

packet = json.loads(pathlib.Path(
    'docs/milestones/v0.93.1/repository-decomposition/rd07-1223/'
    'HISTORICAL_DEMO_COVERAGE_RETENTION.json'
).read_text())
inputs = packet['inputs']
destination = pathlib.Path(tempfile.mkdtemp(
    prefix='adl1223-wp20-', dir='/Volumes/FastWork/c1223'
))
for item in inputs['files']:
    revision_path = inputs['revision'] + ':' + item['path']
    actual = subprocess.check_output(['git', 'rev-parse', revision_path], text=True).strip()
    if actual != item['git_blob']:
        raise RuntimeError('Pinned input identity mismatch: ' + item['path'])
    target = destination / item['path']
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(subprocess.check_output(['git', 'cat-file', 'blob', actual]))
subprocess.run(['bash', 'adl/tools/test_v092_demo_proof_coverage.sh'],
               cwd=destination, check=True)
```

Execution evidence: `/Volumes/FastWork/c1223/historical-demo-coverage-pinned-r1`. Earlier adverse evidence: `/Volumes/FastWork/c1223/historical-demo-coverage-r1`. Stream and input hashes are retained in the JSON record. No runtime, provider, native lifecycle operation, or deletion is established by this result.
