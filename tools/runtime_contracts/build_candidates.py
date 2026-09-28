#!/usr/bin/env python3
"""Prepare two immutable ADL producer candidates; never publish or admit them.

Use an explicit full Git revision. Only version/workspace/dependency manifest
normalization is permitted. Source, tests, schema and license bytes are retained.
This is separate from the accepted five-package RD03 distribution.
"""
import argparse, hashlib, io, json, re, subprocess, tarfile, tomllib
from pathlib import Path

def digest(data):
    return hashlib.sha256(data).hexdigest()

def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--repo',type=Path,required=True)
    p.add_argument('--revision',required=True)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args()
    if not re.fullmatch(r'[0-9a-f]{40}',a.revision):p.error('full immutable source revision required')
    actual=git(a.repo,'rev-parse',a.revision+'^{commit}').decode().strip()
    if actual!=a.revision:raise ValueError('source revision mismatch')
    workspace=tomllib.loads(git(a.repo,'show',a.revision+':adl-v2/Cargo.toml').decode())
    version=workspace['workspace']['package']['version']
    license_bytes=git(a.repo,'show',a.revision+':LICENSE')
    a.output.mkdir(parents=True,exist_ok=False)
    report={'schema':'adl.runtime_contract_candidates.v1','source_repository':'agent-logic/agent-design-language','source_revision':a.revision,'owner':'agent-design-language','accepted':False,'publication':'none','rd03_distribution_changed':False,'packages':[]}
    for name in ('adl-engine','adl-records'):
        source='adl-v2/crates/'+name
        records=git(a.repo,'ls-tree','-r','-z',a.revision,'--',source).split(b'\0')
        files={}; inventory={}
        for item in records:
            if not item:continue
            meta,path=item.split(b'\t',1);mode,kind,oid=meta.decode().split();path=path.decode()
            if kind!='blob' or mode not in ('100644','100755'):raise ValueError('unsupported source member')
            rel=path[len(source)+1:];raw=git(a.repo,'cat-file','blob',oid)
            inventory[rel]={'git_blob':oid,'sha256':digest(raw),'mode':mode}
            if rel=='Cargo.toml':
                text=raw.decode()
                if text.count('version.workspace = true')!=1:raise ValueError('unexpected manifest version')
                text=text.replace('version.workspace = true',f'version = "{version}"')
                for dep in ('adl-compiler','adl-language'):
                    text=text.replace(f'{dep} = {{ path = "../{dep}" }}',f'{dep} = {{ version = "={version}", path = "../{dep}" }}')
                if '[workspace]' in text:raise ValueError('unexpected nested workspace')
                raw=(text+'\n[workspace]\n').encode()
            if name=='adl-engine' and rel=='tests/compiler_fixture_mapping.rs':
                old=b'../../../adl-characterization/corpus/v1/fixtures'
                if raw.count(old)!=1:raise ValueError('unexpected fixture path')
                raw=raw.replace(old,b'tests/fixtures/characterization')
            files[name+'/'+rel]=(raw,int(mode,8)&0o777)
        test_resources={}
        if name=='adl-engine':
            fixture_source='adl-characterization/corpus/v1/fixtures'
            for item in git(a.repo,'ls-tree','-r','-z',a.revision,'--',fixture_source).split(b'\0'):
                if not item:continue
                meta,path=item.split(b'\t',1);mode,kind,oid=meta.decode().split();path=path.decode()
                if kind!='blob' or mode!='100644':raise ValueError('unsupported fixture member')
                raw=git(a.repo,'cat-file','blob',oid);rel=path[len(fixture_source)+1:]
                files[name+'/tests/fixtures/characterization/'+rel]=(raw,0o644)
                test_resources[path]={'git_blob':oid,'sha256':digest(raw),'role':'unchanged inert ADL-owned characterization fixture'}
            if len(test_resources)!=19:raise ValueError('unexpected characterization denominator')
        files[name+'/LICENSE']=(license_bytes,0o644)
        provenance={'source_repository':report['source_repository'],'source_revision':a.revision,'source_path':source,'owner':'agent-design-language','accepted':False,'normalization':'explicit package version; standalone workspace; exact public sibling compiler/language versions only','source_inventory':inventory,'test_resources':test_resources,'test_path_normalization':'engine characterization fixture root points to packaged identical19fixtures; production implementation unchanged','license_sha256':digest(license_bytes)}
        files[name+'/SOURCE_PROVENANCE.json']=((json.dumps(provenance,sort_keys=True,indent=2)+'\n').encode(),0o644)
        archive=a.output/f'{name}-{version}-candidate.tar'
        with tarfile.open(archive,'w',format=tarfile.USTAR_FORMAT) as t:
            for path,(raw,mode) in sorted(files.items()):
                info=tarfile.TarInfo(path);info.size=len(raw);info.mode=mode;info.mtime=0
                t.addfile(info,io.BytesIO(raw))
        report['packages'].append({'name':name,'version':version,'archive':archive.name,'sha256':digest(archive.read_bytes()),'files':len(files),'payload':{path:{'sha256':digest(raw),'mode':mode} for path,(raw,mode) in sorted(files.items())}})
    (a.output/'manifest.json').write_text(json.dumps(report,sort_keys=True,indent=2)+'\n')
    print(json.dumps({'accepted':False,'manifest_sha256':digest((a.output/'manifest.json').read_bytes()),'packages':[{k:x[k] for k in ('name','archive','sha256','files')} for x in report['packages']]}))
if __name__=='__main__':main()
