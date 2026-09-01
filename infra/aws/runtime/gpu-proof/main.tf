locals {
  scheduler_name            = substr("${var.run_id}-terminate", 0, 64)
  termination_at_utc        = trimsuffix(var.termination_at, "Z")
  artifact_read_arns        = [for key in var.artifact_read_keys : "arn:aws:s3:::${var.artifact_bucket}/${key}"]
  gpu_receipt_arn           = "arn:aws:s3:::${var.artifact_bucket}/${var.artifact_prefix}runs/${var.run_id}/gpu-ready.json"
  runtime_receipt_arn       = "arn:aws:s3:::${var.artifact_bucket}/${var.artifact_prefix}runs/${var.run_id}/runtime-final.json"
  runtime_local_receipt_arn = "arn:aws:s3:::${var.artifact_bucket}/${var.artifact_prefix}runs/${var.run_id}/runtime-local-ready.json"
  qualification_receipt_arn = "arn:aws:s3:::${var.artifact_bucket}/${var.artifact_prefix}runs/${var.run_id}/qualification-complete.json"
  warm_enabled              = var.runtime_warm_volume_id != null

  run_tags = {
    "adl:issue"            = tostring(var.issue_number)
    "adl:run-id"           = var.run_id
    "adl:owner-token"      = var.owner_token
    "adl:managed-deadline" = "true"
    "adl:termination-at"   = var.termination_at
    "adl:max-hourly-usd"   = tostring(var.authorized_max_hourly_usd)
    "adl:max-total-usd"    = tostring(var.authorized_max_total_usd)
  }
}

data "aws_subnet" "selected" {
  id = var.subnet_id
}

data "aws_ebs_volume" "runtime_warm" {
  count = var.runtime_warm_volume_id == null ? 0 : 1
  filter {
    name   = "volume-id"
    values = [var.runtime_warm_volume_id]
  }
}

data "aws_ebs_volume" "gpu_warm" {
  count = var.gpu_warm_volume_id == null ? 0 : 1
  filter {
    name   = "volume-id"
    values = [var.gpu_warm_volume_id]
  }
}

check "warm_volume_tuple" {
  assert {
    condition = (
      (var.runtime_warm_volume_id == null && var.gpu_warm_volume_id == null && var.warm_volume_availability_zone == null && var.runtime_warm_seal_sha256 == null && var.gpu_warm_seal_sha256 == null) ||
      (var.runtime_warm_volume_id != null && var.gpu_warm_volume_id != null && var.warm_volume_availability_zone != null && var.runtime_warm_seal_sha256 != null && var.gpu_warm_seal_sha256 != null)
    )
    error_message = "warm volume IDs, exact AZ, and both seal digests must be supplied together or all omitted."
  }

  assert {
    condition     = var.warm_volume_availability_zone == null || data.aws_subnet.selected.availability_zone == var.warm_volume_availability_zone
    error_message = "selected subnet and retained warm volumes must be in the same availability zone."
  }

  assert {
    condition = (
      !local.warm_enabled ||
      (var.issue_number == 607 && var.warm_artifact_generation != null && var.warm_source_commit != null)
    )
    error_message = "warm launch requires issue 607 plus exact artifact generation and source commit."
  }
  assert {
    condition = !local.warm_enabled || (
      var.warm_kms_key_arn != null &&
      data.aws_ebs_volume.runtime_warm[0].encrypted && data.aws_ebs_volume.gpu_warm[0].encrypted &&
      data.aws_ebs_volume.runtime_warm[0].kms_key_id == var.warm_kms_key_arn && data.aws_ebs_volume.gpu_warm[0].kms_key_id == var.warm_kms_key_arn &&
      data.aws_ebs_volume.runtime_warm[0].availability_zone == var.warm_volume_availability_zone && data.aws_ebs_volume.gpu_warm[0].availability_zone == var.warm_volume_availability_zone &&
      data.aws_ebs_volume.runtime_warm[0].tags["adl:issue"] == "607" && data.aws_ebs_volume.gpu_warm[0].tags["adl:issue"] == "607" &&
      data.aws_ebs_volume.runtime_warm[0].tags["adl:compute-owned"] == "false" && data.aws_ebs_volume.gpu_warm[0].tags["adl:compute-owned"] == "false" &&
      data.aws_ebs_volume.runtime_warm[0].tags["adl:artifact-generation"] == var.warm_artifact_generation && data.aws_ebs_volume.gpu_warm[0].tags["adl:artifact-generation"] == var.warm_artifact_generation &&
      data.aws_ebs_volume.runtime_warm[0].tags["adl:seal-sha256"] == var.runtime_warm_seal_sha256 && data.aws_ebs_volume.gpu_warm[0].tags["adl:seal-sha256"] == var.gpu_warm_seal_sha256
    )
    error_message = "live warm-volume AZ, KMS, ownership, generation, or seal tags do not match the authorized launch tuple."
  }
}

