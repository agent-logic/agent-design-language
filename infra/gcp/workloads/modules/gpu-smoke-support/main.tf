data "google_compute_network" "private" {
  name = var.network_name
}

resource "google_service_account" "gpu_smoke" {
  account_id   = substr("${var.support_id}-gpu", 0, 30)
  display_name = "ADL #494 GCP GPU smoke support"
  description  = "Stable support identity for repeated #494 L4 GPU smoke instances."
}

resource "google_compute_firewall" "iap_ssh" {
  name    = "${var.support_id}-iap-ssh"
  network = data.google_compute_network.private.name

  allow {
    protocol = "tcp"
    ports    = ["22"]
  }

  source_ranges = var.ssh_source_ranges
  target_tags   = [var.support_id]
}
