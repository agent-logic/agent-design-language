#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TF_DIR="$ROOT/infra/aws/csm-public-edge"
PLAN="$ROOT/.csdlc/prepared/issues/122/terraform-execution-plan.md"

fail() {
  printf 'validate_csm_public_edge_static: FAIL: %s\n' "$*" >&2
  exit 1
}

test -d "$TF_DIR" || fail "missing Terraform directory: $TF_DIR"
test -f "$PLAN" || fail "missing reviewed Terraform execution plan: $PLAN"

terraform -chdir="$TF_DIR" fmt -check -recursive
terraform -chdir="$TF_DIR" init -backend=false
terraform -chdir="$TF_DIR" validate

find "$TF_DIR" -path "$TF_DIR/.terraform" -prune -o -type f -print0 \
  | xargs -0 grep -E 'CloudFormation|aws_(instance|spot|nat_gateway|eks|codebuild_project)' >/dev/null \
  && fail "Terraform module mentions forbidden non-Terraform, compute, NAT, or CodeBuild surface"

grep -R 'resource "aws_cloudfront_distribution" "wss"' "$TF_DIR" >/dev/null || fail "missing dedicated native WSS CloudFront distribution"
grep -R 'wss_origin_https_url' "$TF_DIR" >/dev/null || fail "missing native WSS HTTPS origin input"
grep -R 'var.wss_origin_https_url == "https://${var.wss_origin_hostname}"' "$TF_DIR" >/dev/null \
  || fail "missing fail-closed host-only WSS origin URL guard"
grep -R 'startswith(var.wss_origin_https_url, "https://${var.wss_origin_hostname}/")' "$TF_DIR" >/dev/null \
  && fail "WSS origin URL guard must not accept path-bearing origins without CloudFront origin_path support"
grep -R 'Sec-WebSocket-Protocol' "$TF_DIR" >/dev/null || fail "missing WSS protocol header forwarding"
grep -R 'wss_forward_viewer_host' "$TF_DIR" >/dev/null || fail "missing WSS Host/SNI guard"

printf 'validate_csm_public_edge_static: PASS\n'
