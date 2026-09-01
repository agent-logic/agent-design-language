#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUNNER="$ROOT/adl/tools/run_issue345_aws_gpu_shepherd_proof.sh"
TF_ROOT="$ROOT/infra/aws/runtime/gpu-proof"
TEST_STATE_ROOT="$ROOT/.adl/local/issue345/tests"
mkdir -p "$TEST_STATE_ROOT"
tmp="$(mktemp -d "$TEST_STATE_ROOT/issue345-two-node-contract.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT

fail() { echo "FAIL issue345-two-node-contract: $*" >&2; exit 1; }

bash -n "$RUNNER"
terraform -chdir="$TF_ROOT" validate >/dev/null

[[ "$(grep -Ec '^resource "aws_instance" "(runtime|gpu)"' "$TF_ROOT/main.tf")" == 2 ]] || fail "Terraform must own exactly two nodes"
grep -q 'profile             = var.aws_profile' "$TF_ROOT/versions.tf" || fail "Terraform provider is not pinned to the business profile"
grep -q 'var.aws_profile == "agent-logic-admin"' "$TF_ROOT/variables.tf" || fail "Terraform profile can drift to the default account"
[[ "$(grep -Ec '^resource "aws_key_pair" "operator"' "$TF_ROOT/main.tf")" == 1 ]] || fail "Terraform must own exactly one shared key pair"
[[ "$(grep -Fc 'key_name                    = aws_key_pair.operator.key_name' "$TF_ROOT/main.tf")" == 2 ]] || fail "both nodes must require the managed key pair"
[[ "$(grep -Fc 'associate_public_ip_address = true' "$TF_ROOT/main.tf")" == 2 ]] || fail "both nodes must always receive public IPv4"
[[ "$(grep -Ec 'from_port   = 22' "$TF_ROOT/main.tf")" == 2 ]] || fail "both node security groups must expose SSH"
[[ "$(grep -Fc 'cidr_blocks = [var.ssh_ingress_cidr]' "$TF_ROOT/main.tf")" == 2 ]] || fail "SSH must be restricted to the required /32"
grep -q 'from_port       = 11434' "$TF_ROOT/main.tf" || fail "GPU Ollama ingress is missing"
grep -q 'security_groups = \[aws_security_group.runtime.id\]' "$TF_ROOT/main.tf" || fail "Ollama must be SG-to-SG only"
[[ "$(grep -Fc 'AmazonSSMManagedInstanceCore' "$TF_ROOT/main.tf")" == 2 ]] || fail "both nodes need SSM recovery policy"
grep -q 'InstanceIds = \[aws_instance.runtime.id, aws_instance.gpu.id\]' "$TF_ROOT/main.tf" || fail "deadline must terminate both exact nodes"
[[ "$(grep -Fc 'delete_on_termination = true' "$TF_ROOT/main.tf")" == 2 ]] || fail "both root disks must delete on termination"
[[ "$(grep -Fc 'encrypted             = true' "$TF_ROOT/main.tf")" == 2 ]] || fail "both root disks must be encrypted"
grep -q 'replace(var.runtime_user_data, "__GPU_PRIVATE_IP__", aws_instance.gpu.private_ip)' "$TF_ROOT/main.tf" || fail "Runtime must receive the GPU private IP"
grep -q 'Resource = local.gpu_receipt_arn' "$TF_ROOT/main.tf" || fail "GPU writes are not scoped to its exact receipt"
grep -q 'Resource = local.runtime_receipt_arn' "$TF_ROOT/main.tf" || fail "Runtime writes are not scoped to its exact receipt"
grep -q 'artifact_read_arns  = \[for key in var.artifact_read_keys' "$TF_ROOT/main.tf" || fail "guest reads are not scoped to exact object keys"
grep -q '!strcontains(key, "/locks/")' "$TF_ROOT/main.tf" || fail "guest read keys can include controller lock objects"
! grep -A10 'artifact_read_statement = {' "$TF_ROOT/main.tf" | grep -q 's3:PutObject' || fail "broad artifact-prefix writes remain"
grep -q -- '--arg ready "$ready_key"' "$RUNNER" || fail "Runtime cannot read the exact GPU-ready receipt"

