variable "project_id" {
  description = "Company GCP project that owns the private platform foundation."
  type        = string
}

variable "region" {
  description = "Region for the private subnet and regional control surfaces."
  type        = string
  default     = "us-west2"
}

variable "environment" {
  description = "Environment label for disposable GCP-D resources."
  type        = string
  default     = "dev"
}

variable "csm_name" {
  description = "CSM identifier used in labels and resource names."
  type        = string
  default     = "platform"
}

variable "network_name" {
  description = "Private custom-mode VPC name."
  type        = string
  default     = "csm-dev-private"
}

variable "subnet_name" {
  description = "Regional private subnet name."
  type        = string
  default     = "csm-dev-private-us-west2"
}

variable "subnet_cidr" {
  description = "CIDR range for private disposable workloads."
  type        = string
  default     = "10.42.0.0/24"
}

variable "iap_tcp_forwarding_cidr" {
  description = "Google IAP TCP forwarding source range for operator access."
  type        = string
  default     = "35.235.240.0/20"
}

variable "operator_group_email" {
  description = "Corporate group allowed to use IAP and OS Login for operator access."
  type        = string
  default     = "gcp-admins@agent-logic.ai"
}

variable "allowed_private_egress_cidrs" {
  description = "Explicit egress posture for private workloads; defaults to Google private API VIP only."
  type        = list(string)
  default     = ["199.36.153.8/30"]
}

variable "labels" {
  description = "Required labels applied to GCP-D resources."
  type        = map(string)
  default = {
    app       = "adl"
    milestone = "v0-92-1"
    issue     = "493"
    lane      = "gcp-d"
    owner     = "agent-logic"
    ttl       = "disposable"
  }
}
