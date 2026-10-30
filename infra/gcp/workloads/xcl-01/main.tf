locals {
  labels = merge(
    var.labels,
    {
      adl_issue            = "495"
      adl_source_issues    = "194-268"
      adl_run_id           = lower(replace(var.run_id, "_", "-"))
      adl_cleanup_required = "true"
      adl_ttl_expires_at   = lower(replace(var.ttl_expires_at, ":", "-"))
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

resource "google_storage_bucket_iam_member" "runtime_artifact_read" {
  count = var.artifact_bucket == null ? 0 : 1

  bucket = var.artifact_bucket
  role   = "roles/storage.objectViewer"
  member = "serviceAccount:${google_service_account.runtime.email}"
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
    enable-oslogin       = "TRUE"
    ssh-keys             = var.operator_ssh_public_key == null ? null : "adl:${var.operator_ssh_public_key}"
    adl-artifact-bucket  = var.artifact_bucket
    adl-artifact-prefix  = var.artifact_prefix
    adl-ttl-expires-at   = var.ttl_expires_at
    adl-cleanup-required = "true"
  }

  metadata_startup_script = <<-EOT
    #!/bin/bash
    set -euo pipefail
    install -d -m 0755 /var/lib/adl /opt/adl-runtime /opt/adl-build-cache
    printf '%s\n' '${var.run_id}' >/var/lib/adl/xcl-01-run-id
    printf '%s\n' '${var.artifact_bucket == null ? "" : var.artifact_bucket}/${var.artifact_prefix}' >/var/lib/adl/xcl-01-artifact-source
    cat >/usr/local/sbin/adl-issue268-mount-runtime <<'SCRIPT'
    #!/bin/bash
    set -euo pipefail
    mount_path="/opt/adl-runtime"
    device="/dev/disk/by-id/google-${var.retained_runtime_disk_device_name}"
    install -d -m 0755 "$${mount_path}"
    for _ in $(seq 1 90); do
      if [ -e "$${device}" ]; then
        break
      fi
      sleep 2
    done
    if [ ! -e "$${device}" ]; then
      echo "retained Runtime disk device not found: $${device}" >&2
      exit 1
    fi
    fs_type="$(blkid -o value -s TYPE "$${device}" || true)"
    fs_uuid="$(blkid -o value -s UUID "$${device}" || true)"
    if [ -z "$${fs_type}" ] || [ -z "$${fs_uuid}" ]; then
      echo "retained Runtime disk must already contain a filesystem" >&2
      exit 1
    fi
    if ! grep -q "UUID=$${fs_uuid} $${mount_path} " /etc/fstab; then
      printf 'UUID=%s %s %s defaults,nofail 0 2\n' "$${fs_uuid}" "$${mount_path}" "$${fs_type}" >>/etc/fstab
    fi
    mount "$${mount_path}"
    case "$${fs_type}" in
      xfs) command -v xfs_growfs >/dev/null 2>&1 && xfs_growfs "$${mount_path}" || true ;;
      ext2|ext3|ext4) command -v resize2fs >/dev/null 2>&1 && resize2fs "$${device}" || true ;;
    esac
    test -d "$${mount_path}/runtime/install"
    touch /var/lib/adl/issue268-bootstrap-ready
SCRIPT
    chmod 0755 /usr/local/sbin/adl-issue268-mount-runtime
    /usr/local/sbin/adl-issue268-mount-runtime
  EOT

  depends_on = [
    google_storage_bucket_iam_member.runtime_artifact_read
  ]
}

resource "google_compute_attached_disk" "runtime_retained" {
  disk        = var.retained_runtime_disk
  instance    = google_compute_instance.runtime.id
  device_name = var.retained_runtime_disk_device_name
}
