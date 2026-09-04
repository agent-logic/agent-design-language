provider "google" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

locals {
  labels = {
    adl_issue      = "663"
    adl_generation = lower(replace(var.generation, "_", "-"))
    adl_state      = "snapshot-catalog"
    adl_retained   = "true"
  }
}

data "google_compute_subnetwork" "private" {
  name   = var.subnet_name
  region = var.region
}

resource "google_compute_snapshot" "runtime" {
  name        = "adl-663-${var.generation}-runtime"
  source_disk = var.runtime_staging_disk
  labels      = local.labels
}

resource "google_compute_snapshot" "ollama" {
  name        = "adl-663-${var.generation}-ollama"
  source_disk = var.ollama_staging_disk
  labels      = local.labels
}

resource "google_compute_disk" "runtime_verifier" {
  count    = var.enable_verifier ? 1 : 0
  name     = "adl-663-${var.generation}-runtime-verify"
  zone     = var.zone
  type     = "pd-balanced"
  snapshot = google_compute_snapshot.runtime.self_link
  labels   = merge(local.labels, { adl_retained = "false" })
}

resource "google_compute_disk" "ollama_verifier" {
  count    = var.enable_verifier ? 1 : 0
  name     = "adl-663-${var.generation}-ollama-verify"
  zone     = var.zone
  type     = "pd-balanced"
  snapshot = google_compute_snapshot.ollama.self_link
  labels   = merge(local.labels, { adl_retained = "false" })
}

resource "google_compute_instance" "verifier" {
  count        = var.enable_verifier ? 1 : 0
  name         = "adl-663-${var.generation}-snapshot-verifier"
  machine_type = "e2-standard-2"
  zone         = var.zone
  labels       = merge(local.labels, { adl_retained = "false" })

  boot_disk {
    auto_delete = true
    initialize_params { image = var.verification_boot_image }
  }
  attached_disk {
    source      = google_compute_disk.runtime_verifier[0].self_link
    device_name = "adl-runtime-verify"
    mode        = "READ_ONLY"
  }
  attached_disk {
    source      = google_compute_disk.ollama_verifier[0].self_link
    device_name = "adl-ollama-verify"
    mode        = "READ_ONLY"
  }
  network_interface { subnetwork = data.google_compute_subnetwork.private.id }
  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }
  metadata = {
    enable-oslogin              = "TRUE"
    adl-generation              = var.generation
    adl-runtime-manifest-sha256 = var.runtime_manifest_sha256
    adl-ollama-manifest-sha256  = var.ollama_manifest_sha256
    adl-paid-deadline-epoch     = tostring(var.paid_deadline_epoch)
  }
  metadata_startup_script = file("${path.module}/verify-snapshots.sh")
}
