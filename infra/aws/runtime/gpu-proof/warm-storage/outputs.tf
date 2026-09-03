output "storage_id" { value = var.storage_id }
output "availability_zone" { value = var.availability_zone }
output "artifact_generation" { value = var.artifact_generation }
output "runtime_volume_id" { value = aws_ebs_volume.runtime.id }
output "runtime_seal_sha256" { value = var.runtime_seal_sha256 }
output "gpu_volume_id" { value = aws_ebs_volume.gpu.id }
output "gpu_seal_sha256" { value = var.gpu_seal_sha256 }
