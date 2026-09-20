import json
import os
from pathlib import Path
import tempfile
import unittest
from manifest import binary_pair,digest,inspect_binary

class BinaryProof(unittest.TestCase):
    def fixture(self,root):
        built=root/'built';installed=root/'installed';built.mkdir();installed.mkdir()
        hashes={}
        for number,name in enumerate(('codefriend-server','codefriend-agent','adl')):
            header=bytearray(64);header[:6]=b'\x7fELF\x02\x01';header[18]=62
            for base in (built,installed):
                path=base/name;path.write_bytes(header+bytes([number]));path.chmod(0o755)
            hashes[name]=digest(built/name)
        return built,installed,{'binary_sha256':hashes}
    def test_matching_built_installed_and_tested_bytes(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root));self.assertEqual(binary_pair(built,installed,smoke),smoke['binary_sha256'])
    def test_installed_substitution_denied(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root))
            (installed/'codefriend-agent').write_bytes((installed/'codefriend-server').read_bytes())
            with self.assertRaises(ValueError):binary_pair(built,installed,smoke)
    def test_different_tested_bytes_denied(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root));smoke['binary_sha256']['codefriend-agent']='0'*64
            with self.assertRaises(ValueError):binary_pair(built,installed,smoke)
    def test_cli_binary_is_required(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root))
            (installed/'adl').unlink()
            with self.assertRaises(ValueError):binary_pair(built,installed,smoke)
    def test_cli_must_be_in_tested_binary_evidence(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root))
            del smoke['binary_sha256']['adl']
            with self.assertRaises(ValueError):binary_pair(built,installed,smoke)
    def test_non_amd64_and_symlink_denied(self):
        with tempfile.TemporaryDirectory() as root:
            built,installed,smoke=self.fixture(Path(root));path=built/'codefriend-agent'
            data=bytearray(path.read_bytes());data[18]=183;path.write_bytes(data)
            with self.assertRaises(ValueError):inspect_binary(path)
            link=built/'link';link.symlink_to(built/'codefriend-server')
            with self.assertRaises(ValueError):inspect_binary(link)

if __name__=='__main__':unittest.main()
