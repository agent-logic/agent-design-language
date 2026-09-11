#!/usr/bin/env python3
"""PVF release-evidence: small deterministic local contract negatives, required."""
import copy
import json
import unittest
import validate as v

class ReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.packet = json.loads((v.HERE / 'reconciliation.json').read_text())
        self.observed = json.loads((v.HERE / 'github-readback.json').read_text())

    def test_positive(self):
        self.assertEqual(v.validate(self.packet, self.observed)['findings'], 14)

    def test_adversarial_mutations(self):
        cases = {
            'stale_open_predecessor': lambda p, o: o['issues'][0].update(state='OPEN'),
            'wrong_closing_pr': lambda p, o: p.update(closing_pr=832),
            'wrong_closure_edge': lambda p, o: o['issues'][0].update(closedByPullRequestsReferences=[]),
            'missing_reciprocal_edge': lambda p, o: o['pull_requests'][0].update(closingIssuesReferences=[]),
            'non_ancestral_merge': lambda p, o: o['pull_requests'][0]['mergeCommit'].update(oid='0'*40),
            'dropped_id': lambda p, o: p['findings'].pop(),
            'duplicate_id': lambda p, o: p['findings'].__setitem__(1, p['findings'][0]),
            'wrong_owner': lambda p, o: p['findings'][0].update(owners=[814]),
            'lost_correction': lambda p, o: p['findings'][0].update(correction_owners=[]),
            'release_overclaim': lambda p, o: p.update(release_ready=True),
            'semantic_overclaim': lambda p, o: p['findings'][0].update(disposition='behavioral_pass'),
            'missing_evidence': lambda p, o: p['owners'][0].update(evidence=[]),
            'corrupt_evidence': lambda p, o: p['owners'][0]['evidence'][0].update(sha256='0'*64),
            'historical_rebase': lambda p, o: p['historical_sources'][0].update(revision=p['source_candidate']),
            'dropped_history': lambda p, o: p.update(historical_sources=[]),
            'wrong_repository': lambda p, o: o.update(repository='example/other'),
            'duplicate_remote': lambda p, o: o['issues'].append(o['issues'][0]),
        }
        for name, mutate in cases.items():
            with self.subTest(name=name):
                p, o = copy.deepcopy(self.packet), copy.deepcopy(self.observed)
                mutate(p, o)
                with self.assertRaises(ValueError):
                    v.validate(p, o)

if __name__ == '__main__':
    unittest.main()
