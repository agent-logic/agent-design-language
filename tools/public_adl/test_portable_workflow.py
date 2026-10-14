#!/usr/bin/env python3
"""PVF local_contract: compile real public workflow code without private ADL crates."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT=Path(__file__).resolve().parents[2]


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',type=Path,required=True)
    baseline=parser.parse_args().baseline.resolve(strict=True)
    environment={k:v for k,v in os.environ.items() if not k.startswith('ADL_')}
    environment['ADL_OBSERVABILITY']='0'
    with tempfile.TemporaryDirectory(prefix='adl-portable-workflow-') as directory:
        root=Path(directory);(root/'src').mkdir()
        (root/'Cargo.toml').write_text('[package]\nname="adl-portable-workflow-probe"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nanyhow="1"\nserde_json="1"\nadl-legacy-contracts={path='+json.dumps(str(ROOT/'adl-legacy-contracts'))+'}\n[workspace]\n')
        source=''
        for module,file in [('runtime_owner','runtime_owner.rs'),('public_workflow','public_workflow.rs'),('usage','usage.rs')]:
            source+='#[path='+json.dumps(str(ROOT/'adl/src/cli'/file))+'] mod '+module+';\n'
        source+='fn usage()-> &\'static str {usage::usage()}\nfn resume_usage()-> &\'static str {usage::resume_usage()}\nfn main(){let a:Vec<String>=std::env::args().skip(1).collect();let r=public_workflow::run(&a);if let Err(e)=r{eprintln!("Error: {e}");std::process::exit(1);}}\n'
        (root/'src/main.rs').write_text(source)
        env={**environment,'CARGO_TARGET_DIR':str(root/'target')}
        subprocess.run(['cargo','build','--offline','--manifest-path',str(root/'Cargo.toml')],env=env,check=True,capture_output=True)
        host=next(line.split(': ',1)[1] for line in subprocess.check_output(['rustc','-vV'],text=True).splitlines() if line.startswith('host: '))
        metadata=json.loads(subprocess.check_output(['cargo','metadata','--locked','--offline','--filter-platform',host,'--format-version','1','--manifest-path',str(root/'Cargo.toml')],env=env))
        forbidden={'adl','adl-runtime','adl-runtime-kernel','adl-provider-core','codefriend','agent-product-support'}
        assert not forbidden.intersection(p['name'] for p in metadata['packages'])
        probe=root/'target/debug/adl-portable-workflow-probe'
        fixture=ROOT/'adl/examples/v0-6-hitl-no-pause.adl.yaml'
        cases=0
        for options in [['--print-plan'],['--print-prompts'],['--print-plan','--print-prompts']]:
            args=[str(fixture),*options]
            old=subprocess.run([str(baseline),*args],env=env,capture_output=True,timeout=10)
            new=subprocess.run([str(probe),*args],env=env,capture_output=True,timeout=10)
            assert old.returncode==new.returncode==0
            assert (old.stdout,old.stderr)==(new.stdout,new.stderr),options
            cases+=1
        result=subprocess.run([str(probe),str(fixture),'--run'],env=env,capture_output=True,timeout=10)
        assert result.returncode==1 and b'ADL_RUNTIME_INSTALL_PREFIX' in result.stderr
    print(json.dumps({'status':'passed','presentation_cases':cases,'missing_owner_case':1,
                      'private_crates_in_host_dependency_graph':0,'limits':'Portable planning and routing boundary only; full public CLI manifest partition remains required'}))


if __name__=='__main__':main()
