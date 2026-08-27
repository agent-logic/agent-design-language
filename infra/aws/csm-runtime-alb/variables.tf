variable "aws_region" {
  description = "AWS region for the Runtime ALB."
  type        = string
  default     = "us-west-2"
}

variable "environment" {
  description = "Deployment environment."
  type        = string
  default     = "dev"
}

variable "csm_name" {
  description = "CSM instance name."
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

variable "hosted_zone_id" {
  description = "Route53 hosted zone id for the origin record and ACM DNS validation."
  type        = string
  default     = null
}

variable "vpc_id" {
  description = "VPC id."
  type        = string
}

variable "subnet_ids" {
  description = "Public subnet ids for the ALB."
  type        = list(string)
}

variable "certificate_arn" {
  description = "Regional ACM cert ARN. Reuse one cert across ALB create/destroy cycles; set create_certificate=true only for intentional first-time certificate creation."
  type        = string
  default     = null
}

variable "reuse_existing_certificate" {
  description = "When certificate_arn is null, look up an existing ISSUED regional ACM certificate for the origin FQDN before considering certificate creation."
  type        = bool
  default     = true
}

variable "certificate_lookup_domain" {
  description = "Optional regional ACM lookup domain. Defaults to origin FQDN; set to a wildcard such as *.wuji.dev.csm.agent-logic.ai when reusing a wildcard cert."
  type        = string
  default     = null
}

variable "create_certificate" {
  description = "Create one regional ACM certificate when certificate_arn is null. Requires hosted_zone_id. Keep false for normal ALB recycling."
  type        = bool
  default     = false
}

variable "create_dns_record" {
  description = "Create Route53 alias for the origin FQDN."
  type        = bool
  default     = true
}

variable "runtime_port" {
  description = "Runtime HTTPS port on targets."
  type        = number
  default     = 20997
}

variable "target_instance_id" {
  description = "Optional Runtime instance id to attach."
  type        = string
  default     = null
}

variable "health_check_path" {
  description = "Runtime health check path."
  type        = string
  default     = "/v1/health"
}

variable "allowed_ingress_cidrs" {
  description = "CIDRs allowed to reach the ALB. Defaults closed; explicitly allow CloudFront/origin smoke CIDRs before exposing the public ALB."
  type        = list(string)
  default     = []
}
