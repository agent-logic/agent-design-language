#!/usr/bin/env python3
"""PVF local_contract: real Rust parser subprocesses, synthetic argv, no providers.

Small CPU/disk; deterministic output/status checks; required for CLI-01 only.
This does not claim full legacy CLI or installed product adapter qualification.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
USAGE = 'legacy usage sentinel'


class WorkflowArguments(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.scratch = tempfile.TemporaryDirectory(prefix='adl-workflow-args-')
        cls.addClassCleanup(cls.scratch.cleanup)
        root = Path(cls.scratch.name)
        module = ROOT / 'adl/src/cli/workflow_args.rs'
        harness = root / 'parser.rs'
        harness.write_text(
            '#[path = ' + json.dumps(str(module)) + '] mod workflow_args;\n'
            'fn main() { let args: Vec<String> = std::env::args().skip(1).collect(); '
            'if let Some(value) = workflow_args::parse(&args, "' + USAGE + '") '
            '{ println!("{:?}", value); } }\n'
        )
        cls.binary = root / 'parser'
        subprocess.run([os.environ.get('RUSTC', 'rustc'), '--edition=2021',
                        str(harness), '-o', str(cls.binary)], check=True,
                       capture_output=True, text=True)

    def invoke(self, args):
        return subprocess.run([str(self.binary), *args], capture_output=True,
                              text=True, timeout=5)

    def assert_options(self, args, **changed):
        values = dict(adl_path=args[0], print_plan=False, print_prompts=False,
                      do_trace=False, do_run=False, out_dir='out', quiet=False,
                      do_open=False, allow_unsigned=False,
                      allow_embedded_signature_key=False, signature_key_path=None,
                      resume_path=None, steer_path=None, overlay_path=None)
        values.update(changed)
        def debug(name, value):
            if value is None:
                return 'None'
            if isinstance(value, bool):
                return str(value).lower()
            text = json.dumps(value, ensure_ascii=False)
            return 'Some(' + text + ')' if name in (
                'signature_key_path', 'resume_path', 'steer_path', 'overlay_path'
            ) else text
        expected = 'WorkflowArgs { ' + ', '.join(
            key + ': ' + debug(key, value) for key, value in values.items()
        ) + ' }\n'
        result = self.invoke(args)
        self.assertEqual((result.returncode, result.stdout, result.stderr), (0, expected, ''))

    def test_defaults_and_literal_paths(self):
        self.assert_options(['missing.adl.yaml'])
        self.assert_options(['space ; $(not-a-command).yaml', '--out', 'out ; literal'],
                            out_dir='out ; literal')

    def test_boolean_switches_and_aliases(self):
        for flag, field in [('--print-plan', 'print_plan'), ('--print-prompts', 'print_prompts'),
                            ('--print-prompt', 'print_prompts'), ('--trace', 'do_trace'),
                            ('--run', 'do_run'), ('--quiet', 'quiet'), ('--no-step-output', 'quiet'),
                            ('--open', 'do_open'), ('--open-artifacts', 'do_open'),
                            ('--allow-unsigned', 'allow_unsigned'),
                            ('--allow-embedded-signature-key', 'allow_embedded_signature_key')]:
            with self.subTest(flag=flag):
                self.assert_options(['plan.yaml', flag], **{field: True})

    def test_values_repetition_and_option_like_operands(self):
        self.assert_options(['plan.yaml', '--resume', 'r.json', '--steer', 's.json',
                             '--overlay', 'o.json', '--signature-key', 'key.pem'],
                            resume_path='r.json', steer_path='s.json', overlay_path='o.json',
                            signature_key_path='key.pem')
        self.assert_options(['plan.yaml', '--out', 'first', '--out', 'last'], out_dir='last')
        # Existing contract consumes the following token even when it starts with --.
        self.assert_options(['plan.yaml', '--out', '--run'], out_dir='--run')

    def test_help_short_circuits_and_is_stdout_only(self):
        for flag in ['--help', '-h']:
            with self.subTest(flag=flag):
                result = self.invoke(['plan.yaml', flag, '--unknown'])
                self.assertEqual((result.returncode, result.stdout, result.stderr),
                                 (0, USAGE + '\n', ''))

    def test_missing_and_unknown_arguments_keep_exit_two(self):
        cases = [([], 'missing ADL yaml path\nTry: adl examples/v0-3-concurrency-fork-join.adl.yaml --print-plan\n'),
                 (['plan.yaml', '--unknown'], "Unknown arg: --unknown\nRun 'adl --help' for usage.\n")]
        for flag, message in [('--resume', 'a run.json path'),
                              ('--steer', 'a path to steering patch JSON'),
                              ('--overlay', 'a path to overlay.json'),
                              ('--out', 'a directory path'),
                              ('--signature-key', 'a public key path')]:
            cases.append((['plan.yaml', flag], flag + ' requires ' + message + '\n'))
        for args, message in cases:
            with self.subTest(args=args):
                result = self.invoke(args)
                self.assertEqual((result.returncode, result.stdout, result.stderr),
                                 (2, '', message + USAGE + '\n'))


if __name__ == '__main__':
    unittest.main()
