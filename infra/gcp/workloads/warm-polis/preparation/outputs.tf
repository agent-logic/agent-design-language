output "runtime_staging_disk" { value = google_compute_disk.runtime_staging.self_link }
output "ollama_staging_disk" { value = google_compute_disk.ollama_staging.self_link }
output "detached_for_snapshot" { value = !var.attach_preparation_vms }