grep -q 'STATE_BASE=.*\.adl/local/issue345' "$RUNNER" || fail "runner state is not worktree-local"
grep -q 'validate_state_root' "$RUNNER" || fail "runner does not enforce worktree-local state containment"
! grep -q 'git-common-dir\|csdlc-v2/issue345' "$RUNNER" || fail "runner still uses Git common state"
! grep -q 'ec2 run-instances\|ssm send-command\|AWS-RunShellScript' "$RUNNER" || fail "controller still owns launch or SSM bootstrap"
! grep -q 'git clone\|git -C /opt/adl-issue345/repo fetch' "$RUNNER" || fail "guest bootstrap still depends on live Git"
grep -q 'SOURCE_ARCHIVE_PATHS=(adl adl-runtime adl-runtime-kernel adl-resilience)' "$RUNNER" || fail "exact reviewed build-source archive does not declare the required local dependency closure"
grep -q 'create_source_archive "$source_archive"' "$RUNNER" || fail "paid run does not use the reviewed source-archive helper"
grep -q 'export ADL_RUNTIME_SOURCE_REVISION="$commit"' "$RUNNER" || fail "archived Runtime validation is not bound to the authorized source revision"
grep -q 'revision=${ADL_RUNTIME_SOURCE_REVISION:-}' "$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh" || fail "Guardian validator cannot consume an archive-safe source revision"
grep -q 'ADL_RUNTIME_SOURCE_REVISION must be an exact lowercase Git commit' "$ROOT/adl/tools/validate_v092_runtime_guardian_lifecycle.sh" || fail "archive-safe source revision is not validated"
grep -q 'source_archive' "$RUNNER" || fail "versioned source archive is not bound into guest configuration"
grep -q 's3 cp "$file" "s3://$ARTIFACT_BUCKET/$key" --only-show-errors' "$RUNNER" || fail "large run artifacts do not use the AWS CLI multipart transfer path"
[[ "$(grep -Fc -- "--if-none-match '*'" "$RUNNER")" -ge 6 ]] || fail "locks, authorization, and guest receipts must be create-only"
grep -q 'terraform .* plan' "$RUNNER" || fail "Terraform plan is missing"
grep -q 'terraform .* apply' "$RUNNER" || fail "Terraform apply is missing"
grep -q 'terraform .* destroy' "$RUNNER" || fail "Terraform destroy is missing"
grep -q 'gpu-ready.json' "$RUNNER" || fail "GPU readiness receipt is missing"
grep -q 'runtime-final.json' "$RUNNER" || fail "Runtime final receipt is missing"
grep -q 'runtime_v3_to_ollama_transit_proved:false' "$RUNNER" || fail "Runtime-v3 transit non-claim is missing"
grep -q 'systemd-run.*adl-issue345-deadline' "$RUNNER" || fail "guest deadline shutdown is missing"
! grep -q 'apt-get install -y -qq awscli' "$RUNNER" || fail "known-broken Ubuntu apt awscli path returned"
grep -q 'snap install aws-cli --classic' "$RUNNER" || fail "cloud-init must install AWS CLI through the available package manager"
grep -q 'terraform_source_sha256' "$RUNNER" || fail "Terraform source identity is not bound"
grep -q 'terraform_plan_sha256' "$RUNNER" || fail "saved Terraform plan digest is not retained"
grep -q 'review_state_sha256' "$RUNNER" || fail "typed review and design state are not authorization-bound"
grep -q 'verify_public_subnet' "$RUNNER" || fail "public-subnet route and network-ACL preflight is missing"
grep -q 'route_table_sha256' "$RUNNER" || fail "public route-table identity is not authorization-bound"
grep -q 'network_acl_sha256' "$RUNNER" || fail "network-ACL identity is not authorization-bound"
grep -q 'socat TCP-LISTEN:11434,bind=127.0.0.1' "$RUNNER" || fail "private GPU forwarding for the local Shepherd contract is missing"
grep -q -- '--task-panel /opt/adl-issue345/repo/adl/tools/issue268_runtime_uts_task_panel.json' "$RUNNER" || fail "six-agent task panel is not explicitly rooted in the restored repository"
grep -q 'remote_runner=/opt/adl-issue345/repo/adl/tools/run-six-resident-remote.py' "$RUNNER" || fail "six-agent runner does not retain its repository-relative root"
grep -q 'verify_resolved_preflight_inputs "$preflight_json"' "$RUNNER" || fail "apply inputs are not rechecked against authorized preflight"
grep -q 'adl.issue345.local_recovery.v1' "$RUNNER" || fail "durable local owner/lock recovery record is missing"
grep -q 'chmod 0600 "$run_dir/recovery.json"' "$RUNNER" || fail "local recovery record is not mode 0600"

