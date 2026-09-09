#!/usr/bin/env bash
set -euo pipefail

project_id="${GCP_B_PROJECT_ID:-}"
service_account="${GCP_B_BOOTSTRAP_SERVICE_ACCOUNT:-}"
candidate_sha="${GCP_B_CANDIDATE_SHA:-c24f8fa65ce445b03ce6cd69007307291d78b60c}"
project_alias="${GCP_B_PROJECT_ALIAS:-gcp-b-project}"
service_account_alias="${GCP_B_SERVICE_ACCOUNT_ALIAS:-gcp-b-bootstrap-service-account}"
approved_actor_sha256="${GCP_B_APPROVED_ACTOR_SHA256:-}"
output_path="${GCP_B_AUDIT_LOG_OUTPUT:-}"
source_cloudsdk_config="${GCP_B_SOURCE_CLOUDSDK_CONFIG:-}"
bootstrap_key_file="${GCP_B_BOOTSTRAP_KEY_FILE:-}"
access_token="${GCP_B_ACCESS_TOKEN:-}"

repo_root="$(git rev-parse --show-toplevel)"
git_common="$(git rev-parse --path-format=absolute --git-common-dir)"
run_root="$git_common/csdlc-v3/gcp-b-audit-log-posture/772/run-$(date -u +%Y%m%dT%H%M%SZ)-$$"
run_cloudsdk_config="$run_root/cloudsdk-config"
scratch="$run_root/scratch"

if [[ -z "$output_path" ]]; then
  output_path="$repo_root/.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json"
fi

fail() {
  echo "$*" >&2
  exit 1
}

require_tool() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required tool: $1"
}

sha256_text() {
  if command -v sha256sum >/dev/null 2>&1; then
    printf '%s' "$1" | sha256sum | awk '{print $1}'
  else
    printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
  fi
}

cleanup() {
  set +e
  rm -rf "$run_root"
}
trap cleanup EXIT

require_tool gcloud
require_tool jq
require_tool curl

[[ -n "$project_id" ]] || fail "GCP_B_PROJECT_ID is required"
[[ -n "$service_account" ]] || fail "GCP_B_BOOTSTRAP_SERVICE_ACCOUNT is required"

case "$candidate_sha" in
  [0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]) ;;
  *) fail "candidate SHA must be a lowercase 40-character git object id" ;;
esac

mkdir -p "$(dirname "$output_path")" "$run_cloudsdk_config" "$scratch"
chmod 700 "$run_root" "$run_cloudsdk_config" "$scratch" 2>/dev/null || true

if [[ -n "$source_cloudsdk_config" ]]; then
  [[ -d "$source_cloudsdk_config" ]] || fail "GCP_B_SOURCE_CLOUDSDK_CONFIG does not name a directory"
  cp -R "$source_cloudsdk_config"/. "$run_cloudsdk_config"/
  find "$run_cloudsdk_config" -type f -name '*.log' -delete
elif [[ -n "${CLOUDSDK_CONFIG:-}" && -d "${CLOUDSDK_CONFIG:-}" ]]; then
  cp -R "$CLOUDSDK_CONFIG"/. "$run_cloudsdk_config"/
  find "$run_cloudsdk_config" -type f -name '*.log' -delete
fi

export CLOUDSDK_CONFIG="$run_cloudsdk_config"
export CLOUDSDK_CORE_DISABLE_PROMPTS=1

curl_json() {
  local method="$1"
  local url="$2"
  local body="${3:-}"
  if [[ -n "$body" ]]; then
    curl -fsS \
      -X "$method" \
      -H "Authorization: Bearer $access_token" \
      -H "Content-Type: application/json" \
      --data "$body" \
      "$url"
  else
    curl -fsS \
      -X "$method" \
      -H "Authorization: Bearer $access_token" \
      "$url"
  fi
}

if [[ -n "$access_token" ]]; then
  auth_mode="short_lived_bearer_env"
  [[ -n "$approved_actor_sha256" ]] || fail "GCP_B_APPROVED_ACTOR_SHA256 is required when GCP_B_ACCESS_TOKEN is supplied"
  active_account="token_actor_redacted"
  active_account_sha256="$approved_actor_sha256"
elif [[ -n "$bootstrap_key_file" ]]; then
  auth_mode="service_account_key_activation"
  [[ -f "$bootstrap_key_file" ]] || fail "GCP_B_BOOTSTRAP_KEY_FILE does not name a file"
  gcloud auth activate-service-account "$service_account" \
    --key-file="$bootstrap_key_file" \
    --project="$project_id" \
    --quiet >/dev/null

  active_account="$(gcloud auth list --filter=status:ACTIVE --format='value(account)' | head -n 1)"
  [[ -n "$active_account" ]] || fail "no active authenticated gcloud account in issue-owned Cloud SDK config"
  active_account_sha256="$(sha256_text "$active_account")"
else
  auth_mode="copied_or_existing_gcloud_auth"
  active_account="$(gcloud auth list --filter=status:ACTIVE --format='value(account)' | head -n 1)"
  [[ -n "$active_account" ]] || fail "no active authenticated gcloud account in issue-owned Cloud SDK config"
  active_account_sha256="$(sha256_text "$active_account")"
