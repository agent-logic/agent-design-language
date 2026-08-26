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
  description = "Deployment environment, for example dev or prod."
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,30}$", var.environment))
    error_message = "environment must be lowercase DNS-safe text."
  }
}

variable "csm_name" {
  description = "CSM instance name, for example axioma."
  type        = string

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,40}$", var.csm_name))
    error_message = "csm_name must be lowercase DNS-safe text."
  }
}

variable "zone_name" {
  description = "Route53 hosted zone name."
  type        = string
  default     = "csm.agent-logic.ai"
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

variable "wss_forward_viewer_host" {
  description = "Whether to forward wss_fqdn as Host to the WSS origin. Default false preserves origin Host/SNI compatibility."
  type        = bool
  default     = false
}

variable "websocket_path_pattern" {
  description = "CloudFront path pattern for public WSS traffic."
  type        = string
  default     = "/v1/observatory/ws*"
}

variable "additional_allowed_origins" {
  description = "Additional exact browser origins allowed for CORS/WSS beyond the Observatory hostname."
  type        = list(string)
  default     = []
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
