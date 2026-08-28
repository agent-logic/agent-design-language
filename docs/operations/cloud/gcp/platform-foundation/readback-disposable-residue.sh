#!/usr/bin/env bash
set -euo pipefail

project=""
zone=""
csm=""
environment=""
network_name=""

while (($#)); do
  case "$1" in
    --project) project="${2:-}"; shift 2 ;;
    --zone) zone="${2:-}"; shift 2 ;;
    --csm) csm="${2:-}"; shift 2 ;;
    --env) environment="${2:-}"; shift 2 ;;
    --network-name) network_name="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -z "$project" || -z "$zone" || -z "$csm" || -z "$environment" || -z "$network_name" ]]; then
  echo "usage: $0 --project PROJECT --zone ZONE --csm NAME --env ENV --network-name NETWORK" >&2
  exit 64
fi

filter="labels.issue=493 AND labels.ttl=disposable AND labels.csm=${csm} AND labels.env=${environment}"
workload_service_account="${csm}-${environment}-workload@${project}.iam.gserviceaccount.com"

echo "instances:"
gcloud compute instances list --project "$project" --zones "$zone" --filter "$filter"

echo "disks:"
gcloud compute disks list --project "$project" --zones "$zone" --filter "$filter"

echo "addresses:"
gcloud compute addresses list --project "$project" --filter "$filter"

echo "firewall rules:"
gcloud compute firewall-rules list --project "$project" --filter "network:${network_name} AND targetTags:csm-disposable"

echo "service accounts:"
gcloud iam service-accounts list --project "$project" --filter "email:${workload_service_account}"

echo "project IAM bindings mentioning issue 493 or disposable workload service account:"
gcloud projects get-iam-policy "$project" --flatten "bindings[].members" --filter "bindings.members:serviceAccount:${workload_service_account} OR bindings.condition.expression:493"

echo "storage buckets:"
gcloud storage buckets list --project "$project" --filter "labels.issue=493 AND labels.csm=${csm} AND labels.env=${environment}"

echo "storage bucket IAM policies and objects:"
for owner in state artifacts models continuity-evidence logs; do
  bucket="gs://${project}-${environment}-${csm}-${owner}"
  gcloud storage buckets get-iam-policy "$bucket" 2>/dev/null || true
  gcloud storage ls --recursive "$bucket/**" 2>/dev/null || true
done

echo "terraform state:"
terraform -chdir=infra/gcp/platform state list 2>/dev/null || true
