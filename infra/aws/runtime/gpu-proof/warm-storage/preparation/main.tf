locals {
  scheduler_name     = substr("${var.run_id}-terminate", 0, 64)
  termination_at_utc = trimsuffix(var.termination_at, "Z")
  tags = {
    "adl:issue"            = "607"
    "adl:run-id"           = var.run_id
    "adl:owner-token"      = var.owner_token
    "adl:preparation"      = "true"
    "adl:cleanup-required" = "true"
    "adl:termination-at"   = var.termination_at
  }
  artifact_read_arns = [for key in var.artifact_read_keys : "arn:aws:s3:::${var.artifact_bucket}/${key}"]
}

resource "aws_security_group" "preparation" {
  name_prefix = "adl-i607-prep-"
  description = "Issue 607 preparation SSH only; no Runtime or Ollama ingress"
  vpc_id      = var.vpc_id

  ingress {
    description = "Operator SSH recovery from exact public IPv4 /32"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [var.ssh_ingress_cidr]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = local.tags
}

resource "aws_key_pair" "operator" {
  key_name_prefix = "adl-i607-"
  public_key      = trimspace(var.ssh_public_key)
  tags            = local.tags
}

resource "aws_iam_role" "preparation" {
  name_prefix = "adl-i607-prep-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "ec2.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = local.tags
}

resource "aws_iam_role_policy" "preparation" {
  role = aws_iam_role.preparation.id
  name = "exact-artifact-read-and-receipt-write"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid      = "ReadExactArtifactPrefix"
        Effect   = "Allow"
        Action   = ["s3:GetObject", "s3:GetObjectVersion"]
        Resource = local.artifact_read_arns
      },
      {
        Sid      = "WriteExactReceiptPrefix"
        Effect   = "Allow"
        Action   = "s3:PutObject"
        Resource = "arn:aws:s3:::${var.artifact_bucket}/${var.receipt_write_prefix}*"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ssm_recovery" {
  role       = aws_iam_role.preparation.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "preparation" {
  name_prefix = "adl-i607-prep-"
  role        = aws_iam_role.preparation.name
  tags        = local.tags
}

resource "aws_instance" "runtime_preparation" {
  ami                         = var.runtime_ami_id
  instance_type               = var.runtime_instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  key_name                    = aws_key_pair.operator.key_name
  vpc_security_group_ids      = [aws_security_group.preparation.id]
  iam_instance_profile        = aws_iam_instance_profile.preparation.name
  user_data = templatefile("${path.module}/runtime-user-data.sh.tftpl", {
    region                       = var.aws_region
    artifact_bucket              = var.artifact_bucket
    artifact_manifest_key        = var.artifact_manifest_key
    artifact_manifest_version_id = var.artifact_manifest_version_id
    artifact_manifest_sha256     = var.artifact_manifest_sha256
    source_commit                = var.source_commit
    source_archive_key           = var.source_archive_key
    source_archive_version_id    = var.source_archive_version_id
    source_archive_sha256        = var.source_archive_sha256
    receipt_write_prefix         = var.receipt_write_prefix
    runtime_volume_id            = var.runtime_volume_id
    runtime_ami_id               = var.runtime_ami_id
    runtime_ami_metadata_json    = var.runtime_ami_metadata_json
    ami_metadata_sha256          = var.ami_metadata_sha256
    availability_zone            = var.availability_zone
    artifact_generation          = var.artifact_generation
  })
  user_data_replace_on_change = true

  instance_initiated_shutdown_behavior = "terminate"

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_type           = "gp3"
    volume_size           = var.root_volume_size_gib
  }

  tags = local.tags
}

resource "aws_volume_attachment" "runtime" {
  device_name = "/dev/sdf"
  volume_id   = var.runtime_volume_id
  instance_id = aws_instance.runtime_preparation.id
}

resource "aws_instance" "gpu_preparation" {
  ami                         = var.gpu_ami_id
  instance_type               = var.gpu_instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  key_name                    = aws_key_pair.operator.key_name
  vpc_security_group_ids      = [aws_security_group.preparation.id]
  iam_instance_profile        = aws_iam_instance_profile.preparation.name
  user_data = templatefile("${path.module}/gpu-user-data.sh.tftpl", {
    region                       = var.aws_region
    artifact_bucket              = var.artifact_bucket
    artifact_manifest_key        = var.artifact_manifest_key
    artifact_manifest_version_id = var.artifact_manifest_version_id
    artifact_manifest_sha256     = var.artifact_manifest_sha256
    source_commit                = var.source_commit
    receipt_write_prefix         = var.receipt_write_prefix
    gpu_volume_id                = var.gpu_volume_id
    gpu_ami_id                   = var.gpu_ami_id
    gpu_ami_metadata_json        = var.gpu_ami_metadata_json
    ami_metadata_sha256          = var.ami_metadata_sha256
    availability_zone            = var.availability_zone
    artifact_generation          = var.artifact_generation
  })
  user_data_replace_on_change = true

  instance_initiated_shutdown_behavior = "terminate"

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_type           = "gp3"
    volume_size           = var.root_volume_size_gib
  }

  tags = local.tags
}

