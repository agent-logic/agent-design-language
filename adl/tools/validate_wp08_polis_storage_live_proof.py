#!/usr/bin/env python3
import json
import hashlib
import re
import sys
from pathlib import Path


def fail(msg):
    raise SystemExit(f"validate_wp08_polis_storage_live_proof: {msg}")


if len(sys.argv) != 2:
    fail("usage: validate_wp08_polis_storage_live_proof.py <polis_storage_proof_summary.json>")

path = Path(sys.argv[1])
data = json.loads(path.read_text())
text = path.read_text()
proof_dir = path.parent

if data.get("schema") != "adl.csm.polis_durable_storage_proof.v1":
    fail("bad schema")
if data.get("issue") != 4913 or data.get("status") != "passed":
    fail("bad issue/status")
if data.get("aws_profile") != "agent-logic-admin":
    fail("bad AWS profile")
if data.get("aws_region") != "us-west-2":
    fail("bad AWS region")
if data.get("aws_account_matches_expected") is not True:
    fail("account match not recorded")
if not re.fullmatch(r"[0-9a-f]{16}", data.get("aws_account_hash", "")):
    fail("missing redacted account hash")
if re.search(r"\b\d{12}\b", text):
    fail("raw account id retained")
if re.search(r"\b[0-9a-f]{64}\b", data.get("aws_account_hash", "")):
    fail("full account digest retained as account hash")
if data.get("redaction", {}).get("aws_credentials_retained") is not False:
    fail("credential redaction not proven")
if data.get("redaction", {}).get("raw_account_id_retained") is not False:
    fail("raw account id redaction not proven")
if data.get("redaction", {}).get("full_account_digest_retained") is not False:
    fail("full account digest redaction not proven")

obj = data.get("object", {})
if not obj.get("key", "").startswith("community-memory/"):
    fail("object key does not use community-memory prefix")
if not obj.get("version_id"):
    fail("missing S3 version id")
if not re.fullmatch(r"[0-9a-f]{64}", obj.get("payload_sha256", "")):
    fail("missing payload sha256")
if obj.get("payload_bytes", 0) <= 0:
    fail("payload size not recorded")
if obj.get("metadata_sha256_matches") is not True:
    fail("metadata sha256 did not match payload")
if obj.get("server_side_encryption") not in ("AES256", "aws:kms"):
    fail("unexpected or missing server-side encryption")
if obj.get("object_lock_mode") != "GOVERNANCE":
    fail("object lock governance retention missing")

restored = data.get("restored_artifact", {})
if restored.get("checksum_matches") is not True:
    fail("restore checksum did not match")
if restored.get("restored_sha256") != obj.get("payload_sha256"):
    fail("restore sha mismatch")

contract = data.get("durability_contract", {})
if "non-12-nines" not in contract.get("target_class", ""):
    fail("target class does not record non-12-nines truth")
if contract.get("artifact_taxonomy_ref") != "artifact_durability_taxonomy.json":
    fail("missing taxonomy ref")

payload_path = proof_dir / "polis_state_snapshot.json"
restore_ref = restored.get("restore_ref")
if not restore_ref:
    fail("missing restore ref")
restore_path = proof_dir / restore_ref
taxonomy_path = proof_dir / contract.get("artifact_taxonomy_ref", "")

for artifact in (payload_path, restore_path, taxonomy_path):
    if not artifact.is_file():
        fail(f"missing retained artifact: {artifact.relative_to(proof_dir)}")

payload_bytes = payload_path.read_bytes()
restored_bytes = restore_path.read_bytes()
payload_sha = hashlib.sha256(payload_bytes).hexdigest()
restored_sha = hashlib.sha256(restored_bytes).hexdigest()
if payload_sha != obj.get("payload_sha256"):
    fail("payload artifact sha mismatch")
if restored_sha != restored.get("restored_sha256"):
    fail("restored artifact sha mismatch")
if payload_bytes != restored_bytes:
    fail("restored artifact content differs from payload")

payload_data = json.loads(payload_bytes.decode("utf-8"))
if payload_data.get("schema") != "adl.csm.polis_state_storage_payload.v1":
    fail("payload artifact schema mismatch")
if payload_data.get("issue") != 4913:
    fail("payload artifact issue mismatch")
if payload_data.get("run_id") != data.get("run_id"):
    fail("payload artifact run id mismatch")

neg = data.get("negative_cases", {})
for name in ("missing_object", "corrupted_restore", "unsigned_access_denial"):
    case = neg.get(name, {})
    if case.get("status") != "passed":
        fail(f"{name} negative case did not pass")
    if case.get("raw_error_retained") is not False:
        fail(f"{name} retained raw error")

taxonomy = json.loads(taxonomy_path.read_text())
if taxonomy.get("schema") != "adl.csm.polis_artifact_durability_taxonomy.v1":
    fail("taxonomy schema mismatch")
if taxonomy.get("issue") != 4913:
    fail("taxonomy issue mismatch")
if taxonomy.get("backend", {}).get("durability_posture") != "vendor_11_nines_per_object_non_12_nines_claim":
    fail("taxonomy durability posture mismatch")
if len(taxonomy.get("artifact_classes", [])) < 6:
    fail("taxonomy artifact coverage too weak")
if len(contract.get("selected_backend_assumptions", [])) < 2:
    fail("backend assumptions too weak")
if len(contract.get("local_proof_scope", [])) < 5:
    fail("local proof scope too weak")
if not any("does not claim mathematical 12-nines" in item for item in data.get("non_claims", [])):
    fail("missing explicit 12-nines non-claim")

print("PASS validate_wp08_polis_storage_live_proof")
