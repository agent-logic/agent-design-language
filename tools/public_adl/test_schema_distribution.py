#!/usr/bin/env python3
"""PVF: required offline schema-distribution contract; tiny CPU/filesystem,
deterministic, no network, providers or Runtime. Anonymous HTTP is checked
separately after publication.
"""
import hashlib
import json
from pathlib import Path
import unittest
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[2]
PUBLIC = "https://raw.githubusercontent.com/agent-logic/agent-design-language/main/"


class SchemaDistribution(unittest.TestCase):
    def test_public_id_and_pinned_payload(self):
        schemas = sorted((ROOT / "adl-spec/schemas/uts").glob("*/*.json"))
        self.assertEqual(3, len(schemas))
        for schema in schemas:
            with self.subTest(schema=schema.name):
                data = schema.read_bytes()
                self.assertEqual(PUBLIC + schema.relative_to(ROOT).as_posix(),
                                 json.loads(data)["$id"])
                suffix = schema.relative_to(ROOT / "adl-spec/schemas/uts")
                self.assertEqual(data, (ROOT / "adl-uts/resources/schemas/uts" / suffix).read_bytes())

    def test_crate_embeds_only_the_pinned_schema_payload(self):
        # Same tiny offline contract/release-gate lane as public-byte parity.
        self.assertFalse((ROOT / "adl-uts" / "schemas").exists())
        source = (ROOT / "adl-uts/src/lib.rs").read_text()
        expected = sorted((ROOT / "adl-uts/resources/schemas/uts").glob("*/*.json"))
        self.assertEqual(3, len(expected))
        for schema in expected:
            relative = schema.relative_to(ROOT / "adl-uts").as_posix()
            self.assertIn('include_str!("../' + relative + '")', source)

    def test_references_are_resolvable_inside_each_schema(self):
        def references(value):
            if isinstance(value, dict):
                if "$ref" in value:
                    yield value["$ref"]
                for child in value.values():
                    yield from references(child)
            elif isinstance(value, list):
                for child in value:
                    yield from references(child)

        for schema in (ROOT / "adl-spec/schemas/uts").glob("*/*.json"):
            document = json.loads(schema.read_text())
            for ref in references(document):
                with self.subTest(schema=schema.name, ref=ref):
                    self.assertTrue(ref.startswith("#/"), ref)
                    target = document
                    for part in unquote(ref[2:]).split("/"):
                        target = target[part.replace("~1", "/").replace("~0", "~")]

    def test_pinned_resource_manifest(self):
        resources = ROOT / "adl-uts/resources"
        manifest = json.loads((resources / "RESOURCE_MANIFEST.json").read_text())
        pin = json.loads((ROOT / "adl-uts/resources-pin.json").read_text())
        for field in ["source_repository", "source_revision", "resource_version"]:
            self.assertEqual(pin[field], manifest[field])
        for member in manifest["files"]:
            with self.subTest(member=member["path"]):
                data = (resources / member["path"]).read_bytes()
                self.assertEqual(member["size"], len(data))
                self.assertEqual(member["sha256"], hashlib.sha256(data).hexdigest())


if __name__ == "__main__":
    unittest.main()
