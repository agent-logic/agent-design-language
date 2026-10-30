output "instance_id" {
  value = aws_instance.runtime.id
}

output "instance_private_ip" {
  value = aws_instance.runtime.private_ip
}

output "security_group_id" {
  value = aws_security_group.runtime.id
}

output "runtime_port" {
  value = var.runtime_port
}
