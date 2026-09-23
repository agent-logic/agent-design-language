#!/usr/bin/env python3
"""Install only reviewed artifact identities; never discover a release at runtime."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import tarfile
import tempfile
from urllib.parse import urlsplit

MAX_ARCHIVE = 256 * 1024 * 1024
MAX_BINARY = 256 * 1024 * 1024


def identity(manifest, binary, target):
    data = json.loads(Path(manifest).read_text())
    if data.get('schema') != 'adl.remote-bootstrap.v1':
        raise ValueError('unsupported bootstrap identity schema')
    record = data['tools'][binary][target]
    if not re.fullmatch(r'v?\d+\.\d+\.\d+(?:[-+][A-Za-z0-9.-]+)?', record.get('version', '')):
        raise ValueError('exact version required')
    if not re.fullmatch(r'[0-9a-f]{64}', record.get('sha256', '')):
        raise ValueError('approved SHA-256 required')
    if record.get('source') == 'https':
        parsed = urlsplit(record['url'])
        if parsed.scheme != 'https' or not parsed.hostname or parsed.username or parsed.password or parsed.fragment:
            raise ValueError('HTTPS artifact URL required')
        if re.search(r'(^|/)(latest|stable)(/|$)', parsed.path):
            raise ValueError('mutable release URL rejected')
    elif record.get('source') == 's3':
        for key in ('bucket', 'key', 'version_id'):
            if not isinstance(record.get(key), str) or not record[key].strip():
                raise ValueError('S3 bucket, key and VersionId required')
        if record['version_id'] == 'null':
            raise ValueError('unversioned S3 object rejected')
    else:
        raise ValueError('unsupported artifact source')
    if binary == 'rustup-init' and not re.fullmatch(r'\d+\.\d+\.\d+', record.get('toolchain', '')):
        raise ValueError('exact Rust toolchain required')
    return record


def verify_digest(path, expected):
    if path.stat().st_size > MAX_ARCHIVE:
        raise ValueError('artifact exceeds size bound')
    with path.open('rb') as stream:
        digest = hashlib.sha256()
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(chunk)
    if digest.hexdigest() != expected:
        raise ValueError('artifact SHA-256 mismatch')


def extract_binary(archive, binary, destination):
    # Never extract a member path: validate every header, then copy one regular
    # file to our chosen destination. Links/devices and ambiguous names fail.
    with tarfile.open(archive, 'r:gz') as bundle:
        selected = None
        names = set()
        total = 0
        for count, member in enumerate(bundle, 1):
            name = PurePosixPath(member.name)
            if count > 10000 or name.is_absolute() or '..' in name.parts or '\\' in member.name:
                raise ValueError('unsafe archive path or member count')
            if member.name in names or not (member.isfile() or member.isdir()):
                raise ValueError('duplicate or nonregular archive member')
            names.add(member.name)
            total += member.size
            if member.size < 0 or total > MAX_ARCHIVE:
                raise ValueError('archive exceeds expanded size bound')
            if name.name == binary and member.isfile():
                if selected is not None or member.size == 0 or member.size > MAX_BINARY:
                    raise ValueError('missing or ambiguous binary')
                selected = member
        if selected is None:
            raise ValueError('archive does not contain requested binary')
        with bundle.extractfile(selected) as source, destination.open('xb') as output:
            shutil.copyfileobj(source, output)
    destination.chmod(0o755)


def download(record, path):
    if record['source'] == 'https':
        subprocess.run(['curl', '--fail', '--silent', '--show-error', '--location',
                        '--proto', '=https', '--proto-redir', '=https',
                        '--max-filesize', str(MAX_ARCHIVE), record['url'], '-o', str(path)], check=True)
    else:
        subprocess.run(['aws', 's3api', 'get-object', '--bucket', record['bucket'],
                        '--key', record['key'], '--version-id', record['version_id'], str(path)],
                       check=True, stdout=subprocess.DEVNULL)


def install(manifest, binary, target, destination, expected_url=''):
    record = identity(manifest, binary, target)
    if expected_url and (record['source'] != 'https' or record['url'] != expected_url):
        raise ValueError('configured URL does not match approved identity')
    destination = Path(destination)
    destination.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='.verified-bootstrap-', dir=destination) as scratch:
        scratch = Path(scratch)
        artifact = scratch / 'artifact'
        download(record, artifact)
        verify_digest(artifact, record['sha256'])
        installed = scratch / binary
        if binary == 'rustup-init':
            artifact.rename(installed)
            installed.chmod(0o755)
            subprocess.run([str(installed), '-y', '--profile', 'minimal', '--default-toolchain', record['toolchain']], check=True)
        else:
            extract_binary(artifact, binary, installed)
            os.replace(installed, destination / binary)
    print(f"verified bootstrap: {binary} version={record['version']} sha256={record['sha256']}", file=__import__('sys').stderr)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', required=True)
    parser.add_argument('--binary', choices=['sccache', 'cargo-nextest', 'rustup-init'], required=True)
    parser.add_argument('--target', required=True)
    parser.add_argument('--destination', required=True)
    parser.add_argument('--expected-url', default='')
    args = parser.parse_args()
    try:
        install(args.manifest, args.binary, args.target, args.destination, args.expected_url)
    except (ValueError, KeyError, OSError, tarfile.TarError, subprocess.CalledProcessError) as error:
        parser.exit(1, f'verified bootstrap rejected: {type(error).__name__}\n')


if __name__ == '__main__':
    main()
