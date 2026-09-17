#!/usr/bin/env python3
"""Build the existing converter's request; never invoke it or mutate lifecycle state."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import sys

from transition import REPOSITORY, git, read_json

ROLES = ('prepared', 'bound_dirty', 'implemented', 'reviewed', 'published',
         'terminal', 'pending_recovery')
ISSUES = (511, 517, 497, 3, 505, 122, 113)
PRIOR_SHA256 = 'c1c1a7a9a9928c25139e1d5d12cbbb275f8c595f69b65b2ff8feb43b9299aeaf'
# Two digests of the same retained SIM06 executable, authenticated by SHA256 below.
PRIOR_BLAKE3 = 'e7fa9aedc8f992151dcfc82ff9bd0da7f5415d69798ac67956f2977f88416f64'


def child(path, root):
    path = Path(path).resolve(strict=True)
    if path == root or root not in path.parents:
        raise ValueError(f'input escapes isolated root: {path}')
    return path


def construct(root, linked, source, prior, operation, probe):
    root = Path(root).resolve(strict=True)
    if read_json(root / '.csdlc-conversion-rehearsal.json').get('isolated') is not True:
        raise ValueError('explicit isolated fixture marker required')
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,79}', operation):
        raise ValueError('operation must be a new simple identifier')
    if type(probe) is not int or probe <= 0 or probe in ISSUES:
        raise ValueError('positive separate writer probe issue required')
    linked, source, prior = (child(p, root) for p in (linked, source, prior))
    if git(linked, 'rev-parse', '--show-toplevel') != str(linked):
        raise ValueError('linked checkout must be exact Git root')
    common = child(git(linked, 'rev-parse', '--path-format=absolute', '--git-common-dir'), root)
    own_common = Path(git(Path(__file__).resolve().parents[4], 'rev-parse',
                          '--path-format=absolute', '--git-common-dir')).resolve()
    if common == own_common:
        raise ValueError('live repository is not an isolated conversion destination')
    git_dir = Path(git(linked, 'rev-parse', '--absolute-git-dir')).resolve()
    if git_dir == common or common not in git_dir.parents:
        raise ValueError('a genuine linked worktree is required')
    if hashlib.sha256(prior.read_bytes()).hexdigest() != PRIOR_SHA256:
        raise ValueError('prior executable differs from retained SIM06 bytes')
    records = []
    for role, issue in zip(ROLES, ISSUES):
        record = child(source / str(issue), source)
        index = read_json(record / 'index.json')
        if index.get('repository') != REPOSITORY or index.get('issue') != issue:
            raise ValueError(f'copied source identity mismatch: {issue}')
        records.append(dict(role=role, issue=issue, source=str(record)))
    # Native preflight remains authoritative for complete source/card/phase checks.
    registry = child(linked / 'docs/templates/prompts/current.json', linked)
    authority = child(linked / 'csdlc-v3/operator/authority-selector.json', linked)
    return dict(schema='csdlc.v3.copied_record_conversion.v1', repository=REPOSITORY,
                operation_id=operation, git_common=str(common), linked_worktree=str(linked),
                linked_branch=git(linked, 'symbolic-ref', '--quiet', '--short', 'HEAD'),
                linked_head=git(linked, 'rev-parse', 'HEAD'), registry_path=str(registry),
                authority_bytes_path=str(authority), prior_executable_path=str(prior),
                prior_executable_blake3=PRIOR_BLAKE3,
                writer_fence_issues=sorted((*ISSUES, probe)), writer_probe_issue=probe,
                writer_fence_probe=False, records=records)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('isolated-root', 'linked-worktree', 'source-root', 'prior-executable', 'output'):
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--operation-id', required=True)
    parser.add_argument('--writer-probe-issue', type=int, required=True)
    args = parser.parse_args()
    request = construct(args.isolated_root, args.linked_worktree, args.source_root,
                        args.prior_executable, args.operation_id, args.writer_probe_issue)
    output = args.output.absolute()
    child(output.parent, args.isolated_root.resolve())
    # Create-only output: never overwrite a retained operation request.
    with output.open('x') as stream:
        json.dump(request, stream, indent=2)
        stream.write('\n')
    print(json.dumps({'request': str(output), 'lifecycle_executed': False,
                      'native_preflight': 'not_run', 'scope': 'isolated_copies_only'}))


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print(json.dumps({'status': 'refused', 'reason': str(error)}), file=sys.stderr)
        sys.exit(2)