resource "aws_security_group" "runtime" {
  name_prefix = "${substr(var.run_id, 0, 28)}-runtime-"
  description = "SSH-only public ingress for the ADL qualification Runtime node"
  vpc_id      = var.vpc_id

  ingress {
    description = "Operator SSH recovery from the authorized public IPv4 address"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [var.ssh_ingress_cidr]
  }

  egress {
    description = "Runtime bootstrap, artifacts, and private Ollama calls"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(local.run_tags, { Name = "${var.run_id}-runtime" })
}

resource "aws_security_group" "gpu" {
  name_prefix = "${substr(var.run_id, 0, 32)}-gpu-"
  description = "SSH recovery plus private Runtime-to-Ollama ingress for the ADL qualification"
  vpc_id      = var.vpc_id

  ingress {
    description = "Operator SSH recovery from the authorized public IPv4 address"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [var.ssh_ingress_cidr]
  }

  ingress {
    description     = "Ollama only from the Runtime security group"
    from_port       = 11434
    to_port         = 11434
    protocol        = "tcp"
    security_groups = [aws_security_group.runtime.id]
  }

  egress {
    description = "GPU bootstrap and artifact egress"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = merge(local.run_tags, { Name = "${var.run_id}-gpu" })
}

resource "aws_key_pair" "operator" {
  key_name_prefix = "adl-i${var.issue_number}-"
  public_key      = trimspace(var.ssh_public_key)
  tags            = merge(local.run_tags, { Name = "${var.run_id}-operator" })
}

resource "aws_iam_role" "runtime" {
  name_prefix = "adl-i${var.issue_number}-runtime-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "ec2.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = merge(local.run_tags, { Name = "${var.run_id}-runtime" })
}

resource "aws_iam_role" "gpu" {
  name_prefix = "adl-i${var.issue_number}-gpu-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "ec2.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = merge(local.run_tags, { Name = "${var.run_id}-gpu" })
}

resource "aws_iam_role_policy" "runtime_artifacts" {
  name   = "issue${var.issue_number}-exact-artifacts-and-runtime-receipt"
  role   = aws_iam_role.runtime.id
  policy = local.runtime_artifact_policy

  lifecycle {
    precondition {
      condition     = alltrue([for key in var.artifact_read_keys : !strcontains(key, "/locks/")])
      error_message = "artifact_read_keys must exclude controller lock objects. Exact object ARNs are enforced by IAM."
    }
  }
}

resource "aws_iam_role_policy" "gpu_artifacts" {
  name   = "issue${var.issue_number}-exact-artifacts-and-gpu-receipt"
  role   = aws_iam_role.gpu.id
  policy = local.gpu_artifact_policy


  lifecycle {
    precondition {
      condition     = alltrue([for key in var.artifact_read_keys : !strcontains(key, "/locks/")])
      error_message = "artifact_read_keys must exclude controller lock objects. Exact object ARNs are enforced by IAM."
    }
  }
}

locals {
  artifact_read_statement = {
    Sid    = "ReadIssue345Artifacts"
    Effect = "Allow"
    Action = [
      "s3:GetObject",
      "s3:GetObjectVersion"
    ]
    Resource = local.artifact_read_arns
  }

  runtime_artifact_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [local.artifact_read_statement, {
      Sid      = "WriteOnlyRuntimeReceipts"
      Effect   = "Allow"
      Action   = "s3:PutObject"
      Resource = [local.runtime_receipt_arn, local.runtime_local_receipt_arn, local.qualification_receipt_arn]
    }]
  })

  gpu_artifact_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [local.artifact_read_statement, {
      Sid      = "WriteOnlyGpuReadyReceipt"
      Effect   = "Allow"
      Action   = "s3:PutObject"
      Resource = local.gpu_receipt_arn
    }]
  })
}

resource "aws_iam_role_policy_attachment" "runtime_ssm_recovery" {
  role       = aws_iam_role.runtime.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_role_policy_attachment" "gpu_ssm_recovery" {
  role       = aws_iam_role.gpu.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "runtime" {
  name_prefix = "adl-i${var.issue_number}-runtime-"
  role        = aws_iam_role.runtime.name
  tags        = merge(local.run_tags, { Name = "${var.run_id}-runtime" })
  depends_on  = [aws_iam_role_policy.runtime_artifacts, aws_iam_role_policy_attachment.runtime_ssm_recovery]
}

resource "aws_iam_instance_profile" "gpu" {
  name_prefix = "adl-i${var.issue_number}-gpu-"
  role        = aws_iam_role.gpu.name
  tags        = merge(local.run_tags, { Name = "${var.run_id}-gpu" })
  depends_on  = [aws_iam_role_policy.gpu_artifacts, aws_iam_role_policy_attachment.gpu_ssm_recovery]
}

resource "aws_instance" "gpu" {
  ami                         = var.gpu_ami_id
  instance_type               = var.gpu_instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  monitoring                  = var.detailed_monitoring
  iam_instance_profile        = aws_iam_instance_profile.gpu.name
  key_name                    = aws_key_pair.operator.key_name
  vpc_security_group_ids      = [aws_security_group.gpu.id]

  instance_initiated_shutdown_behavior = "terminate"
  user_data = local.warm_enabled ? templatefile("${path.module}/warm-gpu-user-data.sh.tftpl", {
    run_id              = var.run_id
    region              = var.aws_region
    artifact_bucket     = var.artifact_bucket
    ready_key           = "${var.artifact_prefix}runs/${var.run_id}/gpu-ready.json"
    volume_id           = var.gpu_warm_volume_id
    root_hash           = var.gpu_warm_seal_sha256
    artifact_generation = var.warm_artifact_generation
    source_commit       = var.warm_source_commit
  }) : var.gpu_user_data
  user_data_replace_on_change = true

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
    instance_metadata_tags      = "enabled"
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_type           = "gp3"
    volume_size           = var.gpu_root_volume_size_gib
    iops                  = var.gpu_root_volume_iops
    throughput            = var.gpu_root_volume_throughput_mibps
    tags                  = merge(local.run_tags, { Name = "${var.run_id}-gpu-root" })
  }

  tags = merge(local.run_tags, {
    Name          = "${var.run_id}-gpu"
    "adl:node"    = "gpu"
    "adl:service" = "ollama"
  })

}

