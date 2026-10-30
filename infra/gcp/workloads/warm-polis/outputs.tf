output "runtime_instance_name" {
  value = module.two_node_ollama_runtime.runtime_instance_name
}

output "ollama_instance_name" {
  value = module.two_node_ollama_runtime.ollama_instance_name
}

output "runtime_private_ip" {
  value = module.two_node_ollama_runtime.runtime_private_ip
}

output "ollama_private_ip" {
  value = module.two_node_ollama_runtime.ollama_private_ip
}

output "runtime_restored_disk" {
  value = google_compute_disk.runtime.self_link
}

output "ollama_restored_disk" {
  value = google_compute_disk.ollama.self_link
}

output "cleanup_contract" {
  value = {
    launch_state_deletes = [google_compute_disk.runtime.name, google_compute_disk.ollama.name]
    retained_snapshots   = [var.runtime_snapshot, var.ollama_snapshot]
  }
}
