provider "google" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

locals {
  labels = {
    adl_issue      = "663"
    adl_generation = lower(replace(var.generation, "_", "-"))
    adl_state      = "preparation"
  }
}

data "google_compute_subnetwork" "private" {
  name   = var.subnet_name
  region = var.region
}

resource "google_compute_disk" "runtime_staging" {
  name   = "adl-663-${var.generation}-runtime-staging"
  zone   = var.zone
  type   = "pd-balanced"
  size   = var.runtime_disk_size_gib
  labels = local.labels
}

resource "google_compute_disk" "ollama_staging" {
  name   = "adl-663-${var.generation}-ollama-staging"
  zone   = var.zone
  type   = "pd-balanced"
  size   = var.ollama_disk_size_gib
  labels = local.labels
}

resource "google_compute_instance" "runtime_preparation" {
  count        = var.attach_preparation_vms ? 1 : 0
  name         = "adl-663-${var.generation}-runtime-prep"
  machine_type = "e2-standard-4"
  zone         = var.zone
  labels       = local.labels

  boot_disk {
    auto_delete = true
    initialize_params { image = var.preparation_boot_image }
  }
  attached_disk {
    source      = google_compute_disk.runtime_staging.self_link
    device_name = "adl-runtime-staging"
    mode        = "READ_WRITE"
  }
  network_interface { subnetwork = data.google_compute_subnetwork.private.id }
  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }
  metadata = {
    enable-oslogin    = "TRUE"
    adl-generation    = var.generation
    adl-bundle-uri    = var.runtime_bundle_uri
    adl-bundle-sha256 = var.runtime_bundle_sha256
    adl-data-device   = "adl-runtime-staging"
  }
  metadata_startup_script = file("${path.module}/seal-disk.sh")
}

resource "google_compute_instance" "ollama_preparation" {
  count        = var.attach_preparation_vms ? 1 : 0
  name         = "adl-663-${var.generation}-ollama-prep"
  machine_type = "e2-standard-4"
  zone         = var.zone
  labels       = local.labels

  boot_disk {
    auto_delete = true
    initialize_params { image = var.preparation_boot_image }
  }
  attached_disk {
    source      = google_compute_disk.ollama_staging.self_link
    device_name = "adl-ollama-staging"
    mode        = "READ_WRITE"
  }
  network_interface { subnetwork = data.google_compute_subnetwork.private.id }
  service_account {
    email  = var.service_account_email
    scopes = ["https://www.googleapis.com/auth/cloud-platform"]
  }
  metadata = {
    enable-oslogin    = "TRUE"
    adl-generation    = var.generation
    adl-bundle-uri    = var.ollama_bundle_uri
    adl-bundle-sha256 = var.ollama_bundle_sha256
    adl-data-device   = "adl-ollama-staging"
  }
  metadata_startup_script = file("${path.module}/seal-disk.sh")
}

check "snapshot_precondition" {
  assert {
    condition     = var.attach_preparation_vms || (length(google_compute_instance.runtime_preparation) == 0 && length(google_compute_instance.ollama_preparation) == 0)
    error_message = "Snapshot inputs are not detached until attach_preparation_vms=false has been applied."
  }
}