resource "aws_volume_attachment" "gpu" {
  device_name = "/dev/sdf"
  volume_id   = var.gpu_volume_id
  instance_id = aws_instance.gpu_preparation.id
}

resource "aws_iam_role" "scheduler" {
  name_prefix = "adl-i607-reaper-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "scheduler.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = local.tags
}

resource "aws_iam_role_policy" "scheduler" {
  role = aws_iam_role.scheduler.id
  name = "terminate-exact-preparation-instance"
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect   = "Allow"
      Action   = "ec2:TerminateInstances"
      Resource = "arn:aws:ec2:${var.aws_region}:${var.aws_account_id}:instance/*"
      Condition = { StringEquals = {
        "ec2:ResourceTag/adl:issue"       = "607"
        "ec2:ResourceTag/adl:run-id"      = var.run_id
        "ec2:ResourceTag/adl:owner-token" = var.owner_token
      } }
    }]
  })
}

resource "aws_scheduler_schedule" "terminate" {
  name                         = local.scheduler_name
  schedule_expression          = "at(${local.termination_at_utc})"
  schedule_expression_timezone = "UTC"
  state                        = "ENABLED"
  action_after_completion      = "DELETE"
  flexible_time_window { mode = "OFF" }
  target {
    arn      = "arn:aws:scheduler:::aws-sdk:ec2:terminateInstances"
    role_arn = aws_iam_role.scheduler.arn
    input    = jsonencode({ InstanceIds = [aws_instance.runtime_preparation.id, aws_instance.gpu_preparation.id] })
  }
}

check "security_inputs" {
  assert {
    condition     = can(regex("^([0-9]{1,3}\\.){3}[0-9]{1,3}/32$", var.ssh_ingress_cidr))
    error_message = "preparation SSH ingress must be one exact IPv4 /32."
  }
  assert {
    condition     = endswith(var.receipt_write_prefix, "/") && alltrue([for key in var.artifact_read_keys : !startswith(key, var.receipt_write_prefix)])
    error_message = "receipt prefix must be a directory distinct from every exact read key."
  }
  assert {
    condition     = data.aws_subnet.selected.availability_zone == var.availability_zone
    error_message = "preparation subnet and retained volumes must share the exact availability zone."
  }
}

data "aws_subnet" "selected" {
  id = var.subnet_id
}

data "aws_ebs_volume" "runtime" {
  filter {
    name   = "volume-id"
    values = [var.runtime_volume_id]
  }
}

data "aws_ebs_volume" "gpu" {
  filter {
    name   = "volume-id"
    values = [var.gpu_volume_id]
  }
}

check "retained_volume_tuple" {
  assert {
    condition = (
      data.aws_ebs_volume.runtime.encrypted && data.aws_ebs_volume.gpu.encrypted &&
      data.aws_ebs_volume.runtime.kms_key_id == var.kms_key_arn && data.aws_ebs_volume.gpu.kms_key_id == var.kms_key_arn &&
      data.aws_ebs_volume.runtime.availability_zone == var.availability_zone && data.aws_ebs_volume.gpu.availability_zone == var.availability_zone &&
      data.aws_ebs_volume.runtime.tags["adl:issue"] == "607" && data.aws_ebs_volume.gpu.tags["adl:issue"] == "607" &&
      data.aws_ebs_volume.runtime.tags["adl:compute-owned"] == "false" && data.aws_ebs_volume.gpu.tags["adl:compute-owned"] == "false"
    )
    error_message = "preparation volume encryption, KMS, AZ, or ownership tags do not match the exact retained tuple."
  }
}
