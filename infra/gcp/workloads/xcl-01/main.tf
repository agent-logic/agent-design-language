locals {
  labels = merge(
    var.labels,
    {
      adl_issue            = "495"
      adl_source_issues    = "194-268"
      adl_run_id           = lower(replace(var.run_id, "_", "-"))
      adl_cleanup_required = "true"
    }
  )
}
provider "google" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

resource "google_compute_network" "runtime" {
  name                    = var.network_name
  auto_create_subnetworks = false
}

resource "google_compute_subnetwork" "runtime_private" {
  name                     = "${var.run_id}-runtime-private"
  ip_cidr_range            = var.subnet_cidr
  region                   = var.region
  network                  = google_compute_network.runtime.id
  private_ip_google_access = true
}

resource "google_service_account" "runtime" {
  account_id   = "${var.run_id}-runtime"
  display_name = "ADL XCL-01 Runtime workload"
}

resource "google_compute_firewall" "runtime_internal" {
  name    = "${var.run_id}-runtime-internal"
  network = google_compute_network.runtime.name

  allow {
    protocol = "tcp"
    ports    = ["22", "443"]
  }

  source_ranges = [var.subnet_cidr]
  target_tags   = ["adl-xcl-01-runtime"]
}

resource "google_compute_instance" "runtime" {
  name         = "${var.run_id}-runtime-host"
  machine_type = var.machine_type
  zone         = var.zone
  labels       = local.labels
  tags         = ["adl-xcl-01-runtime"]

  boot_disk {
    initialize_params {
      image = var.boot_image
      size  = 64
      type  = "pd-balanced"
    }
  }

  network_interface {
    subnetwork = google_compute_subnetwork.runtime_private.id
  }

  service_account {
    email  = google_service_account.runtime.email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }

  metadata = {
    enable-oslogin = "TRUE"
    ssh-keys       = var.operator_ssh_public_key == null ? null : "adl:${var.operator_ssh_public_key}"
  }

  metadata_startup_script = <<-EOT
    #!/bin/bash
    set -euo pipefail
    install -d -m 0755 /var/lib/adl /opt/adl-runtime /opt/adl-build-cache
    printf '%s\n' '${var.run_id}' >/var/lib/adl/xcl-01-run-id
    touch /var/lib/adl/issue268-bootstrap-ready
  EOT
}

resource "google_compute_attached_disk" "runtime_retained" {
  count = var.retained_runtime_disk == null ? 0 : 1

  disk     = var.retained_runtime_disk
  instance = google_compute_instance.runtime.id
}
