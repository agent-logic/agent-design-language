data "aws_caller_identity" "current" {}

check "aws_account_identity" {
  assert {
    condition     = data.aws_caller_identity.current.account_id == var.expected_aws_account_id
    error_message = "AWS-F alb-origin must run in the expected Agent Logic AWS account."
  }
}

check "terraform_workspace" {
  assert {
    condition     = terraform.workspace == var.expected_terraform_workspace
    error_message = "AWS-F alb-origin must run in the expected Terraform workspace."
  }
}

module "runtime_alb" {
  source = "../../modules/csm-runtime-alb"

  name_prefix                = local.resource_prefix
  vpc_id                     = var.vpc_id
  subnet_ids                 = var.public_subnet_ids
  origin_fqdn                = local.origin_fqdn
  certificate_arn            = var.certificate_arn
  reuse_existing_certificate = var.reuse_existing_certificate
  certificate_lookup_domain  = var.certificate_lookup_domain
  runtime_port               = var.runtime_port
  target_instance_id         = var.target_instance_id
  health_check_path          = var.health_check_path
  allowed_ingress_cidrs      = var.allowed_ingress_cidrs
  tags                       = local.common_tags
}
