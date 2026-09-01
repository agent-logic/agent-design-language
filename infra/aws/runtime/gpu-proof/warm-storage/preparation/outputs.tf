output "preparation_instance_id" { value = aws_instance.preparation.id }
output "preparation_public_ip" { value = aws_instance.preparation.public_ip }
output "termination_schedule_arn" { value = aws_scheduler_schedule.terminate.arn }
output "runtime_volume_id" { value = var.runtime_volume_id }
output "gpu_volume_id" { value = var.gpu_volume_id }
