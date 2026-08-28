variable "organization_id" {
  description = "Accepted Agent Logic GCP organization numeric id."
  type        = string
  default     = "321515087273"

  validation {
    condition     = var.organization_id == "321515087273"
    error_message = "GCP-C is scoped only to organization 321515087273."
  }
}

variable "foundation_folder_id" {
  description = "Accepted long-term foundation folder numeric id."
  type        = string
  default     = "929563862525"

  validation {
    condition     = var.foundation_folder_id == "929563862525"
    error_message = "GCP-C is scoped only to foundation folder 929563862525."
  }
}

variable "host_project_id" {
  description = "Accepted company host project for the organization/billing baseline."
  type        = string
  default     = "cs-host-377d41e71a824f92802120"

  validation {
    condition     = var.host_project_id == "cs-host-377d41e71a824f92802120"
    error_message = "GCP-C is scoped only to cs-host-377d41e71a824f92802120."
  }
}

variable "billing_account_id" {
  description = "Accepted billing account id, with or without billingAccounts/ prefix."
  type        = string
  default     = "01FA88-CC4968-ADF817"

  validation {
    condition     = var.billing_account_id == "01FA88-CC4968-ADF817" || var.billing_account_id == "billingAccounts/01FA88-CC4968-ADF817"
    error_message = "GCP-C is scoped only to billing account 01FA88-CC4968-ADF817."
  }
}

variable "corporate_owner_group" {
  description = "Corporate Google Group granted baseline ownership visibility and administration on the host project."
  type        = string
  default     = "gcp-admins@agent-logic.ai"
}

variable "region" {
  description = "Accepted primary region for first workload planning."
  type        = string
  default     = "us-west2"
}

variable "monthly_budget_amount_usd" {
  description = "Notification budget amount for the host-project baseline."
  type        = number
  default     = 20
}

variable "labels" {
  description = "Required cost-attribution labels for #492-managed resources."
  type        = map(string)
  default = {
    app       = "adl"
    milestone = "v0-92-1"
    issue     = "492"
    lane      = "gcp-c"
  }
}
