#!/usr/bin/env python3
"""PVF: required deterministic local contract tests with synthetic evidence."""

import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest


SPEC = importlib.util.spec_from_file_location("qualification", Path(__file__).with_name("validate_v0922_runtime_qualification.py"))
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)


class QualificationContract(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="issue902-")
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

    def archive(self, issue, values):
        path = self.protected / f"evidence/{issue}/retained/qualification-authority.tar.gz"
        path.parent.mkdir(parents=True)
        members = []
        with tarfile.open(path, "w:gz") as handle:
            for name, value in values.items():
                data = json.dumps(value, sort_keys=True).encode()
                info = tarfile.TarInfo(name)
                info.size = len(data)
                handle.addfile(info, io.BytesIO(data))
                members.append({"path": name, "sha256": V.digest(data)})
        return {"path": str(path.relative_to(self.protected)), "sha256": V.digest(path.read_bytes())}, members

    def fixture(self):
        inventory = {"schema":"adl.revision_validation_comparison.v1","baseline_counts":{"test_bodies_run":0,"registered_targets":33,"enumerated_cases":10},"candidate_counts":{"test_bodies_run":0,"registered_targets":33,"enumerated_cases":14},"cases_added":["a","b","c","d"],"cases_removed":[],"targets_added":[],"targets_removed":[]}
        roles = {f"r{i}": {"pre_view": f"v{i}", "post_view": f"v{i}"} for i in range(6)}
        resident = {"issue":900,"positive":{"resident_count":6,"distinct_role_count":6,"distinct_workload_view_count":6,"pre_workloads_executed":6,"post_restore_workloads_executed":6,"workload_execution_count":12,"role_workload_bindings":roles}}
        provider = {"schema":"adl.issue901.runtime_provider_recovery.v1","runtime_identity":{"id":"r"},"runtime_identity_after":{"id":"r"},"paid_calls":0,"proxy_request_count":3,"scenarios":{"loss":{"terminal":{"status":"failed","reply_present":False}},"recovery":{"terminal":{"status":"delivered","correlation_matched":True}},"interruption":{"session_interrupted":True}},"raw_artifacts":{"checkpoint":"a"*64},"validation":{"status":"passed","errors":[]}}
        runtime = {"issue":852,"ingress_tests_passed":7,"wss_tests_passed":1,"stdout_stderr_separation":"passed","private_error_canary_absent_from_stderr":True,"diff_check":"passed","fmt":"passed","artifacts":{str(i):{"sha256":"b"*64} for i in range(6)}}
        artifacts = {}
        for issue, data in [(899,inventory),(900,resident),(901,provider),(852,runtime)]:
            p = self.root / f"evidence/{issue}.json"
            artifacts[issue] = {"path": str(p.relative_to(self.root)), "sha256": self.write(p,data)}
        signed="s"*64; plan="p"*64; dehydrated="d"*64; restored="r"*64; completed="c"*64; state="u"*64
        receipt={"status":"passed","resident_count":6,"continuation_verified":True,"dehydration_receipt_sha256":dehydrated,"restore_receipt_sha256":restored,"completion_receipt_sha256":completed,"completed_uts_state_sha256":state,"residents":[{"pre_agent_test_outcome":"executed","post_agent_test_outcome":"executed","producer":{"source_revision":"f76f0600154e2863d71f2d822311432612e881e0"}} for _ in range(6)]}
        negative={"scenario_count":7,"all_restore_denied":True,"all_no_inappropriate_effect":True,"scenarios":[{"scenario":s} for s in ["changed_signature","changed_payload","removed_resident","substituted_provider","substituted_configuration","substituted_lineage","stale_snapshot"]]}
        dehydration={"resident_count":6,"capsule_count":6,"population_sha256":signed}
        restore={"resident_count":6,"capsule_count":6,"population_sha256":signed}
        completion={"resident_count":6,"capsule_count":6,"population_sha256":signed}
        uts={"resident_count":6,"residents":{f"r{i}":{} for i in range(6)},"all_pending_empty":True,"phase":"post_complete","plan_sha256":plan,"restore_receipt_sha256":restored}
        resident["positive"].update(dehydration_receipt_sha256=dehydrated,restore_receipt_sha256=restored,completion_receipt_sha256=completed,completed_uts_state_sha256=state,signed_population_sha256=signed,materialized_plan_sha256=plan,producer_source_revision="f76f0600154e2863d71f2d822311432612e881e0")
        a900, m900 = self.archive(900, {"x/qualification-receipt.json":receipt,"x/continuity-negatives/summary.json":negative,"x/continuity-uts/dehydration.json":dehydration,"x/continuity-uts/restore.json":restore,"x/runtime-state/restore-receipt.json":completion,"x/uts-state.json":uts})
        observations={"source_revision":"q"*40,"runtime_identity":{"id":"r"},"runtime_identity_after":{"id":"r"},"scenarios":provider["scenarios"]}
        checkpoint={"schema":"adl.runtime_v3.agent_checkpoint.v1","checkpoint_digest":"k"*64}
        provider["source_revision"]="q"*40; provider["runtime_identity"]={"id":"r"}; provider["runtime_identity_after"]={"id":"r"}; provider["checkpoint"]={"binding":{"checkpoint_digest":"k"*64}}; provider["proxy_requests"]=[{"request_id":f"request-{i}","body_sha256":str(i)*64} for i in range(3)]
        proxy=[{"request_id":f"request-{i}","body_sha256":str(i)*64} for i in range(3)]
        install={"schema":"adl.runtime_v3.install_generation.v1","source_revision":"q"*40,"artifacts":{"csm":{},"guardian":{},"kernel":{}}}
        a901, m901 = self.archive(901, {"x/runtime-observations.json":observations,"x/checkpoint.json":checkpoint,"x/proxy-requests.json":proxy,"x/receipt.json":install})
        artifacts[900]["sha256"] = self.write(self.root / artifacts[900]["path"], resident)
        artifacts[901]["sha256"] = self.write(self.root / artifacts[901]["path"], provider)
        rows=[]
        for cid,(text,tdigest,source,producer) in V.CRITERIA.items():
            row={"criterion_id":cid,"criterion_text":text,"criterion_digest":tdigest,"canonical_source_issue":source,"canonical_source_revision":V.CANONICAL_SOURCE_REVISION,"producer_issue":producer,"producer_pr":V.PRODUCERS[producer][0],"producer_revision":V.PRODUCERS[producer][1],"producer_artifact":artifacts[producer],"execution_profile":"synthetic deterministic fixture","required_scenarios":V.EXPECTED_SCENARIOS[cid]}
            reviewer="fixture-reviewer"; author="fixture-author"
            review_bytes=f"independent review {producer}".encode(); evidence_digest=V.digest(review_bytes)
            review_path=self.protected/f"evidence/{producer}/retained/independent-review.md"; review_path.parent.mkdir(parents=True,exist_ok=True); review_path.write_bytes(review_bytes)
            receipt={"schema":"csdlc.v3.typed_review_receipt.v1","issue":producer,"reviewed_revision":row["producer_revision"],"expected_head_sha":row["producer_revision"],"reviewer":reviewer,"implementer":author,"evidence_digest":evidence_digest}
            rp=self.protected/f"evidence/{producer}/retained/typed-review-receipt.json"
            row["review_receipt"]={"path":str(rp.relative_to(self.protected)),"sha256":self.write(rp,receipt)}
            row["review_evidence"]={"path":str(review_path.relative_to(self.protected)),"sha256":V.digest(review_bytes),"receipt_evidence_digest":evidence_digest}
            if producer==900: row.update(protected_archive=a900,protected_members=m900)
            if producer==901: row.update(protected_archive=a901,protected_members=m901)
            digests={artifacts[producer]["sha256"]}
            if producer in (900,901):
                archive,members=(a900,m900) if producer==900 else (a901,m901)
                digests.add(archive["sha256"]); digests.update(m["sha256"] for m in members)
            digests.update([row["review_receipt"]["sha256"],row["review_evidence"]["sha256"]])
            row["independent_review"]={"result":"pass","independent":True,"reviewer":reviewer,"producer_author":author,"candidate_revision":row["producer_revision"],"unresolved_findings":[],"artifact_sha256s":sorted(digests)}
            rows.append(row)
        self.expected={}
        for row in rows:
            issue=row["producer_issue"]
            values={"producer":row["producer_artifact"]["sha256"],"review":row["review_receipt"]["sha256"],"review_evidence":row["review_evidence"]["sha256"],"receipt_evidence_digest":row["review_evidence"]["receipt_evidence_digest"]}
            if "protected_archive" in row: values["archive"]=row["protected_archive"]["sha256"]
            self.expected[issue]=values
        self.expected_members={900:{x["path"]:x["sha256"] for x in m900},901:{x["path"]:x["sha256"] for x in m901}}
        return {"schema":"adl.v0922.runtime_criterion_evidence.v1","release_authorized":False,"historical_boundary":{"historical_consumers":[522,833],"original_findings":19,"cloud_control_gaps":5,"execution_proof_gaps":2,"original_findings_newly_proved":False},"rows":rows,"residual_risks":V.EXPECTED_RISKS}

    def assert_rejected(self, mutation, code):
        value=copy.deepcopy(self.manifest); mutation(value)
        with self.assertRaisesRegex(ValueError, f"^{code}$"):
            V.validate(value,self.root,self.protected,self.expected,self.expected_members)

    def mutate_producer(self, issue, mutation, code):
        manifest=copy.deepcopy(self.manifest); expected=copy.deepcopy(self.expected)
        rows=[r for r in manifest["rows"] if r["producer_issue"]==issue]
        path=self.root/rows[0]["producer_artifact"]["path"]
        value=json.loads(path.read_text()); mutation(value)
        sha=self.write(path,value); expected[issue]["producer"]=sha
        for row in rows:
            old=row["producer_artifact"]["sha256"]; row["producer_artifact"]["sha256"]=sha
            scope=set(row["independent_review"]["artifact_sha256s"]); scope.remove(old); scope.add(sha)
            row["independent_review"]["artifact_sha256s"]=sorted(scope)
        with self.assertRaisesRegex(ValueError,f"^{code}$"):
            V.validate(manifest,self.root,self.protected,expected,self.expected_members)

    def mutate_archive_member(self, issue, suffix, mutation, code, refresh_expected=True):
        manifest=copy.deepcopy(self.manifest); expected=copy.deepcopy(self.expected); members=copy.deepcopy(self.expected_members)
        rows=[r for r in manifest["rows"] if r["producer_issue"]==issue]
        archive_path=self.protected/rows[0]["protected_archive"]["path"]
        values={}
        with tarfile.open(archive_path,"r:gz") as source:
            for item in source.getmembers():
                if item.isfile(): values[item.name]=source.extractfile(item).read()
        name=next(n for n in values if n.endswith(suffix)); payload=json.loads(values[name]); mutation(payload)
        values[name]=json.dumps(payload,sort_keys=True).encode()
        with tarfile.open(archive_path,"w:gz") as target:
            for item_name,data in values.items():
                info=tarfile.TarInfo(item_name); info.size=len(data); target.addfile(info,io.BytesIO(data))
        archive_sha=V.digest(archive_path.read_bytes()); member_sha=V.digest(values[name])
        if refresh_expected:
            expected[issue]["archive"]=archive_sha; members[issue][name]=member_sha
        for row in rows:
            old_archive=row["protected_archive"]["sha256"]; row["protected_archive"]["sha256"]=archive_sha
            old_member=next(x["sha256"] for x in row["protected_members"] if x["path"]==name)
            next(x for x in row["protected_members"] if x["path"]==name)["sha256"]=member_sha
            scope=set(row["independent_review"]["artifact_sha256s"])
            scope.discard(old_archive); scope.discard(old_member); scope.update([archive_sha,member_sha])
            row["independent_review"]["artifact_sha256s"]=sorted(scope)
        with self.assertRaisesRegex(ValueError,f"^{code}$"):
            V.validate(manifest,self.root,self.protected,expected,members)

    def test_positive_fixture(self):
        self.assertEqual(V.validate(self.manifest,self.root,self.protected,self.expected,self.expected_members)["complete"],5)

    def test_required_negative_cases(self):
        cases=[
            (lambda m:m["rows"][0].update(criterion_text="changed"),"criterion_identity"),
            (lambda m:m["rows"][0].update(criterion_digest="0"*64),"criterion_identity"),
            (lambda m:m["rows"][0].update(canonical_source_revision="stale"),"criterion_source_revision"),
            (lambda m:m["rows"][0].update(producer_revision="0"*40),"producer_revision"),
            (lambda m:m["rows"][0].update(required_scenarios=["wrong_scenario"]),"execution_profile_or_scenarios_missing"),
            (lambda m:m["rows"][0]["producer_artifact"].update(sha256="0"*64),"canonical_artifact_identity"),
            (lambda m:m["rows"][1]["protected_members"][0].update(sha256="0"*64),"protected_member_set"),
            (lambda m:m["rows"][0]["independent_review"].update(result="missing"),"independent_review_missing"),
            (lambda m:m["rows"][0]["independent_review"].update(reviewer="fixture-author"),"self_authored_approval"),
            (lambda m:m["rows"][0].update(producer_issue=900),"cross_criterion_substitution"),
            (lambda m:m["rows"].pop(),"five_row_denominator"),
            (lambda m:m.update(historical_boundary={}),"historical_boundary"),
            (lambda m:m.update(release_authorized=True),"release_authority_boundary"),
            (lambda m:m["rows"][0]["independent_review"].update(unresolved_findings=["open"]),"unresolved_review_findings"),
            (lambda m:m.update(residual_risks=[]),"residual_risk_boundary"),
            (lambda m:m["rows"][1]["protected_members"].pop(),"protected_member_set"),
            (lambda m:m["rows"][0]["review_evidence"].update(receipt_evidence_digest="0"*64),"typed_review_evidence_digest_mismatch"),
        ]
        for mutation,code in cases:
            with self.subTest(code=code): self.assert_rejected(mutation,code)

    def test_refreshed_partial_producer_outcome_is_rejected(self):
        self.mutate_producer(901,lambda d:d["scenarios"].pop("interruption"),"provider_scenario_denominator")

    def test_refreshed_execution_log_substitution_is_rejected(self):
        self.mutate_archive_member(901,"proxy-requests.json",lambda d:d[0].update(request_id="substituted"),"provider_proxy_cross_link")

    def test_refreshed_provenance_substitution_is_rejected(self):
        self.mutate_archive_member(901,"receipt.json",lambda d:d.update(source_revision="0"*40),"provider_runtime_cross_link")

    def test_coherent_synthetic_archive_cannot_replace_canonical_identity(self):
        self.mutate_archive_member(901,"runtime-observations.json",lambda d:d.update(paid_calls=999),"canonical_artifact_identity",refresh_expected=False)


if __name__ == "__main__":
    unittest.main()
