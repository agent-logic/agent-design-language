# Structured Output Record

Template: 1.0.0

Issue: 509

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Refreshed #509 DRT-D GCP portability proof at current exact head using the existing GCS runtime/model artifact bundle, with no local runtime rebuild. The disposable two-node GCP run passed, produced runtime and Ollama receipts, destroyed both nodes, and retained bounded-cost plus source-revision truth.

## Artifacts

- .csdlc/prepared/issues/509/validate-implementation.rb
- adl-runtime/tests/distributed_contract/main.rs
- adl-runtime/tests/distributed_contract/validate_drt_d.sh
- docs/milestones/v0.92.1/evidence/runtime/drt-d/qualification.json
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/preflight.json
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/runtime-final.json
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/ollama-ready.json
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/terraform-apply.log
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/terraform-destroy.log
- .csdlc/evidence/509/live-adl-509-drt-d-20260902192222/cleanup-readback.json

## Execution

- Reran the live GCP DRT-D qualification at current head using the already-stored GCS artifact manifest instead of rebuilding Runtime/Ollama binaries.
- Set Terraform provider auth to the approved service-account key and disabled per-run NAT creation because an existing regional ALL_SUBNETWORKS_ALL_IP_RANGES NAT was already present.
- Updated retained qualification evidence to run adl-509-drt-d-20260902192222 with source_revision f61de6ac171253db3d0afb47ec3e4c1838b47c54.
- Added executable stale-proof guards so the Ruby implementation validator and DRT-D retained-proof test reject qualification receipts whose source_revision does not match the current exact head.

## Validation

[
  {
    "command": [
      "CLOUDSDK_CONFIG=.csdlc/evidence/509/gcloud-config",
      "CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json",
      "GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json",
      "ADL_ISSUE509_CREATE_CLOUD_NAT=false",
      "bash",
      "adl/tools/run_issue509_gcp_drt_d_qualification.sh",
      "run",
      "--execute"
    ],
    "purpose": "Run live two-node GCP DRT-D qualification using existing GCS artifact bundle and destroy ephemeral resources.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/509/live-adl-509-drt-d-20260902192222"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/509/validate-implementation.rb"
    ],
    "purpose": "Verify retained live GCP qualification, bounded-cost semantics, cleanup, and current source_revision binding.",
    "outcome": "passed",
    "evidence_ref": "manual:post-live-refresh-implementation"
  },
  {
    "command": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_d.sh",
      "gcp-portability"
    ],
    "purpose": "Verify DRT-D retained proof denominator including current source_revision binding.",
    "outcome": "passed",
    "evidence_ref": "manual:post-live-refresh-drt-d-contract"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker drift after live refresh and stale-proof guard repair.",
    "outcome": "passed",
    "evidence_ref": "manual:post-live-refresh-diff-check"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
