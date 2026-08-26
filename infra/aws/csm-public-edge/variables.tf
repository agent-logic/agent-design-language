variable "aws_region" {
  description = "Primary AWS region for regional resources such as API Gateway and logs."
  type        = string
  default     = "us-west-2"
}

variable "approved_aws_account_id" {
  description = "Approved Agent Logic AWS account id supplied out-of-band. Never commit real values."
  type        = string
  sensitive   = true
}

variable "environment" {
  description = "Deployment environment. Non-prod appears in hostnames, for example api.wuji.dev.csm.agent-logic.ai. Prod omits the segment, for example api.wuji.csm.agent-logic.com."
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,30}$", var.environment))
    error_message = "environment must be lowercase DNS-safe text."
  }
}

variable "csm_name" {
  description = "CSM instance name, for example wuji."
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,40}$", var.csm_name))
    error_message = "csm_name must be lowercase DNS-safe text."
  }
}

variable "zone_name" {
  description = "Route53 hosted zone name for the CSM namespace, for example csm.agent-logic.ai for dev/stage or csm.agent-logic.com for prod."
  type        = string
  default     = "csm.agent-logic.ai"
}

variable "create_hosted_zone" {
  description = "Create the CSM Route53 hosted zone in this AWS account. Use for first-time business-account setup, then delegate this zone from the parent domain."
  type        = bool
  default     = false
}

variable "hosted_zone_id" {
  description = "Existing Route53 hosted zone id for zone_name. Leave null to look up zone_name or create it when create_hosted_zone is true."
  type        = string
  default     = null
}

variable "edge_acm_certificate_arn" {
  description = "Optional existing us-east-1 ACM certificate ARN for the CloudFront viewer aliases. Leave null for Terraform to request exact SANs."
  type        = string
  default     = null
}

variable "observatory_asset_source" {
  description = "Local static Observatory build output directory. Used by validators; Terraform does not upload local files in this first module."
  type        = string
}

variable "runtime_origin_mode" {
  description = "How API Gateway reaches the Runtime HTTP origin."
  type        = string

  validation {
    condition = contains([
      "external_https",
      "aws_alb_public",
      "aws_alb_private"
    ], var.runtime_origin_mode)
    error_message = "runtime_origin_mode must be external_https, aws_alb_public, or aws_alb_private."
  }
}

variable "runtime_origin_url" {
  description = "HTTPS Runtime HTTP origin for external_https or aws_alb_public."
  type        = string
  default     = null
}

variable "private_alb_listener_arn" {
  description = "Private ALB/NLB listener for aws_alb_private mode. Placeholder for later private HTTP integration."
  type        = string
  default     = null
}

variable "wss_origin_mode" {
  description = "How the WSS CloudFront distribution reaches the native realtime Runtime origin."
  type        = string

  validation {
    condition = contains([
      "external_wss",
      "aws_alb_public_wss"
    ], var.wss_origin_mode)
    error_message = "wss_origin_mode must be external_wss or aws_alb_public_wss."
  }
}

variable "wss_origin_https_url" {
  description = "HTTPS custom-origin endpoint for WSS. The origin must accept WebSocket upgrade; public viewer URL remains wss://."
  type        = string
}

variable "wss_origin_hostname" {
  description = "Origin hostname used by CloudFront for WSS Host/SNI/TLS validation. Must match the origin certificate."
  type        = string
}

variable "origin_cname_target" {
  description = "Optional DNS CNAME target for origin_fqdn, for example an existing DDNS hostname such as wuji.agent-logic.ai. Leave null when DNS is managed elsewhere."
  type        = string
  default     = null

  validation {
    condition     = var.origin_cname_target == null || can(regex("^[A-Za-z0-9][A-Za-z0-9.-]*\\.?$", var.origin_cname_target))
    error_message = "origin_cname_target must be a DNS hostname without scheme or path."
  }
}

variable "wss_forward_viewer_host" {
  description = "Whether to forward wss_fqdn as Host to the WSS origin. Default false preserves origin Host/SNI compatibility."
  type        = bool
  default     = false
}

variable "websocket_path_pattern" {
  description = "CloudFront path pattern for public WSS traffic."
  type        = string
  default     = "/v1/observatory/ws*"

  validation {
    condition     = startswith(var.websocket_path_pattern, "/") && strcontains(var.websocket_path_pattern, "*")
    error_message = "websocket_path_pattern must be a CloudFront path pattern such as /v1/observatory/ws*."
  }
}

variable "additional_allowed_origins" {
  description = "Additional exact browser origins allowed for CORS/WSS beyond the Observatory hostname. Use exact scheme://host[:port] origins only; wildcards and paths are rejected."
  type        = list(string)
  default     = []

  validation {
    condition = alltrue([
      for origin in var.additional_allowed_origins :
      !strcontains(origin, "*")
      && (
        can(regex("^https://[A-Za-z0-9][A-Za-z0-9.-]*(:[0-9]{1,5})?$", origin))
        || can(regex("^http://localhost(:[0-9]{1,5})?$", origin))
      )
    ])
    error_message = "additional_allowed_origins must contain exact https://host[:port] origins, or http://localhost[:port] for local development; wildcards, paths, and patterns are not allowed."
  }
}

variable "waf_rate_limit" {
  description = "WAF rate limit per five-minute window."
  type        = number
  default     = 2000
}

variable "log_retention_days" {
  description = "CloudWatch log retention for API Gateway logs."
  type        = number
  default     = 14
}

variable "wss_forwarded_query_strings" {
  description = "Declared-safe query strings to forward to the WSS origin."
  type        = list(string)
  default     = []
}

variable "wss_forwarded_cookies" {
  description = "Declared-safe cookies to forward to the WSS origin."
  type        = list(string)
  default     = []
}
