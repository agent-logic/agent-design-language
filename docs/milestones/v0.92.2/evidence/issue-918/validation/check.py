"""Require real validator success, nonempty proof, and retained approval guards."""
import json
import subprocess
import sys
from pathlib import Path

mode = sys.argv[1]
assert mode in ('default', 'self-test')
packet = Path('docs/milestones/v0.92.2/evidence/issue-918')
command = [sys.executable, str(packet / 'validate_packet.py')]
if mode == 'self-test':
    command.append('--self-test')
result = subprocess.run(command, capture_output=True, text=True)
print(result.stdout, end='')
print(result.stderr, end='', file=sys.stderr)
if result.returncode:
    raise SystemExit(result.returncode)
data = json.loads(result.stdout)
manifest = json.loads((packet / 'PUBLICATION_MANIFEST.json').read_text())
assert data['status'] == 'pass' and data['failures'] == []
assert data['artifacts'] == len(manifest['artifacts']) > 0
assert data['release_approved'] is False and data['publication_authorized'] is False
assert data['final_acceptance'] == 'pending'
assert data['negative_fixtures'] > 0 if mode == 'self-test' else data['negative_fixtures'] == 0
