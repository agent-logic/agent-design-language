#!/usr/bin/env python3
"""Retain local build evidence; this document is not a cryptographic attestation."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import struct
import subprocess

BINARIES=('codefriend-server','codefriend-agent')

def require(condition, message):
    if not condition: raise ValueError(message)

def digest(path):
    result=hashlib.sha256()
    with path.open('rb') as source:
        for block in iter(lambda:source.read(1024*1024),b''):result.update(block)
    return result.hexdigest()

def inspect_binary(path):
    require(path.is_file() and not path.is_symlink() and os.access(path,os.X_OK),'regular executable required')
    with path.open('rb') as source:header=source.read(64)
    require(len(header)==64 and header[:6]==b'\x7fELF\x02\x01' and struct.unpack('<H',header[18:20])[0]==62,'Linux amd64 ELF required')
    return digest(path)

def binary_pair(built, installed, smoke):
    hashes={}
    for name in BINARIES:
        value=inspect_binary(built/name)
        require(value==inspect_binary(installed/name),'installed bytes differ from build')
        require(smoke['binary_sha256'].get(name)==value,'tested bytes differ from build')
        hashes[name]=value
    return hashes

def command(*args):
    return subprocess.check_output(args,text=True).strip()

def assemble(args):
    require(re.fullmatch('[0-9a-f]{40}',args.candidate),'exact revision required')
    require(command('git','rev-parse','HEAD')==args.candidate,'checkout differs from selected head')
    require(not command('git','status','--porcelain','--untracked-files=all'),'source checkout dirty')
    require(command('uname','-m')=='x86_64','native x86_64 builder required')
    require(os.environ.get('CARGO_PROFILE_DEV_DEBUG')=='0','declared debug setting required')
    rust=command('rustc','-vV');require('release: 1.92.0\n' in rust+'\n','unexpected Rust version')
    smoke=json.loads(args.smoke.read_text())
    require(smoke.get('status')=='PASS' and smoke.get('candidate')==args.candidate and smoke.get('platform')=='linux','matching Linux smoke required')
    require(smoke.get('schema')=='codefriend.host_smoke.v1' and len(smoke.get('cases',[]))==15 and smoke.get('graceful_stops')==2,'complete smoke denominator required')
    require(smoke.get('provider_connections')==0 and smoke.get('operation_reservations')==0,'fixture side effects')
    hashes=binary_pair(args.built,args.installed,smoke)
    context={name:os.environ.get(name,'') for name in ('GITHUB_REPOSITORY','GITHUB_RUN_ID','GITHUB_RUN_ATTEMPT',
             'GITHUB_EVENT_NAME','GITHUB_SHA','GITHUB_WORKFLOW_REF','GITHUB_WORKFLOW_SHA','RUNNER_OS','RUNNER_ARCH')}
    require(all(context.values()) and context['GITHUB_EVENT_NAME']=='pull_request','complete PR builder context required')
    args.output.mkdir(mode=0o700)
    binaries=args.output/'bin';binaries.mkdir()
    for name in BINARIES:
        shutil.copyfile(args.installed/name,binaries/name);(binaries/name).chmod(0o755)
        require(digest(binaries/name)==hashes[name],'artifact copy differs')
    shutil.copyfile(args.smoke,args.output/'smoke.json')
    sources={}
    for name in command('git','ls-files','--','Cargo.lock','*/Cargo.lock','rust-toolchain.toml',
                        '.github/workflows/codefriend-host-candidate.yml','tools/codefriend_host/ci','adl/build.rs').splitlines():
        path=Path(name)
        if path.is_file():sources[name]=digest(path)
    manifest={'schema':'codefriend.host_build.v1','source_revision':args.candidate,
              'source_tree':command('git','rev-parse','HEAD^{tree}'),'source_clean':True,'builder_context':context,
              'runner_architecture':command('uname','-m'),'rustc':rust,'cargo':command('cargo','-vV'),
              'build_profile':'dev','build_environment':{'CARGO_PROFILE_DEV_DEBUG':'0'},
              'build_command':['cargo','build','--locked','--manifest-path','adl/Cargo.toml','--bin','codefriend-server','--bin','codefriend-agent'],
              'binary_sha256':hashes,'input_sha256':sources,'smoke_sha256':digest(args.output/'smoke.json'),
              'authentication':'requires_authenticated_GitHub_run_job_and_artifact_digest_readback',
              'attestation':False,'ready_for_deployment':False,
              'non_claims':['systemd acceptance','OAuth','providers','host shutdown','production deployment']}
    (args.output/'build.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
    checksums={str(path.relative_to(args.output)):digest(path) for path in sorted(args.output.rglob('*')) if path.is_file()}
    (args.output/'SHA256SUMS').write_text(''.join(f'{value}  {name}\n' for name,value in checksums.items()))
    print(json.dumps({'status':'retained_build_evidence','candidate':args.candidate,'attestation':False}))

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ('built','installed','smoke','output'):parser.add_argument('--'+name,type=Path,required=True)
    parser.add_argument('--candidate',required=True)
    assemble(parser.parse_args())
