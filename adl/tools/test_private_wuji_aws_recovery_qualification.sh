#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE_ROOT="${REPO_ROOT}/.csdlc/evidence/194/qualification-fixtures"
QUOTA_RUN_ROOT="${REPO_ROOT}/.csdlc/evidence/194/live-runs/issue-194-dryrun-fixture"
ACCOUNT_ID="123456789012"
ACCOUNT_SHA="$(python3 - "${ACCOUNT_ID}" <<'PY'
import hashlib
import sys
print(hashlib.sha256(sys.argv[1].encode()).hexdigest())
PY
)"

cd "${REPO_ROOT}"
rm -rf "${FIXTURE_ROOT}"
rm -rf "${QUOTA_RUN_ROOT}"
trap 'rm -rf "${FIXTURE_ROOT}" "${QUOTA_RUN_ROOT}"' EXIT
mkdir -p "${FIXTURE_ROOT}"

python3 - "${FIXTURE_ROOT}" "${ACCOUNT_SHA}" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
account_sha = sys.argv[2]
plan = {
    "schema": "adl.issue194.private_wuji_aws_recovery.plan.v1",
    "issue": 194,
    "run_id": "issue-194-dryrun-fixture",
    "region": "us-west-2",
    "profile": "agent-logic-admin",
    "expected_account_id_sha256": account_sha,
    "ttl_minutes": 30,
    "allow_public_runtime_exposure": False,
    "allow_hosted_model_fallback": False,
    "wuji_voter": {
        "node_id": "wuji-voter-1",
        "cleanup_receipt_path": ".csdlc/evidence/194/wuji-cleanup.json",
    },
    "shepherd": {
        "node_id": "aws-shepherd",
        "non_voting": True,
        "cannot_mint_authority": True,
    },
    "aws_voters": [
        {
            "node_id": "aws-voter-a",
            "az": "us-west-2a",
            "subnet_id": "subnet-private-a",
            "security_group_id": "sg-private",
            "ami_id": "ami-1234567890abcdef0",
            "instance_type": "m7i.large",
            "instance_profile_name": "ADLRemoteValidationPermanentProfile",
            "model_profile": {"provider": "ollama_http", "hosted_fallback": False},
        },
        {
            "node_id": "aws-voter-b",
            "az": "us-west-2b",
            "subnet_id": "subnet-private-b",
            "security_group_id": "sg-private",
            "ami_id": "ami-1234567890abcdef0",
            "instance_type": "m7i.large",
            "instance_profile_name": "ADLRemoteValidationPermanentProfile",
            "model_profile": {"provider": "ollama_http", "hosted_fallback": False},
        },
    ],
}
inventory = {
    "account_identity": {"Account": "123456789012"},
    "subnets": [
        {"SubnetId": "subnet-private-a", "VpcId": "vpc-private", "AvailabilityZone": "us-west-2a", "MapPublicIpOnLaunch": False, "State": "available"},
        {"SubnetId": "subnet-private-b", "VpcId": "vpc-private", "AvailabilityZone": "us-west-2b", "MapPublicIpOnLaunch": False, "State": "available"},
    ],
    "security_groups": [
        {
            "GroupId": "sg-private",
            "GroupName": "issue-194-private",
            "VpcId": "vpc-private",
            "IpPermissions": [
                {"IpProtocol": "-1", "UserIdGroupPairs": [{"GroupId": "sg-private"}]}
            ],
            "IpPermissionsEgress": [
                {"IpProtocol": "-1", "UserIdGroupPairs": [{"GroupId": "sg-private"}]},
                {"IpProtocol": "tcp", "FromPort": 443, "ToPort": 443, "UserIdGroupPairs": [{"GroupId": "sg-endpoint"}]}
            ],
            "Tags": []
        }
    ],
    "instance_profiles": [
        {"InstanceProfileName": "ADLRemoteValidationPermanentProfile"}
    ],
    "instances": [],
    "volumes": [],
    "network_interfaces": [],
}
(root / "plan.json").write_text(json.dumps(plan, indent=2, sort_keys=True))
(root / "inventory.json").write_text(json.dumps(inventory, indent=2, sort_keys=True))
(root / "inventory-private-baseline.json").write_text(json.dumps(inventory, indent=2, sort_keys=True))
PY

