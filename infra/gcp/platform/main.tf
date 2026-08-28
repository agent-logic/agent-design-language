locals {
  required_labels = merge(var.labels, {
    csm = var.csm_name
    env = var.environment
  })

  workload_service_account_id = "csm-${var.environment}-workload"
  state_bucket_name           = "${var.project_id}-${var.environment}-${var.csm_name}-state"
  artifact_bucket_name        = "${var.project_id}-${var.environment}-${var.csm_name}-artifacts"
  model_bucket_name           = "${var.project_id}-${var.environment}-${var.csm_name}-models"
  evidence_bucket_name        = "${var.project_id}-${var.environment}-${var.csm_name}-continuity-evidence"
  log_bucket_name             = "${var.project_id}-${var.environment}-${var.csm_name}-logs"
}

resource "google_compute_network" "private" {
  name                    = var.network_name
  auto_create_subnetworks = false
  routing_mode            = "REGIONAL"
}

resource "google_compute_subnetwork" "private" {
  name                     = var.subnet_name
  ip_cidr_range            = var.subnet_cidr
  network                  = google_compute_network.private.id
  region                   = var.region
  private_ip_google_access = true
}

resource "google_compute_firewall" "iap_operator_access" {
  name    = "${var.network_name}-iap-operator-access"
  network = google_compute_network.private.name

  allow {
    protocol = "tcp"
    ports    = ["22"]
  }

  source_ranges = [var.iap_tcp_forwarding_cidr]
  target_tags   = ["csm-disposable"]
}

resource "google_compute_firewall" "explicit_private_egress" {
  name      = "${var.network_name}-explicit-private-egress"
  network   = google_compute_network.private.name
  direction = "EGRESS"

  allow {
    protocol = "tcp"
    ports    = ["443"]
  }

  destination_ranges = var.allowed_private_egress_cidrs
  target_tags        = ["csm-disposable"]
}

resource "google_project_metadata_item" "os_login" {
  key   = "enable-oslogin"
  value = "TRUE"
}

resource "google_project_iam_member" "operator_iap_tunnel" {
  project = var.project_id
  role    = "roles/iap.tunnelResourceAccessor"
  member  = "group:${var.operator_group_email}"
}

resource "google_project_iam_member" "operator_os_login" {
  project = var.project_id
  role    = "roles/compute.osLogin"
  member  = "group:${var.operator_group_email}"
}

resource "google_service_account" "workload" {
  account_id   = local.workload_service_account_id
  display_name = "CSM ${var.environment} disposable workload service account"
  description  = "Dedicated workload identity for #493 GCP-D disposable private workloads."
}

resource "google_storage_bucket" "state" {
  name                        = local.state_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  versioning {
    enabled = true
  }
  labels = merge(local.required_labels, { owner = "state" })
}

resource "google_storage_bucket" "artifacts" {
  name                        = local.artifact_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  versioning {
    enabled = true
  }
  labels = merge(local.required_labels, { owner = "artifacts" })
}

resource "google_storage_bucket" "models" {
  name                        = local.model_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  versioning {
    enabled = true
  }
  labels = merge(local.required_labels, { owner = "models" })
}

resource "google_storage_bucket" "continuity_evidence" {
  name                        = local.evidence_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  versioning {
    enabled = true
  }
  labels = merge(local.required_labels, { owner = "continuity-evidence" })
}

resource "google_storage_bucket" "logs" {
  name                        = local.log_bucket_name
  location                    = var.region
  uniform_bucket_level_access = true
  versioning {
    enabled = true
  }
  labels = merge(local.required_labels, { owner = "logs" })
}

resource "google_logging_metric" "disposable_without_deadline" {
  name        = "csm_${var.environment}_disposable_without_deadline"
  description = "Watchdog/readback command support: counts disposable workload log entries missing deadline labels."

  filter = join(" AND ", [
    "resource.type=\"gce_instance\"",
    "labels.issue=\"493\"",
    "labels.ttl=\"disposable\"",
    "-labels.deadline:*",
  ])

  metric_descriptor {
    metric_kind = "DELTA"
    value_type  = "INT64"
  }
}
