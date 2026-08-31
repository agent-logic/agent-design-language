output "gpu_smoke" {
  description = "Disposable #494 GPU smoke proof selectors."
  value = {
    project_id        = var.project_id
    region            = var.region
    zone              = var.zone
    run_id            = var.run_id
    instance_name     = google_compute_instance.gpu_smoke.name
    machine_type      = var.machine_type
    accelerator_type  = var.accelerator_type
    accelerator_count = var.accelerator_count
    service_account   = google_service_account.gpu_smoke.email
    network_name      = data.google_compute_network.private.name
    subnet_name       = data.google_compute_subnetwork.private.name
    max_budget_usd    = var.max_budget_usd
    ttl_expires_at    = var.ttl_expires_at
    startup_log       = "/var/log/adl/issue494-gpu-smoke.log"
    readiness_marker  = "/var/lib/adl/issue494-startup-complete"
    cleanup_selector  = "labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${lower(replace(var.run_id, "_", "-"))}"
  }
}
