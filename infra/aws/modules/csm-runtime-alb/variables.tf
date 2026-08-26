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

variable "hosted_zone_id" {
  description = "Optional Route53 hosted zone id for origin_fqdn DNS and ACM validation."
  type        = string
  default     = null
}

variable "create_dns_record" {
  description = "Create an alias record for origin_fqdn pointing at the ALB."
  type        = bool
  default     = true
}

variable "certificate_arn" {
  description = "Regional ACM certificate ARN for origin_fqdn. Reuse one cert across ALB create/destroy cycles; set create_certificate=true only for an intentional first-time cert request."
  type        = string
  default     = null
}

variable "reuse_existing_certificate" {
  description = "When certificate_arn is null, look up an existing ISSUED ACM certificate for origin_fqdn before considering certificate creation."
  type        = bool
  default     = true
}

variable "certificate_lookup_domain" {
  description = "Optional ACM lookup domain. Defaults to origin_fqdn; set to a wildcard such as *.wuji.dev.csm.agent-logic.ai to reuse a wildcard certificate."
  type        = string
  default     = null
}

variable "create_certificate" {
  description = "Request and DNS-validate one regional ACM certificate for origin_fqdn when certificate_arn is null. Keep false for normal ALB recycling."
  type        = bool
  default     = false
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
  description = "CIDRs allowed to reach ALB HTTPS."
  type        = list(string)
  default     = ["0.0.0.0/0"]
}

variable "tags" {
  description = "Additional tags."
  type        = map(string)
  default     = {}
}
