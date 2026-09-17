"""PVF tooling: request/identity checks only; no conversion, guardian or rehearsal."""
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import conversion_request as c


class ConversionRequestTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.linked = self.root / 'linked'
        self.source = self.root / 'source'
        self.common = self.root / 'primary/.git'
        self.common.mkdir(parents=True)
        (self.root / '.csdlc-conversion-rehearsal.json').write_text('{"isolated":true}')
        for relative in ('docs/templates/prompts/current.json',
                         'csdlc-v3/operator/authority-selector.json'):
            p = self.linked / relative
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text('{}')
        for issue in c.ISSUES:
            p = self.source / str(issue)
            p.mkdir(parents=True)
            (p / 'index.json').write_text(json.dumps(dict(issue=issue, repository=c.REPOSITORY)))
        self.prior = self.root / 'prior'
        self.prior.write_bytes(b'unit-test-only')
        self.addCleanup(patch.stopall)
        patch.object(c, 'PRIOR_SHA256', c.hashlib.sha256(self.prior.read_bytes()).hexdigest()).start()
        patch.object(c, 'git', side_effect=self.git).start()

    def git(self, root, *args):
        if Path(root) != self.linked:
            return '/unrelated/live/.git'
        if '--show-toplevel' in args:
            return str(self.linked)
        if '--git-common-dir' in args:
            return str(self.common)
        if '--absolute-git-dir' in args:
            return str(self.common / 'worktrees/linked')
        return 'fixture' if args[0] == 'symbolic-ref' else 'a' * 40

    def build(self, **overrides):
        args = dict(root=self.root, linked=self.linked, source=self.source,
                    prior=self.prior, operation='packet-check', probe=868)
        return c.construct(**(args | overrides))

    def test_request_covers_exact_retained_denominator(self):
        request = self.build()
        self.assertEqual([r['role'] for r in request['records']], list(c.ROLES))
        self.assertEqual(request['writer_fence_issues'], sorted((*c.ISSUES, 868)))
        self.assertFalse(request['writer_fence_probe'])
        self.assertEqual(request['linked_head'], 'a' * 40)
        self.assertNotIn('fault_injection', request)

    def test_missing_isolation_marker_refuses(self):
        (self.root / '.csdlc-conversion-rehearsal.json').write_text('{}')
        with self.assertRaisesRegex(ValueError, 'marker'): self.build()

    def test_source_cannot_escape_through_symlink(self):
        with tempfile.TemporaryDirectory() as elsewhere:
            link = self.root / 'outside'
            link.symlink_to(elsewhere)
            with self.assertRaisesRegex(ValueError, 'escapes'): self.build(source=link)

    def test_wrong_source_issue_refuses(self):
        (self.source / '511/index.json').write_text('{"issue":512}')
        with self.assertRaisesRegex(ValueError, 'identity'): self.build()

    def test_changed_prior_binary_refuses(self):
        self.prior.write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError, 'executable'): self.build()

    def test_operation_path_traversal_refuses(self):
        with self.assertRaisesRegex(ValueError, 'identifier'): self.build(operation='../live')

    def test_probe_cannot_alias_a_converted_issue(self):
        with self.assertRaisesRegex(ValueError, 'probe'): self.build(probe=511)

    def test_live_common_directory_refuses(self):
        original = self.git
        with patch.object(c, 'git', side_effect=lambda root, *args:
                          original(root, *args) if Path(root) == self.linked else str(self.common)):
            with self.assertRaisesRegex(ValueError, 'live repository'): self.build()

    def test_primary_is_not_a_linked_checkout(self):
        original = self.git
        with patch.object(c, 'git', side_effect=lambda root, *args:
                          str(self.common) if '--absolute-git-dir' in args else original(root, *args)):
            with self.assertRaisesRegex(ValueError, 'genuine linked'): self.build()


if __name__ == '__main__':
    unittest.main()
