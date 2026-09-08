output "runtime_preparation_instance_id" { value = aws_instance.runtime_preparation.id }
output "gpu_preparation_instance_id" { value = aws_instance.gpu_preparation.id }
output "runtime_preparation_public_ip" { value = aws_instance.runtime_preparation.public_ip }
output "gpu_preparation_public_ip" { value = aws_instance.gpu_preparation.public_ip }
output "runtime_volume_id" { value = var.runtime_volume_id }
output "gpu_volume_id" { value = var.gpu_volume_id }
