#!/usr/bin/env bash
set -euo pipefail

project=""
zone=""
csm=""
environment=""

while (($#)); do
  case "$1" in
    --project) project="${2:-}"; shift 2 ;;
    --zone) zone="${2:-}"; shift 2 ;;
    --csm) csm="${2:-}"; shift 2 ;;
    --env) environment="${2:-}"; shift 2 ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

if [[ -z "$project" || -z "$zone" || -z "$csm" || -z "$environment" ]]; then
  echo "usage: $0 --project PROJECT --zone ZONE --csm NAME --env ENV" >&2
  exit 64
fi

filter="labels.issue=493 AND labels.ttl=disposable AND labels.csm=${csm} AND labels.env=${environment}"

echo "instances:"
gcloud compute instances list --project "$project" --zones "$zone" --filter "$filter"

echo "disks:"
gcloud compute disks list --project "$project" --zones "$zone" --filter "$filter"

echo "addresses:"
gcloud compute addresses list --project "$project" --filter "$filter"

echo "firewall rules:"
gcloud compute firewall-rules list --project "$project" --filter "$filter"
