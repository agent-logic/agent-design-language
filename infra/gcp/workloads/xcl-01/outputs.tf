output "portable_contract" {
  value = {
    provider              = "gcp"
    network_id            = google_compute_network.runtime.id
    private_subnet_id     = google_compute_subnetwork.runtime_private.id
    service_account       = google_service_account.runtime.email
    runtime_instance      = google_compute_instance.runtime.id
    retained_disk         = var.retained_runtime_disk
    runtime_mount_path    = "/opt/adl-runtime"
    readiness_marker      = "/var/lib/adl/issue268-bootstrap-ready"
    private_google_access = true
  }
}
