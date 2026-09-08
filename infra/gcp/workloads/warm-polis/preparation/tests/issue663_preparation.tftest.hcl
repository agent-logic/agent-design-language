mock_provider "google" {}

variables {
  project_id             = "agent-logic-test"
  region                 = "us-west1"
  zone                   = "us-west1-a"
  generation             = "g1"
  service_account_email  = "runtime@agent-logic-test.iam.gserviceaccount.com"
  subnet_name            = "default"
  preparation_boot_image = "projects/agent-logic-test/global/images/adl-preparation-20260903"
  runtime_bundle_uri     = "gs://agent-logic-test/runtime/g1.tar#123456"
  runtime_bundle_sha256  = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  ollama_bundle_uri      = "gs://agent-logic-test/ollama/g1.tar#654321"
  ollama_bundle_sha256   = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
  attach_preparation_vms = true
}

run "hydrate_both_disks_concurrently" {
  command = plan
  assert {
    condition = (
      length(google_compute_instance.runtime_preparation) == 1 &&
      length(google_compute_instance.ollama_preparation) == 1 &&
      google_compute_instance.runtime_preparation[0].attached_disk[0].device_name == "adl-runtime-staging" &&
      google_compute_instance.ollama_preparation[0].attached_disk[0].device_name == "adl-ollama-staging"
    )
    error_message = "preparation must hydrate both independently owned staging disks"
  }
}

run "detach_without_deleting_staging_disks" {
  command = plan
  variables { attach_preparation_vms = false }
  assert {
    condition = (
      length(google_compute_instance.runtime_preparation) == 0 &&
      length(google_compute_instance.ollama_preparation) == 0 &&
      google_compute_disk.runtime_staging.name != "" &&
      google_compute_disk.ollama_staging.name != ""
    )
    error_message = "detach phase must retain staging disks while removing both VMs"
  }
}
