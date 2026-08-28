variable "name_prefix" {
  description = "Stable name prefix for the disposable Runtime ALB."
  type        = string
}

variable "vpc_id" {
  description = "VPC id for the ALB and target group."
  type        = string
}

variable "subnet_ids" {
  description = "Public subnet ids for the ALB. AWS ALB normally requires at least two AZs."
  type        = list(string)
}

variable "origin_fqdn" {
  description = "Origin DNS name for the ALB, for example wuji.dev.csm.agent-logic.ai."
  type        = string
}

variable "certificate_arn" {
  description = "Regional ACM certificate ARN for origin_fqdn. AWS-F consumes an existing certificate; #122 owns public certificate issuance."
  type        = string
  default     = null
}

variable "reuse_existing_certificate" {
  description = "When certificate_arn is null, look up an existing ISSUED ACM certificate for origin_fqdn."
  type        = bool
  default     = true
}

variable "certificate_lookup_domain" {
  description = "Optional ACM lookup domain. Defaults to origin_fqdn; set to a wildcard such as *.wuji.dev.csm.agent-logic.ai to reuse a wildcard certificate."
  type        = string
  default     = null
}

variable "runtime_port" {
  description = "Runtime HTTPS port on the target."
  type        = number
  default     = 20997
}

variable "target_instance_id" {
  description = "Optional Runtime EC2 instance id to attach. Null creates ALB without a target attachment."
  type        = string
  default     = null
}

variable "health_check_path" {
  description = "Runtime health check path."
  type        = string
  default     = "/v1/health"
}

variable "allowed_ingress_cidrs" {
  description = "CIDRs allowed to reach ALB HTTPS. Defaults closed; operators must explicitly allow CloudFront/origin smoke CIDRs when exposing a public ALB."
  type        = list(string)
  default     = []
}

variable "tags" {
  description = "Additional tags."
  type        = map(string)
  default     = {}
}
