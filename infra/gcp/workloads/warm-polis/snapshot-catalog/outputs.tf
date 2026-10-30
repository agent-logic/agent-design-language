output "generation" { value = var.generation }
output "runtime_snapshot" { value = google_compute_snapshot.runtime.self_link }
output "ollama_snapshot" { value = google_compute_snapshot.ollama.self_link }
output "runtime_snapshot_id" { value = google_compute_snapshot.runtime.id }
output "ollama_snapshot_id" { value = google_compute_snapshot.ollama.id }
output "verification_resources_enabled" { value = var.enable_verifier }
