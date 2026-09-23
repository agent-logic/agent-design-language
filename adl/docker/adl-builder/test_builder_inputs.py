#!/usr/bin/env python3
"""PVF: deterministic local bootstrap-contract proof, tiny fixtures, required #1160 gate."""
import hashlib
import json
import re
import subprocess
import tempfile
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parent


def check_contract(text):
    if not re.match(r'FROM ubuntu:24\.04@sha256:[a-f0-9]{64}\n', text):
        raise ValueError('mutable base')
    args = dict(re.findall(r'^ARG ([A-Z0-9_]+)=(.*)$', text, re.M))
    if not re.fullmatch(r'\d+\.\d+\.\d+', args['RUST_TOOLCHAIN']):
        raise ValueError('mutable toolchain')
    for tool in ('RUSTUP', 'AWSCLI', 'SCCACHE', 'CARGO_NEXTEST', 'CARGO_LLVM_COV'):
        for arch in ('X86_64', 'AARCH64'):
            if not re.fullmatch('[0-9a-f]{64}', args.get(f'{tool}_{arch}_SHA256', '')):
                raise ValueError('missing architecture digest')
    if 'sh.rustup.rs' in text or 'releases/latest' in text or '${awscli_arch}.zip' in text:
        raise ValueError('mutable executable source')
    # Every download RUN must verify bytes before its first extract or execution.
    blocks = re.split(r'\n(?=RUN |COPY |WORKDIR |ENTRYPOINT )', text)
    seen = 0
    for block in blocks:
        if 'curl -fsSL' not in block:
            continue
        seen += 1
        after = block[block.index('curl -fsSL'):]
        verification = re.search(r'(adl-verify-sha256\.sh|sha256sum -c -)', after)
        execution = re.search(r'(&& (?:tar |unzip |chmod |sh |/tmp/))', after)
        if not verification or not execution or verification.start() > execution.start():
            raise ValueError('unchecked download')
    if seen != 6:
        raise ValueError('incomplete download denominator')
    record = text.index('> /usr/local/share/adl-builder-toolchain.txt')
    if record < text.index('install -m 0755 "$llvm_cov_bin"') or '"cargo-llvm-cov=$(cargo llvm-cov --version)"' not in text:
        raise ValueError('incomplete installed identity record')


class Inputs(unittest.TestCase):
    def test_actual_verifier(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / 'artifact'
            archive.write_bytes(b'approved archive bytes')
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            for expected, succeeds in [(digest, True), ('0' * 64, False), ('', False), ('latest', False)]:
                with self.subTest(expected=expected):
                    result = subprocess.run(['bash', str(ROOT / 'verify_sha256.sh'), expected, str(archive)], capture_output=True)
                    self.assertEqual(result.returncode == 0, succeeds)
            archive.write_bytes(b'tampered')
            self.assertNotEqual(subprocess.run(['bash', str(ROOT / 'verify_sha256.sh'), digest, str(archive)], capture_output=True).returncode, 0)

    def test_docker_contract(self):
        check_contract((ROOT / 'Dockerfile').read_text())

    def test_negative_contracts(self):
        source = (ROOT / 'Dockerfile').read_text()
        mutations = [source.replace('ARG RUST_TOOLCHAIN=1.92.0', 'ARG RUST_TOOLCHAIN=stable'),
                     source.replace('FROM ubuntu:24.04@sha256:', 'FROM ubuntu:24.04 #'),
                     source.replace('bash /usr/local/share/adl-verify-sha256.sh "$awscli_sha256" /tmp/awscliv2.zip', 'true'),
                     re.sub(r'ARG SCCACHE_AARCH64_SHA256=.*', 'ARG SCCACHE_AARCH64_SHA256=', source),
                     source.replace('"cargo-llvm-cov=$(cargo llvm-cov --version)"', '"omitted"')]
        for index, mutated in enumerate(mutations):
            with self.subTest(case=index), self.assertRaises(ValueError):
                check_contract(mutated)

    def test_provenance_matches_args(self):
        pins = json.loads((ROOT / 'INPUT_PROVENANCE.json').read_text())
        self.assertEqual(pins['pins'], dict(re.findall(r'^ARG ([A-Z0-9_]+)=(.*)$', (ROOT / 'Dockerfile').read_text(), re.M)))

if __name__ == '__main__':
    unittest.main()
