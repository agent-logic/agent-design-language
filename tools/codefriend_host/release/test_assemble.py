"""PVF deterministic packaging guard tests; fixture bytes are not executable proof."""
import os
from pathlib import Path
import tempfile
import unittest
from assemble import binary


class PackagingGuards(unittest.TestCase):
    def test_macho_wrong_arch_and_nonexecutable_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            source, target = Path(directory) / 'input', Path(directory) / 'output'
            source.write_bytes(b'not Linux' * 10)
            source.chmod(0o755)
            with self.assertRaises(ValueError):
                binary(source, target)
            elf = bytearray(64)
            elf[:6] = b'\x7fELF\x02\x01'
            elf[18:20] = (183).to_bytes(2, 'little')
            source.write_bytes(elf)
            with self.assertRaises(ValueError):
                binary(source, target)
            elf[18:20] = (62).to_bytes(2, 'little')
            source.write_bytes(elf)
            source.chmod(0o600)
            with self.assertRaises(ValueError):
                binary(source, target)
            self.assertFalse(target.exists())

    def test_elf_header_is_only_structural_copy_not_authentication(self):
        with tempfile.TemporaryDirectory() as directory:
            source, target = Path(directory) / 'input', Path(directory) / 'output'
            elf = bytearray(64)
            elf[:6] = b'\x7fELF\x02\x01'
            elf[18:20] = (62).to_bytes(2, 'little')
            source.write_bytes(elf)
            source.chmod(0o755)
            binary(source, target)
            self.assertEqual(target.read_bytes(), elf)
            # No process is started: this fixture cannot prove a Linux executable.
            link = Path(directory) / 'link'
            link.symlink_to(source)
            with self.assertRaises(ValueError):
                binary(link, Path(directory) / 'other')


if __name__ == '__main__':
    unittest.main()

class PacketIntegrity(unittest.TestCase):
    def test_changed_or_added_bytes_fail_integrity(self):
        import hashlib
        import json
        from verify_packet import verify
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data = root / 'binary'
            data.write_bytes(b'fixture')
            manifest = dict(schema='codefriend.release_preparation.v1', ready_for_activation=False,
                            adl_revision='a'*40, website_revision='b'*40,
                            files={'binary': hashlib.sha256(b'fixture').hexdigest()})
            (root / 'manifest.json').write_text(json.dumps(manifest))
            self.assertEqual(verify(root), 1)
            data.write_bytes(b'changed')
            with self.assertRaises(ValueError): verify(root)
            data.write_bytes(b'fixture')
            (root / 'extra').write_bytes(b'unknown')
            with self.assertRaises(ValueError): verify(root)

class AssemblyPipeline(unittest.TestCase):
    def test_clean_pinned_fixture_assembly_and_create_only_output(self):
        import argparse
        import contextlib
        import io
        import json
        import subprocess
        from assemble import assemble, git
        from verify_packet import verify
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def repo(name, files):
                path = root / name
                path.mkdir()
                subprocess.run(['git','init','-q',str(path)],check=True)
                git(path,'config','user.name','Fixture')
                git(path,'config','user.email','fixture@example.invalid')
                for filename, content in files.items():
                    target = path / filename
                    target.parent.mkdir(parents=True,exist_ok=True)
                    target.write_text(content)
                git(path,'add','.')
                git(path,'commit','-qm','fixture')
                return path, git(path,'rev-parse','HEAD')
            adl, adl_revision = repo('adl', {'tools/codefriend_host/idle_stop.py':'# fixture only\n',
                'tools/codefriend_host/systemd/fixture.service':'fixture\n',
                'tools/codefriend_host/terraform/main.tf':'excluded\n', '.gitignore':'private\n'})
            website, website_revision = repo('website', {'app/main.mjs':'// fixture\n',
                'package.json':'{}\n', 'package-lock.json':'{}\n', '.gitignore':'private\nnode_modules/\n'})
            (website/'private').write_text('must never copy')
            (adl/'private').write_text('must never copy')
            (website/'node_modules').mkdir()
            (website/'node_modules'/'fixture').write_text('must never infer')
            elf = bytearray(64)
            elf[:6] = b'\x7fELF\x02\x01'
            elf[18:20] = (62).to_bytes(2,'little')
            executable = root/'synthetic-elf'
            executable.write_bytes(elf)
            executable.chmod(0o755)
            output = root/'output'
            args=argparse.Namespace(adl=adl,adl_revision=adl_revision,website=website,
                website_revision=website_revision,gateway=executable,verifier=executable,output=output)
            with contextlib.redirect_stdout(io.StringIO()): assemble(args)
            self.assertTrue((output/'website/.git').is_dir())
            self.assertEqual(git(output/'website','rev-parse','HEAD'),website_revision)
            self.assertEqual(git(output/'website','remote'),'')
            self.assertTrue((output/'host/systemd/fixture.service').is_file())
            self.assertFalse((output/'host/terraform').exists())
            self.assertFalse((output/'website/private').exists())
            self.assertFalse((output/'website/node_modules').exists())
            self.assertGreater(verify(output),3)
            manifest=json.loads((output/'manifest.json').read_text())
            self.assertFalse(manifest['ready_for_activation'])
            with self.assertRaises(FileExistsError): assemble(args)
