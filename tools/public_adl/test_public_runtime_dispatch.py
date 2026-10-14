#!/usr/bin/env python3
"""PVF local_contract: actual public CLI wiring against a synthetic qualified layout.

This is dispatch and logging-control proof, not Runtime package qualification.
"""
import argparse
import json
import os
from pathlib import Path
import platform
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    binary = parser.parse_args().binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix='adl-public-dispatch-') as directory:
        root = Path(directory).resolve()
        prefix = root/'installed'
        identity = 'a'*64
        package = prefix/'versions'/identity
        (package/'bin').mkdir(parents=True)
        executable = package/'bin/adl-runtime'
        executable.write_text('#!/bin/sh\nprintf "owner\\n"\nprintf "<%s>\\n" "$@"\nprintf "control=%s\\nlog=%s\\n" "${ADL_OBSERVABILITY-unset}" "${ADL_OBSERVABILITY_LOG-unset}"\nprintf "owner-stderr\\n" >&2\nexit 17\n')
        executable.chmod(0o755)
        (package/'archive.sha256').write_text(identity+'\n')
        (package/'archive.tar').write_bytes(b'synthetic fixture, not qualification')
        target = {'arm64':'aarch64'}.get(platform.machine(),platform.machine())
        target += {'Darwin':'-apple-darwin','Linux':'-unknown-linux-gnu'}[platform.system()]
        (package/'manifest.json').write_text(json.dumps({'schema':'runtime.local-package.v1','target':target,
            'files':{'bin/adl-runtime':{'mode':0o755,'sha256':'b'*64}}}))
        (prefix/'current').symlink_to('versions/'+identity)
        fixture = root/'workflow space ; literal.yaml'
        fixture.write_bytes((ROOT/'adl/examples/v0-6-hitl-no-pause.adl.yaml').read_bytes())
        environment = {k:v for k,v in os.environ.items() if not k.startswith('ADL_')}
        selected = {'ADL_RUNTIME_INSTALL_PREFIX':str(prefix),'ADL_RUNTIME_ARCHIVE_SHA256':identity}
        cases = []
        def call(name,args,extra=None,qualified=True):
            result = subprocess.run([str(binary),*map(str,args)],cwd=root,capture_output=True,text=True,
                                    env={**environment,**(selected if qualified else {}),**(extra or {})},timeout=10)
            cases.append(name)
            return result
        r=call('explicit-run',[fixture,'--run','--allow-unsigned'])
        assert r.returncode==17 and r.stdout.startswith('owner\n<run>\n<'+str(fixture)+'>\n'),r
        assert 'command=adl stage=dispatch' in r.stderr and r.stderr.endswith('owner-stderr\n'),r
        r=call('implicit-run',[fixture])
        assert r.returncode==17 and r.stdout.startswith('owner\n<run>\n'),r
        r=call('disabled-observability',[fixture,'--run'],{'ADL_OBSERVABILITY':'0'})
        assert r.returncode==17 and 'control=0\n' in r.stdout and r.stderr=='owner-stderr\n',r
        log=root/'compatibility.log'
        r=call('file-log-quiet-stderr',[fixture,'--run'],{'ADL_OBSERVABILITY_STDERR':'0','ADL_OBSERVABILITY_LOG':str(log)})
        assert r.returncode==17 and r.stderr=='owner-stderr\n' and 'log='+str(log)+'\n' in r.stdout,r
        assert 'command=adl stage=dispatch' in log.read_text()
        r=call('local-plan',[fixture,'--print-plan'],qualified=False)
        assert r.returncode==0 and r.stdout.startswith('Resolved run:') and 'owner\n' not in r.stdout,r
        r=call('resume-help',['resume','--help'],qualified=False)
        assert r.returncode==0 and 'Semantics:' in r.stdout and 'owner\n' not in r.stdout,r
        r=call('resume-missing-id',['resume'],qualified=False)
        assert r.returncode==2 and 'resume requires <run_id>' in r.stderr and 'Semantics:' in r.stderr,r
        r=call('resume-forward',['resume','run-id','--adl',fixture])
        assert r.returncode==17 and r.stdout.startswith('owner\n<resume>\n<run-id>\n<--adl>\n'),r
        r=call('missing-owner',[fixture,'--run'],qualified=False)
        assert r.returncode==1 and not r.stdout and 'ADL_RUNTIME_INSTALL_PREFIX' in r.stderr,r
    print(json.dumps({'status':'passed','cases':cases,'count':len(cases),
                      'limits':'Synthetic owner layout; real installed Runtime qualification recorded separately'}))


if __name__=='__main__':
    main()
