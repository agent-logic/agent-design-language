#!/usr/bin/env python3
"""Offline deterministic product regressions; no provider, network or lifecycle."""
import argparse, hashlib, json, pathlib, subprocess, tempfile, unittest
import package, verify_install
class Installed(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.root=pathlib.Path(self.temp.name).resolve();self.archive=self.root/'package.tar';package.build(BINARY,self.archive);self.installed=self.root/'installed';verify_install.install(self.archive,hashlib.sha256(self.archive.read_bytes()).hexdigest(),self.installed);self.output=self.root/'output'
    def tearDown(self):self.temp.cleanup()
    def run_demo(self):
        return subprocess.run([str(self.installed/'bin/transpiler_demo'),'--input-root',str(self.installed),'--output-root',str(self.output)],cwd=self.root,capture_output=True,text=True)
    def test_original_mapping(self):
        r=self.run_demo();self.assertEqual(r.returncode,0,r.stderr);d=json.loads((self.output/package.DATA[0].replace('workflow/rust_transpiler_demo.yaml','output/transpiler_verification.v0.8.json')).read_text());self.assertEqual(d['mapping']['order_check'],'PASS');self.assertEqual(len(d['mapping']['pairs']),3);self.assertTrue(all(p['status']=='PASS' for p in d['mapping']['pairs']));self.assertEqual(d['adaptive_execution']['attempts_executed'],0)
    def test_missing_input(self):
        (self.installed/package.DATA[0]).unlink();self.assertNotEqual(self.run_demo().returncode,0);self.assertFalse(self.output.exists())
    def test_mismatched_mapping(self):
        p=self.installed/package.DATA[0];p.write_text(p.read_text().replace('step_prepare_input','step_wrong'));r=self.run_demo();self.assertNotEqual(r.returncode,0);self.assertIn('ordering do not match',r.stderr)
    def test_existing_output_refused(self):
        self.output.mkdir();(self.output/'sentinel').write_text('preserved');self.assertNotEqual(self.run_demo().returncode,0);self.assertEqual((self.output/'sentinel').read_text(),'preserved')
    def test_symlink_input_refused(self):
        p=self.installed/package.DATA[0];other=self.root/'outside.yaml';p.rename(other);p.symlink_to(other);self.assertNotEqual(self.run_demo().returncode,0);self.assertFalse(self.output.exists())
    def test_tampered_archive_refused(self):
        digest=hashlib.sha256(self.archive.read_bytes()).hexdigest();self.archive.write_bytes(self.archive.read_bytes()+b'changed');dest=self.root/'bad';self.assertRaises(ValueError,verify_install.install,self.archive,digest,dest);self.assertFalse(dest.exists())
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--binary',type=pathlib.Path,required=True);args,remaining=p.parse_known_args();BINARY=args.binary.resolve();unittest.main(argv=['test_installed.py']+remaining,verbosity=2)
