#!/usr/bin/env python3
"""PVF local_contract: deterministic runner argv/order/failure tests using fake
Cargo; small filesystem/CPU, no build/network; required public routing proof.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class PackageChecks(unittest.TestCase):
    def run_checks(self, fail=""):
        with tempfile.TemporaryDirectory(prefix="package-checks-") as directory:
            tmp = Path(directory)
            log = tmp / "calls.jsonl"
            cargo = tmp / "cargo"
            cargo.write_text("#!/usr/bin/env python3\n"
                             "import json,os,sys\n"
                             "with open(os.environ['CALL_LOG'],'a') as f: "
                             "f.write(json.dumps(sys.argv[1:])+'\\n')\n"
                             "sys.exit(23 if sys.argv[1]==os.environ.get('FAIL_COMMAND') else 0)\n")
            cargo.chmod(0o755)
            env = dict(os.environ, PATH=str(tmp) + os.pathsep + os.environ["PATH"],
                       CALL_LOG=str(log), FAIL_COMMAND=fail)
            result = subprocess.run(["bash", str(ROOT / "tools/public_adl/run_package_checks.sh")],
                                    cwd=tmp, env=env, capture_output=True, text=True)
            return result, [json.loads(line) for line in log.read_text().splitlines()]

    def test_all_public_packages_keep_their_exact_validation_arguments(self):
        result, calls = self.run_checks()
        self.assertEqual(result.returncode, 0, result.stderr)
        packages = [("adl-schema", "adl-schema"), ("adl-uts", "adl-uts"),
                    ("adl-v2/crates/adl-language", "adl-language"),
                    ("adl-v2/crates/adl-compiler", "adl-compiler"),
                    ("adl-legacy-contracts", "adl-legacy-contracts")]
        expected = []
        for path, package in packages:
            common = ["--manifest-path", path + "/Cargo.toml", "--package", package]
            expected += [["test", "--locked", *common],
                         ["fmt", *common, "--", "--check"],
                         ["clippy", "--locked", *common, "--all-targets", "--", "-D", "warnings"]]
        self.assertEqual(calls, expected)

    def test_each_failure_stops_the_remaining_commands_and_propagates_status(self):
        for command, count in [("test", 1), ("fmt", 2), ("clippy", 3)]:
            with self.subTest(command=command):
                result, calls = self.run_checks(command)
                self.assertEqual(result.returncode, 23)
                self.assertEqual(len(calls), count)
                self.assertEqual(calls[-1][0], command)


if __name__ == "__main__":
    unittest.main()