printf '%s\n' 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI0123456789abcdefghijklmnopqrstuv issue345' >"$tmp/operator.pub"
export ADL_ISSUE345_STATE_ROOT="$tmp/state"
export ADL_ISSUE345_SSH_INGRESS_CIDR=203.0.113.10/32
export ADL_ISSUE345_SSH_PUBLIC_KEY_FILE="$tmp/operator.pub"
export ADL_ISSUE345_VPC_ID=vpc-0123456789abcdef0
ADL_ISSUE345_LIBRARY_MODE=1 source "$RUNNER"
if (ADL_ISSUE345_STATE_ROOT=/private/tmp/issue345 ADL_ISSUE345_LIBRARY_MODE=1 source "$RUNNER") 2>/dev/null; then fail "state-root escape passed"; fi
write_gpu_bootstrap "$tmp/gpu-bootstrap.sh"
write_runtime_bootstrap "$tmp/runtime-bootstrap.sh"
bash -n "$tmp/gpu-bootstrap.sh"
bash -n "$tmp/runtime-bootstrap.sh"
[[ "$(grep -Fc 'export HOME=/root' "$tmp/gpu-bootstrap.sh")" == 1 ]] || fail "GPU bootstrap does not define HOME"
[[ "$(grep -Fc 'Environment=HOME=/root' "$tmp/gpu-bootstrap.sh")" == 1 ]] || fail "Ollama systemd service does not define HOME"
[[ "$(grep -Fc 'export HOME=/root' "$tmp/runtime-bootstrap.sh")" == 1 ]] || fail "Runtime bootstrap does not define HOME"
write_user_data "$tmp/gpu-user-data.sh" script-key script-version "$(printf 'a%.0s' {1..64})" config-key config-version "$(printf 'b%.0s' {1..64})" ready-key ready-key
write_user_data "$tmp/runtime-user-data.sh" script-key script-version "$(printf 'a%.0s' {1..64})" config-key config-version "$(printf 'b%.0s' {1..64})" ready-key final-key __GPU_PRIVATE_IP__
bash -n "$tmp/gpu-user-data.sh"
bash -n "$tmp/runtime-user-data.sh"
[[ "$(grep -Fc 'systemctl enable --now' "$tmp/gpu-user-data.sh")" -ge 3 ]] || fail "GPU cloud-init does not enable SSM recovery"
[[ "$(grep -Fc 'systemctl enable --now' "$tmp/runtime-user-data.sh")" -ge 3 ]] || fail "Runtime cloud-init does not enable SSM recovery"

aws() {
  if [[ "$*" == *"s3 cp"* ]]; then return 0; fi
  if [[ "$*" == *"s3api head-object"* ]]; then printf 'None\n'; return 0; fi
  return 2
}
if upload_versioned "$tmp/operator.pub" runs/no-version; then fail "multipart upload without an immutable S3 VersionId passed"; fi
unset -f aws

head="$(git -C "$ROOT" rev-parse HEAD)"
SOURCE_COMMIT="$head"
create_source_archive "$tmp/source.tar"
mkdir -p "$tmp/source"
tar -xf "$tmp/source.tar" -C "$tmp/source"
[[ ! -e "$tmp/source/.git" ]] || fail "source archive unexpectedly contains Git metadata"
for component in adl adl-runtime adl-runtime-kernel adl-resilience; do
  [[ -f "$tmp/source/$component/Cargo.toml" ]] || fail "source archive omitted $component/Cargo.toml"
