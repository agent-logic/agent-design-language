locals {
  run_id_label = lower(replace(var.run_id, "_", "-"))

  labels = merge(
    var.labels,
    {
      app       = "adl"
      milestone = "v0-92-1"
      issue     = var.issue_id
      lane      = var.lane
      owner     = "agent-logic"
      run_id    = local.run_id_label
      ttl       = var.retention
    }
  )

  metadata_common = {
    enable-oslogin               = var.enable_oslogin ? "TRUE" : "FALSE"
    adl-issue                    = var.issue_id
    adl-lane                     = var.lane
    adl-run-id                   = var.run_id
    adl-source-revision          = var.source_revision
    adl-max-budget-usd           = tostring(var.max_budget_usd)
    adl-resident-models          = jsonencode(var.resident_models)
    adl-artifact-bucket          = var.artifact_bucket
    adl-artifact-manifest-object = var.artifact_manifest_object
    adl-artifact-manifest-sha256 = var.artifact_manifest_sha256
    adl-ttl-expires-at           = var.ttl_expires_at
    adl-cleanup-required         = "true"
  }
}

data "google_compute_subnetwork" "private" {
  name   = var.subnet_name
  region = var.region
}

resource "google_compute_instance" "ollama" {
  name         = "${var.run_id}-ollama"
  machine_type = var.ollama_machine_type
  zone         = var.zone
  labels       = local.labels
  tags         = [var.support_id, "${var.support_id}-ollama"]

  boot_disk {
    auto_delete = true
    initialize_params {
      image = var.ollama_boot_image
      size  = 200
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

    dynamic "access_config" {
      for_each = var.assign_external_ip ? [1] : []
      content {}
    }
  }

  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }

  metadata = merge(local.metadata_common, {
    adl-node-role               = "ollama-gpu"
    adl-data-device-name        = var.ollama_data_device_name
    adl-artifact-generation     = var.artifact_generation
    adl-content-manifest-sha256 = var.ollama_content_sha256
  })

  metadata_startup_script = var.ollama_startup_script
}

resource "google_compute_instance" "runtime" {
  name         = "${var.run_id}-runtime"
  machine_type = var.runtime_machine_type
  zone         = var.zone
  labels       = local.labels
  tags         = [var.support_id, "${var.support_id}-runtime"]

  boot_disk {
    auto_delete = true
    initialize_params {
      image = var.runtime_boot_image
      size  = 80
      type  = "pd-balanced"
    }
  }

  scheduling {
    automatic_restart   = false
    on_host_maintenance = "MIGRATE"
    provisioning_model  = "STANDARD"
  }

  network_interface {
    subnetwork = data.google_compute_subnetwork.private.id

    dynamic "access_config" {
      for_each = var.assign_external_ip ? [1] : []
      content {}
    }
  }

  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }

  metadata = merge(local.metadata_common, {
    adl-node-role               = "runtime-csm"
    adl-ollama-private-ip       = google_compute_instance.ollama.network_interface[0].network_ip
    adl-data-device-name        = var.runtime_data_device_name
    adl-artifact-generation     = var.artifact_generation
    adl-content-manifest-sha256 = var.runtime_content_sha256
  })

  metadata_startup_script = var.runtime_startup_script
}

resource "google_compute_attached_disk" "runtime_data" {
  count = var.attach_data_disks ? 1 : 0

  disk        = var.runtime_data_disk
  instance    = google_compute_instance.runtime.id
  device_name = var.runtime_data_device_name
}

resource "google_compute_attached_disk" "ollama_data" {
  count = var.attach_data_disks ? 1 : 0

  disk        = var.ollama_data_disk
  instance    = google_compute_instance.ollama.id
  device_name = var.ollama_data_device_name
}
