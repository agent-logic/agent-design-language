#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
expected_candidate="${GCP_B_CANDIDATE_SHA:-c24f8fa65ce445b03ce6cd69007307291d78b60c}"
expected_project_alias="${GCP_B_PROJECT_ALIAS:-gcp-b-project}"
expected_service_account_alias="${GCP_B_SERVICE_ACCOUNT_ALIAS:-gcp-b-bootstrap-service-account}"
expected_project_id_sha256="${GCP_B_PROJECT_ID_SHA256:-e50962811283c5d194b59e96b42dd1d7b87e52676df03267903cba458cd87434}"
expected_service_account_sha256="${GCP_B_BOOTSTRAP_SERVICE_ACCOUNT_SHA256:-15c4fc636a9fba0e7da0b299db21e93f68a9ceee3512f77907fce6a9920c0ab6}"
expected_actor_sha256="${GCP_B_APPROVED_ACTOR_SHA256:-}"
proof_path="${1:-$repo_root/.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json}"

fail() {
  echo "$*" >&2
  exit 1
}

require_tool() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required tool: $1"
}

require_tool jq

scan_forbidden_retention() {
  local path="$1"
  if grep -Eiq '(/Users/|/private/tmp|/Volumes/FastWork|/Volumes/home|/Volumes/models|/keys/|credentials\.db|GOOGLE_APPLICATION_CREDENTIALS|CLOUDSDK_CONFIG|BEGIN PRIVATE KEY|BEGIN RSA PRIVATE KEY|BEGIN EC PRIVATE KEY|BEGIN OPENSSH PRIVATE KEY|private_key|client_secret|refresh_token|access_token|ya29\.|Bearer[[:space:]]+[A-Za-z0-9._-]+|cs-host-[A-Za-z0-9_-]+|[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.iam\.gserviceaccount\.com)' "$path"; then
    echo "artifact retains credential material, token text, raw identifiers, or machine-local paths: $path" >&2
    return 1
  fi
}

validate_one() {
  local path="$1"
  [[ -f "$path" ]] || { echo "missing proof artifact: $path" >&2; return 1; }

  jq -e \
    --arg candidate "$expected_candidate" \
    --arg project_alias "$expected_project_alias" \
    --arg service_account_alias "$expected_service_account_alias" \
    --arg project_id_sha256 "$expected_project_id_sha256" \
    --arg service_account_sha256 "$expected_service_account_sha256" \
    --arg actor_sha256 "$expected_actor_sha256" '
    .schema == "adl.gcp_b.audit_log_posture.redacted.v1"
    and .issue == 772
    and .command_assertion_class == "read_only_gcp_audit_log_posture"
    and .no_cloud_mutation_performed == true
    and .raw_log_payload_retained == false
    and .credential_material_retained == false
    and .candidate_sha == $candidate
    and (.proof_runner_head | test("^[0-9a-f]{40}$"))
    and .environment.project_alias == $project_alias
    and .environment.project_id_sha256 == $project_id_sha256
    and .environment.project_lifecycle_state == "ACTIVE"
    and (.environment.project_number_sha256 | test("^[0-9a-f]{64}$"))
    and .provider_identity.service_account_alias == $service_account_alias
    and .provider_identity.service_account_sha256 == $service_account_sha256
    and (.provider_identity.auth_mode | test("^(short_lived_bearer_env|service_account_key_activation|copied_or_existing_gcloud_auth)$"))
    and .provider_identity.service_account_exists == true
    and .provider_identity.service_account_disabled == false
    and (.provider_identity.active_operator_account_sha256 | test("^[0-9a-f]{64}$"))
    and (.provider_identity.approved_actor_sha256 | test("^[0-9a-f]{64}$"))
    and .provider_identity.approved_actor_matches_active == true
    and (($actor_sha256 == "") or (.provider_identity.approved_actor_sha256 == $actor_sha256))
    and .audit_assertions.audit_configuration_readback.status == "readable"
    and .audit_assertions.audit_configuration_readback.admin_activity == "platform_always_enabled"
    and .audit_assertions.audit_configuration_readback.system_event == "platform_always_enabled"
    and .audit_assertions.audit_configuration_readback.policy_denied == "platform_available_when_policy_denies"
    and (.audit_assertions.audit_log_classes_declared | sort) == (["admin_activity", "data_access", "policy", "system_event"] | sort)
    and .audit_assertions.required_services.logging_googleapis_com == true
    and .audit_assertions.required_services.cloudresourcemanager_googleapis_com == true
    and .audit_assertions.required_services.iam_googleapis_com == true
    and .audit_assertions.required_services.serviceusage_googleapis_com == true
    and (.audit_assertions.log_buckets | length) >= 1
    and (any(.audit_assertions.log_buckets[]; .name == "_Required"))
    and (.representative_readback.count >= 1)
    and (.representative_readback.entries | length) >= 1
    and (all(.representative_readback.entries[]; (.timestamp | type == "string") and (.log_class | type == "string") and (.service_name | type == "string") and (.method_name | type == "string") and (.resource_type | type == "string") and (.principal_class | test("^(bootstrap_service_account|agent_logic_user|other_principal_redacted|not_present)$"))))
    and .redaction.retained_payload == "summary_only"
    and .redaction.retained_local_paths == false
    and .redaction.retained_tokens_or_keys == false
  ' "$path" >/dev/null || { echo "proof artifact failed semantic validation: $path" >&2; return 1; }

  scan_forbidden_retention "$path" || return 1

  jq -e '
    [
      paths(scalars) as $p
      | {path: ($p | join(".")), value: (getpath($p) | tostring)}
      | select((.path | test("request|response|principalEmail|insertId|receiveTimestamp|operation")) or (.value | test("(^|/)Users/|/private/tmp|/Volumes/FastWork|/keys/|BEGIN .*PRIVATE KEY|ya29\\.|Bearer ")))
    ] | length == 0
  ' "$path" >/dev/null || { echo "proof artifact contains a forbidden raw-log or local-secret field" >&2; return 1; }
}

