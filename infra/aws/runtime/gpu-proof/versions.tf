terraform {
  required_version = ">= 1.10.0, < 2.0.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }
  }
}

provider "aws" {
  region              = var.aws_region
  profile             = var.aws_profile
  allowed_account_ids = [var.aws_account_id]

  default_tags {
    tags = {
      "adl:issue"       = "345"
      "adl:run-id"      = var.run_id
      "adl:owner-token" = var.owner_token
      ManagedBy         = "terraform"
    }
  }
}
