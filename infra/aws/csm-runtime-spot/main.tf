module "runtime_spot" {
  source = "../modules/csm-runtime-spot"

  name_prefix            = local.resource_prefix
  vpc_id                 = var.vpc_id
  subnet_id              = var.subnet_id
  ami_id                 = var.ami_id
  instance_type          = var.instance_type
  spot_max_price         = var.spot_max_price
  runtime_port           = var.runtime_port
  alb_security_group_id  = var.alb_security_group_id
  operator_ingress_cidrs = var.operator_ingress_cidrs
  ssh_ingress_cidrs      = var.ssh_ingress_cidrs
  key_name               = var.key_name
  iam_instance_profile   = var.iam_instance_profile
  user_data              = var.user_data != null ? var.user_data : (var.user_data_file == null ? null : file(var.user_data_file))
  root_volume_size_gb    = var.root_volume_size_gb
  tags                   = local.common_tags
}
