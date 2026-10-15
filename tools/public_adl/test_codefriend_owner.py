#!/usr/bin/env python3
"""PVF local_contract: owner receipt selection; synthetic layouts are not qualification."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[2]
ARCHIVE='a'*64
CANDIDATE='b'*40


class CodeFriendSelection(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.build=tempfile.TemporaryDirectory(prefix='adl-codefriend-probe-')
        cls.addClassCleanup(cls.build.cleanup)
        root=Path(cls.build.name);(root/'src').mkdir()
        (root/'Cargo.toml').write_text('[package]\nname="adl-codefriend-probe"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nanyhow="1"\nserde_json="1"\n[workspace]\n')
        (root/'src/main.rs').write_text('#[path='+json.dumps(str(ROOT/'adl/src/cli/codefriend_owner.rs'))+'] mod owner;\nfn main(){let a:Vec<String>=std::env::args().skip(1).collect();let result=owner::SelectedCodeFriend::from_installation(std::path::Path::new(&a[0]),&a[1],&a[2]);match result{Ok(s)=>{if a.get(3).map(String::as_str)==Some("inspect"){println!("{}",s.command(&[]).get_program().to_string_lossy());}else if let Err(e)=s.exec(&a[3..]){eprintln!("{e:#}");std::process::exit(2);}},Err(e)=>{eprintln!("{e:#}");std::process::exit(2);}}}\n')
        subprocess.run(['cargo','build','--offline','--manifest-path',str(root/'Cargo.toml')],check=True,capture_output=True,
                       env={**os.environ,'CARGO_TARGET_DIR':str(root/'target')})
        cls.binary=root/'target/debug/adl-codefriend-probe'
        prefix=os.environ.get('CODEFRIEND_QUALIFIED_SELECTION_FIXTURE')
        if prefix:
            receipt=json.loads((Path(prefix)/'INSTALLATION.json').read_text())
            result=subprocess.run([str(cls.binary),prefix,receipt['archive_sha256'],receipt['candidate'],'inspect'],capture_output=True,text=True,timeout=5)
            if result.returncode!=0:raise AssertionError(result.stderr)
            print('Existing owner receipt selected without executing its foreign-platform binary')

    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(prefix='adl-codefriend-layout-');self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name).resolve();(self.root/'bin').mkdir()
        self.executable=self.root/'bin/codefriend';self.executable.write_text('#!/bin/sh\nprintf "%s\\n" "$@"\nexit 17\n');self.executable.chmod(0o755)
        self.receipt={'schema':'codefriend.product_installation.v1','authentication':'external_owner_pins_not_self_authorization','archive_sha256':ARCHIVE,'candidate':CANDIDATE,'run_id':'1','run_attempt':'1','workflow_ref':'agent-logic/codefriend/.github/workflows/product-ci.yaml@refs/heads/main','workflow_sha':'c'*40,'job_name':'installed-product'}
        self.manifest={'schema':'codefriend.product_distribution.v1','source_revision':CANDIDATE,'source_clean':True,'builder_context':{'GITHUB_REPOSITORY':'agent-logic/codefriend','GITHUB_RUN_ID':'1','GITHUB_RUN_ATTEMPT':'1','GITHUB_WORKFLOW_REF':self.receipt['workflow_ref'],'GITHUB_WORKFLOW_SHA':'c'*40,'GITHUB_JOB':'installed-product'},'resources':[{'path':'bin/codefriend','mode':0o755,'sha256':'d'*64}]}
        self.write()

    def write(self):
        (self.root/'INSTALLATION.json').write_text(json.dumps(self.receipt));(self.root/'manifest.json').write_text(json.dumps(self.manifest))

    def invoke(self,*args,archive=ARCHIVE,candidate=CANDIDATE):
        return subprocess.run([str(self.binary),str(self.root),archive,candidate,*args],capture_output=True,text=True,timeout=5)

    def refused(self,result):
        self.assertEqual(result.returncode,2,result.stderr);self.assertEqual(result.stdout,'')

    def test_literal_arguments_and_status(self):
        result=self.invoke('ingest','space ; $(literal)')
        self.assertEqual((result.returncode,result.stdout),(17,'ingest\nspace ; $(literal)\n'))

    def test_pin_and_producer_refusals(self):
        self.refused(self.invoke(archive='e'*64));self.refused(self.invoke(candidate='f'*40))
        self.manifest['builder_context']['GITHUB_REPOSITORY']='foreign/repo';self.write();self.refused(self.invoke())

    def test_mismatched_receipt_context(self):
        self.receipt['run_id']='2';self.write();self.refused(self.invoke())

    def test_duplicate_or_missing_executable(self):
        self.manifest['resources']*=2;self.write();self.refused(self.invoke())
        self.manifest['resources']=[];self.write();self.refused(self.invoke())

    def test_symlink_and_missing_receipt(self):
        target=self.root/'actual';self.executable.rename(target);self.executable.symlink_to(target);self.refused(self.invoke())
        self.executable.unlink();target.rename(self.executable);(self.root/'INSTALLATION.json').unlink();self.refused(self.invoke())


if __name__=='__main__':unittest.main()
