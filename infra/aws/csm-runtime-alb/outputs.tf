output "origin_fqdn" {
  value = module.runtime_alb.origin_fqdn
}

output "alb_dns_name" {
  value = module.runtime_alb.alb_dns_name
}

output "alb_security_group_id" {
  value = module.runtime_alb.alb_security_group_id
}

output "target_group_arn" {
  value = module.runtime_alb.target_group_arn
}

output "listener_arn" {
  value = module.runtime_alb.listener_arn
}

output "certificate_arn" {
  value = module.runtime_alb.certificate_arn
}
