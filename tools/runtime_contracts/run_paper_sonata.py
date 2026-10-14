#!/usr/bin/env python3
"""Run exact owner Paper Sonata producers with explicit source inputs."""
import hashlib,os,pathlib,shutil,subprocess,sys,tempfile
RUNTIME = [{'path': 'adl/tools/demo_v088_paper_sonata.sh', 'sha256': '32e7c6df82943fef5b22089a5a541b22451473a80d39d9aaf3519ab507e71561', 'owner_mode': '100755'}, {'path': 'adl/tools/test_demo_v088_paper_sonata.sh', 'sha256': '1d7ac415764ad82a1f0bcaf10b52bc9ef98d72408e461d1aebc1d9e99603394a', 'owner_mode': '100755'}, {'path': 'adl/examples/v0-88-paper-sonata.adl.yaml', 'sha256': '0d757cffaf6f28804578c7832a7361dbdf0a26b994d69410e9ddc902f394b1ca', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/citations_seed.json', 'sha256': '936948e61aca7f4446c690c0b4f2b9cc7a638feea11ca9792e3a7531b9561b93', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/experiment_results.json', 'sha256': '960610b1326ee733fce82246e71cd347219dad18d09bf35c69d54b8bdecad238', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/idea_summary.md', 'sha256': '63af8664824a23cc5cabfb9f6e80996e3fba616dbbe53246610d755edf78cc64', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/lab_notes.md', 'sha256': 'dd61e45e601a02a1b29a11e2c61b4a6869780eeb301610740b215eb2d1e9f6dc', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/paper_constraints.md', 'sha256': 'd8da46ad3fb854ff9aaf8e063ab6a8671643e79097761ec03fe6cf13fd3428bb', 'owner_mode': '100644'}, {'path': 'demos/fixtures/paper_sonata/target_venue.md', 'sha256': '41b4ab42fe34f591b5121ec5d0886d9e2b15fc686c73e5b388b234a672686547', 'owner_mode': '100644'}]
INFRASTRUCTURE = [{'path': 'adl/tools/demo_v0902_paper_sonata_expansion.sh', 'sha256': '4177332568b41ba79af7ae62821f6b769868457b4959d387e95860f8772bb3f1', 'owner_mode': '100644'}, {'path': 'adl/tools/test_demo_v0902_paper_sonata_expansion.sh', 'sha256': '9ccf7bf652c6c2e5e58ad48f197a80887044625b212f0aeac88dc692bd91d26a', 'owner_mode': '100644'}, {'path': 'adl/tools/validate_v0902_paper_sonata_expansion.py', 'sha256': '8b958b13513eca1e9b9c1d0887073e16ff34d96bdd4964a71d085e76c589d3e6', 'owner_mode': '100644'}]

def main():
    check_only = len(sys.argv)==3 and sys.argv[1]=='--check'
    mode = sys.argv[2] if check_only else (sys.argv[1] if len(sys.argv)>1 else '')
    if len(sys.argv)!=3 or mode not in ('paper','expansion'):
        raise SystemExit('Usage: run_paper_sonata.py paper|expansion OUTPUT_DIRECTORY; or --check paper|expansion')
    output=None if check_only else pathlib.Path(sys.argv[2]).absolute()
    groups=[('ADL_RUNTIME_SOURCE_ROOT',RUNTIME if mode=='paper' else [x for x in RUNTIME if x['path'].startswith('demos/')])]
    if mode=='expansion':groups.append(('ADL_INFRASTRUCTURE_SOURCE_ROOT',INFRASTRUCTURE))
    verified=[]
    for variable,inputs in groups:
        root=pathlib.Path(os.environ.get(variable,''))
        if not root.is_absolute() or not root.is_dir():raise SystemExit('Set absolute '+variable+' to the pinned owner source checkout')
        for item in inputs:
            path=root/item['path']
            if path.is_symlink() or not path.is_file() or path.stat().st_mode&0o777!=int(item['owner_mode'],8)&0o777 or hashlib.sha256(path.read_bytes()).hexdigest()!=item['sha256']:
                raise SystemExit('Owner input missing or changed: '+item['path'])
            verified.append((path,item['path']))
    env=dict(os.environ)
    if mode=='paper':
        binary=pathlib.Path(env.get('ADL_DEMO_ADL_BIN',''))
        if not binary.is_absolute() or not binary.is_file() or not os.access(binary,os.X_OK):
            raise SystemExit('Set absolute executable ADL_DEMO_ADL_BIN; no Cargo fallback')
    if check_only:return 0
    with tempfile.TemporaryDirectory(prefix='adl-paper-owner-') as temp:
        root=pathlib.Path(temp)
        for source,name in verified:
            target=root/name;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(source,target)
        producer='demo_v088_paper_sonata.sh' if mode=='paper' else 'demo_v0902_paper_sonata_expansion.sh'
        return subprocess.call(['bash',str(root/'adl/tools'/producer),str(output)],cwd=root,env=env)
if __name__=='__main__':raise SystemExit(main())
