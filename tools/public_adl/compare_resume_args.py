#!/usr/bin/env python3
"""PVF local_contract: compare extracted resume diagnostics to an explicit baseline CLI."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
CASES = [[], ['--help'], ['run-id', '--adl'], ['run-id', '--steer'],
         ['run-id', 'unexpected'], ['run-id', '--adl', 'literal file', '--bogus']]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    args = parser.parse_args()
    baseline = args.baseline.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix='adl-resume-args-') as directory:
        root = Path(directory)
        source = root/'main.rs'
        source.write_text(
            '#[path=' + json.dumps(str(ROOT/'adl/src/cli/resume_args.rs')) + '] mod resume_args;\n'
            '#[path=' + json.dumps(str(ROOT/'adl/src/cli/usage.rs')) + '] mod usage;\n'
            'fn main(){let args:Vec<String>=std::env::args().skip(1).collect();'
            'if resume_args::parse(&args,usage::resume_usage()).is_some(){std::process::exit(3);}}\n')
        binary = root/'probe'
        subprocess.run(['rustc', '--edition=2021', str(source), '-o', str(binary)],
                       check=True, capture_output=True)
        environment = {**os.environ, 'ADL_OBSERVABILITY': '0'}
        for case in CASES:
            old = subprocess.run([str(baseline), 'resume', *case], env=environment,
                                 capture_output=True, timeout=10)
            new = subprocess.run([str(binary), *case], env=environment,
                                 capture_output=True, timeout=10)
            if (old.returncode, old.stdout, old.stderr) != (new.returncode, new.stdout, new.stderr):
                raise AssertionError({'args': case, 'baseline_exit': old.returncode,
                                      'parser_exit': new.returncode})
    print(json.dumps({'status': 'passed', 'cases': len(CASES),
                      'claim': 'exact diagnostic/help stdout stderr and exit; no resume execution'}))


if __name__ == '__main__':
    main()
