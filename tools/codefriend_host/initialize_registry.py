#!/usr/bin/env python3
"""Explicit installer step: create the initial deny-all registry once."""
import argparse
import json
import os
from pathlib import Path
import pwd
import re
import stat
from idle_stop import require

def initialize(directory, uid, gid):
    directory = Path(directory)
    for path in (directory, *directory.parents):
        require(not path.is_symlink(), 'symlink directory denied')
    info = directory.stat()
    require(stat.S_ISDIR(info.st_mode) and info.st_uid == uid and not info.st_mode & 0o077,
            'private service-owned directory required')
    parent = os.open(directory, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        pinned = os.fstat(parent)
        require(pinned.st_ino == info.st_ino and pinned.st_dev == info.st_dev,
                'directory changed during initialization')
        try:
            fd = os.open('gateway-credentials.json', os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                         0o600, dir_fd=parent)
        except FileExistsError:
            existing = os.open('gateway-credentials.json', os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
            with os.fdopen(existing, 'rb') as source:
                metadata = os.fstat(source.fileno())
                require(stat.S_ISREG(metadata.st_mode) and metadata.st_uid == uid and
                        not metadata.st_mode & 0o077 and metadata.st_size <= 131072,
                        'invalid existing registry metadata')
                value = json.loads(source.read(131073))
                require(isinstance(value, list) and len(value) <= 512 and
                        all(isinstance(item, dict) for item in value), 'invalid existing registry shape')
                hashes = set()
                for item in value:
                    require(set(item) == {'token_hash','subject','mode','expires_at'}, 'invalid credential fields')
                    require(isinstance(item['subject'],str) and re.fullmatch('[A-Za-z0-9_-]{1,80}',item['subject']), 'invalid credential subject')
                    require(isinstance(item['token_hash'],str) and re.fullmatch('[A-Fa-f0-9]{64}',item['token_hash']) and item['token_hash'] not in hashes, 'invalid credential hash')
                    hashes.add(item['token_hash'])
                    require(item['mode'] in ('hosted','local_model') and type(item['expires_at']) is int and 0 <= item['expires_at'] < 2**64, 'invalid credential mode or expiry')
            return 'existing_registry_preserved' 
        with os.fdopen(fd, 'wb') as output:
            os.fchmod(output.fileno(), 0o600)
            os.fchown(output.fileno(), uid, gid)
            output.write(b'[]\n'); output.flush(); os.fsync(output.fileno())
        os.fsync(parent)
        return 'deny_all_registry_created'
    finally:
        os.close(parent)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--website-private-directory', required=True)
    args=parser.parse_args()
    require(os.geteuid()==0, 'installer requires root')
    directory=Path(args.website_private_directory)
    require(directory.is_absolute() and directory.is_relative_to('/var/lib/codefriend') and '..' not in directory.parts,
            'directory must be inside /var/lib/codefriend')
    user=pwd.getpwnam('codefriend')
    print(initialize(directory,user.pw_uid,user.pw_gid))

if __name__=='__main__': main()
