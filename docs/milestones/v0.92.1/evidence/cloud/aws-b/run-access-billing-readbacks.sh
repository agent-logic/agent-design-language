#!/usr/bin/env bash
set -euo pipefail

LANE="all"
if [[ "${1:-}" == "--lane" ]]; then
  LANE="${2:-}"
fi

PROFILE="${AWS_PROFILE:-agent-logic-admin}"
REGION="${AWS_REGION:-us-east-1}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
READBACK_DIR="${SCRIPT_DIR}/readbacks"
mkdir -p "${READBACK_DIR}"

redact() {
  sed -E \
    -e 's/("sessionToken"[[:space:]]*:[[:space:]]*")[^"]+/\1[AWS_SESSION_TOKEN_REDACTED]/g' \
    -e 's/(\\"sessionToken\\"[[:space:]]*:[[:space:]]*\\")[^\\"]+/\1[AWS_SESSION_TOKEN_REDACTED]/g' \
    -e 's/("secretAccessKey"[[:space:]]*:[[:space:]]*")[^"]+/\1[AWS_SECRET_ACCESS_KEY_REDACTED]/g' \
    -e 's/(\\"secretAccessKey\\"[[:space:]]*:[[:space:]]*\\")[^\\"]+/\1[AWS_SECRET_ACCESS_KEY_REDACTED]/g' \
    -e 's/[0-9]{12}/[AWS_ACCOUNT_ID_REDACTED]/g' \
    -e 's/arn:aws:[A-Za-z0-9_:+/.,@=-]+/[AWS_ARN_REDACTED]/g' \
    -e 's/[A-Z0-9]{16,}/[AWS_IDENTIFIER_REDACTED]/g' \
    -e 's/[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}/[EMAIL_REDACTED]/g'
}

write_header() {
  local outfile="$1"
  local title="$2"
  {
    echo "# ${title}"
    echo
    echo "- issue: #485"
    echo "- profile: ${PROFILE}"
    echo "- region: ${REGION}"
    echo "- generated_at_utc: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "- posture: read-only evidence collection"
    echo
  } > "${outfile}"
}

run_readback() {
  local outfile="$1"
  local label="$2"
  shift 2
  {
    echo "## ${label}"
    echo
    echo '```text'
    if "$@" 2>&1 | redact; then
      :
    else
      local status=$?
      echo "READBACK_UNAVAILABLE status=${status}"
    fi
    echo '```'
    echo
  } >> "${outfile}"
}

aws_read() {
  aws --profile "${PROFILE}" --region "${REGION}" "$@"
}

emit_root_recovery() {
  local outfile="${READBACK_DIR}/root-recovery.md"
  write_header "${outfile}" "Root recovery and administrator continuity"
  {
    echo "## Baseline assertions"
    echo
    echo "- Existing administrator access is retained; this issue performs no administrator removal."
    echo "- Corporate recovery must not depend on one personal factor before any replacement/removal is considered proven."
    echo "- Any future change to administrator access requires a separate typed lane and operator approval."
    echo
  } >> "${outfile}"
  run_readback "${outfile}" "STS caller identity" aws_read sts get-caller-identity --output json
  run_readback "${outfile}" "IAM account aliases" aws_read iam list-account-aliases --output json
  run_readback "${outfile}" "IAM account summary" aws_read iam get-account-summary --output json
}

emit_identity_census() {
  local outfile="${READBACK_DIR}/identity-census.md"
  write_header "${outfile}" "Human workload and agent identity census"
  {
    echo "## Classification rule"
    echo
    echo "- Humans: IAM users or roles explicitly intended for named human administration."
    echo "- Workloads: service, runtime, CI, Terraform, or deployment roles."
    echo "- Agents: Codex/agent-toolkit roles or profiles constrained by read-only default posture."
    echo "- Unknowns remain gaps until reviewed; they are not silently treated as disposable."
    echo
  } >> "${outfile}"
  run_readback "${outfile}" "IAM users" aws_read iam list-users --max-items 100 --output json
  run_readback "${outfile}" "IAM roles" aws_read iam list-roles --max-items 100 --output json
  run_readback "${outfile}" "IAM groups" aws_read iam list-groups --max-items 100 --output json
}

