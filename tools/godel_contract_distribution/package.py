#!/usr/bin/env python3
"""Frozen data distribution; verification does not imply producer acceptance."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

HERE = Path(__file__).resolve().parent
LOCK = json.loads((HERE / 'artifact-lock.json').read_text())
MANIFEST_BYTES = (HERE / 'source-manifest.json').read_bytes()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def manifest():
    if sha(MANIFEST_BYTES) != LOCK['manifest_sha256']:
        raise ValueError('manifest digest mismatch')
    value = json.loads(MANIFEST_BYTES)
    if any(value[k] != LOCK[k] for k in ('package', 'version', 'source_revision')):
        raise ValueError('distribution identity mismatch')
    return value


def expected():
    rows = {r['path']: r for r in manifest()['files']}
    rows['source-manifest.json'] = {'bytes': len(MANIFEST_BYTES), 'sha256': sha(MANIFEST_BYTES)}
    return rows


def inspect_archive(data):
    rows, payloads = expected(), {}
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
        for member in archive:
            if (member.name not in rows or member.name in payloads or not member.isfile()
                    or member.size != rows[member.name]['bytes'] or member.mode != 0o644):
                raise ValueError('invalid archive inventory')
            payload = archive.extractfile(member).read()
            if sha(payload) != rows[member.name]['sha256']:
                raise ValueError('payload digest mismatch')
            payloads[member.name] = payload
    if payloads.keys() != rows.keys():
        raise ValueError('missing archive input')
    return payloads


def pack(payloads):
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode='w', format=tarfile.USTAR_FORMAT) as archive:
        for name, data in sorted(payloads.items()):
            member = tarfile.TarInfo(name)
            member.size, member.mode, member.mtime = len(data), 0o644, 0
            archive.addfile(member, io.BytesIO(data))
    data = bytearray(gzip.compress(raw.getvalue(), compresslevel=9, mtime=0))
    data[9] = 255
    return bytes(data)


def verify(artifact, output=None):
    data = Path(artifact).read_bytes()
    if len(data) != LOCK['artifact']['bytes'] or sha(data) != LOCK['artifact']['sha256']:
        raise ValueError('artifact digest or size mismatch')
    payloads = inspect_archive(data)
    if output is not None:
        output = Path(output)
        output.mkdir(parents=True, exist_ok=False)
        for name, payload in payloads.items():
            target = output / name
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as stream:
                stream.write(payload)
        verify_root(output)
    return LOCK


def verify_root(root):
    root = Path(root)
    if not root.is_dir() or root.is_symlink():
        raise ValueError('explicit real installed root required')
    if any(p.is_symlink() for p in root.parents):
        raise ValueError('aliased installed root')
    rows = expected()
    observed = set()
    for path in root.rglob('*'):
        if path.is_symlink():
            raise ValueError('aliased installed input')
        if path.is_file():
            name = path.relative_to(root).as_posix()
            if name not in rows:
                raise ValueError('unexpected installed input')
            data = path.read_bytes()
            if len(data) != rows[name]['bytes'] or sha(data) != rows[name]['sha256']:
                raise ValueError('installed input changed')
            observed.add(name)
        elif not path.is_dir():
            raise ValueError('unsupported installed input')
    if observed != rows.keys():
        raise ValueError('missing installed input')
    return LOCK


def export(repository, output):
    info = manifest()
    payloads = {'source-manifest.json': MANIFEST_BYTES}
    for row in info['files']:
        spec = info['source_revision'] + ':' + row['path']
        data = subprocess.check_output(['git', '-C', str(repository), 'show', spec])
        blob = subprocess.check_output(['git', '-C', str(repository), 'rev-parse', spec], text=True).strip()
        if blob != row['git_blob'] or sha(data) != row['sha256'] or len(data) != row['bytes']:
            raise ValueError('frozen source provenance mismatch')
        payloads[row['path']] = data
    data = pack(payloads)
    if sha(data) != LOCK['artifact']['sha256']:
        raise ValueError('reproduction digest mismatch')
    inspect_archive(data)
    output = Path(output)
    with output.open('xb') as stream:
        stream.write(data)
    return LOCK


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    check = sub.add_parser('verify')
    check.add_argument('--artifact', required=True)
    check.add_argument('--extract')
    root = sub.add_parser('verify-root')
    root.add_argument('--root', required=True)
    build = sub.add_parser('export')
    build.add_argument('--repository', required=True)
    build.add_argument('--output', required=True)
    args = parser.parse_args()
    result = (verify(args.artifact, args.extract) if args.command == 'verify' else
              verify_root(args.root) if args.command == 'verify-root' else
              export(args.repository, args.output))
    print(json.dumps({'verified_distribution': result, 'producer_acceptance_implied': False}))


if __name__ == '__main__':
    main()
