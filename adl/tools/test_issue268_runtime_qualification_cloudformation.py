#!/usr/bin/env python3
"""Focused static contract for the issue #268 CloudFormation host."""
from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[2]
TEMPLATE = ROOT / "adl/tools/issue268_runtime_qualification.cloudformation.yaml"
text = TEMPLATE.read_text()


def require(fragment: str) -> None:
    if fragment not in text:
        raise SystemExit(f"missing CloudFormation contract: {fragment}")


require("InstanceType: r7i.2xlarge")
require("adl:purchase_option")
require("Value: on_demand")
require("Type: AWS::EC2::Volume")
require("DeletionPolicy: Retain")
require("UpdateReplacePolicy: Retain")
require("SnapshotId: !Ref RuntimeSnapshotId")
require("ADL_RUNTIME_CONTINUITY_ROOT=/opt/adl-runtime/runtime")
require("ADL_ISSUE268_BUILD_CACHE_ROOT=/opt/adl-build-cache")
require("OLLAMA_MODELS=/opt/adl-runtime/runtime/install/current/ollama-models")
require("test -d /opt/adl-runtime/runtime/install")
require("dnf install -y gcc gcc-c++ make")
require("systemctl enable --now amazon-ssm-agent")
require("systemctl start --no-block adl-issue268-runtime-volume.service")
require('lsblk -ndo NAME,SERIAL')
require('Restart=on-failure')
require('StartLimitIntervalSec=0')
require('touch /var/lib/adl/issue268-bootstrap-ready')
require('stack completion alone is insufficient')
require('for attempt in $(seq 1 450); do')
require('journalctl -u adl-issue268-runtime-volume.service')
require("s3:GetObjectVersion")
require("HttpTokens: required")
require("Type: AWS::EC2::SecurityGroup")
require("VpcId: !Ref VpcId")
require("SecurityGroupIngress: []")
require("SecurityGroupIds: [!Ref RuntimeSecurityGroup]")

for forbidden in (
    "MarketOptions",
    "SpotOptions",
    "KeyName:",
    "SecurityGroupId:",
    "AWS::CertificateManager",
    "CertificateArn",
    "TlsCertificate",
    "TlsPrivateKey",
    "ruby",
    "gem install",
    "RuntimeVolumeSizeGiB",
    "/dev/disk/by-id/nvme-Amazon_Elastic_Block_Store",
):
    if forbidden in text:
        raise SystemExit(f"forbidden CloudFormation contract: {forbidden}")

if text.splitlines().count("    Type: AWS::EC2::Instance") != 1:
    raise SystemExit("template must create exactly one EC2 instance")
if text.splitlines().count("    Type: AWS::EC2::Volume") != 1:
    raise SystemExit("template must create exactly one retained Runtime volume")
if "/opt/adl-runtime" == "/opt/adl-build-cache":
    raise SystemExit("Runtime and build cache roots must remain separate")

print("issue268_runtime_qualification_cloudformation: PASS")
