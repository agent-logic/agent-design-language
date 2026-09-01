locals {
  retained_tags = {
    "adl:issue"               = "607"
    "adl:storage-id"          = var.storage_id
    "adl:owner-token"         = var.owner_token
    "adl:artifact-generation" = var.artifact_generation
    "adl:retained"            = "true"
    "adl:compute-owned"       = "false"
  }
}

resource "aws_ebs_volume" "runtime" {
  availability_zone = var.availability_zone
  encrypted         = true
  kms_key_id        = var.kms_key_arn
  type              = "gp3"
  size              = var.runtime_size_gib
  iops              = var.runtime_iops
  throughput        = var.runtime_throughput_mibps

  tags = merge(local.retained_tags, {
    Name              = "${var.storage_id}-runtime"
    "adl:node"        = "runtime"
    "adl:seal-sha256" = var.runtime_seal_sha256
  })

  lifecycle {
    prevent_destroy = true
  }
}

resource "aws_ebs_volume" "gpu" {
  availability_zone = var.availability_zone
  encrypted         = true
  kms_key_id        = var.kms_key_arn
  type              = "gp3"
  size              = var.gpu_size_gib
  iops              = var.gpu_iops
  throughput        = var.gpu_throughput_mibps

  tags = merge(local.retained_tags, {
    Name              = "${var.storage_id}-gpu"
    "adl:node"        = "gpu"
    "adl:seal-sha256" = var.gpu_seal_sha256
  })

  lifecycle {
    prevent_destroy = true
  }
}

check "qualification_performance_floor" {
  assert {
    condition = (
      var.runtime_size_gib >= 80 && var.runtime_iops >= 3000 && var.runtime_throughput_mibps >= 125 &&
      var.gpu_size_gib >= 200 && var.gpu_iops >= 3000 && var.gpu_throughput_mibps >= 500
    )
    error_message = "warm storage is below the reviewed qualification size/IOPS/throughput floor."
  }
}
