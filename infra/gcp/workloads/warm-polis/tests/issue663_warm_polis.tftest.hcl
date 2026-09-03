mock_provider "google" {}

variables {
  project_id             = "agent-logic-test"
  region                 = "us-west1"
  zone                   = "us-west1-a"
  run_id                 = "adl-663-test"
  support_id             = "adl-support"
  service_account_email  = "runtime@agent-logic-test.iam.gserviceaccount.com"
  subnet_name            = "default"
  runtime_boot_image     = "projects/agent-logic-test/global/images/adl-runtime-20260903"
  ollama_boot_image      = "projects/agent-logic-test/global/images/adl-ollama-l4-20260903"
  runtime_snapshot       = "https://www.googleapis.com/compute/v1/projects/agent-logic-test/global/snapshots/adl-runtime-g1"
  ollama_snapshot        = "https://www.googleapis.com/compute/v1/projects/agent-logic-test/global/snapshots/adl-ollama-g1"
  artifact_generation    = "g1"
  runtime_content_sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  ollama_content_sha256  = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
  resident_models        = ["llama3.1:8b", "qwen3:8b"]
  assign_external_ip     = false
}

run "snapshot_restore_is_disposable_and_private" {
  command = plan

  assert {
    condition = (
      google_compute_disk.runtime.snapshot == var.runtime_snapshot &&
      google_compute_disk.ollama.snapshot == var.ollama_snapshot &&
      module.two_node_ollama_runtime.runtime_data_disk_attached &&
      module.two_node_ollama_runtime.ollama_data_disk_attached
    )
    error_message = "launch must restore and attach both exact snapshots"
  }

  assert {
    condition = (
      toset(google_compute_firewall.runtime_to_ollama.source_tags) == toset(["${var.support_id}-runtime"]) &&
      toset(google_compute_firewall.runtime_to_ollama.target_tags) == toset(["${var.support_id}-ollama"]) &&
      !var.assign_external_ip
    )
    error_message = "Ollama ingress must remain private to the Runtime node"
  }

  assert {
    condition = (
      !strcontains(var.runtime_boot_image, "/family/") &&
      !strcontains(var.ollama_boot_image, "/family/") &&
      strcontains(module.two_node_ollama_runtime.runtime_instance_name, var.run_id) &&
      strcontains(module.two_node_ollama_runtime.ollama_instance_name, var.run_id)
    )
    error_message = "launch must use immutable images and the existing two-node topology"
  }
}

run "image_family_alias_is_rejected" {
  command = plan
  variables {
    runtime_boot_image = "projects/agent-logic-test/global/images/family/adl-runtime"
  }
  expect_failures = [var.runtime_boot_image]
}
