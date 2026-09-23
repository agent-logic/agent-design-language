#!/usr/bin/env python3
"""PVF: small deterministic local policy regressions; required #1160 proof.
Real Git old/new objects and actual coverage-impact command, no coverage build.
"""
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('policy', HERE / 'coverage_test_only.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)
SOURCE = '''pub fn production() -> u32 { 1 }
#[cfg(test)]
mod tests {
    #[test]
    fn fixture() { assert_eq!(super::production(), 1); }
}
'''


class Policy(unittest.TestCase):
    def test_test_body_only(self):
        self.assertTrue(policy.test_only(SOURCE, SOURCE.replace('assert_eq!', 'assert_ne!')))
        middle = SOURCE + 'pub fn following() -> u32 { 7 }\n'
        self.assertTrue(policy.test_only(middle, middle.replace('assert_eq!', 'assert_ne!')))
        self.assertFalse(policy.test_only(middle, middle.replace('u32 { 7 }', 'u32 { 8 }')))

    def test_mixed_production_and_unknown_fail_closed(self):
        mutations = [SOURCE.replace('u32 { 1 }', 'u32 { 2 }'),
                     SOURCE.replace('u32 { 1 }', 'u32 { 2 }').replace('assert_eq!', 'assert_ne!'),
                     SOURCE.replace('#[cfg(test)]', '#[cfg(all())]'),
                     SOURCE+'pub fn outside() {}\n', SOURCE[:-2],
                     SOURCE.replace('assert_eq!', '/* assert_eq!'),
                     SOURCE.replace('assert_eq!', '"assert_eq!'), SOURCE,
                     SOURCE.replace('mod tests', 'mod other')]
        for index, new in enumerate(mutations):
            with self.subTest(case=index):
                self.assertFalse(policy.test_only(SOURCE, new))

    def test_literals_and_comments_do_not_grant_fake_modules(self):
        body = SOURCE.replace('assert_eq!(super::production(), 1);',
            '''let s = r##"} #[cfg(test)] mod fake {"##;
        let c = '}'; /* } /* { */ } */ let t = "}\\\"{";''')
        self.assertTrue(policy.test_only(SOURCE, body))
        fake = 'pub const X: &str = r###"\n'+SOURCE+'"###;\n'
        self.assertFalse(policy.test_only(fake, fake.replace('assert_eq!', 'assert_ne!')))
        fake = '/*\n'+SOURCE+'*/\n'
        self.assertFalse(policy.test_only(fake, fake.replace('assert_eq!', 'assert_ne!')))

    def test_real_polis_fixture_only(self):
        # Retained source regression may not exist in a shallow CI checkout;
        # the deterministic fixture tests remain mandatory everywhere.
        root = HERE.parents[1]
        try:
            old = policy.git(root, 'show', '36ec0814683d0c8cc136bc9af27a0163ed506460^:adl-runtime/src/distributed/transport/governed/polis_runtime.rs')
            new = policy.git(root, 'show', '36ec0814683d0c8cc136bc9af27a0163ed506460:adl-runtime/src/distributed/transport/governed/polis_runtime.rs')
        except subprocess.CalledProcessError:
            self.skipTest('historical #1151 Git objects unavailable; portable fixtures still run')
        self.assertTrue(policy.test_only(old, new))

    def test_actual_gate_with_git_objects(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            tools = root / 'adl/tools'
            tools.mkdir(parents=True)
            for name in ('check_coverage_impact.sh', 'coverage_test_only.py'):
                shutil.copyfile(HERE / name, tools / name)
            file = root / 'adl-runtime/src/distributed/transport/governed/polis_runtime.rs'
            file.parent.mkdir(parents=True)
            file.write_text(SOURCE)
            def git(*args):
                return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.DEVNULL).decode().strip()
            git('init', '-q'); git('config', 'user.email', 'fixture@example.invalid'); git('config', 'user.name', 'Fixture')
            git('add', '.'); git('commit', '-qm', 'base')
            base = git('rev-parse', 'HEAD')
            summary = root / 'summary.json'
            summary.write_text(json.dumps({'data':[{'files':[{'filename':'adl-runtime/src/distributed/transport/governed/polis_runtime.rs','summary':{'lines':{'covered':3170,'count':4804}}}]}]}))
            def gate(base_ref=base):
                return subprocess.run(['bash', str(tools/'check_coverage_impact.sh'), '--base',base_ref,'--head','HEAD','--summary',str(summary)],cwd=root,capture_output=True,text=True)
            for label, source, success in [('test-only', SOURCE.replace('assert_eq!', 'assert_ne!'), True),
                   ('mixed', SOURCE.replace('assert_eq!', 'assert_ne!').replace('u32 { 1 }', 'u32 { 2 }'), False),
                   ('production', SOURCE.replace('u32 { 1 }', 'u32 { 2 }'), False),
                   ('malformed', SOURCE.replace('assert_eq!', '/* assert_eq!'), False)]:
                file.write_text(source);git('add', str(file));git('commit','-qm',label)
                result = gate()
                with self.subTest(label=label):
                    self.assertEqual(result.returncode==0,success,result.stdout+result.stderr)
            # Missing Git identity cannot classify an explicitly supplied path.
            rows = root/'rows';rows.write_text('M\tadl-runtime/src/distributed/transport/governed/polis_runtime.rs\n')
            result=subprocess.run(['bash',str(tools/'check_coverage_impact.sh'),'--base','unknown','--changed-files',str(rows),'--summary',str(summary)],cwd=root,capture_output=True,text=True)
            self.assertNotEqual(result.returncode,0)

if __name__ == '__main__':
    unittest.main()
