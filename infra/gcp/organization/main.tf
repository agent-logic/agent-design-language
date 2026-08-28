locals {
  billing_account_name = startswith(var.billing_account_id, "billingAccounts/") ? var.billing_account_id : "billingAccounts/${var.billing_account_id}"
  corporate_member     = "group:${var.corporate_owner_group}"
  required_labels      = merge(var.labels, { managed_by = "terraform" })
}

data "google_project" "host" {
  project_id = var.host_project_id
}

resource "google_project_iam_member" "corporate_owner_project_viewer" {
  project = var.host_project_id
  role    = "roles/viewer"
  member  = local.corporate_member
}

resource "google_project_iam_member" "corporate_owner_iam_reviewer" {
  project = var.host_project_id
  role    = "roles/iam.securityReviewer"
  member  = local.corporate_member
}

resource "google_billing_budget" "host_project_guardrail" {
  billing_account = local.billing_account_name
  display_name    = "ADL v0.92.1 GCP-C host-project budget"

  amount {
    specified_amount {
      currency_code = "USD"
      units         = tostring(var.monthly_budget_amount_usd)
    }
  }

  budget_filter {
    projects = ["projects/${data.google_project.host.number}"]
    labels = {
      "issue" = "492"
    }
  }

  threshold_rules {
    threshold_percent = 0.5
  }

  threshold_rules {
    threshold_percent = 0.9
  }

  threshold_rules {
    threshold_percent = 1.0
  }
}
