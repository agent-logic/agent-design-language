output "portable_contract" {
  value = {
    provider              = "gcp"
    network_id            = google_compute_network.runtime.id
    private_subnet_id     = google_compute_subnetwork.runtime_private.id
    service_account       = google_service_account.runtime.email
    runtime_instance      = google_compute_instance.runtime.id
    retained_disk         = var.retained_runtime_disk
    retained_disk_device  = var.retained_runtime_disk_device_name
    artifact_source       = var.artifact_bucket == null ? null : "gs://${var.artifact_bucket}/${var.artifact_prefix}"
    cleanup_deadline      = var.ttl_expires_at
    runtime_mount_path    = "/opt/adl-runtime"
    readiness_marker      = "/var/lib/adl/issue268-bootstrap-ready"
    readiness_command     = "test -f /var/lib/adl/issue268-bootstrap-ready && mountpoint -q /opt/adl-runtime && test -d /opt/adl-runtime/runtime/install"
    private_google_access = true
  }
}
