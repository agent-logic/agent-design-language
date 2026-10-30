data "aws_caller_identity" "current" {}

check "aws_account_identity" {
  assert {
    condition     = data.aws_caller_identity.current.account_id == var.expected_aws_account_id
    error_message = "AWS-F private-node must run in the expected Agent Logic AWS account."
  }
}

check "terraform_workspace" {
  assert {
    condition     = terraform.workspace == var.expected_terraform_workspace
    error_message = "AWS-F private-node must run in the expected Terraform workspace."
  }
}

module "private_runtime_node" {
  source = "../modules/private-runtime-node"

  name_prefix           = local.resource_prefix
  vpc_id                = var.vpc_id
  private_subnet_id     = var.private_subnet_id
  ami_id                = var.ami_id
  instance_type         = var.instance_type
  spot_max_price        = var.spot_max_price
  runtime_port          = var.runtime_port
  alb_security_group_id = var.alb_security_group_id
  key_name              = var.key_name
  iam_instance_profile  = var.iam_instance_profile
  user_data             = var.user_data
  root_volume_size_gb   = var.root_volume_size_gb
  tags                  = local.common_tags
}
