#!/usr/bin/env python3
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
ISSUE = 679

candidate_artifacts = [
    ROOT / "infra" / "aws" / "observatory",
    ROOT / "docs" / "operations" / "cloud" / "aws" / "observatory",
    ROOT
    / "docs"
    / "milestones"
    / "v0.92.1"
    / "evidence"
    / "observatory"
    / "s3-deployable-observatory",
    ROOT / ".csdlc" / "evidence" / str(ISSUE),
]

existing = [str(path.relative_to(ROOT)) for path in candidate_artifacts if path.exists()]
tf_root = ROOT / "infra" / "aws" / "observatory"
main_tf = (tf_root / "main.tf").read_text() if (tf_root / "main.tf").exists() else ""
variables_tf = (tf_root / "variables.tf").read_text() if (tf_root / "variables.tf").exists() else ""
readme = (tf_root / "README.md").read_text() if (tf_root / "README.md").exists() else ""
runbook_path = ROOT / "docs" / "operations" / "cloud" / "aws" / "observatory" / "S3_CLOUDFRONT_DEPLOYMENT_RUNBOOK.md"
runbook = runbook_path.read_text() if runbook_path.exists() else ""
readback_path = tf_root / "readback.sh"
readback = readback_path.read_text() if readback_path.exists() else ""
bundle_root = ROOT / "demos" / "html-observatory"
bundle_index = (bundle_root / "index.html").read_text() if (bundle_root / "index.html").exists() else ""
bundle_app = (bundle_root / "app.js").read_text() if (bundle_root / "app.js").exists() else ""
bundle_config = (bundle_root / "runtime-v3.config.json").read_text() if (bundle_root / "runtime-v3.config.json").exists() else ""
contract = "\n".join((main_tf, variables_tf, readme, runbook, readback))

readback_commands = []
current_command = []
for line in readback.splitlines():
    if line.startswith("aws "):
        current_command = [line]
    elif current_command:
        current_command.append(line)
    if current_command and not line.rstrip().endswith("\\"):
        readback_commands.append(" ".join(part.strip().rstrip("\\").strip() for part in current_command))
        current_command = []

required_readbacks = (
    ("aws cloudfront get-distribution ", ("Distribution.{status:Status,enabled:DistributionConfig.Enabled,domain_name:DomainName,logging:DistributionConfig.Logging}",)),
    ("aws s3api get-public-access-block ", ('${site_bucket}', "PublicAccessBlockConfiguration")),
    ("aws s3api get-bucket-versioning ", ('${site_bucket}', "{status:Status}")),
    ("aws s3api list-object-versions ", ('${site_bucket}', "Versions[].{key:Key,version_id:VersionId,is_latest:IsLatest,last_modified:LastModified}")),
    ("aws s3api get-public-access-block ", ('${log_bucket}', "PublicAccessBlockConfiguration")),
    ("aws s3api get-bucket-lifecycle-configuration ", ('${log_bucket}', "Rules[].{id:ID,status:Status,expiration_days:Expiration.Days}")),
    ("aws acm list-certificates ", ("CertificateSummaryList[?DomainName==", ".{domain_name:DomainName,status:Status,type:Type,in_use:InUse}")),
    ("aws route53 list-resource-record-sets ", ("ResourceRecordSets[?Name==", ".{name:Name,type:Type,alias_dns_name:AliasTarget.DNSName}")),
)
readback_denominator_ok = len(readback_commands) == len(required_readbacks)
for command_prefix, required_tokens in required_readbacks:
    matches = [command for command in readback_commands if command.startswith(command_prefix) and all(token in command for token in required_tokens)]
    readback_denominator_ok &= len(matches) == 1
readback_denominator_ok &= all(command.count("--query ") == 1 and command.count("--output json") == 1 for command in readback_commands)

