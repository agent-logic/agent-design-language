output "runtime_instance_name" {
  description = "Disposable Runtime/CSM instance name."
  value       = module.two_node_ollama_runtime.runtime_instance_name
}

output "ollama_instance_name" {
  description = "Disposable Ollama/GPU instance name."
  value       = module.two_node_ollama_runtime.ollama_instance_name
}

output "runtime_private_ip" {
  description = "Runtime/CSM node private IPv4 address."
  value       = module.two_node_ollama_runtime.runtime_private_ip
}

output "ollama_private_ip" {
  description = "Ollama/GPU node private IPv4 address."
  value       = module.two_node_ollama_runtime.ollama_private_ip
}

output "instance_cleanup_selector" {
  description = "Selector for checking that the per-run disposable instances are gone."
  value       = module.two_node_ollama_runtime.instance_cleanup_selector
}

output "cloud_nat_name" {
  description = "Disposable Cloud NAT name when NAT bootstrap egress is enabled."
  value       = try(google_compute_router_nat.issue509[0].name, null)
}

output "cloud_router_name" {
  description = "Disposable Cloud Router name when NAT bootstrap egress is enabled."
  value       = try(google_compute_router.issue509[0].name, null)
}

output "runtime_receipt_path" {
  description = "Runtime-side qualification receipt path."
  value       = module.two_node_ollama_runtime.runtime_receipt_path
}

output "ollama_receipt_path" {
  description = "Ollama-side model residency receipt path."
  value       = module.two_node_ollama_runtime.ollama_receipt_path
}