emit_agent_toolkit_configuration() {
  local outfile="${READBACK_DIR}/agent-toolkit-configuration.md"
  write_header "${outfile}" "Agent Toolkit for AWS configuration"
  {
    echo "## Approved path"
    echo
    echo "- Agent Toolkit for AWS is documented for the approved Codex path only."
    echo "- AWS CLI must be 2.35 or newer before this lane can be accepted as configured."
    echo "- Toolkit use does not authorize resource creation or IAM writes in this issue."
    echo
    echo "## Local CLI version"
    echo
    echo '```text'
    aws --version 2>&1 | redact || echo "READBACK_UNAVAILABLE aws_cli_version"
    echo '```'
    echo
  } >> "${outfile}"
  run_readback "${outfile}" "STS caller identity for approved profile" aws_read sts get-caller-identity --output json
}

emit_agent_iam_guardrails() {
  local outfile="${READBACK_DIR}/agent-iam-guardrails.md"
  write_header "${outfile}" "Agent IAM guardrails"
  {
    echo "## Required guardrail posture"
    echo
    echo "- Default agent access is read-only."
    echo "- Elevated actions require typed approval, scoped IAM context, and retained CloudTrail/CloudWatch attribution."
    echo "- This issue performs no IAM create/update/delete operation."
    echo
  } >> "${outfile}"
  run_readback "${outfile}" "Local AWS managed read-only policy metadata" aws_read iam get-policy --policy-arn arn:aws:iam::aws:policy/ReadOnlyAccess --output json
  run_readback "${outfile}" "Customer managed policy list" aws_read iam list-policies --scope Local --max-items 100 --output json
}

emit_agent_activity_audit() {
  local outfile="${READBACK_DIR}/agent-activity-audit.md"
  write_header "${outfile}" "CloudWatch and CloudTrail attribution"
  run_readback "${outfile}" "CloudTrail trails" aws_read cloudtrail describe-trails --include-shadow-trails --output json
  run_readback "${outfile}" "Recent CloudTrail events" aws_read cloudtrail lookup-events --max-results 10 --output json
  run_readback "${outfile}" "CloudWatch AWS Usage metrics" aws_read cloudwatch list-metrics --namespace AWS/Usage --recently-active PT3H --output json
}

emit_billing_readback() {
  local outfile="${READBACK_DIR}/billing-readback.md"
  write_header "${outfile}" "Billing budget anomaly export and cost attribution"
  local start_date
  local end_date
  local account_id
  start_date="$(date -u -v-7d +%Y-%m-%d 2>/dev/null || date -u -d '7 days ago' +%Y-%m-%d)"
  end_date="$(date -u +%Y-%m-%d)"
  account_id="$(aws_read sts get-caller-identity --query Account --output text 2>/dev/null || echo 000000000000)"
  run_readback "${outfile}" "Cost Explorer seven-day unblended cost" aws_read ce get-cost-and-usage --time-period "Start=${start_date},End=${end_date}" --granularity DAILY --metrics UnblendedCost --output json
  run_readback "${outfile}" "Budgets" aws_read budgets describe-budgets --account-id "${account_id}" --output json
  run_readback "${outfile}" "Cost anomaly monitors" aws_read ce get-anomaly-monitors --output json
  run_readback "${outfile}" "Cost allocation tags" aws_read ce list-cost-allocation-tags --output json
  run_readback "${outfile}" "Billing exports" aws_read bcm-data-exports list-exports --output json
}

case "${LANE}" in
  all)
    emit_root_recovery
    emit_identity_census
    emit_agent_toolkit_configuration
    emit_agent_iam_guardrails
    emit_agent_activity_audit
    emit_billing_readback
    ;;
  root-recovery) emit_root_recovery ;;
  identity-census) emit_identity_census ;;
  agent-toolkit-configuration) emit_agent_toolkit_configuration ;;
  agent-iam-guardrails) emit_agent_iam_guardrails ;;
  agent-activity-audit) emit_agent_activity_audit ;;
  billing-readback|budget-and-anomaly) emit_billing_readback ;;
  *)
    echo "unknown lane: ${LANE}" >&2
    exit 64
    ;;
esac

echo "wrote redacted readbacks for lane ${LANE} under ${READBACK_DIR}"