resource "aws_instance" "runtime" {
  ami                         = var.runtime_ami_id
  instance_type               = var.runtime_instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  monitoring                  = var.detailed_monitoring
  iam_instance_profile        = aws_iam_instance_profile.runtime.name
  key_name                    = aws_key_pair.operator.key_name
  vpc_security_group_ids      = [aws_security_group.runtime.id]

  instance_initiated_shutdown_behavior = "terminate"
  user_data = local.warm_enabled ? templatefile("${path.module}/warm-runtime-user-data.sh.tftpl", {
    run_id              = var.run_id
    region              = var.aws_region
    artifact_bucket     = var.artifact_bucket
    gpu_ready_key       = "${var.artifact_prefix}runs/${var.run_id}/gpu-ready.json"
    runtime_ready_key   = "${var.artifact_prefix}runs/${var.run_id}/runtime-local-ready.json"
    qualification_key   = "${var.artifact_prefix}runs/${var.run_id}/qualification-complete.json"
    volume_id           = var.runtime_warm_volume_id
    gpu_volume_id       = var.gpu_warm_volume_id
    root_hash           = var.runtime_warm_seal_sha256
    gpu_root_hash       = var.gpu_warm_seal_sha256
    artifact_generation = var.warm_artifact_generation
    source_commit       = var.warm_source_commit
    gpu_private_ip      = aws_instance.gpu.private_ip
  }) : replace(var.runtime_user_data, "__GPU_PRIVATE_IP__", aws_instance.gpu.private_ip)
  user_data_replace_on_change = true

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
    instance_metadata_tags      = "enabled"
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_type           = "gp3"
    volume_size           = var.runtime_root_volume_size_gib
    iops                  = var.runtime_root_volume_iops
    throughput            = var.runtime_root_volume_throughput_mibps
    tags                  = merge(local.run_tags, { Name = "${var.run_id}-runtime-root" })
  }

  tags = merge(local.run_tags, {
    Name          = "${var.run_id}-runtime"
    "adl:node"    = "runtime"
    "adl:service" = "guardian-runtime-agents"
  })
}

resource "aws_volume_attachment" "runtime_warm" {
  count = var.runtime_warm_volume_id == null ? 0 : 1

  device_name                    = var.runtime_warm_device_name
  volume_id                      = var.runtime_warm_volume_id
  instance_id                    = aws_instance.runtime.id
  force_detach                   = false
  stop_instance_before_detaching = true
}

resource "aws_volume_attachment" "gpu_warm" {
  count = var.gpu_warm_volume_id == null ? 0 : 1

  device_name                    = var.gpu_warm_device_name
  volume_id                      = var.gpu_warm_volume_id
  instance_id                    = aws_instance.gpu.id
  force_detach                   = false
  stop_instance_before_detaching = true
}

resource "aws_iam_role" "scheduler" {
  name_prefix = "adl-i${var.issue_number}-reaper-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "scheduler.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = local.run_tags
}

resource "aws_iam_role_policy" "scheduler_terminate" {
  name = "terminate-only-owned-issue${var.issue_number}-instances"
  role = aws_iam_role.scheduler.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid      = "TerminateOnlyOwnedIssueInstances"
      Effect   = "Allow"
      Action   = "ec2:TerminateInstances"
      Resource = "arn:aws:ec2:${var.aws_region}:${var.aws_account_id}:instance/*"
      Condition = {
        StringEquals = {
          "ec2:ResourceTag/adl:issue"       = tostring(var.issue_number)
          "ec2:ResourceTag/adl:run-id"      = var.run_id
          "ec2:ResourceTag/adl:owner-token" = var.owner_token
        }
      }
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
    input    = jsonencode({ InstanceIds = [aws_instance.runtime.id, aws_instance.gpu.id] })

    retry_policy {
      maximum_event_age_in_seconds = 300
      maximum_retry_attempts       = 3
    }
  }

  depends_on = [aws_iam_role_policy.scheduler_terminate]
}
