# PVF: bounded offline audit integration probes; local CPU/disk; required audit evidence, not an extraction or release gate.
import os,json,subprocess,pathlib,tarfile,io,time,concurrent.futures
import argparse, hashlib, re
parser=argparse.ArgumentParser(description='Offline RD-01 source-subset audit; requires a fresh output directory.')
parser.add_argument('--output',default='.adl/runs/977-replay')
options=parser.parse_args()
root=pathlib.Path.cwd();base='f69019c24a9b61511e912c93f95442f96fa66d92'
run=(root/options.output).resolve()
if not run.is_relative_to((root/'.adl/runs').resolve()) or run==(root/'.adl/runs').resolve():
 raise SystemExit('output must be a child of .adl/runs')
if run.exists() and any(run.iterdir()): raise SystemExit('output must be fresh; prior results are preserved')
run.mkdir(parents=True,exist_ok=True)
# Archive immutable source only. No sibling source or repository credentials copied.
components=['csdlc-v3','adl','adl-runtime','adl-runtime-kernel','adl-provider-core','adl-uts','csdlc-v2','adl-v2','infra']
for c in components:
 d=run/c;d.mkdir(exist_ok=True)
 raw=subprocess.check_output(['git','archive',base,c])
 with tarfile.open(fileobj=io.BytesIO(raw)) as t:t.extractall(d,filter='data')
home=run/'empty-home';home.mkdir(exist_ok=True)
env={'PATH':os.environ['PATH'],'HOME':str(home),'RUSTUP_HOME':str(pathlib.Path.home()/'.rustup'),'CARGO_HOME':str(pathlib.Path.home()/'.cargo'),'CARGO_TARGET_DIR':str(run/'build-cache'),'TMPDIR':str(run),'LANG':'en_US.UTF-8','CARGO_NET_OFFLINE':'true'}
results=[]
def probe(label,c,args,timeout=120):
 start=time.monotonic()
 try:
  p=subprocess.run(args,cwd=run/c,env=env,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=timeout);status='passed' if p.returncode==0 else 'failed';code=p.returncode;out=p.stdout
 except subprocess.TimeoutExpired as e:status='timeout';code=None;out=(e.stdout or b'').decode() if isinstance(e.stdout,bytes) else (e.stdout or '')
 out=out.replace(str(root),'<audit-worktree>').replace(str(pathlib.Path.home()),'<operator-home>')
 (run/(label+'.log')).write_text(out)
 r=dict(id=label,component=c,argv=args,status=status,exit_code=code,elapsed_seconds=round(time.monotonic()-start,3),log=str((run/(label+'.log')).relative_to(root)),tail=out[-1400:]);results.append(r)
 (run/'probe-results.json').write_text(json.dumps(results,indent=2));print(label,status,flush=True)
for c in components[:-1]:
 probe(c+'-metadata',c,['cargo','metadata','--offline','--no-deps','--format-version','1','--manifest-path',c+'/Cargo.toml'])
probe('csdlc-build-cold', 'csdlc-v3',['cargo','build','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml'],240)
probe('csdlc-build-warm', 'csdlc-v3',['cargo','build','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml'],120)
probe('csdlc-unit','csdlc-v3',['cargo','test','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml','--lib'],240)
probe('csdlc-package','csdlc-v3',['cargo','package','--offline','--allow-dirty','--no-verify','--manifest-path','csdlc-v3/Cargo.toml'],120)
probe('csdlc-publish-dry-run','csdlc-v3',['cargo','publish','--dry-run','--offline','--allow-dirty','--manifest-path','csdlc-v3/Cargo.toml'],120)
probe('adl-build','adl',['cargo','check','--offline','--manifest-path','adl/Cargo.toml'],60)
probe('runtime-build','adl-runtime',['cargo','check','--offline','--manifest-path','adl-runtime/Cargo.toml'],60)
probe('uts-test','adl-uts',['cargo','test','--offline','--manifest-path','adl-uts/Cargo.toml','--lib'],120)
probe('provider-core-test','adl-provider-core',['cargo','test','--offline','--manifest-path','adl-provider-core/Cargo.toml','--lib'],120)

raw=subprocess.check_output(['git','archive',base,'docs/csdlc-v3','docs/templates/prompts','.adl/worktree-policy.json'])
with tarfile.open(fileobj=io.BytesIO(raw)) as t:t.extractall(run/'csdlc-v3',filter='data')
probe('csdlc-owned-bundle-build','csdlc-v3',['cargo','build','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml'],240)
probe('csdlc-owned-bundle-unit','csdlc-v3',['cargo','test','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml','--lib'],240)
probe('csdlc-owned-bundle-warm','csdlc-v3',['cargo','build','--locked','--offline','--manifest-path','csdlc-v3/Cargo.toml'],60)
for c in ['adl','adl-runtime','adl-runtime-kernel','csdlc-v2']:
 probe(c+'-package',c,['cargo','package','--offline','--allow-dirty','--no-verify','--manifest-path',c+'/Cargo.toml'],60)

for record in results:
 data=(root/record['log']).read_bytes()
 record['log_sha256']=hashlib.sha256(data).hexdigest()
 record['test_results']=[dict(zip(['passed','failed','ignored','measured','filtered'],map(int,m))) for m in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out',data.decode())]
 if record['id'].endswith('-metadata') and record['status']=='passed': record['tail']='Cargo manifest metadata only; --no-deps does not resolve or build dependencies.'
(run/'portable-results.json').write_text(json.dumps({'source_revision':base,'probes':results},indent=2)+'\n')
