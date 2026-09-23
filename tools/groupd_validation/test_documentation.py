#!/usr/bin/env python3
"""PVF: deterministic, small local documentation-contract regressions (#1160)."""
import re
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / 'docs/milestones/v0.92.2'
PAGES = ('README.md', 'DEMO_MATRIX_v0.92.2.md', 'FEATURE_PROOF_COVERAGE_v0.92.2.md',
         'RELEASE_PLAN_v0.92.2.md', 'RELEASE_NOTES_v0.92.2.md', 'NEXT_MILESTONE_HANDOFF_v0.92.2.md')


def current_routing(text):
    current = text.split('## Current qualification disposition\n', 1)[1].split('\n## ', 1)[0]
    for required in ('#915', 'NOT_PLANNED', 'incomplete', 'deferred', 'not a qualification pass',
                     '#1148', '#1149', '#1150', 'v0.93.1', 'v0.93.2'):
        if required not in current:
            raise ValueError(required)
    if re.search(r'#915.{0,50}(pending|open|must rerun)', current):
        raise ValueError('stale qualification routing')


class Contracts(unittest.TestCase):
    def test_current_qualification(self):
        for name in PAGES:
            with self.subTest(page=name):
                current_routing((DOCS / name).read_text())

    def test_stale_and_missing_routing_rejected(self):
        source = (DOCS / PAGES[0]).read_text()
        for old, new in (('NOT_PLANNED', 'OPEN'), ('#1150', '#915'),
                         ('v0.93.1', 'unselected'), ('not a qualification pass', 'qualification passed')):
            with self.subTest(old=old), self.assertRaises(ValueError):
                current_routing(source.replace(old, new))
        with self.assertRaises(ValueError):
            current_routing(source.replace('#915 is closed', '#915 pending'))

    def test_parser_docs_match_shared_bounds(self):
        source = (ROOT / 'adl/src/codefriend/rust_parse.rs').read_text()
        self.assertIn('MAX_SOURCE_BYTES: usize = 400 * 1024', source)
        self.assertIn('MAX_TOKENS: usize = 32 * 1024', source)
        self.assertIn('MAX_DEPTH: usize = 32', source)
        self.assertIn('MAX_PATH_UNITS: usize = 2048', source)
        for name in ('ARCHITECTURE.md', 'LOCAL_FITNESS.md'):
            text = (ROOT / 'docs/codefriend' / name).read_text()
            self.assertIn('400 KiB', text)
            self.assertIn('32,768', text)
            self.assertIn('RUST_PARSER_BOUNDS.md', text)
            self.assertNotIn('128 lexical units', text)
            self.assertNotIn('128 raw', text)
            self.assertNotIn('limited to 32 KiB', text)

if __name__ == '__main__':
    unittest.main()
