#!/usr/bin/env python3
"""PVF local_contract: actual ADL facade against synthetic owner receipt fixtures."""
import os
from pathlib import Path
import subprocess
import unittest
import test_codefriend_owner as fixtures


class PublicCodeFriendDispatch(fixtures.CodeFriendSelection):
    @classmethod
    def setUpClass(cls):
        cls.binary = Path(os.environ['ADL_PUBLIC_CLI']).resolve(strict=True)

    def invoke(self, *args, archive=fixtures.ARCHIVE, candidate=fixtures.CANDIDATE):
        environment = {k: v for k, v in os.environ.items() if not k.startswith('ADL_')}
        environment.update(ADL_CODEFRIEND_INSTALL_PREFIX=str(self.root),
                           ADL_CODEFRIEND_ARCHIVE_SHA256=archive,
                           ADL_CODEFRIEND_CANDIDATE=candidate,
                           ADL_OBSERVABILITY='0')
        return subprocess.run([str(self.binary), 'codefriend', *args], env=environment,
                              capture_output=True, text=True, timeout=5)

    def refused(self, result):
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(result.stdout, '')


if __name__ == '__main__':
    unittest.main()
