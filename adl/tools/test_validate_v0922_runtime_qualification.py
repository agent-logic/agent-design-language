#!/usr/bin/env python3
"""PVF: deterministic admission tests using worktree-local synthetic evidence."""

import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest


SPEC = importlib.util.spec_from_file_location(
    "qualification", Path(__file__).with_name("validate_v0922_runtime_qualification.py"))
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)


class QualificationContract(unittest.TestCase):
    def setUp(self):
        local_temp = V.ROOT / ".csdlc/evidence/902/test-tmp"
        local_temp.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(prefix="issue902-", dir=local_temp)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.protected = self.root / "protected"
        self.protected.mkdir()
        self.manifest = self.fixture()

    @staticmethod
    def write(path, value):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value, sort_keys=True))
        return hashlib.sha256(path.read_bytes()).hexdigest()

    @staticmethod
    def add_archive(path, values):
        path.parent.mkdir(parents=True, exist_ok=True)
        members = []
        with tarfile.open(path, "w:gz") as handle:
            for name, value in values.items():
                data = value if isinstance(value, bytes) else json.dumps(value, sort_keys=True).encode()
                info = tarfile.TarInfo(name)
                info.size = len(data)
                handle.addfile(info, io.BytesIO(data))
                members.append({"path": name, "sha256": V.digest(data)})
        return V.digest(path.read_bytes()), members

    def protected_archive(self, issue, values):
        path = self.protected / f"evidence/{issue}/retained/qualification-authority.tar.gz"
        archive_sha, members = self.add_archive(path, values)
        return {"path": str(path.relative_to(self.protected)), "sha256": archive_sha}, members

    def fixture(self):
        counts = {
            "baseline": {"test_bodies_run": 0, "registered_targets": 33, "selected_targets": 33,
                         "listed_targets": 33, "ignored_cases": 2, "enumerated_cases": 10},
            "candidate": {"test_bodies_run": 0, "registered_targets": 33, "selected_targets": 33,
                          "listed_targets": 33, "ignored_cases": 2, "enumerated_cases": 14},
        }
        inventory = {
            "schema": "adl.revision_validation_comparison.v1",
            "baseline_counts": counts["baseline"], "candidate_counts": counts["candidate"],
            "cases_added": ["a", "b", "c", "d"], "cases_removed": [],
            "targets_added": [], "targets_removed": [], "profile": V.INVENTORY_PROFILE,
        }
        execution_archives = []
        self.expected_inventory_archives = {}
        for label in ("baseline", "candidate"):
            record = {
                "schema": "adl.revision_validation_inventory.v1", "status": "complete",
                "failures": [], "profile": V.INVENTORY_PROFILE, "counts": counts[label],
                "commands": [{"exit_code": 0}],
            }
            raw = json.dumps(record, sort_keys=True).encode()
            path = self.root / f"inventory/{label}.tar.gz"
            archive_sha, _ = self.add_archive(path, {"inventory.json": raw, "command.stdout": b"ok\n"})
            item = {"label": label, "path": str(path.relative_to(self.root)),
                    "sha256": archive_sha, "inventory_sha256": V.digest(raw), "file_count": 2}
            execution_archives.append(item)
            self.expected_inventory_archives[label] = {k: v for k, v in item.items() if k != "label"}

        roles = {f"r{i}": {"pre_view": f"v{i}", "post_view": f"v{i}"} for i in range(6)}
        resident = {
            "issue": 900,
            "execution_profile": {
                "authorization": "operator-approved bounded local production-provider qualification",
                "remote_or_paid_calls": 0, "task_owned_service_stopped": True,
                "ollama_url": "loopback task-owned port 11436",
            },
            "positive": {
                "resident_count": 6, "distinct_role_count": 6, "distinct_workload_view_count": 6,
                "pre_workloads_executed": 6, "post_restore_workloads_executed": 6,
                "workload_execution_count": 12, "role_workload_bindings": roles,
            },
        }
        provider = {
            "schema": "adl.issue901.runtime_provider_recovery.v1",
            "runtime_identity": {"id": "r"}, "runtime_identity_after": {"id": "r"},
            "registered_provider": {"provider": "openai-compatible", "model": "gemma:2b", "adapter": "http"},
            "timeout_evidence_scope": "standalone_adapter_retained",
            "paid_calls": 0, "proxy_request_count": 3,
            "scenarios": {
                "loss": {"terminal": {"status": "failed", "reply_present": False}},
                "recovery": {"terminal": {"status": "delivered", "correlation_matched": True}},
                "interruption": {"session_interrupted": True},
            },
            "raw_artifacts": {"checkpoint": "a" * 64},
            "validation": {"status": "passed", "errors": []},
        }
        runtime_payloads = {
            "wss-final.stdout": b"test result: ok. 1 passed\n",
            "wss-final.stderr": b"diagnostics\n",
            "ingress-tests.stdout": b"test result: ok. 7 passed\n",
            "ingress-tests.stderr": b"diagnostics\n",
            "lib-tests-list.txt": b"".join(f"test-{i}\n".encode() for i in range(222)),
            "governed-list.stdout": b"".join(f"operation-{i}\n".encode() for i in range(28)),
        }
        runtime = {
            "issue": 852,
            "scope": "local runtime regression; production ingress and authenticated WSS with test executor",
            "library_enumerated": 222, "governed_operations_enumerated_not_executed": 28,
            "ingress_tests_passed": 7, "wss_tests_passed": 1,
            "stdout_stderr_separation": "passed", "private_error_canary_absent_from_stderr": True,
            "diff_check": "passed", "fmt": "passed",
            "artifacts": {name: {"sha256": V.digest(value)} for name, value in runtime_payloads.items()},
        }
        runtime_path = self.root / ".csdlc/evidence/902/retained/qual-runtime-852-execution.tar.gz"
        runtime_sha, runtime_members = self.add_archive(
            runtime_path, {f".csdlc/evidence/852/{k}": v for k, v in runtime_payloads.items()})
        self.expected_runtime_archive = {
            "path": str(runtime_path.relative_to(self.root)), "sha256": runtime_sha}

        artifacts = {}
        for issue, data in [(899, inventory), (900, resident), (901, provider), (852, runtime)]:
            path = self.root / f"evidence/{issue}.json"
            artifacts[issue] = {"path": str(path.relative_to(self.root)), "sha256": self.write(path, data)}

        signed = "s" * 64
        plan = "p" * 64
        dehydrated = "d" * 64
        restored = "r" * 64
        completed = "c" * 64
        state = "u" * 64
        receipt = {
            "status": "passed", "resident_count": 6, "continuation_verified": True,
            "dehydration_receipt_sha256": dehydrated, "restore_receipt_sha256": restored,
            "completion_receipt_sha256": completed, "completed_uts_state_sha256": state,
            "residents": [{"pre_agent_test_outcome": "executed", "post_agent_test_outcome": "executed",
                           "producer": {"source_revision": "f" * 40}} for _ in range(6)],
        }
        negative = {
            "scenario_count": 7, "all_restore_denied": True, "all_no_inappropriate_effect": True,
            "scenarios": [{"scenario": s} for s in
                          ["changed_signature", "changed_payload", "removed_resident",
                           "substituted_provider", "substituted_configuration",
                           "substituted_lineage", "stale_snapshot"]],
        }
        population = {"resident_count": 6, "capsule_count": 6, "population_sha256": signed}
        uts = {"resident_count": 6, "residents": {f"r{i}": {} for i in range(6)},
               "all_pending_empty": True, "phase": "post_complete",
               "plan_sha256": plan, "restore_receipt_sha256": restored}
        resident["positive"].update(
            dehydration_receipt_sha256=dehydrated, restore_receipt_sha256=restored,
            completion_receipt_sha256=completed, completed_uts_state_sha256=state,
            signed_population_sha256=signed, materialized_plan_sha256=plan,
            producer_source_revision="f" * 40)
        a900, m900 = self.protected_archive(900, {
            "x/qualification-receipt.json": receipt,
            "x/continuity-negatives/summary.json": negative,
            "x/continuity-uts/dehydration.json": population,
            "x/continuity-uts/restore.json": population,
            "x/runtime-state/restore-receipt.json": population,
            "x/uts-state.json": uts,
        })

        provider["source_revision"] = "q" * 40
        provider["checkpoint"] = {"binding": {"checkpoint_digest": "k" * 64}}
        provider["proxy_requests"] = [
            {"request_id": f"request-{i}", "body_sha256": str(i) * 64} for i in range(3)]
        observations = {"source_revision": "q" * 40, "runtime_identity": {"id": "r"},
                        "runtime_identity_after": {"id": "r"}, "scenarios": provider["scenarios"]}
        execution = {
            "schema": "adl.issue901.execution_observations.v1",
            "scenarios": {"timeout": {
                "request_ref": "timeout-request.json", "result_ref": "timeout-result.json",
                "request_id": "issue901-timeout", "generate_forwarded": True,
                "timeout_ms": 150, "wall_elapsed_ms": 331}},
        }
        timeout_request = {"run_id": "issue901-timeout", "request_id": "issue901-timeout",
                           "attempt_policy": {"max_attempts": 1, "timeout_ms": 150}}
        timeout_result = {
            "schema_version": "provider_communication.v1", "request_id": "issue901-timeout",
            "final_status": "failed", "duration_ms": 283,
            "failure": {"kind": "provider_timeout"},
            "attempts": [{"status": "timeout", "duration_ms": 283,
                          "failure": {"kind": "provider_timeout"}}],
        }
        proxy = [{"request_id": f"request-{i}", "body_sha256": str(i) * 64} for i in range(3)]
        checkpoint = {"schema": "adl.runtime_v3.agent_checkpoint.v1", "checkpoint_digest": "k" * 64}
        install = {"schema": "adl.runtime_v3.install_generation.v1", "source_revision": "q" * 40,
                   "artifacts": {"csm": {}, "guardian": {}, "kernel": {}}}
        a901, m901 = self.protected_archive(901, {
            "x/runtime-observations.json": observations, "x/checkpoint.json": checkpoint,
            "x/proxy-requests.json": proxy, "x/receipt.json": install,
            "x/live-run-09/execution-observations.json": execution,
            "x/live-run-09/timeout-request.json": timeout_request,
            "x/live-run-09/timeout-result.json": timeout_result,
        })
        artifacts[900]["sha256"] = self.write(self.root / artifacts[900]["path"], resident)
        artifacts[901]["sha256"] = self.write(self.root / artifacts[901]["path"], provider)

        rows = []
        for criterion, (text, text_digest, source, producer) in V.CRITERIA.items():
            row = {
                "criterion_id": criterion, "criterion_text": text, "criterion_digest": text_digest,
                "canonical_source_issue": source, "canonical_source_revision": V.CANONICAL_SOURCE_REVISION,
                "producer_issue": producer, "producer_pr": V.PRODUCERS[producer][0],
                "producer_revision": V.PRODUCERS[producer][1], "producer_artifact": artifacts[producer],
                "execution_profile": V.EXPECTED_PROFILES[criterion],
                "required_scenarios": V.EXPECTED_SCENARIOS[criterion],
            }
            if producer == 899:
                row["execution_archives"] = execution_archives
            if producer == 900:
                row.update(protected_archive=a900, protected_members=m900)
            if producer == 901:
                row.update(protected_archive=a901, protected_members=m901)
            if producer == 852:
                row.update(execution_archive=self.expected_runtime_archive,
                           execution_members=runtime_members)
            reviewer, author = "fixture-reviewer", "fixture-author"
            review_bytes = f"independent review {producer}".encode()
            review_path = self.protected / f"evidence/{producer}/retained/independent-review.md"
            review_path.parent.mkdir(parents=True, exist_ok=True)
            review_path.write_bytes(review_bytes)
            evidence_digest = V.digest(review_bytes)
            receipt_data = {
                "schema": "csdlc.v3.typed_review_receipt.v1", "issue": producer,
                "reviewed_revision": row["producer_revision"], "expected_head_sha": row["producer_revision"],
                "reviewer": reviewer, "implementer": author, "evidence_digest": evidence_digest,
            }
            receipt_path = self.protected / f"evidence/{producer}/retained/typed-review-receipt.json"
            row["review_receipt"] = {"path": str(receipt_path.relative_to(self.protected)),
                                     "sha256": self.write(receipt_path, receipt_data)}
            row["review_evidence"] = {"path": str(review_path.relative_to(self.protected)),
                                      "sha256": evidence_digest,
                                      "receipt_evidence_digest": evidence_digest}
            digests = {artifacts[producer]["sha256"], row["review_receipt"]["sha256"], evidence_digest}
            if producer == 899:
                digests.update(x["sha256"] for x in execution_archives)
                digests.update(x["inventory_sha256"] for x in execution_archives)
            if producer in (900, 901):
                archive, members = (a900, m900) if producer == 900 else (a901, m901)
                digests.add(archive["sha256"])
                digests.update(x["sha256"] for x in members)
            if producer == 852:
                digests.add(self.expected_runtime_archive["sha256"])
                digests.update(x["sha256"] for x in runtime_members)
            row["independent_review"] = {
                "result": "pass", "independent": True, "reviewer": reviewer,
                "producer_author": author, "candidate_revision": row["producer_revision"],
                "unresolved_findings": [], "artifact_sha256s": sorted(digests),
            }
            rows.append(row)

        self.expected = {}
        for row in rows:
            issue = row["producer_issue"]
            values = {"producer": row["producer_artifact"]["sha256"],
                      "review": row["review_receipt"]["sha256"],
                      "review_evidence": row["review_evidence"]["sha256"],
                      "receipt_evidence_digest": row["review_evidence"]["receipt_evidence_digest"]}
            if "protected_archive" in row:
                values["archive"] = row["protected_archive"]["sha256"]
            self.expected[issue] = values
        self.expected_members = {
            900: {x["path"]: x["sha256"] for x in m900},
            901: {x["path"]: x["sha256"] for x in m901},
        }
        return {
            "schema": "adl.v0922.runtime_criterion_evidence.v1", "release_authorized": False,
            "historical_boundary": {"historical_consumers": [522, 833], "original_findings": 19,
                                    "cloud_control_gaps": 5, "execution_proof_gaps": 2,
                                    "original_findings_newly_proved": False},
            "rows": rows, "residual_risks": V.EXPECTED_RISKS,
        }

    def validate(self, manifest):
        return V.validate(manifest, self.root, self.protected, self.expected, self.expected_members,
                          self.expected_inventory_archives, self.expected_runtime_archive)

    def assert_rejected(self, mutation, code):
        value = copy.deepcopy(self.manifest)
        mutation(value)
        with self.assertRaisesRegex(ValueError, f"^{code}$"):
            self.validate(value)

    def mutate_archive_member(self, suffix, mutation, code, refresh_expected=True):
        manifest = copy.deepcopy(self.manifest)
        expected = copy.deepcopy(self.expected)
        members = copy.deepcopy(self.expected_members)
        row = next(r for r in manifest["rows"] if r["producer_issue"] == 901)
        archive_path = self.protected / row["protected_archive"]["path"]
        values = {}
        with tarfile.open(archive_path, "r:gz") as source:
            for item in source.getmembers():
                if item.isfile():
                    values[item.name] = source.extractfile(item).read()
        name = next(n for n in values if n.endswith(suffix))
        payload = json.loads(values[name])
        mutation(payload)
        values[name] = json.dumps(payload, sort_keys=True).encode()
        archive_sha, _ = self.add_archive(archive_path, values)
        member_sha = V.digest(values[name])
        if refresh_expected:
            expected[901]["archive"] = archive_sha
            members[901][name] = member_sha
        old_archive = row["protected_archive"]["sha256"]
        old_member = next(x["sha256"] for x in row["protected_members"] if x["path"] == name)
        row["protected_archive"]["sha256"] = archive_sha
        next(x for x in row["protected_members"] if x["path"] == name)["sha256"] = member_sha
        scope = set(row["independent_review"]["artifact_sha256s"])
        scope.discard(old_archive)
        scope.discard(old_member)
        scope.update([archive_sha, member_sha])
        row["independent_review"]["artifact_sha256s"] = sorted(scope)
        with self.assertRaisesRegex(ValueError, f"^{code}$"):
            V.validate(manifest, self.root, self.protected, expected, members,
                       self.expected_inventory_archives, self.expected_runtime_archive)

    def mutate_producer(self, issue, mutation, code):
        manifest = copy.deepcopy(self.manifest)
        expected = copy.deepcopy(self.expected)
        rows = [row for row in manifest["rows"] if row["producer_issue"] == issue]
        path = self.root / rows[0]["producer_artifact"]["path"]
        value = json.loads(path.read_text())
        mutation(value)
        sha = self.write(path, value)
        expected[issue]["producer"] = sha
        for row in rows:
            old = row["producer_artifact"]["sha256"]
            row["producer_artifact"]["sha256"] = sha
            scope = set(row["independent_review"]["artifact_sha256s"])
            scope.remove(old)
            scope.add(sha)
            row["independent_review"]["artifact_sha256s"] = sorted(scope)
        with self.assertRaisesRegex(ValueError, f"^{code}$"):
            V.validate(manifest, self.root, self.protected, expected, self.expected_members,
                       self.expected_inventory_archives, self.expected_runtime_archive)

    def test_positive_fixture(self):
        self.assertEqual(self.validate(self.manifest)["complete"], 5)

    def test_required_negative_cases(self):
        cases = [
            (lambda m: m["rows"][0].update(criterion_text="changed"), "criterion_identity"),
            (lambda m: m["rows"][0].update(criterion_digest="0" * 64), "criterion_identity"),
            (lambda m: m["rows"][0].update(canonical_source_revision="stale"), "criterion_source_revision"),
            (lambda m: m["rows"][0].update(producer_revision="0" * 40), "producer_revision"),
            (lambda m: m["rows"][0].update(required_scenarios=["wrong"]), "required_scenarios_mismatch"),
            (lambda m: m["rows"][0].update(execution_profile="synthetic metadata only; no Runtime execution"),
             "execution_profile_mismatch"),
            (lambda m: m["rows"][0]["producer_artifact"].update(sha256="0" * 64),
             "canonical_artifact_identity"),
            (lambda m: m["rows"][1]["protected_members"].pop(), "protected_member_set"),
            (lambda m: m["rows"][1]["protected_members"][0].update(sha256="0" * 64),
             "protected_member_set"),
            (lambda m: m["rows"][0]["independent_review"].update(result="missing"),
             "independent_review_missing"),
            (lambda m: m["rows"][0]["independent_review"].update(reviewer="fixture-author"),
             "self_authored_approval"),
            (lambda m: m["rows"][0].update(producer_issue=900), "cross_criterion_substitution"),
            (lambda m: m["rows"].pop(), "five_row_denominator"),
            (lambda m: m.update(historical_boundary={}), "historical_boundary"),
            (lambda m: m.update(release_authorized=True), "release_authority_boundary"),
            (lambda m: m["rows"][0]["independent_review"].update(unresolved_findings=["open"]),
             "unresolved_review_findings"),
            (lambda m: m.update(residual_risks=[]), "residual_risk_boundary"),
            (lambda m: m["rows"][0]["review_evidence"].update(receipt_evidence_digest="0" * 64),
             "typed_review_evidence_digest_mismatch"),
        ]
        for mutation, code in cases:
            with self.subTest(code=code):
                self.assert_rejected(mutation, code)

    def test_missing_inventory_execution_archive_is_rejected(self):
        row = next(r for r in self.manifest["rows"] if r["producer_issue"] == 899)
        (self.root / row["execution_archives"][0]["path"]).unlink()
        with self.assertRaisesRegex(ValueError, "^execution_archive_unavailable$"):
            self.validate(self.manifest)

    def test_missing_runtime_execution_archive_is_rejected(self):
        row = next(r for r in self.manifest["rows"] if r["producer_issue"] == 852)
        (self.root / row["execution_archive"]["path"]).unlink()
        with self.assertRaisesRegex(ValueError, "^execution_archive_unavailable$"):
            self.validate(self.manifest)

    def test_review_evidence_path_escape_is_rejected(self):
        manifest = copy.deepcopy(self.manifest)
        row = manifest["rows"][0]
        source = self.protected / row["review_evidence"]["path"]
        escaped = self.root / "escaped-review.md"
        escaped.write_bytes(source.read_bytes())
        row["review_evidence"]["path"] = "../escaped-review.md"
        with self.assertRaisesRegex(ValueError, "^protected_review_evidence_path$"):
            self.validate(manifest)

    def test_timeout_outcome_substitution_is_rejected(self):
        self.mutate_archive_member(
            "timeout-result.json",
            lambda data: data["failure"].update(kind="provider_error"),
            "provider_timeout_outcomes")

    def test_timeout_deadline_substitution_is_rejected(self):
        self.mutate_archive_member(
            "timeout-request.json",
            lambda data: data["attempt_policy"].update(timeout_ms=999),
            "provider_timeout_deadline_binding")

    def test_partial_provider_outcome_is_rejected_after_hash_refresh(self):
        self.mutate_producer(
            901, lambda data: data["scenarios"].pop("interruption"),
            "provider_scenario_denominator")

    def test_runtime_profile_is_bound_to_producer_after_hash_refresh(self):
        self.mutate_producer(
            852, lambda data: data.update(scope="synthetic metadata only; no Runtime execution"),
            "runtime_execution_profile")

    def test_execution_log_substitution_is_rejected_after_hash_refresh(self):
        self.mutate_archive_member(
            "proxy-requests.json",
            lambda data: data[0].update(request_id="substituted"),
            "provider_proxy_cross_link")

    def test_install_provenance_substitution_is_rejected_after_hash_refresh(self):
        self.mutate_archive_member(
            "receipt.json",
            lambda data: data.update(source_revision="0" * 40),
            "provider_runtime_cross_link")

    def test_coherent_archive_cannot_replace_canonical_identity(self):
        self.mutate_archive_member(
            "runtime-observations.json",
            lambda data: data.update(paid_calls=999),
            "canonical_artifact_identity",
            refresh_expected=False)

    def test_late_failure_preserves_prior_row_results(self):
        manifest = copy.deepcopy(self.manifest)
        manifest["rows"][-1]["criterion_text"] = "changed"
        try:
            self.validate(manifest)
        except V.AdmissionError as error:
            report = V.failure_report(error)
        else:
            self.fail("late invalid row accepted")
        self.assertEqual(report["complete"], 4)
        self.assertEqual(report["missing"], 0)
        self.assertEqual([r["status"] for r in report["rows"]],
                         ["pass", "pass", "pass", "pass", "fail"])


if __name__ == "__main__":
    unittest.main()
