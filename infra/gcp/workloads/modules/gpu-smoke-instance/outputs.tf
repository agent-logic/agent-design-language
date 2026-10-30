output "instance_name" {
  description = "Disposable GPU smoke instance name."
  value       = google_compute_instance.gpu_smoke.name
}

output "instance_cleanup_selector" {
  description = "Selector for checking that the per-run disposable instance is gone."
  value       = "labels.issue=494 AND labels.lane=gcp-e AND labels.run_id=${lower(replace(var.run_id, "_", "-"))}"
}

output "startup_log" {
  description = "Startup log path read by the smoke script."
  value       = "/var/log/adl/issue494-gpu-smoke.log"
}

output "readiness_marker" {
  description = "Readiness marker path checked by the smoke script."
  value       = "/var/lib/adl/issue494-startup-complete"
}