python3 adl/tools/private_wuji_aws_recovery_qualification.py \
  --plan "${FIXTURE_ROOT}/plan.json" \
  --inventory "${FIXTURE_ROOT}/inventory.json" \
  --out "${FIXTURE_ROOT}/receipt.json"

python3 - "${FIXTURE_ROOT}/receipt.json" <<'PY'
import json
import pathlib
import sys

receipt = json.loads(pathlib.Path(sys.argv[1]).read_text())
assert receipt["status"] == "dry_run_preflight_passed"
assert len(receipt["aws_voters"]) == 2
assert "713332525889" not in json.dumps(receipt)
assert "<redacted-subnet>" in receipt["aws_voters"][0]["aws_cli_preview"]
PY

python3 - "${FIXTURE_ROOT}/inventory.json" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
inventory = json.loads(path.read_text())
inventory["subnets"][0]["MapPublicIpOnLaunch"] = True
path.write_text(json.dumps(inventory, indent=2, sort_keys=True))
PY

if python3 adl/tools/private_wuji_aws_recovery_qualification.py \
  --plan "${FIXTURE_ROOT}/plan.json" \
  --inventory "${FIXTURE_ROOT}/inventory.json" \
  --out "${FIXTURE_ROOT}/public-subnet-failure.json"; then
  echo "expected public subnet fixture to fail" >&2
  exit 1
fi

python3 - "${FIXTURE_ROOT}" "${ACCOUNT_SHA}" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
account_sha = sys.argv[2]
plan = json.loads((root / "plan.json").read_text())
inventory = json.loads((root / "inventory.json").read_text())
inventory["subnets"][0]["MapPublicIpOnLaunch"] = False
inventory["security_groups"][0]["IpPermissions"] = [
    {"IpProtocol": "-1", "UserIdGroupPairs": [{"GroupId": "sg-private"}]},
    {"IpProtocol": "tcp", "FromPort": 22, "ToPort": 22, "IpRanges": [{"CidrIp": "0.0.0.0/0"}]},
]
(root / "inventory-public-ingress.json").write_text(json.dumps(inventory, indent=2, sort_keys=True))
PY

if python3 adl/tools/private_wuji_aws_recovery_qualification.py \
  --plan "${FIXTURE_ROOT}/plan.json" \
  --inventory "${FIXTURE_ROOT}/inventory-public-ingress.json" \
  --out "${FIXTURE_ROOT}/public-ingress-failure.json"; then
  echo "expected public ingress fixture to fail" >&2
  exit 1
fi

python3 - "${FIXTURE_ROOT}/plan.json" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
plan = json.loads(path.read_text())
plan["ttl_minutes"] = 180
path.write_text(json.dumps(plan, indent=2, sort_keys=True))
PY

if python3 adl/tools/private_wuji_aws_recovery_qualification.py \
  --plan "${FIXTURE_ROOT}/plan.json" \
  --inventory "${FIXTURE_ROOT}/inventory-private-baseline.json" \
  --out "${FIXTURE_ROOT}/ttl-failure.json"; then
  echo "expected excessive TTL fixture to fail" >&2
  exit 1
fi

python3 - "${FIXTURE_ROOT}/plan.json" "${FIXTURE_ROOT}/inventory-private-baseline.json" <<'PY'
import json
import pathlib
import sys

plan_path = pathlib.Path(sys.argv[1])
inventory_path = pathlib.Path(sys.argv[2])
plan = json.loads(plan_path.read_text())
plan["ttl_minutes"] = 30
plan_path.write_text(json.dumps(plan, indent=2, sort_keys=True))
inventory = json.loads(inventory_path.read_text())
inventory["instances"] = [
    {
        "InstanceId": "i-terminated-old",
        "State": {"Name": "terminated"},
        "Tags": [{"Key": "adl:issue", "Value": "194"}, {"Key": "adl:run_id", "Value": "issue-194-old"}],
    }
]
inventory_path.write_text(json.dumps(inventory, indent=2, sort_keys=True))
PY

python3 adl/tools/private_wuji_aws_recovery_qualification.py \
  --plan "${FIXTURE_ROOT}/plan.json" \
  --inventory "${FIXTURE_ROOT}/inventory-private-baseline.json" \
  --out "${FIXTURE_ROOT}/terminated-history-receipt.json"

