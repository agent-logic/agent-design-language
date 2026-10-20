"""PVF planning_contract: deterministic, small CPU/files, umbrella evidence gate."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parent

def read(name):
    return json.loads((ROOT / "evidence" / name).read_text())

for name, repo, issue, pr in [
    ("codefriend-120-terminal-finish.json", "agent-logic/codefriend", 120, 131),
    ("security-9-terminal-finish.json", "agent-logic/agent-logic-enterprise-security", 9, 18),
]:
    record = read(name)
    delivery = record["delivery"]
    assert delivery["status"] == "delivered", name
    assert delivery["repository"] == repo and delivery["issue"] == issue, name
    assert delivery["pull_request"] == pr and delivery["issue_closure"] == "closed", name

head = "da372408a1035ea4c71d975bf862fe3f1a8bc1f5"
ci = read("security-9-final-ci.json")
assert ci["headSha"] == head and ci["conclusion"] == "success"
assert ci["jobs"] and all(job["conclusion"] == "success" for job in ci["jobs"])
provenance = read("security-9-final-linux-provenance.json")
assert provenance["workflow_run"]["head_sha"] == head
assert provenance["workflow_run"]["id"] == 39489599848
report = read("security-9-final-linux-qualification.json")
assert report["status"] == "PASS" and report["consumer"]["status"] == "PASS"
assert report["consumer"]["case_count"] == 34
assert report["consumer"]["external_provider_calls"] == 0
assert read("security-9-terminal-finish.json")["delivery"]["head_sha"] == head
assert report["codefriend_input"] == read("codefriend-127-linux-input-pin.json")
assert report["runtime_input"] == read("runtime-evaluator-prerequisite.json")["security_consumer_qualification_pin"]
consumer = report["consumer"]
assert consumer["candidate"] == report["codefriend_input"]["source_revision"]
assert consumer["platform"] == "Linux"
assert consumer["model_loopback_calls"] == 4
EXPECTED_CASES = {
    'allowed_actual_gateway_runtime_adapter_four_loopback_calls',
    'cross_repository_before_export_zero_effects',
    'cross_tenant_before_export_zero_effects',
    'current_sequence_result_allowed',
    'declared_operation_content_deleted',
    'deleted_content_inaccessible',
    'deleted_export_inaccessible',
    'deletion_receipt_access',
    'denied_role_zero_effects',
    'empty_paired_zero_effects',
    'empty_service_zero_effects',
    'invalid_token_zero_effects',
    'live_repository_audit_enforced',
    'live_repository_enforced',
    'live_repository_export_enforced',
    'live_restore_audit_enforced',
    'live_restore_enforced',
    'live_restore_export_enforced',
    'live_revoked_audit_enforced',
    'live_revoked_enforced',
    'live_revoked_export_enforced',
    'live_tenant_audit_enforced',
    'live_tenant_enforced',
    'live_tenant_export_enforced',
    'missing_audit_key_zero_effects',
    'missing_provider_zero_effects',
    'out_of_order_refused',
    'post_delete_audit_access',
    'revoked_zero_effects',
    'scoped_redacted_signed_audit_export',
    'submission_replay_zero_additional_effects',
    'supported_markdown_export_created_for_security_fixture',
    'tampered_audit_zero_effects',
    'tampered_policy_zero_effects',
}
assert len(consumer["cases"]) == len(set(consumer["cases"])) == consumer["case_count"] == 34
assert set(consumer["cases"]) == EXPECTED_CASES
review = read("security-9-final-review-judgment.json")
assert review["verdict"] == "pass" and review["reviewed_revision"] == head
for link in re.findall(r"\]\(([^)]+)\)", (ROOT / "EXECUTION_ACCEPTANCE.md").read_text()):
    if "://" not in link:
        assert (ROOT / link.split("#")[0]).exists(), link
print("PASS: child delivery identities, final CI/report/review binding and acceptance links")
