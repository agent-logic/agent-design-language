mock_provider "google" {}

run "normal_launch_consumes_prepared_artifacts" {
  command = plan

  variables {
    project_id               = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
    region                   = "us-west1"
    zone                     = "us-west1-a"
    run_id                   = "adl-509-drt-d-test"
    support_id               = "adl-494-gpu-smoke"
    service_account_email    = "adl-494-gpu-smoke@example.iam.gserviceaccount.com"
    subnet_name              = "default"
    runtime_machine_type     = "e2-standard-4"
    ollama_machine_type      = "g2-standard-4"
    runtime_boot_image       = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
    ollama_boot_image        = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
    accelerator_type         = "nvidia-l4"
    accelerator_count        = 1
    max_budget_usd           = 20
    ttl_expires_at           = "2026-09-03T00:00:00Z"
    source_revision          = "0123456789abcdef0123456789abcdef01234567"
    assign_external_ip       = false
    resident_models          = ["llama3.1:8b", "qwen3:8b", "phi4-mini:latest"]
    artifact_bucket          = "adl-test-artifacts"
    artifact_manifest_object = "models/ollama/issue509/test/portable-model-bundle.json"
    artifact_manifest_sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    runtime_startup_script   = file("../../drt-d-six-resident/startup-runtime.sh")
    ollama_startup_script    = file("../../drt-d-six-resident/startup-ollama.sh")
  }

  assert {
    condition = (
      strcontains(google_compute_instance.ollama.metadata_startup_script, "gcloud storage cp") &&
      strcontains(google_compute_instance.runtime.metadata_startup_script, "gcloud storage cp") &&
      strcontains(google_compute_instance.ollama.metadata_startup_script, "portable-model-bundle.json") &&
      strcontains(google_compute_instance.ollama.metadata_startup_script, "ollama_model_blob") &&
      strcontains(google_compute_instance.runtime.metadata_startup_script, "runtime_bundle")
    )
    error_message = "normal launch must consume prepared GCS artifacts on both nodes"
  }

  assert {
    condition = (
      !strcontains(google_compute_instance.ollama.metadata_startup_script, "ollama pull") &&
      !strcontains(google_compute_instance.ollama.metadata_startup_script, "apt-get") &&
      !strcontains(google_compute_instance.runtime.metadata_startup_script, "apt-get") &&
      !strcontains(google_compute_instance.runtime.metadata_startup_script, "git clone") &&
      !strcontains(google_compute_instance.runtime.metadata_startup_script, "rustup") &&
      !strcontains(google_compute_instance.runtime.metadata_startup_script, "cargo build") &&
      !strcontains(google_compute_instance.ollama.metadata_startup_script, "ollama-model-store.tar.gz")
    )
    error_message = "normal launch rendered cold model pull, source checkout, dependency installation, or Rust build work"
  }
}
