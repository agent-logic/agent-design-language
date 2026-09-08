#!/usr/bin/env bash
set -euo pipefail
sleep "1"
".csdlc/evidence/731/remediation-guards/mock-bin/gcloud" --impersonate-service-account "axioma-dev-workload@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com" compute instances delete "adl-gcp-d1-localguard" --project "cs-host-377d41e71a824f92802120" --zone "us-west2-a" --delete-disks=all --quiet
