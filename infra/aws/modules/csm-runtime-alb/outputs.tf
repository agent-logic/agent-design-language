output "alb_arn" {
  value = aws_lb.runtime.arn
}

output "alb_dns_name" {
  value = aws_lb.runtime.dns_name
}

output "alb_zone_id" {
  value = aws_lb.runtime.zone_id
}

output "alb_security_group_id" {
  value = aws_security_group.alb.id
}

output "target_group_arn" {
  value = aws_lb_target_group.runtime.arn
}

output "listener_arn" {
  value = aws_lb_listener.https.arn
}

output "origin_fqdn" {
  value = var.origin_fqdn
}

output "certificate_arn" {
  value = local.certificate_arn
}
