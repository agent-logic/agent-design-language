module "runtime_alb" {
  source = "../../modules/csm-runtime-alb"

  name_prefix                = local.resource_prefix
  vpc_id                     = var.vpc_id
  subnet_ids                 = var.public_subnet_ids
  origin_fqdn                = local.origin_fqdn
  hosted_zone_id             = var.hosted_zone_id
  create_dns_record          = var.create_dns_record
  certificate_arn            = var.certificate_arn
  reuse_existing_certificate = var.reuse_existing_certificate
  certificate_lookup_domain  = var.certificate_lookup_domain
  create_certificate         = var.create_certificate
  runtime_port               = var.runtime_port
  target_instance_id         = var.target_instance_id
  health_check_path          = var.health_check_path
  allowed_ingress_cidrs      = var.allowed_ingress_cidrs
  tags                       = local.common_tags
}