done
while IFS= read -r -d '' manifest; do
  manifest_dir="$(dirname "$manifest")"
  while IFS= read -r relative_path; do
    [[ -e "$manifest_dir/$relative_path" ]] || fail "archived Cargo path is unresolved: ${manifest#"$tmp/source/"} -> $relative_path"
  done < <(sed -nE 's/.*path[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "$manifest")
done < <(find "$tmp/source" -name Cargo.toml -print0)
run_id=adl-issue345-contract
load_ssh_inputs
jq -n --arg commit "$head" --arg run "$run_id" --arg account "$EXPECTED_ACCOUNT_SHA256" \
  --arg cidr "$SSH_INGRESS_CIDR" --arg key_hash "$SSH_PUBLIC_KEY_SHA256" \
  --arg bucket "$ARTIFACT_BUCKET" --arg key "$ARTIFACT_MANIFEST_KEY" --arg version "$ARTIFACT_MANIFEST_VERSION_ID" --arg manifest_sha "$ARTIFACT_MANIFEST_SHA256" \
  --arg runtime "$RUNTIME_INSTANCE_TYPE" --arg gpu "$GPU_INSTANCE_TYPE" --argjson models "$MODEL_IDENTITIES_JSON" '
  {schema:"adl.issue345.paid_run_authorization.v3",authorized:true,authorization_id:"contract",
   source_commit:$commit,reviewed_revision:("git-blake3:"+$commit+":"+("0"*64)),run_id:$run,region:"us-west-2",
   runtime_instance_type:$runtime,gpu_instance_type:$gpu,model_identities:$models,
   ssh_ingress_cidr:$cidr,ssh_public_key_sha256:$key_hash,
   max_instance_seconds:3300,max_reaper_lag_seconds:300,max_billable_seconds:3600,
   max_runtime_hourly_usd:0.70,max_gpu_hourly_usd:0.85,max_combined_hourly_usd:1.55,max_total_cost_usd:20,
   cost_overheads:{runtime_gp3_gib:80,gpu_gp3_gib:200,gp3_monthly_usd_per_gib:0.08,public_ipv4_count:2,public_ipv4_hourly_usd:0.005,aws_request_overhead_usd:0.05},
   expires_epoch:(now+3600|floor),bindings:{aws_account_sha256:$account,
    artifact_manifest:{bucket:$bucket,key:$key,version_id:$version,sha256:$manifest_sha},
    runtime_ami_sha256:("1"*64),gpu_ami_sha256:("2"*64),subnet_sha256:("3"*64),vpc_sha256:("4"*64),
    route_table_sha256:("6"*64),network_acl_sha256:("7"*64),terraform_source_sha256:("5"*64),
    review_state_sha256:("8"*64)}}' >"$tmp/authorization.json"

SOURCE_COMMIT="$head"
RUN_ID="$run_id"
AUTHORIZATION_FILE="$tmp/authorization.json"
load_authorization
awk -v r="$MAX_RUNTIME_HOURLY_USD" -v g="$MAX_GPU_HOURLY_USD" -v c="$MAX_COMBINED_HOURLY_USD" 'BEGIN{exit !(r==0.70 && g==0.85 && c==1.55)}' || fail "two-node hourly authorization was not loaded"
awk -v d="$GP3_MONTHLY_USD_PER_GIB" -v i="$PUBLIC_IPV4_HOURLY_USD" -v q="$AWS_REQUEST_OVERHEAD_USD" 'BEGIN{exit !(d==0.08 && i==0.005 && q==0.05)}' || fail "authorized cost overheads were not loaded"
preflight_fixture="$(jq -n --arg account "$EXPECTED_ACCOUNT_SHA256" --arg key_hash "$SSH_PUBLIC_KEY_SHA256" --arg cidr_hash "$(sha256_text "$SSH_INGRESS_CIDR")" \
  '{account_sha256:$account,runtime_ami_sha256:("1"*64),gpu_ami_sha256:("2"*64),subnet_sha256:("3"*64),vpc_sha256:("4"*64),
    route_table_sha256:("6"*64),network_acl_sha256:("7"*64),terraform_source_sha256:("5"*64),ssh_public_key_sha256:$key_hash,ssh_ingress_cidr_sha256:$cidr_hash}')"
