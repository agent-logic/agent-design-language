output "instance_id" {
  value = module.private_runtime_node.instance_id
}

output "instance_private_ip" {
  value = module.private_runtime_node.instance_private_ip
}

output "security_group_id" {
  value = module.private_runtime_node.security_group_id
}

output "runtime_port" {
  value = module.private_runtime_node.runtime_port
}
