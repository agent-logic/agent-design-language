#!/usr/bin/env python3
"""PVF: deterministic local docs/manifest audit; small; offline, no build or release approval."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[4]
REPORT = ROOT / 'docs/milestones/v0.92.1/evidence/release/tail-02/cargo-manifest-audit.json'


def command(*args):
    return subprocess.run(args, cwd=ROOT, text=True, capture_output=True, timeout=60, check=True).stdout


def relative(path):
    return Path(path).resolve().relative_to(ROOT).as_posix()


def dependency_paths(value, section=()):
    if not isinstance(value, dict):
        return
    for name, entry in value.items():
        here = section + (name,)
        if isinstance(entry, dict) and 'path' in entry and any(
            part in ('dependencies', 'dev-dependencies', 'build-dependencies', 'patch', 'replace')
            for part in section
        ):
            yield '.'.join(here), name, entry
        yield from dependency_paths(entry, here)


def audit():
    manifests = sorted(p for p in command('git', 'ls-files').splitlines() if Path(p).name == 'Cargo.toml')
    assert manifests, 'empty tracked manifest denominator'
    documents = {p: tomllib.loads((ROOT / p).read_text()) for p in manifests}
    rows, dependencies = [], 0
    for path in manifests:
        document = documents[path]
        metadata = json.loads(command('cargo', 'metadata', '--offline', '--locked', '--no-deps',
                                      '--format-version', '1', '--manifest-path', path))
        packages = {relative(p['manifest_path']): p for p in metadata['packages']}
        package = packages.get(path)
        assert not document.get('package') or package, f'package omitted from Cargo metadata: {path}'
        if package:
            assert package['name'] == document['package']['name'], f'package name mismatch: {path}'
        local = []
        for section, name, entry in dependency_paths(document):
            target = (ROOT / path).parent / entry['path'] / 'Cargo.toml'
            target_path = relative(target)
            assert target_path in documents, f'untracked or missing dependency manifest: {path} {section}'
            target_package = documents[target_path]['package']
            assert target_package['name'] == entry.get('package', name.split(':')[0]), f'local package mismatch: {path} {section}'
            requested_features = entry.get('features', [])
            available_features = set(documents[target_path].get('features', {}))
            # Cargo implicitly creates a feature for an optional dependency unless disabled by dep: usage.
            disabled = {item[4:] for items in documents[target_path].get('features', {}).values()
                        for item in items if item.startswith('dep:')}
            available_features |= {n for n, v in documents[target_path].get('dependencies', {}).items()
                                   if isinstance(v, dict) and v.get('optional') and n not in disabled}
            assert set(requested_features) <= available_features, f'unknown local feature: {path} {section}'
            local.append({'section': section, 'target_manifest': target_path,
                          'package': target_package['name'], 'version_requirement': entry.get('version'),
                          'requested_features': requested_features})
        dependencies += len(local)
        members = sorted(relative(p['manifest_path']) for p in metadata['packages']
                         if p['id'] in metadata['workspace_members'])
        assert all(p in documents for p in members), f'untracked workspace member: {path}'
        rows.append({'manifest': path, 'sha256': hashlib.sha256((ROOT / path).read_bytes()).hexdigest(),
                     'package': package['name'] if package else None,
                     'effective_version': package['version'] if package else None,
                     'declared_version': document.get('package', {}).get('version'),
                     'workspace_root': relative(metadata['workspace_root']),
                     'workspace_member_manifests': members, 'local_dependencies': local,
                     'cargo_metadata': 'passed_offline_locked_no_deps'})
    return {'schema': 'adl.tail02.cargo_manifest_audit.v1', 'manifest_count': len(rows),
            'local_dependency_count': dependencies, 'status': 'pass', 'manifests': rows,
            'proof_scope': 'Tracked TOML parsing, Cargo workspace/package/version metadata, local dependency manifest identity and requested local features.',
            'limits': ['No compilation, dependency fetch or vulnerability audit.',
                       'No full dependency resolution or lockfile reproducibility claim: metadata uses --no-deps.',
                       'No release-version approval or automatic version bump. Different package versions remain explicit release-owner decisions.',
                       'Not final milestone acceptance; issue 517 passing reviewed merge remains required.']}


def main():
    parser = argparse.ArgumentParser()
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument('--write', action='store_true')
    modes.add_argument('--check', action='store_true')
    args = parser.parse_args()
    result = audit()
    if args.write:
        REPORT.write_text(json.dumps(result, indent=2) + '\n')
    else:
        assert json.loads(REPORT.read_text()) == result, 'manifest audit snapshot is stale; review and regenerate'
    print(json.dumps({'status': 'pass', 'manifests': result['manifest_count'],
                      'local_dependencies': result['local_dependency_count'], 'final_acceptance': False}))


if __name__ == '__main__':
    main()
