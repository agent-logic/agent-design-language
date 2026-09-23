#!/usr/bin/env python3
"""Conservative proof that a diff changes only existing top-level cfg(test) modules.

Unknown syntax, unavailable Git objects and mixed edits never grant an exemption.
This does not suppress test selection or replace compilation/coverage generation.
"""
import argparse
from pathlib import Path
import re
import subprocess


class Unknown(ValueError):
    pass


def tokens(source):
    """Minimal lossless-position lexer; only delimiter context is interpreted."""
    result = []
    i = 0
    while i < len(source):
        start = i
        c = source[i]
        if c.isspace():
            i += 1
            continue
        if source.startswith('//', i):
            end = source.find('\n', i)
            i = len(source) if end < 0 else end + 1
            continue
        if source.startswith('/*', i):
            depth = 1
            i += 2
            while depth and i < len(source):
                if source.startswith('/*', i):
                    depth += 1
                    i += 2
                elif source.startswith('*/', i):
                    depth -= 1
                    i += 2
                else:
                    i += 1
            if depth:
                raise Unknown('unclosed comment')
            continue
        raw = re.match(r'(?:br|cr|r)(#{0,255})"', source[i:])
        if raw:
            close = '"' + raw[1]
            end = source.find(close, i + raw.end())
            if end < 0:
                raise Unknown('unclosed raw literal')
            i = end + len(close)
            result.append(('literal', start, i))
            continue
        quoted = re.match(r'(?:b|c)?"', source[i:])
        if quoted:
            i += quoted.end()
            while i < len(source):
                if source[i] == '\\':
                    i += 2
                elif source[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
            else:
                raise Unknown('unclosed string')
            result.append(('literal', start, i))
            continue
        # Characters (including byte characters) and lifetimes are disjoint.
        char = re.match(r"b?'(?:[^'\\\n]|\\(?:[nrt0\\'\"]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}))'", source[i:])
        if char:
            i += char.end()
            result.append(('literal', start, i))
            continue
        lifetime = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", source[i:])
        if lifetime:
            i += lifetime.end()
            result.append(('lifetime', start, i))
            continue
        if c in "'\"" or not c.isascii():
            raise Unknown('unclassified literal or identifier')
        word = re.match(r'[A-Za-z_][A-Za-z0-9_]*', source[i:])
        if word:
            i += word.end()
            result.append((word[0], start, i))
        else:
            i += 1
            result.append((c, start, i))
    return result


def production_projection(source):
    ts = tokens(source)
    stack = []
    candidate = None
    pieces = []
    retained_start = 0
    pattern = ['#', '[', 'cfg', '(', 'test', ')', ']', 'mod']
    for n, (token, start, end) in enumerate(ts):
        if not stack and [t[0] for t in ts[n:n+8]] == pattern:
            if n + 9 < len(ts) and re.fullmatch('[A-Za-z_][A-Za-z0-9_]*', ts[n+8][0]) and ts[n+9][0] == '{':
                candidate = n+9
        if token in ('{', '[', '('):
            stack.append(token)
        elif token in ('}', ']', ')'):
            if not stack or stack.pop() != {'}':'{', ']':'[', ')':'('}[token]:
                raise Unknown('unbalanced delimiters')
            if candidate is not None and n > candidate and not stack:
                if token != '}':
                    raise Unknown('unclassified module boundary')
                # Retain every byte outside the body, including its exact cfg,
                # name and braces. Preserve each segment separately so removing
                # or adding modules cannot splice formerly distinct items.
                pieces.append(source[retained_start:ts[candidate][2]])
                retained_start = start
                candidate = None
    if stack or candidate is not None or not pieces:
        raise Unknown('no complete test modules or unbalanced source')
    pieces.append(source[retained_start:])
    return tuple(pieces)


def test_only(old, new):
    if old == new:
        return False
    try:
        return production_projection(old) == production_projection(new)
    except (Unknown, IndexError):
        return False


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], stderr=subprocess.DEVNULL).decode('utf-8')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True)
    parser.add_argument('--base', required=True)
    parser.add_argument('--head', required=True)
    parser.add_argument('--path', required=True)
    parser.add_argument('--working-tree', action='store_true')
    args = parser.parse_args()
    try:
        root = Path(args.root).resolve()
        path = Path(args.path)
        if path.is_absolute() or '..' in path.parts or not args.path.endswith('.rs'):
            return 1
        base = git(root, 'rev-parse', '--verify', '--end-of-options', args.base+'^{commit}').strip()
        if args.working_tree:
            file = root / path
            if file.is_symlink() or not file.resolve().is_relative_to(root):
                return 1
            new = file.read_text()
        else:
            head = git(root, 'rev-parse', '--verify', '--end-of-options', args.head+'^{commit}').strip()
            # Match the coverage owner's three-dot comparison; unknown ancestry
            # is not a license to exempt a file.
            base = git(root, 'merge-base', base, head).strip()
            new = git(root, 'show', head+':'+args.path)
        old = git(root, 'show', base+':'+args.path)
        return 0 if test_only(old, new) else 1
    except (OSError, ValueError, subprocess.CalledProcessError, UnicodeError):
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
