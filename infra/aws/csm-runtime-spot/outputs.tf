output "instance_id" {
  value = module.runtime_spot.instance_id
}

output "instance_private_ip" {
  value = module.runtime_spot.instance_private_ip
}

output "instance_public_ip" {
  value = module.runtime_spot.instance_public_ip
}

output "instance_public_dns" {
  value = module.runtime_spot.instance_public_dns
}

output "security_group_id" {
  value = module.runtime_spot.security_group_id
}

output "runtime_port" {
  value = module.runtime_spot.runtime_port
}
