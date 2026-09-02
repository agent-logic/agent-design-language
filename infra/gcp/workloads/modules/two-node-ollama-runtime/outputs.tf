output "runtime_instance_name" {
  description = "Disposable Runtime/CSM instance name."
  value       = google_compute_instance.runtime.name
}

output "ollama_instance_name" {
  description = "Disposable Ollama/GPU instance name."
  value       = google_compute_instance.ollama.name
}

output "runtime_private_ip" {
  description = "Runtime/CSM node private IPv4 address."
  value       = google_compute_instance.runtime.network_interface[0].network_ip
}

output "ollama_private_ip" {
  description = "Ollama/GPU node private IPv4 address."
  value       = google_compute_instance.ollama.network_interface[0].network_ip
}

output "instance_cleanup_selector" {
  description = "Selector for checking that the per-run disposable instances are gone."
  value       = "labels.issue=509 AND labels.lane=drt-d AND labels.run_id=${local.run_id_label}"
}

output "runtime_receipt_path" {
  description = "Runtime-side qualification receipt path."
  value       = "/var/lib/adl/issue509/final.json"
}

output "ollama_receipt_path" {
  description = "Ollama-side model residency receipt path."
  value       = "/var/lib/adl/issue509/ollama-ready.json"
}
