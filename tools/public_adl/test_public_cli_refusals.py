#!/usr/bin/env python3
"""PVF local_contract: real CLI diagnostics and owner refusal boundaries.
Synthetic layouts only; bounded subprocesses; no provider/network or installation.
"""
import argparse
import json
import os
from pathlib import Path
import platform
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    binary = parser.parse_args().binary.resolve(strict=True)
    environment = {k: v for k, v in os.environ.items() if not k.startswith('ADL_')}
    environment['ADL_OBSERVABILITY'] = '0'
    count = 0
    with tempfile.TemporaryDirectory(prefix='adl-public-refusals-') as directory:
        root = Path(directory).resolve()
        def refused(args, message, code=2, extra=None):
            nonlocal count
            r = subprocess.run([str(binary), *args], cwd=root,
                               env={**environment, **(extra or {})},
                               capture_output=True, text=True, timeout=10)
            assert r.returncode == code and not r.stdout and message in r.stderr, r
            count += 1
        for flag in ['--resume', '--steer', '--overlay', '--out', '--signature-key']:
            refused(['missing.yaml', flag], flag + ' requires')
        refused(['missing.yaml', '--unknown'], 'Unknown arg: --unknown')
        for flag in ['--adl', '--steer']:
            refused(['resume', 'run-id', flag], 'resume ' + flag + ' requires')
        refused(['resume', 'run-id', 'unexpected'], 'resume requires <run_id> plus --adl')
        refused(['resume', 'run-id', '--adl', 'literal', '--unknown'], 'unknown arg: --unknown')
        refused(['resume', 'run-id', '--adl', 'literal path', '--steer', 'patch'], 'ADL_RUNTIME_INSTALL_PREFIX', 1)
        refused(['missing.yaml', '--print-plan', '--print-prompt', '--trace', '--run',
                 '--resume', 'run.json', '--steer', 'patch', '--overlay', 'overlay',
                 '--out', 'literal out', '--no-step-output', '--open-artifacts',
                 '--allow-unsigned', '--allow-embedded-signature-key', '--signature-key', 'key'],
                'ADL_RUNTIME_INSTALL_PREFIX', 1)
        prefix = root / 'installed'; identity = 'a' * 64
        package = prefix / 'versions' / identity
        (package / 'bin').mkdir(parents=True)
        executable = package / 'bin/adl-runtime'
        # A valid selected file whose invocation returns an OS error, so the
        # facade reports the real boundary failure without replacing itself.
        executable.write_text('#!' + str(root / 'absent-interpreter') + '\n'); executable.chmod(0o755)
        (package / 'archive.sha256').write_text(identity + '\n')
        (package / 'archive.tar').write_bytes(b'synthetic receipt')
        target = {'arm64': 'aarch64'}.get(platform.machine(), platform.machine())
        target += {'Darwin': '-apple-darwin', 'Linux': '-unknown-linux-gnu'}[platform.system()]
        manifest = {'schema': 'runtime.local-package.v1', 'target': target,
                    'files': {'bin/adl-runtime': {'mode': 0o755, 'sha256': 'b' * 64}}}
        manifest_path = package / 'manifest.json'
        def write_manifest():
            manifest_path.write_text(json.dumps(manifest))
        write_manifest()
        (prefix / 'current').symlink_to('versions/' + identity)
        selected = {'ADL_RUNTIME_INSTALL_PREFIX': str(prefix), 'ADL_RUNTIME_ARCHIVE_SHA256': identity}
        def selection_error(message, override=None):
            refused(['csm', 'status'], message, 1, {**selected, **(override or {})})
        selection_error('selected Runtime invocation failed')
        selection_error('exact lowercase archive', {'ADL_RUNTIME_ARCHIVE_SHA256': 'A' * 64})
        selection_error('absolute and unaliased', {'ADL_RUNTIME_INSTALL_PREFIX': 'installed'})
        for key, value in [('schema', 'foreign-schema'), ('target', 'foreign-target')]:
            saved = manifest[key]; manifest[key] = value; write_manifest()
            selection_error('schema or host target mismatch')
            manifest[key] = saved
        member = manifest['files']['bin/adl-runtime']
        for key, value in [('mode', 0o644), ('sha256', 'bad')]:
            saved = member[key]; member[key] = value; write_manifest()
            selection_error('absent from the qualified manifest'); member[key] = saved
        write_manifest()
        (package / 'archive.sha256').write_text('c' * 64)
        selection_error('archive identity mismatch')
        (package / 'archive.sha256').write_text(identity)
        executable.chmod(0o644); selection_error('executable mode mismatch'); executable.chmod(0o755)
        manifest_path.write_bytes(b'\xff'); selection_error('metadata is not UTF-8')
        with manifest_path.open('wb') as f:
            f.truncate(8 * 1024 * 1024 + 1)
        selection_error('metadata is too large'); write_manifest()
        archive = package / 'archive.tar'; archive.unlink(); archive.mkdir()
        selection_error('not a regular file'); archive.rmdir(); archive.write_bytes(b'synthetic')
        current = prefix / 'current'; current.unlink(); current.symlink_to('versions/' + 'c' * 64)
        selection_error('selection does not match')
    print(json.dumps({'status': 'passed', 'cases': count,
                      'scope': 'public diagnostics and synthetic owner refusals; no qualification claim'}))


if __name__ == '__main__':
    main()