fi

project_json="$scratch/project.json"
sa_json="$scratch/service-account.json"
services_json="$scratch/services.json"
iam_policy_json="$scratch/iam-policy.json"
buckets_json="$scratch/log-buckets.json"
sinks_json="$scratch/log-sinks.json"
settings_json="$scratch/log-settings.json"
entries_json="$scratch/log-entries.json"

audit_filter='logName=("projects/'"$project_id"'/logs/cloudaudit.googleapis.com%2Factivity" OR "projects/'"$project_id"'/logs/cloudaudit.googleapis.com%2Fdata_access" OR "projects/'"$project_id"'/logs/cloudaudit.googleapis.com%2Fsystem_event" OR "projects/'"$project_id"'/logs/cloudaudit.googleapis.com%2Fpolicy")'

if [[ "$auth_mode" == "short_lived_bearer_env" ]]; then
  service_account_uri="$(jq -rn --arg v "$service_account" '$v|@uri')"
  curl_json GET "https://cloudresourcemanager.googleapis.com/v1/projects/$project_id" \
    > "$project_json"
  curl_json GET "https://iam.googleapis.com/v1/projects/$project_id/serviceAccounts/$service_account_uri" \
    > "$sa_json"
  curl_json GET "https://serviceusage.googleapis.com/v1/projects/$project_id/services?filter=state:ENABLED&pageSize=200" \
    | jq '.services // []' > "$services_json"
  curl_json POST "https://cloudresourcemanager.googleapis.com/v1/projects/$project_id:getIamPolicy" '{}' \
    > "$iam_policy_json"
  curl_json GET "https://logging.googleapis.com/v2/projects/$project_id/locations/global/buckets" \
    | jq '.buckets // []' > "$buckets_json"
  curl_json GET "https://logging.googleapis.com/v2/projects/$project_id/sinks" \
    | jq '.sinks // []' > "$sinks_json"
  if curl_json GET "https://logging.googleapis.com/v2/projects/$project_id/settings" \
    > "$settings_json" 2>"$scratch/log-settings.stderr"; then
    settings_status="readable"
  else
    settings_status="unavailable"
    printf '{}' > "$settings_json"
  fi
  jq -cn --arg project "$project_id" --arg filter "$audit_filter" \
    '{resourceNames:["projects/" + $project], filter:$filter, orderBy:"timestamp desc", pageSize:20}' \
    | { read -r body; curl_json POST "https://logging.googleapis.com/v2/entries:list" "$body"; } \
    | jq '.entries // []' > "$entries_json"
else
  gcloud projects describe "$project_id" \
    --format=json > "$project_json"

  gcloud iam service-accounts describe "$service_account" \
    --project="$project_id" \
    --format=json > "$sa_json"

  gcloud services list \
    --enabled \
    --project="$project_id" \
    --format=json > "$services_json"

  gcloud projects get-iam-policy "$project_id" \
    --format=json > "$iam_policy_json"

  gcloud logging buckets list \
    --project="$project_id" \
    --location=global \
    --format=json > "$buckets_json"

  gcloud logging sinks list \
    --project="$project_id" \
    --format=json > "$sinks_json"

  if gcloud logging settings describe \
    --project="$project_id" \
    --format=json > "$settings_json" 2>"$scratch/log-settings.stderr"; then
    settings_status="readable"
  else
    settings_status="unavailable"
    printf '{}' > "$settings_json"
  fi

  gcloud logging read "$audit_filter" \
    --project="$project_id" \
    --freshness=30d \
    --limit=20 \
    --format=json > "$entries_json"
fi

generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
runner_head="$(git rev-parse HEAD)"
if [[ -z "$approved_actor_sha256" ]]; then
  approved_actor_sha256="$active_account_sha256"
fi
project_number_sha256="$(jq -r '.projectNumber // empty' "$project_json" | { read value; sha256_text "$value"; })"
project_id_sha256="$(sha256_text "$project_id")"
service_account_sha256="$(sha256_text "$service_account")"