if ADL_ISSUE194_INSTANCE_TYPE_VCPUS=4 \
  ADL_ISSUE194_INSTANCE_TYPE_GPU_MEMORY_MIB=22888 \
  ADL_ISSUE194_QUOTA_PRECHECK_VCPUS=4 \
  bash adl/tools/issue194_private_wuji_aws_runner.sh quota-preflight issue-194-dryrun-fixture g6.xlarge 2 \
  >"${FIXTURE_ROOT}/quota-two-gpu.out" 2>"${FIXTURE_ROOT}/quota-two-gpu.err"; then
  echo "expected two g6.xlarge voters to fail under a 4 vCPU G-family quota" >&2
  exit 1
fi
grep -F "required 8 vCPUs for 2 voter(s), quota 4" "${FIXTURE_ROOT}/quota-two-gpu.err" >/dev/null

ADL_ISSUE194_INSTANCE_TYPE_VCPUS=4 \
  ADL_ISSUE194_INSTANCE_TYPE_GPU_MEMORY_MIB=22888 \
  ADL_ISSUE194_QUOTA_PRECHECK_VCPUS=4 \
  bash adl/tools/issue194_private_wuji_aws_runner.sh quota-preflight issue-194-dryrun-fixture g6.xlarge 1

python3 - "${QUOTA_RUN_ROOT}/quota-preflight.redacted.json" <<'PY'
import json
import pathlib
import sys

receipt = json.loads(pathlib.Path(sys.argv[1]).read_text())
assert receipt["status"] == "passed"
assert receipt["instance_type"] == "g6.xlarge"
assert receipt["required_vcpus"] == 4
assert receipt["quota_vcpus"] == 4
assert receipt["model_capable_gpu"] is True
assert "713332525889" not in json.dumps(receipt)
PY

if ADL_ISSUE194_INSTANCE_TYPE_VCPUS=2 \
  ADL_ISSUE194_INSTANCE_TYPE_GPU_MEMORY_MIB=2861 \
  ADL_ISSUE194_QUOTA_PRECHECK_VCPUS=4 \
  bash adl/tools/issue194_private_wuji_aws_runner.sh quota-preflight issue-194-dryrun-fixture g6f.large 2 \
  >"${FIXTURE_ROOT}/quota-tiny-gpu.out" 2>"${FIXTURE_ROOT}/quota-tiny-gpu.err"; then
  echo "expected two tiny GPU voters to fail the model-health memory gate" >&2
  exit 1
fi
grep -F "GPU memory 2861 MiB is below model-health minimum 16000 MiB" "${FIXTURE_ROOT}/quota-tiny-gpu.err" >/dev/null

ADL_ISSUE194_GPU_FEASIBILITY_INSTANCE_TYPES_JSON='[
  {"Type":"g6f.large","VCpus":2,"MemoryMiB":8192,"Gpus":[{"Name":"L4","MemoryMiB":2861}]},
  {"Type":"g6.xlarge","VCpus":4,"MemoryMiB":16384,"Gpus":[{"Name":"L4","MemoryMiB":22888}]},
  {"Type":"p5.4xlarge","VCpus":16,"MemoryMiB":262144,"Gpus":[{"Name":"H100","MemoryMiB":81920}]}
]' \
  ADL_ISSUE194_GVT_QUOTA_VCPUS=4 \
  ADL_ISSUE194_P_QUOTA_VCPUS=8 \
  bash adl/tools/issue194_private_wuji_aws_runner.sh gpu-feasibility issue-194-dryrun-fixture 2 \
  >"${FIXTURE_ROOT}/gpu-feasibility.out" 2>"${FIXTURE_ROOT}/gpu-feasibility.err" && {
    echo "expected GPU feasibility to fail when only fractional G6 fits quota" >&2
    exit 1
  }
grep -F "no current model-capable GPU instance type fits" "${FIXTURE_ROOT}/gpu-feasibility.err" >/dev/null

python3 - "${QUOTA_RUN_ROOT}/gpu-feasibility.redacted.json" <<'PY'
import json
import pathlib
import sys

