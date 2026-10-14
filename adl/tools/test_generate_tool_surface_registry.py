#!/usr/bin/env python3
"""Fast deterministic source-inventory contract; no product or network execution."""
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import generate_tool_surface_registry as registry


class DirectoryInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="adl-registry-directories-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.tools = self.root / "adl" / "tools"
        self.tools.mkdir(parents=True)
        self.manifest = self.root / "adl" / "Cargo.toml"
        self.manifest.write_text('[[bin]]\nname = "adl"\n', encoding="utf-8")
        (self.tools / "existing_helper.sh").write_text("#!/bin/sh\n", encoding="utf-8")
        self.globals = patch.multiple(
            registry,
            REPO_ROOT=self.root,
            TOOLS_DIR=self.tools,
            SKILLS_DIR=self.tools / "skills",
            CARGO_TOML=self.manifest,
        )
        self.globals.start()
        self.addCleanup(self.globals.stop)

    def test_empty_directory_trees_match_clean_checkout(self):
        clean = registry.build_document()
        (self.tools / "empty").mkdir()
        (self.tools / "mcp" / "midi-bridge" / "nested-empty").mkdir(parents=True)
        self.assertEqual(registry.build_document(), clean)

    def test_nested_file_keeps_directory_classification_until_removed(self):
        clean = registry.build_document()
        asset = self.tools / "assets" / "nested" / "input.json"
        asset.parent.mkdir(parents=True)
        asset.write_text("{}\n", encoding="utf-8")
        populated = registry.build_document()
        expected = (
            "| `adl/tools/assets` | tool surface | internal helper | "
            "Directory containing bounded helper assets, scripts, or proof inputs. |"
        )
        self.assertEqual(populated.count(expected), 1)
        self.assertIn(
            "| `adl/tools/existing_helper.sh` | tool surface | internal helper |",
            populated,
        )
        self.assertNotIn("`adl/tools/assets/nested`", populated)
        asset.unlink()
        self.assertEqual(registry.build_document(), clean)


if __name__ == "__main__":
    unittest.main()