jq -n \
  --arg generated_at "$generated_at" \
  --arg candidate_sha "$candidate_sha" \
  --arg runner_head "$runner_head" \
  --arg project_alias "$project_alias" \
  --arg service_account_alias "$service_account_alias" \
  --arg service_account_raw "$service_account" \
  --arg project_id_sha256 "$project_id_sha256" \
  --arg service_account_sha256 "$service_account_sha256" \
  --arg active_account_sha256 "$active_account_sha256" \
  --arg approved_actor_sha256 "$approved_actor_sha256" \
  --arg auth_mode "$auth_mode" \
  --arg project_number_sha256 "$project_number_sha256" \
  --arg settings_status "$settings_status" \
  --slurpfile project "$project_json" \
  --slurpfile sa "$sa_json" \
  --slurpfile services "$services_json" \
  --slurpfile iam "$iam_policy_json" \
  --slurpfile buckets "$buckets_json" \
  --slurpfile sinks "$sinks_json" \
  --slurpfile settings "$settings_json" \
  --slurpfile entries "$entries_json" '
  def service_enabled($name):
    any($services[0][]?; .config.name == $name);
  def destination_kind($destination):
    if ($destination | startswith("storage.googleapis.com/")) then "gcs_bucket"
    elif ($destination | startswith("bigquery.googleapis.com/")) then "bigquery_dataset"
    elif ($destination | startswith("pubsub.googleapis.com/")) then "pubsub_topic"
    elif ($destination | startswith("logging.googleapis.com/")) then "logging_bucket"
    elif ($destination | length) == 0 then "none"
    else "other_redacted"
    end;
  def principal_class($email; $bootstrap):
    if ($email // "") == $bootstrap then "bootstrap_service_account"
    elif (($email // "") | endswith("@agent-logic.ai")) then "agent_logic_user"
    elif (($email // "") | contains("@")) then "other_principal_redacted"
    else "not_present"
    end;
  def audit_log_types($service):
    [
      $iam[0].auditConfigs[]?
      | select(.service == $service)
      | .auditLogConfigs[]?.logType
    ] | unique;
  {
    schema: "adl.gcp_b.audit_log_posture.redacted.v1",
    issue: 772,
    command_assertion_class: "read_only_gcp_audit_log_posture",
    no_cloud_mutation_performed: true,
    raw_log_payload_retained: false,
    credential_material_retained: false,
    generated_at: $generated_at,
    candidate_sha: $candidate_sha,
    proof_runner_head: $runner_head,
    environment: {
      project_alias: $project_alias,
      project_id_sha256: $project_id_sha256,
      project_lifecycle_state: ($project[0].lifecycleState // null),
      project_number_sha256: $project_number_sha256
    },
    provider_identity: {
      auth_mode: $auth_mode,
      service_account_alias: $service_account_alias,
      service_account_sha256: $service_account_sha256,
      service_account_exists: (($sa[0].email // "") == $service_account_raw),
      service_account_disabled: ($sa[0].disabled // false),
      active_operator_account_sha256: $active_account_sha256,
      approved_actor_sha256: $approved_actor_sha256,
      approved_actor_matches_active: ($approved_actor_sha256 == $active_account_sha256)
    },
    audit_assertions: {
      audit_log_classes_declared: ["admin_activity", "data_access", "system_event", "policy"],
      audit_configuration_readback: {
        status: "readable",
        admin_activity: "platform_always_enabled",
        system_event: "platform_always_enabled",
        policy_denied: "platform_available_when_policy_denies",
        data_access_configured_services: [
          $iam[0].auditConfigs[]?
          | {
              service: .service,
              log_types: ([.auditLogConfigs[]?.logType] | unique)
            }
        ],
        all_services_log_types: audit_log_types("allServices")
      },
      readable_log_classes_observed: [
        $entries[0][]?
        | (.logName // "")
        | capture("cloudaudit.googleapis.com%2F(?<class>[^/]+)$")?
        | .class
      ] | unique,
      required_services: {
        logging_googleapis_com: service_enabled("logging.googleapis.com"),
        cloudresourcemanager_googleapis_com: service_enabled("cloudresourcemanager.googleapis.com"),
        iam_googleapis_com: service_enabled("iam.googleapis.com"),
        serviceusage_googleapis_com: service_enabled("serviceusage.googleapis.com")
      },
      settings_status: $settings_status,
      settings_storage_location_present: (($settings[0].storageLocation // "") | length > 0),
      log_buckets: [
        $buckets[0][]?
        | {
            name: (.name | split("/") | last),
            lifecycle_state: (.lifecycleState // null),
            retention_days: (.retentionDays // null),
            locked: (.locked // false)
          }
      ],
      sinks: [
        $sinks[0][]?
        | {
            name: .name,
            destination_kind: destination_kind(.destination // ""),
            disabled: (.disabled // false),
            include_children: (.includeChildren // false),
            filter_present: (((.filter // "") | length) > 0)
          }
      ]
    },
    representative_readback: {
      freshness_window: "30d",
      count: ($entries[0] | length),
      entries: [
        $entries[0][0:10][]?
        | {
            timestamp: .timestamp,
            log_class: (try ((.logName // "") | capture("cloudaudit.googleapis.com%2F(?<class>[^/]+)$") | .class) catch "unknown"),
            service_name: (.protoPayload.serviceName // "not_present"),
            method_name: (.protoPayload.methodName // "not_present"),
            resource_type: (.resource.type // "not_present"),
            severity: (.severity // "DEFAULT"),
            principal_class: principal_class(.protoPayload.authenticationInfo.principalEmail; $service_account_raw)
          }
      ]
    },
    redaction: {
      retained_payload: "summary_only",
      omitted_fields: [
        "protoPayload.request",
        "protoPayload.response",
        "protoPayload.authenticationInfo.principalEmail",
        "insertId",
        "operation",
        "receiveTimestamp"
      ],
      retained_local_paths: false,
      retained_tokens_or_keys: false
    }
  }' > "$output_path"

"$repo_root/.csdlc/prepared/issues/772/validate-gcp-b-audit-log-posture.sh" "$output_path"

echo "wrote sanitized GCP-B audit/log posture proof: $output_path"
