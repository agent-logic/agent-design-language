#!/usr/bin/env python3
"""PVF local_contract: current public docs/commands, deterministic local CPU/files.
Required for this public journey; no network/provider or product qualification.
--self-test runs adverse fixtures as well as the actual selected docs contract.
"""
import argparse
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
DOCS = ['README.md', 'CODE_OF_CONDUCT.md', 'CONTRIBUTING.md', 'REVIEW.md', 'AGENTS.md',
        'docs/onboarding.md', 'docs/default_workflow.md', 'docs/README.md',
        'docs/COMPATIBILITY.md', 'docs/PUBLIC_BUILD.md', 'demos/README.md']
DOCS += ['docs/specs/uts/' + name for name in
         ['README.md', 'PACKAGE.md', 'ADL_UNIVERSAL_TOOL_SCHEMA.md', 'UTS_V1.1_SCHEMA.md']]
DOCS += ['docs/specs/acc/' + name for name in
         ['README.md', 'ACC_V1.0_SPEC.md', 'ACC_V1.1_SPEC.md']]


def prose(text):
    return re.sub(r'^\s*(```|~~~)[\s\S]*?^\s*\1[^\n]*$', '', text, flags=re.M)


def anchors(text):
    seen = {}; result = set()
    for heading in re.findall(r'^#{1,6}\s+(.+?)\s*#*$', prose(text), flags=re.M):
        slug = re.sub(r'[^\w\- ]', '', heading.lower()).replace(' ', '-')
        count = seen.get(slug, 0); seen[slug] = count + 1
        result.add(slug + (f'-{count}' if count else ''))
    return result


def link_errors(root, relative):
    path = root / relative; errors = []
    for target in re.findall(r'\]\(([^\s)]+)(?:\s+"[^"]*")?\)', prose(path.read_text())):
        if re.match(r'^[a-zA-Z][a-zA-Z0-9+.-]*:', target):
            continue
        target = target.strip('<>'); file, _, anchor = target.partition('#')
        dest = (path.parent / file).resolve() if file else path.resolve()
        if not dest.is_relative_to(root.resolve()):
            errors.append(f'{relative}: outside repository link {target}')
        elif not dest.exists():
            errors.append(f'{relative}: missing target {target}')
        elif anchor and dest.suffix == '.md' and anchor not in anchors(dest.read_text()):
            errors.append(f'{relative}: missing anchor {target}')
    return errors


def check(root):
    errors = []
    for name in DOCS:
        if not (root/name).is_file(): errors.append(f'missing current doc {name}'); continue
        errors.extend(link_errors(root, name))
        if re.search(r'adl/src/(?:uts|acc)\.rs', (root/name).read_text()):
            errors.append(f'{name}: retired current implementation path')
    example = 'adl/examples/v0-3-fork-join-seq-run.adl.yaml'
    if not (root/example).is_file(): errors.append('missing public example input')
    for target, expected in {
        'build': 'cargo build --locked --manifest-path adl/Cargo.toml --bin adl',
        'plan': f'cargo run --locked --manifest-path adl/Cargo.toml --bin adl -- {example} --print-plan',
        'check': 'bash tools/public_adl/run_cli_ci.sh',
        'packages': 'bash tools/public_adl/run_package_checks.sh',
        'docs': 'python3 tools/public_adl/check_public_docs.py --self-test',
    }.items():
        result = subprocess.run(['make', '--no-print-directory', '-n', target], cwd=root,
                                capture_output=True, text=True, timeout=10)
        if result.returncode or result.stdout.strip() != expected:
            errors.append(f'Make target {target} does not delegate the declared public command')
    # Read manifests rather than equating CLI and wire/schema versions.
    import tomllib
    version = tomllib.loads((root/'adl/Cargo.toml').read_text())['package']['version']
    if f'`adl` {version}' not in (root/'docs/COMPATIBILITY.md').read_text():
        errors.append('CLI version table differs from manifest')
    toolchain = tomllib.loads((root/'rust-toolchain.toml').read_text())['toolchain']['channel']
    if f'Rust {toolchain}' not in (root/'docs/COMPATIBILITY.md').read_text():
        errors.append('Rust version table differs from toolchain')
    for name in ['.github/ISSUE_TEMPLATE/bug_report.md', '.github/ISSUE_TEMPLATE/proposal.md', '.github/pull_request_template.md']:
        if not (root/name).is_file(): errors.append(f'missing public intake {name}')
    return errors


class LinkTests(unittest.TestCase):
    def test_missing_file_and_anchor(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d); (root/'guide.md').write_text('# Guide\n[bad](absent.md)\n[bad](target.md#absent)\n')
            (root/'target.md').write_text('# Present\n')
            self.assertEqual(len(link_errors(root, 'guide.md')), 2)
    def test_valid_duplicate_heading_and_code_example(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d); (root/'guide.md').write_text('# Guide\n[ok](target.md#same-1)\n```md\n[example](missing.md)\n```\n')
            (root/'target.md').write_text('# Same\n# Same\n')
            self.assertEqual(link_errors(root, 'guide.md'), [])
    def test_escape_refused(self):
        with tempfile.TemporaryDirectory() as d:
            root = Path(d); (root/'guide.md').write_text('[outside](../outside.md)\n')
            self.assertIn('outside repository', link_errors(root, 'guide.md')[0])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test', action='store_true'); args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner().run(unittest.defaultTestLoader.loadTestsFromTestCase(LinkTests))
        if not result.wasSuccessful(): raise SystemExit(1)
    errors = check(ROOT)
    if errors: raise SystemExit('\n'.join(errors))
    print('PASS public docs contract: links, anchors, command delegation and version sources')


if __name__ == '__main__':
    main()
