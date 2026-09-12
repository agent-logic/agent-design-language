#!/usr/bin/env python3
"""Narrow acquisition gate selection; unknown diff/event forces execution upstream."""
import sys

def selected(path):
    return (path.startswith(('adl/src/codefriend/', 'adl/tools/codefriend/'))
            or path.startswith('adl/src/cli/codefriend')
            or path.startswith('adl/tests/codefriend')
            or path in {'adl/src/cli/mod.rs', 'adl/src/lib.rs', 'adl/src/main.rs',
                        'adl/Cargo.toml', 'adl/Cargo.lock', 'Cargo.lock', 'rust-toolchain.toml',
                        '.github/workflows/ci.yaml', 'adl/tools/ci_path_policy.sh',
                        'adl/tools/install_owner_binaries.sh'})

if __name__ == '__main__':
    print('true' if any(selected(p.strip()) for p in sys.stdin) else 'false')