checks = {
    "has_implementation_artifact": bool(existing),
    "has_static_deployment_contract": all(
        token in main_tf
        for token in (
            'resource "aws_s3_bucket" "site"',
            'resource "aws_cloudfront_distribution" "site"',
            'resource "aws_acm_certificate" "site"',
            'resource "aws_route53_record" "site_a"',
        )
    ),
    "has_private_versioned_oac_origin": all(
        token in main_tf
        for token in (
            'resource "aws_s3_bucket_public_access_block" "site"',
            'resource "aws_s3_bucket_versioning" "site"',
            'resource "aws_cloudfront_origin_access_control" "site"',
            'identifiers = ["cloudfront.amazonaws.com"]',
        )
    ),
    "has_access_logging": all(
        token in main_tf
        for token in (
            'resource "aws_s3_bucket" "logs"',
            "logging_config",
            "aws_s3_bucket_lifecycle_configuration",
            "days = 90",
        )
    ),
    "has_cloudfront_legacy_logging_acl_guard": all(
        token in main_tf
        for token in (
            'data "aws_canonical_user_id" "current"',
            'resource "aws_s3_bucket_ownership_controls" "logs"',
            'object_ownership = "BucketOwnerPreferred"',
            'resource "aws_s3_bucket_acl" "logs"',
            "access_control_policy",
            'id   = data.aws_canonical_user_id.current.id',
            'uri  = "http://acs.amazonaws.com/groups/s3/LogDelivery"',
            'id   = "c4c1ede66af53448b93c283ce9448c4ba468c9432aa01d700d3878632f77d2d0"',
            'permission = "FULL_CONTROL"',
            'aws_s3_bucket_ownership_controls.logs',
            'aws_s3_bucket_acl.logs',
        )
    )
    and 'acl        = "log-delivery-write"' not in main_tf,
    "has_csp_or_response_headers": all(
        token in main_tf
        for token in (
            'aws_cloudfront_response_headers_policy',
            'content_security_policy',
            'connect-src ${local.connect_src}',
            'minimum_protocol_version = "TLSv1.2_2021"',
        )
    ),
    "has_agent_logic_admin_guard": 'var.aws_profile == "agent-logic-admin"' in variables_tf,
    "has_no_live_mutation_default": "explicit operator authorization" in contract.lower()
    and "deferred" in contract.lower(),
    "has_redaction_check": "never retain aws account ids" in contract.lower()
    and "tokens" in contract.lower()
    and "credentials" in contract.lower(),
    "has_rollback_or_invalidation_policy": "object versions" in contract.lower()
    and "invalidate" in contract.lower(),
    "has_canonical_hostname": 'default     = "observatory.csm.agent-logic.ai"' in variables_tf,
    "has_credential_free_origin_validation": '!strcontains(origin, "@")' in variables_tf
    and "^https://" in variables_tf
    and "^wss://" in variables_tf,
    "has_profile_gated_redacted_readback": readback_denominator_ok
    and '"${1:-}" != "--execute"' in readback
    and '"${AWS_PROFILE:-}" != "agent-logic-admin"' in readback
    and "observatory.csm.agent-logic.ai" in readback
    and "--certificate-arn" not in readback
    and "CertificateArn" not in readback,
    "has_safe_entrypoint_caching": 'data.aws_cloudfront_cache_policy.disabled.id' in main_tf
    and 'path_pattern               = "*.css"' in main_tf
    and 'path_pattern               = "*.js"' in main_tf,
    "bundle_assets_are_relative": 'href="./styles.css' in bundle_index
    and 'src="./app.js' in bundle_index
    and '"./runtime-v3.config.json"' in bundle_app,
    "bundle_has_no_localhost_dependency": "localhost" not in bundle_config
    and "127.0.0.1" not in bundle_config,
    "bundle_does_not_durably_persist_credentials": not re.search(
        r"localStorage[^\n]{0,160}adl\.runtimeV3\.observatoryToken|adl\.runtimeV3\.observatoryToken[^\n]{0,160}localStorage",
        bundle_app,
    ),
}

forbidden = re.compile(r"(?i)(AKIA[0-9A-Z]{16}|aws_secret_access_key\s*=|bearer\s+[A-Za-z0-9._-]{20,}|https?://[^\s/@]+:[^\s/@]+@)")
scanned_files = [
    p
    for base in candidate_artifacts[:2]
    if base.exists()
    for p in base.rglob("*")
    if p.is_file() and ".terraform" not in p.parts
]
scanned_files.extend(
    p for p in bundle_root.rglob("*") if p.is_file() and "tests" not in p.parts
)
checks["no_embedded_credentials"] = not any(forbidden.search(path.read_text(errors="ignore")) for path in scanned_files)

failed = [name for name, passed in checks.items() if not passed]
print(
    json.dumps(
        {
            "schema": "adl.issue_679.deployability_check.v1",
            "ok": not failed,
            "checked_paths": existing,
            "failed": failed,
        },
        indent=2,
    )
)
if failed:
    sys.exit(1)
