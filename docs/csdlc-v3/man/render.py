#!/usr/bin/env python3
"""Render the reviewed operator manual; --check rejects stale generated pages."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def escape(text):
    return str(text).replace('\\', r'\e').replace('-', r'\-').replace('"', r'\(dq')


def render(page):
    name = page['name']
    roff = [f'.TH "{name.upper()}" "1" "September 11, 2026" "C-SDLC v3" "ADL Operator Manual"',
            '.SH NAME', escape(name + ' - ' + page['summary'])]
    for title, paragraphs in page['sections'].items():
        roff.append('.SH ' + title.upper())
        for index, paragraph in enumerate(paragraphs):
            if isinstance(paragraph, dict):
                roff.extend(['.nf', *[r'\&' + escape(line) for line in paragraph['code'].splitlines()], '.fi'])
            else:
                if index:
                    roff.append('.PP')
                roff.append(r'\&' + escape(paragraph))
    return '\n'.join(roff) + '\n'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    manual = json.loads((ROOT / 'manual.json').read_text())
    expected = set()
    for page in manual['pages']:
        path = ROOT / 'man1' / (page['name'] + '.1')
        expected.add(path.name)
        output = render(page)
        if args.check:
            if not path.is_file() or path.read_text() != output:
                raise SystemExit('stale manual page: ' + path.name)
        else:
            path.parent.mkdir(exist_ok=True)
            path.write_text(output)
    if {p.name for p in (ROOT / 'man1').glob('*.1')} != expected:
        raise SystemExit('unexpected manual page; remove stale output explicitly')
    print(f'{len(expected)} manual pages ' + ('verified' if args.check else 'rendered'))


if __name__ == '__main__':
    main()
