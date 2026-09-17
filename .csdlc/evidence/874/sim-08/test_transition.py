"""PVF tooling: pure request/argv negative guards; no processes or conversion rehearsal."""
import copy
from pathlib import Path
import unittest
import transition as t


class RequestTests(unittest.TestCase):
    def setUp(self):
        self.root = Path('/isolated/checkout')
        self.index = dict(issue=875, repository=t.REPOSITORY, phase='ready',
                          branch='codex/875-pilot', worktree=str(self.root))
        self.cards = {k: dict(issue=875, repository=t.REPOSITORY, slug='pilot', card=k)
                      for k in t.CARDS}
        self.plan = dict(schema='csdlc.v3.intent_plan.v1', slug='pilot', cards=self.cards,
                         validators=[{'id': 'existing'}], publication={'body': 'Closes #875'})

    def build(self):
        return t.construct_plan(self.index, self.cards, self.plan, 875, self.root)

    def test_preserves_existing_contract_exactly(self):
        self.assertEqual(self.build(), self.plan)

    def test_wrong_slug_does_not_rewrite_branch(self):
        self.plan['slug'] = 'invented'
        with self.assertRaisesRegex(ValueError, 'slug'): self.build()

    def test_changed_current_cards_require_explicit_amendment(self):
        self.plan = copy.deepcopy(self.plan)
        self.cards['spp']['intent'] = 'changed'
        with self.assertRaisesRegex(ValueError, 'differs'): self.build()

    def test_missing_validator_declarations_are_not_inferred(self):
        del self.plan['validators']
        with self.assertRaisesRegex(ValueError, 'declarations'): self.build()

    def test_bound_record_rejects_primary_checkout(self):
        self.index.update(phase='bound', worktree='/another/checkout')
        with self.assertRaisesRegex(ValueError, 'registered worktree'): self.build()

    def test_published_record_is_not_downgraded(self):
        self.index['phase'] = 'published'
        with self.assertRaisesRegex(ValueError, 'unsupported'): self.build()

    def test_wrong_card_issue_rejected(self):
        self.cards['sor']['issue'] = 874
        with self.assertRaisesRegex(ValueError, 'identity'): self.build()

    def test_recovery_inspection_does_not_execute(self):
        self.assertNotIn('--execute', t.native_argv('/candidate', 875, self.root, 'recover'))

    def test_recovery_execution_binds_preview(self):
        argv = t.native_argv('/candidate', 875, self.root, 'recover', preview='fresh-digest')
        self.assertEqual(argv[3:6], ['--execute', '--preview', 'fresh-digest'])

    def test_live_restore_is_not_mapped_to_fixture(self):
        with self.assertRaisesRegex(ValueError, 'unsupported'):
            t.native_argv('/candidate', 875, self.root, 'restore')


if __name__ == '__main__':
    unittest.main()
