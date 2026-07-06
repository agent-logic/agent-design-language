#!/usr/bin/env python3
import json, re, sys
from pathlib import Path

def fail(msg):
    raise SystemExit(f"validate_wp08_s3_obsmem_archive_policy: {msg}")

if len(sys.argv) != 2:
    fail("usage: validate_wp08_s3_obsmem_archive_policy.py <archive_policy_summary.json>")
path=Path(sys.argv[1])
data=json.loads(path.read_text())
text=path.read_text()
if data.get("schema")!="adl.wp08.s3_obsmem_archive_policy.v1": fail("bad schema")
if data.get("issue")!=4688 or data.get("status")!="passed": fail("bad issue/status")
if data.get("aws_profile")!="agent-logic-admin": fail("bad profile")
if data.get("aws_region")!="us-west-2": fail("bad region")
if data.get("aws_account_matches_expected") is not True: fail("account match not recorded")
if not re.fullmatch(r"[0-9a-f]{16}", data.get("aws_account_hash","")): fail("missing account hash")
if re.search(r"\b\d{12}\b", text): fail("raw account id retained")
if re.search(r"\b[0-9a-f]{64}\b", text): fail("full account digest retained")
cfg=data.get("live_configuration",{})
pab=cfg.get("public_access_block",{}).get("PublicAccessBlockConfiguration",{})
if not all(pab.get(k) is True for k in ["BlockPublicAcls","IgnorePublicAcls","BlockPublicPolicy","RestrictPublicBuckets"]): fail("public access block incomplete")
if cfg.get("versioning",{}).get("Status")!="Enabled": fail("versioning disabled")
enc=cfg.get("encryption",{}).get("ServerSideEncryptionConfiguration",{}).get("Rules",[{}])[0]
if enc.get("ApplyServerSideEncryptionByDefault",{}).get("SSEAlgorithm")!="AES256": fail("encryption not SSE-S3")
lock=cfg.get("object_lock",{}).get("ObjectLockConfiguration",{})
if lock.get("ObjectLockEnabled")!="Enabled": fail("object lock disabled")
ret=lock.get("Rule",{}).get("DefaultRetention",{})
if ret.get("Mode")!="GOVERNANCE" or ret.get("Days")!=365: fail("retention mismatch")
rules=cfg.get("lifecycle",{}).get("Rules",[])
if not rules or rules[0].get("Filter",{}).get("Prefix")!="community-memory/": fail("lifecycle prefix missing")
rule=rules[0]
if rule.get("Status")!="Enabled": fail("lifecycle rule disabled")
transitions={(item.get("Days"),item.get("StorageClass")) for item in rule.get("Transitions",[])}
if transitions!={(90,"GLACIER_IR"),(365,"DEEP_ARCHIVE")}: fail("current lifecycle transitions mismatch")
noncurrent={(item.get("NoncurrentDays"),item.get("StorageClass")) for item in rule.get("NoncurrentVersionTransitions",[])}
if noncurrent!={(30,"GLACIER_IR"),(180,"DEEP_ARCHIVE")}: fail("noncurrent lifecycle transitions mismatch")
if rule.get("AbortIncompleteMultipartUpload",{}).get("DaysAfterInitiation")!=7: fail("multipart cleanup mismatch")
print("PASS validate_wp08_s3_obsmem_archive_policy")
