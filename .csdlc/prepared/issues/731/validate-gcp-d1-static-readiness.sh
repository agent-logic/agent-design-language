#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

require_file() {
  test -f "$1" || fail "missing required file: $1"
}

require_text() {
  local file="$1"
  local needle="$2"
  grep -Fq "$needle" "$file" || fail "missing expected text in $file: $needle"
}

reject_text() {
  local file="$1"
  local needle="$2"
  if grep -Fq "$needle" "$file"; then
    fail "forbidden text present in $file: $needle"
  fi
}

require_file ".csdlc/issues/731/index.json"
require_file ".csdlc/issues/731/cards/stp.md"
require_file ".csdlc/issues/731/cards/sip.md"
require_file ".csdlc/issues/731/cards/spp.md"
require_file ".csdlc/issues/731/cards/vpp.md"
require_file ".csdlc/issues/731/cards/srp.md"
require_file ".csdlc/issues/731/cards/sor.md"
require_file ".csdlc/prepared/issues/731/design.md"
require_file "infra/gcp/platform/main.tf"
require_file "infra/gcp/platform/variables.tf"
require_file "infra/gcp/platform/terraform.tfvars.example"

require_text ".csdlc/prepared/issues/731/design.md" "fresh operator authorization"
require_text ".csdlc/prepared/issues/731/design.md" "cs-host-377d41e71a824f92802120"
require_text ".csdlc/prepared/issues/731/design.md" "e2-micro"
require_text ".csdlc/prepared/issues/731/design.md" "no external IP"
require_text ".csdlc/prepared/issues/731/design.md" "zero disposable residue"
require_text ".csdlc/prepared/issues/731/design.md" "generic defaults are not live #731 authority"

require_text "infra/gcp/platform/main.tf" "resource \"google_compute_network\" \"private\""
require_text "infra/gcp/platform/main.tf" "resource \"google_compute_subnetwork\" \"private\""
require_text "infra/gcp/platform/main.tf" "resource \"google_compute_firewall\" \"iap_operator_access\""
require_text "infra/gcp/platform/main.tf" "resource \"google_compute_firewall\" \"explicit_private_egress\""
require_text "infra/gcp/platform/main.tf" "resource \"google_compute_firewall\" \"deny_unapproved_egress\""
require_text "infra/gcp/platform/main.tf" "resource \"google_compute_project_metadata_item\" \"os_login\""
require_text "infra/gcp/platform/main.tf" "resource \"google_service_account\" \"workload\""
require_text "infra/gcp/platform/main.tf" "resource \"google_storage_bucket\" \"state\""
require_text "infra/gcp/platform/main.tf" "resource \"google_storage_bucket\" \"artifacts\""
require_text "infra/gcp/platform/main.tf" "resource \"google_storage_bucket\" \"models\""
require_text "infra/gcp/platform/main.tf" "resource \"google_storage_bucket\" \"continuity_evidence\""
require_text "infra/gcp/platform/main.tf" "resource \"google_storage_bucket\" \"logs\""
require_text "infra/gcp/platform/main.tf" "resource \"google_logging_metric\" \"disposable_without_deadline\""

require_text "infra/gcp/platform/variables.tf" "default     = \"us-west2\""
require_text "infra/gcp/platform/variables.tf" "default     = \"10.42.0.0/24\""
require_text "infra/gcp/platform/variables.tf" "issue     = \"493\""
require_text "infra/gcp/platform/variables.tf" "ttl       = \"disposable\""
require_text "infra/gcp/platform/terraform.tfvars.example" "project_id  = \"cs-host-377d41e71a824f92802120\""
require_text "infra/gcp/platform/terraform.tfvars.example" "region      = \"us-west2\""
require_text "infra/gcp/platform/terraform.tfvars.example" "environment = \"dev\""
require_text "infra/gcp/platform/terraform.tfvars.example" "csm_name    = \"axioma\""
require_text "infra/gcp/platform/terraform.tfvars.example" "network_name = \"axioma-dev-csm-private\""
require_text "infra/gcp/platform/terraform.tfvars.example" "subnet_name  = \"axioma-dev-csm-private-us-west2\""
require_text "infra/gcp/platform/terraform.tfvars.example" "subnet_cidr  = \"10.42.0.0/24\""
require_text "infra/gcp/platform/terraform.tfvars.example" "operator_member = \"user:daniel@agent-logic.ai\""

reject_text "infra/gcp/platform/main.tf" "google_compute_instance"
reject_text "infra/gcp/platform/main.tf" "google_compute_address"
reject_text "infra/gcp/platform/main.tf" "access_config"
reject_text "infra/gcp/platform/main.tf" "google_compute_forwarding_rule"
reject_text "infra/gcp/platform/main.tf" "google_dns"

network_count="$(grep -Ec '^resource "google_compute_network" ' infra/gcp/platform/main.tf)"
subnet_count="$(grep -Ec '^resource "google_compute_subnetwork" ' infra/gcp/platform/main.tf)"
firewall_count="$(grep -Ec '^resource "google_compute_firewall" ' infra/gcp/platform/main.tf)"
bucket_count="$(grep -Ec '^resource "google_storage_bucket" ' infra/gcp/platform/main.tf)"
service_account_count="$(grep -Ec '^resource "google_service_account" ' infra/gcp/platform/main.tf)"
metric_count="$(grep -Ec '^resource "google_logging_metric" ' infra/gcp/platform/main.tf)"

test "$network_count" = 1 || fail "expected exactly one compute network, found $network_count"
test "$subnet_count" = 1 || fail "expected exactly one compute subnetwork, found $subnet_count"
test "$firewall_count" = 3 || fail "expected exactly three firewall rules, found $firewall_count"
test "$bucket_count" = 5 || fail "expected exactly five storage buckets, found $bucket_count"
test "$service_account_count" = 1 || fail "expected exactly one workload service account, found $service_account_count"
test "$metric_count" = 1 || fail "expected exactly one logging metric, found $metric_count"

printf 'PASS: #731 static readiness and private-foundation denominator guard\n'