validate_auth_readiness() {
  local path="$1"
  [[ -f "$path" ]] || { echo "missing auth-readiness artifact: $path" >&2; return 1; }

  jq -e \
    --arg candidate "$expected_candidate" \
    --arg project_alias "$expected_project_alias" \
    --arg service_account_alias "$expected_service_account_alias" \
    --arg project_id_sha256 "$expected_project_id_sha256" \
    --arg service_account_sha256 "$expected_service_account_sha256" '
    .schema == "adl.gcp_b.audit_log_auth_readiness.redacted.v1"
    and .issue == 772
    and .candidate_sha == $candidate
    and (.proof_runner_head | test("^[0-9a-f]{40}$"))
    and .environment.project_alias == $project_alias
    and .environment.project_id_sha256 == $project_id_sha256
    and .provider_identity.service_account_alias == $service_account_alias
    and .provider_identity.service_account_sha256 == $service_account_sha256
    and (.attempts | length) >= 3
    and (any(.attempts[]; .route == "copied_existing_user_auth_config" and .result == "reauthentication_required"))
    and (any(.attempts[]; .route == "application_default_credentials" and .result == "reauthentication_required"))
    and (any(.attempts[]; .route == "service_account_impersonation_from_user_account" and .result == "reauthentication_required"))
    and (
      (
        .live_audit_log_posture_proof.status == "not_generated"
        and .live_audit_log_posture_proof.reason == "no_noninteractive_current_gcp_auth_source_available"
      )
      or
      (
        .live_audit_log_posture_proof.status == "generated_after_auth_refresh"
        and .live_audit_log_posture_proof.evidence_path == ".csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json"
        and (.live_audit_log_posture_proof.evidence_sha256 | test("^[0-9a-f]{64}$"))
      )
    )
    and .redaction.retained_payload == "auth_status_summary_only"
    and .redaction.retained_local_paths == false
    and .redaction.retained_tokens_or_keys == false
    and .redaction.retained_raw_project_or_provider_identifiers == false
  ' "$path" >/dev/null || { echo "auth-readiness artifact failed semantic validation: $path" >&2; return 1; }

  scan_forbidden_retention "$path" || return 1
}

self_test() {
  local proof="$1"
  validate_one "$proof"

  local git_common
  git_common="$(git rev-parse --path-format=absolute --git-common-dir)"
  local test_root="$git_common/csdlc-v3/test-artifacts/772-audit-log-posture/self-test-$$"
  mkdir -p "$test_root"
  chmod 700 "$test_root" 2>/dev/null || true

  jq '.candidate_sha = "0000000000000000000000000000000000000000"' "$proof" > "$test_root/stale-candidate.json"
  if validate_one "$test_root/stale-candidate.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted stale candidate"
  fi

  jq '.environment.project_alias = "wrong-project"' "$proof" > "$test_root/wrong-project.json"
  if validate_one "$test_root/wrong-project.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted wrong project"
  fi

  jq '.environment.project_id_sha256 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' "$proof" > "$test_root/wrong-project-digest.json"
  if validate_one "$test_root/wrong-project-digest.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted wrong project digest"
  fi

  jq '.provider_identity.service_account_alias = "wrong-provider"' "$proof" > "$test_root/wrong-provider.json"
  if validate_one "$test_root/wrong-provider.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted wrong provider identity"
  fi

  jq '.provider_identity.service_account_sha256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' "$proof" > "$test_root/wrong-provider-digest.json"
  if validate_one "$test_root/wrong-provider-digest.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted wrong provider identity digest"
  fi

  jq '.audit_assertions.audit_configuration_readback.status = "missing"' "$proof" > "$test_root/missing-audit-config.json"
  if validate_one "$test_root/missing-audit-config.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted missing audit configuration"
  fi

  jq '.provider_identity.approved_actor_matches_active = false' "$proof" > "$test_root/wrong-actor.json"
  if validate_one "$test_root/wrong-actor.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted wrong active actor"
  fi

  jq '.representative_readback.count = 0 | .representative_readback.entries = []' "$proof" > "$test_root/missing-readback.json"
  if validate_one "$test_root/missing-readback.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted missing log readback"
  fi

  jq '.redaction.retained_local_paths = true | .unsafe = "/Users/example/keys/gcp.json"' "$proof" > "$test_root/unsafe-retention.json"
  if validate_one "$test_root/unsafe-retention.json" 2>/dev/null; then
    fail "negative fixture unexpectedly accepted unsafe retained content"
  fi

  rm -rf "$test_root"
  echo "issue-772 validator self-test passed"
}

case "${2:-}" in
  --auth-readiness) validate_auth_readiness "$proof_path"; echo "issue-772 auth-readiness validator passed" ;;
  --self-test) self_test "$proof_path" ;;
  "") validate_one "$proof_path"; echo "issue-772 GCP-B audit/log posture validator passed" ;;
  *) fail "unknown argument: $2" ;;
esac
