output "runtime_instance_id" { value = aws_instance.runtime.id }
output "runtime_public_ip" { value = aws_instance.runtime.public_ip }
output "gpu_instance_id" { value = aws_instance.gpu.id }
output "gpu_public_ip" { value = aws_instance.gpu.public_ip }

output "gpu_private_ip" {
  description = "Private Ollama address injected into Runtime cloud-init."
  value       = aws_instance.gpu.private_ip
}

output "key_pair_name" { value = aws_key_pair.operator.key_name }
output "termination_schedule_arn" { value = aws_scheduler_schedule.terminate.arn }
output "termination_at" { value = var.termination_at }
