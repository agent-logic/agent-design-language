#!/usr/bin/env python3
"""Check packet integrity against its manifest; this does not authenticate it."""
import argparse
import hashlib
import json
from pathlib import Path
import re


def verify(root):
    manifest = json.loads((root / 'manifest.json').read_text())
    if manifest['schema'] != 'codefriend.release_preparation.v1' or manifest['ready_for_activation'] is not False:
        raise ValueError('unsupported_manifest')
    for name in ('adl_revision', 'website_revision'):
        if not re.fullmatch(r'[0-9a-f]{40}', manifest[name]):
            raise ValueError('invalid_revision')
    actual = {}
    for path in sorted(root.rglob('*')):
        if path.is_symlink():
            raise ValueError('symlink_forbidden')
        if not path.is_file() and not path.is_dir():
            raise ValueError('nonregular_object_forbidden')
        if path.is_file() and path != root / 'manifest.json':
            actual[str(path.relative_to(root))] = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != manifest['files']:
        raise ValueError('packet_integrity_mismatch')
    return len(actual)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('packet', type=Path)
    args = parser.parse_args()
    try:
        count = verify(args.packet)
        print(json.dumps(dict(status='integrity_only_not_authenticated', files=count, ready_for_activation=False)))
    except (ValueError, KeyError, TypeError, OSError):
        parser.exit(1, 'packet_integrity_failed\n')
