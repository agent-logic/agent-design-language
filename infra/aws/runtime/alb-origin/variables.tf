variable "aws_region" {
  description = "AWS region for the Runtime ALB."
  type        = string
  default     = "us-west-2"
}

variable "expected_aws_account_id" {
  description = "Expected Agent Logic AWS account id. Terraform checks fail closed when the active profile resolves to a different account."
  type        = string

  validation {
    condition     = can(regex("^[0-9]{12}$", var.expected_aws_account_id))
    error_message = "expected_aws_account_id must be a 12 digit AWS account id."
  }
}

variable "expected_terraform_workspace" {
  description = "Expected Terraform workspace for this root, used to keep ALB-origin state separate from private-node state."
  type        = string

  validation {
    condition     = length(trimspace(var.expected_terraform_workspace)) > 0 && var.expected_terraform_workspace != "default"
    error_message = "expected_terraform_workspace must be explicit and must not be default."
  }
}

variable "environment" {
  description = "Deployment environment."
  type        = string
  default     = "dev"
}

variable "csm_name" {
  description = "CSM instance name, for example wuji."
  type        = string
}

variable "zone_name" {
  description = "CSM hosted zone name, for example csm.agent-logic.ai."
  type        = string
  default     = "csm.agent-logic.ai"
}

variable "origin_fqdn_override" {
  description = "Optional explicit origin FQDN. Default is <csm>.<env>.<zone_name>."
  type        = string
  default     = null
}

variable "vpc_id" {
  description = "VPC id."
  type        = string
}

variable "public_subnet_ids" {
  description = "Public subnet ids for the ALB."
  type        = list(string)
}

variable "certificate_arn" {
  description = "Regional ACM cert ARN. AWS-F consumes an existing cert; #122 owns public certificate issuance."
  type        = string
  default     = null
}

variable "reuse_existing_certificate" {
  description = "When certificate_arn is null, look up an existing ISSUED regional ACM certificate."
  type        = bool
  default     = true
}

variable "certificate_lookup_domain" {
  description = "Optional regional ACM lookup domain. Defaults to origin FQDN; may be a wildcard."
  type        = string
  default     = null
}

variable "runtime_port" {
  description = "Runtime HTTPS port on targets."
  type        = number
  default     = 20997
}

variable "target_instance_id" {
  description = "Optional Runtime instance id to attach. Null creates the ALB without a target attachment."
  type        = string
  default     = null
}

variable "health_check_path" {
  description = "Runtime health check path."
  type        = string
  default     = "/v1/health"
}

variable "allowed_ingress_cidrs" {
  description = "CIDRs allowed to reach ALB HTTPS. Defaults closed; explicitly allow CloudFront/origin smoke CIDRs before exposing."
  type        = list(string)
  default     = []
}
