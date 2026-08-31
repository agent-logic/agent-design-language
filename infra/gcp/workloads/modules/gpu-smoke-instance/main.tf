locals {
  labels = merge(
    var.labels,
    {
      app       = "adl"
      milestone = "v0-92-1"
      issue     = "494"
      lane      = "gcp-e"
      owner     = "agent-logic"
      run_id    = lower(replace(var.run_id, "_", "-"))
      ttl       = "disposable"
    }
  )
}

data "google_compute_subnetwork" "private" {
  name   = var.subnet_name
  region = var.region
}

resource "google_compute_instance" "gpu_smoke" {
  name         = "${var.run_id}-vm"
  machine_type = var.machine_type
  zone         = var.zone
  labels       = local.labels
  tags         = [var.support_id]

  boot_disk {
    auto_delete = true
    initialize_params {
      image = var.boot_image
      size  = 100
      type  = "pd-balanced"
    }
  }

  guest_accelerator {
    type  = var.accelerator_type
    count = var.accelerator_count
  }

  scheduling {
    automatic_restart   = false
    on_host_maintenance = "TERMINATE"
    provisioning_model  = "STANDARD"
  }

  network_interface {
    subnetwork = data.google_compute_subnetwork.private.id
  }

  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }

  metadata = {
    enable-oslogin       = "TRUE"
    adl-issue            = "494"
    adl-run-id           = var.run_id
    adl-max-budget-usd   = tostring(var.max_budget_usd)
    adl-model-name       = var.model_name
    adl-ttl-expires-at   = var.ttl_expires_at
    adl-cleanup-required = "true"
  }

  metadata_startup_script = <<-EOT
    #!/bin/bash
    set -euo pipefail
    install -d -m 0755 /var/lib/adl /var/log/adl
    {
      printf 'issue=494\n'
      printf 'run_id=%s\n' '${var.run_id}'
      printf 'machine_type=%s\n' '${var.machine_type}'
      printf 'accelerator_type=%s\n' '${var.accelerator_type}'
      printf 'accelerator_count=%s\n' '${var.accelerator_count}'
      printf 'model_name=%s\n' '${var.model_name}'
      date -u '+started_at=%Y-%m-%dT%H:%M:%SZ'
      if command -v nvidia-smi >/dev/null 2>&1; then
        nvidia-smi --query-gpu=name,driver_version,memory.total,memory.free --format=csv,noheader || true
      else
        printf 'nvidia_smi=missing\n'
      fi
      free -h || true
      df -h / || true
      date -u '+finished_at=%Y-%m-%dT%H:%M:%SZ'
    } >/var/log/adl/issue494-gpu-smoke.log 2>&1
    touch /var/lib/adl/issue494-startup-complete
  EOT
}
