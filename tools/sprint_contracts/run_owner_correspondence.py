#!/usr/bin/env python3
"""PVF: deterministic offline mapped historical conductor fixtures, small CPU/disk.
All native effects in the owner fixtures use local test doubles. This is not
full original 421-expression or native lifecycle acceptance.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

PIN_SHA256 = '99fc722126dc1594803351535b3545f7c62ed224fc5be635a91869cc60fc20c6'
VALIDATION = 'resources/docs/skill-distributions/original-contracts/validation'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check-inputs', action='store_true')
    args = parser.parse_args()
    selected = os.environ.get('CSDLC_CONDUCTOR_PACKAGE_ROOT', '')
    root = Path(selected)
    if not selected or not root.is_absolute() or not root.is_dir():
        print('Select the exact reviewed installed package using CSDLC_CONDUCTOR_PACKAGE_ROOT.', file=sys.stderr)
        return 2
    pin_bytes = (Path(__file__).resolve().parent / 'owner_inputs.json').read_bytes()
    if hashlib.sha256(pin_bytes).hexdigest() != PIN_SHA256:
        print('Conductor resource inventory changed; review required.', file=sys.stderr)
        return 2
    pins = json.loads(pin_bytes)
    inputs = {}
    for row in pins['resources']:
        path = root / row['path']
        try:
            data = path.read_bytes()
            mode = path.stat().st_mode & 0o777
        except OSError:
            print('Missing conductor resource: ' + row['path'], file=sys.stderr)
            return 2
        if hashlib.sha256(data).hexdigest() != row['sha256'] or mode != row['mode']:
            print('Changed conductor resource: ' + row['path'], file=sys.stderr)
            return 2
        inputs[row['path']] = (data, mode)
    if args.check_inputs:
        print('Exact mapped conductor inputs verified; no tests executed.')
        return 0
    with tempfile.TemporaryDirectory(prefix='adl-mapped-conductor-') as directory:
        scratch = Path(directory)
        for relative, (data, mode) in inputs.items():
            path = scratch / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            path.chmod(mode)
        scripts = scratch / 'composed-scripts'
        scripts.mkdir()
        for name, relative in pins['composed_scripts'].items():
            data, mode = inputs[relative]
            (scripts / name).write_bytes(data)
            (scripts / name).chmod(mode)
        home = scratch / 'home'
        home.mkdir()
        tmp = scratch / 'tmp'
        tmp.mkdir()
        env = {'PATH': os.environ.get('PATH', ''), 'HOME': str(home),
               'TMPDIR': str(tmp), 'PYTHONDONTWRITEBYTECODE': '1',
               'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null'}
        driver = scratch / VALIDATION / 'check_conductor_correspondence.py'
        result = subprocess.run([sys.executable, '-B', str(driver),
                                 '--scripts-dir', str(scripts), '--out', str(scratch / 'proof'),
                                 '--component', 'all'], cwd=scratch, env=env)
        # Preserve diagnostics before temporary fixture cleanup, including failures.
        proof = scratch / 'proof'
        for path in sorted(proof.rglob('*')) if proof.exists() else []:
            if path.is_file() and path.suffix in ('.stdout', '.stderr'):
                data = path.read_text()
                if data:
                    print(str(path.relative_to(proof)) + ':\n' + data, file=sys.stderr)
        return result.returncode


if __name__ == '__main__':
    raise SystemExit(main())
