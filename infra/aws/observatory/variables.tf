variable "aws_profile" {
  description = "Approved Agent Logic business AWS profile."
  type        = string
  default     = "agent-logic-admin"

  validation {
    condition     = var.aws_profile == "agent-logic-admin"
    error_message = "This deployment is restricted to the agent-logic-admin business profile."
  }
}

variable "aws_region" {
  description = "Region for the private S3 origin and logs."
  type        = string
  default     = "us-west-2"
}

variable "hosted_zone_name" {
  description = "Existing public Route53 zone."
  type        = string
  default     = "agent-logic.ai"
}

variable "observatory_fqdn" {
  description = "Canonical static Observatory hostname."
  type        = string
  default     = "observatory.csm.agent-logic.ai"

  validation {
    condition     = var.observatory_fqdn == "observatory.csm.agent-logic.ai"
    error_message = "Issue #679 owns only observatory.csm.agent-logic.ai."
  }
}

variable "runtime_connect_origins" {
  description = "Explicit HTTPS/WSS Runtime origins allowed by the Observatory CSP. Never include credentials or URL paths."
  type        = list(string)
  default     = []

  validation {
    condition = alltrue([
      for origin in var.runtime_connect_origins :
      can(regex("^https://[A-Za-z0-9.-]+(:[0-9]+)?$|^wss://[A-Za-z0-9.-]+(:[0-9]+)?$", origin)) &&
      !strcontains(origin, "@")
    ])
    error_message = "Runtime origins must be credential-free HTTPS or WSS origins with no path."
  }
}

variable "artifact_version" {
  description = "Immutable bundle version used for deployments and rollback."
  type        = string

  validation {
    condition     = can(regex("^[A-Za-z0-9._-]+$", var.artifact_version))
    error_message = "artifact_version must be a safe immutable identifier."
  }
}

variable "tags" {
  description = "Additional non-secret resource tags."
  type        = map(string)
  default     = {}
}
