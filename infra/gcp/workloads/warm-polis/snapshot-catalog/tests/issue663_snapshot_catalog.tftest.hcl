mock_provider "google" {}

variables {
  project_id              = "agent-logic-test"
  region                  = "us-west1"
  zone                    = "us-west1-a"
  generation              = "g1"
  runtime_staging_disk    = "projects/agent-logic-test/zones/us-west1-a/disks/runtime-staging"
  ollama_staging_disk     = "projects/agent-logic-test/zones/us-west1-a/disks/ollama-staging"
  runtime_manifest_sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  ollama_manifest_sha256  = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
  verification_boot_image = "projects/agent-logic-test/global/images/adl-verifier-20260903"
  service_account_email   = "runtime@agent-logic-test.iam.gserviceaccount.com"
  subnet_name             = "default"
  enable_verifier         = true
}

run "snapshots_and_restored_content_verifier" {
  command = plan
  assert {
    condition = (
      google_compute_snapshot.runtime.source_disk == var.runtime_staging_disk &&
      google_compute_snapshot.ollama.source_disk == var.ollama_staging_disk &&
      length(google_compute_instance.verifier) == 1 &&
      length(google_compute_disk.runtime_verifier) == 1 &&
      length(google_compute_disk.ollama_verifier) == 1
    )
    error_message = "catalog must create both snapshots and one restored-content verifier topology"
  }
}

run "retained_state_contains_snapshots_only" {
  command = plan
  variables { enable_verifier = false }
  assert {
    condition = (
      length(google_compute_instance.verifier) == 0 &&
      length(google_compute_disk.runtime_verifier) == 0 &&
      length(google_compute_disk.ollama_verifier) == 0 &&
      google_compute_snapshot.runtime.name != "" &&
      google_compute_snapshot.ollama.name != ""
    )
    error_message = "catalog steady state must retain only the two snapshots"
  }
}