verify_authorized_preflight_bindings "$preflight_fixture"
if (verify_authorized_preflight_bindings "$(jq '.gpu_ami_sha256=("f"*64)' <<<"$preflight_fixture")") 2>/dev/null; then fail "GPU AMI drift passed"; fi

resolved_runtime_ami=ami-runtime
resolved_gpu_ami=ami-gpu
resolved_subnet=subnet-public
resolved_subnet_proof="$(jq -n '{route_table_sha256:("6"*64),network_acl_sha256:("7"*64)}')"
resolved_fixture="$(jq -n \
  --arg runtime "$(sha256_text "$resolved_runtime_ami")" \
  --arg gpu "$(sha256_text "$resolved_gpu_ami")" \
  --arg subnet "$(sha256_text "$resolved_subnet")" \
  --arg vpc "$(sha256_text "$VPC_ID")" \
  '{runtime_ami_sha256:$runtime,gpu_ami_sha256:$gpu,subnet_sha256:$subnet,vpc_sha256:$vpc,
    route_table_sha256:("6"*64),network_acl_sha256:("7"*64)}')"
verify_resolved_preflight_inputs "$resolved_fixture" "$resolved_runtime_ami" "$resolved_gpu_ami" "$resolved_subnet" "$resolved_subnet_proof"
if (verify_resolved_preflight_inputs "$resolved_fixture" "$resolved_runtime_ami" ami-drift "$resolved_subnet" "$resolved_subnet_proof") 2>/dev/null; then fail "post-preflight GPU AMI drift passed"; fi

revision="$(jq -r .reviewed_revision "$tmp/authorization.json")"
jq -n --arg revision "$revision" '{phase:"reviewed",review:{completed:true,findings:[],reviewed_revision:$revision}}' >"$tmp/review.json"
jq --arg review_state_sha256 "$(review_state_sha256 "$ROOT")" '.bindings.review_state_sha256=$review_state_sha256' "$tmp/authorization.json" >"$tmp/authorization-bound.json"
mv "$tmp/authorization-bound.json" "$tmp/authorization.json"
SOURCE_COMMIT="$head" AUTHORIZATION_FILE="$tmp/authorization.json" verify_review_authority "$tmp/review.json" "$ROOT" "$head"
if (SOURCE_COMMIT="$head" AUTHORIZATION_FILE="$tmp/authorization.json" verify_review_authority <(jq '.review.reviewed_revision=("git-blake3:"+("f"*40)+":"+("f"*64))' "$tmp/review.json") "$ROOT" "$head") 2>/dev/null; then fail "typed review mismatch passed"; fi
jq '.bindings.review_state_sha256=("f"*64)' "$tmp/authorization.json" >"$tmp/authorization-drift.json"
if (SOURCE_COMMIT="$head" AUTHORIZATION_FILE="$tmp/authorization-drift.json" verify_review_authority "$tmp/review.json" "$ROOT" "$head") 2>/dev/null; then fail "typed review-state drift passed"; fi

if (SSH_INGRESS_CIDR=0.0.0.0/0; load_ssh_inputs) 2>/dev/null; then fail "non-/32 SSH passed"; fi
if (SSH_PUBLIC_KEY_FILE="$tmp/missing.pub"; load_ssh_inputs) 2>/dev/null; then fail "missing public key passed"; fi

if ADL_ISSUE345_SSH_INGRESS_CIDR="$SSH_INGRESS_CIDR" ADL_ISSUE345_SSH_PUBLIC_KEY_FILE="$SSH_PUBLIC_KEY_FILE" \
  "$RUNNER" run --commit "$head" --run-id adl-issue345-no-authorization --execute >"$tmp/noauth.out" 2>"$tmp/noauth.err"; then
  fail "paid run without authorization passed"
fi
grep -q 'requires --authorization-file' "$tmp/noauth.err" || fail "missing authorization did not fail before AWS"

jq -n '{schema:"adl.issue345.two_node_runner_contract.v1",status:"pass",paid_launches:0,
  terraform_nodes:2,managed_key_pairs:1,public_ssh_cidrs:1,ollama_public:false,
  controller_ssm_bootstrap:false,real_git:true,fake_aws:false,negative_cases:24}'
