output "instance_name" {
  description = "Disposable GPU smoke instance name."
  value       = module.gpu_smoke_instance.instance_name
}

output "instance_cleanup_selector" {
  description = "Selector for checking that the per-run disposable instance is gone."
  value       = module.gpu_smoke_instance.instance_cleanup_selector
}

output "startup_log" {
  description = "Startup log path read by the smoke script."
  value       = module.gpu_smoke_instance.startup_log
}

output "readiness_marker" {
  description = "Readiness marker path checked by the smoke script."
  value       = module.gpu_smoke_instance.readiness_marker
}
