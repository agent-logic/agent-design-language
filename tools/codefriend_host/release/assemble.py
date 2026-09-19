#!/usr/bin/env python3
"""Assemble an offline review packet; never execute inputs or claim attestation."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args], stderr=subprocess.PIPE).decode().strip()


def candidate(repo, revision):
    if not re.fullmatch(r'[0-9a-f]{40}', revision):
        raise ValueError('exact_revision_required')
    if git(repo, 'rev-parse', 'HEAD') != revision:
        raise ValueError('candidate_head_mismatch')
    if git(repo, 'status', '--porcelain', '--untracked-files=all'):
        raise ValueError('clean_candidate_required')


def binary(source, destination):
    if source.is_symlink() or not source.is_file() or not os.access(source, os.X_OK):
        raise ValueError('regular_executable_required')
    with source.open('rb') as stream:
        header = stream.read(64)
    if len(header) < 64 or header[:6] != b'\x7fELF\x02\x01' or struct.unpack('<H', header[18:20])[0] != 62:
        raise ValueError('linux_x86_64_elf_required')
    shutil.copyfile(source, destination)
    destination.chmod(0o755)


def assemble(args):
    adl, website = args.adl.resolve(), args.website.resolve()
    candidate(adl, args.adl_revision)
    candidate(website, args.website_revision)
    output = args.output.absolute()
    # Create-only output: failure leaves its partial packet for inspection.
    output.mkdir(mode=0o700)
    (output / 'bin').mkdir()
    binary(args.gateway, output / 'bin' / 'codefriend-server')
    binary(args.verifier, output / 'bin' / 'codefriend-agent')
    subprocess.run(['git', 'clone', '--no-local', '--no-hardlinks', '--no-checkout', '--', str(website), str(output / 'website')], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    git(output / 'website', 'checkout', '--detach', args.website_revision)
    git(output / 'website', 'remote', 'remove', 'origin')
    # No ignored/untracked state, secrets, dependency directories or credentials
    # from source directories are copied. Only committed host files are selected.
    for name in git(adl, 'ls-tree', '-r', '--name-only', args.adl_revision, '--', 'tools/codefriend_host').splitlines():
        relative = Path(name).relative_to('tools/codefriend_host')
        if relative.parts[0] == 'terraform':
            continue
        target = output / 'host' / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        data = subprocess.check_output(['git', '-C', str(adl), 'show', f'{args.adl_revision}:{name}'])
        target.write_bytes(data)
        target.chmod(0o755 if name.endswith('.py') else 0o644)
    if not (output / 'host' / 'idle_stop.py').is_file():
        raise ValueError('committed_host_controller_required')
    candidate(adl, args.adl_revision)
    candidate(website, args.website_revision)
    candidate(output / 'website', args.website_revision)
    files = {}
    for path in sorted(output.rglob('*')):
        if path.is_symlink():
            raise ValueError('release_symlinks_forbidden')
        if not path.is_file() and not path.is_dir():
            raise ValueError('release_nonregular_object_forbidden')
        if path.is_file():
            files[str(path.relative_to(output))] = hashlib.sha256(path.read_bytes()).hexdigest()
    manifest = dict(schema='codefriend.release_preparation.v1', adl_revision=args.adl_revision,
                    website_revision=args.website_revision, platform='linux-x86_64', files=files,
                    report_verifier=dict(path='bin/codefriend-agent', argv=['verify-report', '--report-file', '<private-report-file>']),
                    binary_authentication='pending_installer_verification', dependencies='not_installed',
                    ready_for_activation=False)
    (output / 'manifest.json').write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n')
    print(json.dumps(dict(status='assembled_not_authenticated', files=len(files), ready_for_activation=False)))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for option in ('adl', 'website', 'gateway', 'verifier', 'output'):
        parser.add_argument('--' + option, required=True, type=Path)
    for option in ('adl-revision', 'website-revision'):
        parser.add_argument('--' + option, required=True)
    try:
        assemble(parser.parse_args())
    except (ValueError, OSError, subprocess.SubprocessError):
        parser.exit(1, 'release_assembly_failed; inspect inputs and any create-only partial output\n')


if __name__ == '__main__':
    main()
