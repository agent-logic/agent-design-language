"""PVF local_contract, deterministic temp Git fixtures, small CPU/disk; #836 required."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('measure', Path(__file__).with_name('measure.py'))
m = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(m)

class RecursiveMeasurement(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.run_git('init', '-q')
        self.run_git('config', 'user.email', 'fixture@example.invalid')
        self.run_git('config', 'user.name', 'Fixture')
        self.write('adl/src/resilience.rs', 'alpha\nbeta\n\ngamma')
        self.write('adl/src/resilience/deep/old.rs', 'rename me\n')
        self.write('adl/tests/nested/stable.rs', 'unchanged\n')
        self.write('adl/tests/ignored.txt', 'not Rust\n')
        self.before = self.commit()
        self.write('adl/src/resilience.rs', 'alpha\n')
        self.write('adl/src/resilience/deep/new.rs', 'beta\ngamma\nnew line\n')
        old = self.root/'adl/src/resilience/deep/old.rs'
        old.rename(old.with_name('renamed.rs'))
        self.after = self.commit()

    def run_git(self, *args):
        return subprocess.check_output(['git','-C',str(self.root),*args], stderr=subprocess.DEVNULL).decode().strip()

    def write(self, path, body):
        p = self.root/path
        p.parent.mkdir(parents=True,exist_ok=True)
        p.write_text(body)

    def commit(self):
        self.run_git('add','.')
        self.run_git('commit','-qm','fixture')
        return self.run_git('rev-parse','HEAD')

    def test_recursive_accounting_and_relocations(self):
        r=m.measure(self.root,self.before,self.after)
        self.assertEqual(r['totals']['baseline'],6)
        self.assertEqual(r['totals']['candidate'],6)
        self.assertEqual(r['totals']['added'],4)
        self.assertEqual(r['totals']['deleted'],4)
        self.assertEqual(r['totals']['unchanged'],2)
        self.assertEqual(r['totals']['cross_path_identical_nonblank_line_pairs'],3)
        self.assertEqual(r['rename_evidence'],[{'status':'R100','from':'adl/src/resilience/deep/old.rs','to':'adl/src/resilience/deep/renamed.rs'}])
        self.assertIn('adl/tests/nested/stable.rs',[f['path'] for f in r['files']])
        self.assertNotIn('adl/tests/ignored.txt',[f['path'] for f in r['files']])
        self.assertEqual(r,m.measure(self.root,self.before,self.after))

    def test_reject_revision_free_claim(self):
        for bad in ['HEAD','main','',self.before[:12],'f'*40]:
            with self.subTest(bad=bad), self.assertRaises((ValueError,subprocess.CalledProcessError)):
                m.measure(self.root,bad,self.after)

    def test_report_guardrails(self):
        r=m.measure(self.root,self.before,self.after)
        report=self.root/'measurement.json'
        mutations=[lambda d:d.update(recursive=False), lambda d:d.update(scope=['adl/src/resilience.rs']),
                   lambda d:d['files'].pop(), lambda d:d['totals'].update(candidate=1),
                   lambda d:d.update(candidate='HEAD'),lambda d:d.update(behavioral_pass_claim=True)]
        for mutate in mutations:
            d=copy.deepcopy(r);mutate(d)
            report.write_text(json.dumps(d));report.with_suffix('.md').write_text(m.markdown(r))
            result=subprocess.run(['python3',str(Path(m.__file__)),'--repo',str(self.root),'--check',str(report)],capture_output=True)
            self.assertNotEqual(result.returncode,0,result.stdout)
        report.write_text(json.dumps(r));report.with_suffix('.md').write_text('Unsupported top-level code reduction claim')
        result=subprocess.run(['python3',str(Path(m.__file__)),'--repo',str(self.root),'--check',str(report)],capture_output=True)
        self.assertNotEqual(result.returncode,0)

if __name__=='__main__':
    unittest.main()
