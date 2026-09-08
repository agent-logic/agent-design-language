terraform {
  required_version = ">= 1.5.0"

  backend "gcs" {}
}

variable "canary_nonce" {
  type = string
}

resource "terraform_data" "backend_canary" {
  input = {
    issue = "740"
    nonce = var.canary_nonce
  }
}

output "backend_canary_nonce" {
  value = terraform_data.backend_canary.output.nonce
}