receipt = json.loads(pathlib.Path(sys.argv[1]).read_text())
assert receipt["status"] == "failed_no_feasible_two_voter_model_shape"
assert receipt["feasible_model_capable_options"] == []
assert receipt["quota_fit_but_model_insufficient_options"][0]["instance_type"] == "g6f.large"
assert receipt["quota_fit_but_model_insufficient_options"][0]["model_capable_gpu"] is False
assert receipt["lowest_vcpu_model_capable_options"][0]["instance_type"] == "g6.xlarge"
assert receipt["lowest_vcpu_model_capable_options"][0]["quota_ready"] is False
assert "713332525889" not in json.dumps(receipt)
PY

if bash adl/tools/issue194_private_wuji_aws_runner.sh serial-hybrid-recovery issue-194-dryrun-fixture \
  >"${FIXTURE_ROOT}/serial-missing-wuji.out" 2>"${FIXTURE_ROOT}/serial-missing-wuji.err"; then
  echo "expected serial hybrid recovery to require a Wuji receipt" >&2
  exit 1
fi
grep -F "ADL_ISSUE194_WUJI_RECEIPT is required" "${FIXTURE_ROOT}/serial-missing-wuji.err" >/dev/null

python3 - "${FIXTURE_ROOT}" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
(root / "wuji-clean.json").write_text(json.dumps({
    "status": "passed",
    "snapshot_recovery": True,
    "partition_proof": True,
    "cleanup_zero": True,
}, indent=2, sort_keys=True) + "\n")
(root / "wuji-structured.json").write_text(json.dumps({
    "schema": "adl.issue194.wuji_recovery_receipt.v1",
    "issue": 194,
    "source_host": "wuji",
    "status": "passed",
    "snapshot_recovery": True,
    "partition_proof": True,
    "cleanup_zero": True,
    "phases": {
        "snapshot_recovery": True,
        "partition_isolation": True,
        "cleanup_zero": True,
    },
    "evidence": {
        "runtime_semantic_test": "three_secure_voters_commit_with_two_halt_with_one_and_restart_snapshot_state",
        "snapshot_digest_sha256": "0" * 64,
        "cleanup_receipt_sha256": "1" * 64,
    },
}, indent=2, sort_keys=True) + "\n")
(root / "aws-mesh.json").write_text(json.dumps({
    "status": "passed",
    "directions": ["aws-voter-a-to-b", "aws-voter-b-to-a"],
}, indent=2, sort_keys=True) + "\n")
(root / "aws-model-one-node.json").write_text(json.dumps({
    "status": "passed",
    "model": "gemma4:12b",
    "runtime_surface": "ollama_http_loopback",
    "node_count": 1,
}, indent=2, sort_keys=True) + "\n")
PY

if ADL_ISSUE194_WUJI_RECEIPT="${FIXTURE_ROOT}/wuji-clean.json" \
  ADL_ISSUE194_AWS_MESH_RECEIPT="${FIXTURE_ROOT}/aws-mesh.json" \
  ADL_ISSUE194_AWS_MODEL_RECEIPT="${FIXTURE_ROOT}/aws-model-one-node.json" \
  bash adl/tools/issue194_private_wuji_aws_runner.sh serial-hybrid-recovery issue-194-dryrun-fixture \
  >"${FIXTURE_ROOT}/serial-loose-wuji.out" 2>"${FIXTURE_ROOT}/serial-loose-wuji.err"; then
  echo "expected loose Wuji receipt to fail the serial hybrid gate" >&2
  exit 1
fi
grep -F "Wuji receipt schema is not issue #194 v1" "${FIXTURE_ROOT}/serial-loose-wuji.err" >/dev/null

if ADL_ISSUE194_WUJI_RECEIPT="${FIXTURE_ROOT}/wuji-structured.json" \
  ADL_ISSUE194_AWS_MESH_RECEIPT="${FIXTURE_ROOT}/aws-mesh.json" \
  ADL_ISSUE194_AWS_MODEL_RECEIPT="${FIXTURE_ROOT}/aws-model-one-node.json" \
  bash adl/tools/issue194_private_wuji_aws_runner.sh serial-hybrid-recovery issue-194-dryrun-fixture \
  >"${FIXTURE_ROOT}/serial-one-gpu.out" 2>"${FIXTURE_ROOT}/serial-one-gpu.err"; then
  echo "expected split one-GPU model proof to fail the serial hybrid gate" >&2
  exit 1
fi
grep -F "requires two AWS model-health voters" "${FIXTURE_ROOT}/serial-one-gpu.err" >/dev/null

echo "#194 private Wuji/AWS qualification fixtures: PASS"
