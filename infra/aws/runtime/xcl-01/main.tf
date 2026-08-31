locals {
  private_network_cidr = "10.194.0.0/16"
  private_subnet_cidrs = ["10.194.10.0/24", "10.194.20.0/24"]

  common_tags = merge(
    var.common_tags,
    {
      "adl:issue"            = "495"
      "adl:source_issues"    = "194,268"
      "adl:run_id"           = var.run_id
      "adl:cleanup_required" = "true"
      "adl:ttl_expires_at"   = var.ttl_expires_at
    }
  )
}

data "aws_partition" "current" {}

resource "aws_vpc" "private_runtime" {
  cidr_block           = local.private_network_cidr
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-private-vpc"
  })
}

resource "aws_subnet" "private" {
  count = 2

  vpc_id                  = aws_vpc.private_runtime.id
  cidr_block              = local.private_subnet_cidrs[count.index]
  availability_zone       = var.availability_zones[count.index]
  map_public_ip_on_launch = false

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-private-${count.index + 1}"
  })
}

resource "aws_route_table" "private" {
  count = 2

  vpc_id = aws_vpc.private_runtime.id

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-private-rt-${count.index + 1}"
  })
}

resource "aws_route_table_association" "private" {
  count = 2

  subnet_id      = aws_subnet.private[count.index].id
  route_table_id = aws_route_table.private[count.index].id
}

resource "aws_security_group" "runtime_instance" {
  name        = "${var.run_id}-runtime-private-sg"
  description = "Issue #495/#194 private Runtime nodes: no public ingress."
  vpc_id      = aws_vpc.private_runtime.id

  tags = merge(local.common_tags, {
    Name                          = "${var.run_id}-runtime-private-sg"
    "adl:public_runtime_exposure" = "false"
    "adl:hosted_model_fallback"   = "false"
  })
}

resource "aws_security_group" "ssm_endpoint" {
  name        = "${var.run_id}-ssm-endpoint-sg"
  description = "Issue #495 SSM endpoint access from private Runtime nodes only."
  vpc_id      = aws_vpc.private_runtime.id

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-ssm-endpoint-sg"
  })
}

resource "aws_vpc_security_group_egress_rule" "instance_to_endpoint" {
  security_group_id            = aws_security_group.runtime_instance.id
  referenced_security_group_id = aws_security_group.ssm_endpoint.id
  ip_protocol                  = "tcp"
  from_port                    = 443
  to_port                      = 443
  description                  = "HTTPS to private management endpoints only."
}

resource "aws_vpc_security_group_ingress_rule" "endpoint_from_instance" {
  security_group_id            = aws_security_group.ssm_endpoint.id
  referenced_security_group_id = aws_security_group.runtime_instance.id
  ip_protocol                  = "tcp"
  from_port                    = 443
  to_port                      = 443
  description                  = "Private Runtime nodes to endpoint security group."
}

resource "aws_vpc_security_group_ingress_rule" "private_voter_mesh" {
  security_group_id            = aws_security_group.runtime_instance.id
  referenced_security_group_id = aws_security_group.runtime_instance.id
  ip_protocol                  = "-1"
  description                  = "Private voter-to-voter mesh; no public ingress."
}

resource "aws_vpc_security_group_egress_rule" "private_voter_mesh" {
  security_group_id            = aws_security_group.runtime_instance.id
  referenced_security_group_id = aws_security_group.runtime_instance.id
  ip_protocol                  = "-1"
  description                  = "Private voter-to-voter mesh; no public egress."
}

resource "aws_vpc_security_group_egress_rule" "instance_to_s3_gateway" {
  security_group_id = aws_security_group.runtime_instance.id
  prefix_list_id    = var.s3_prefix_list_id
  ip_protocol       = "tcp"
  from_port         = 443
  to_port           = 443
  description       = "HTTPS to the regional S3 prefix list; route target is the private S3 gateway endpoint."
}

# The three interface endpoints and the S3 gateway endpoint preserve #194's
# private management/artifact path without creating an internet gateway or NAT.
resource "aws_vpc_endpoint" "ssm" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.region}.ssm"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ssm-endpoint" })
}

resource "aws_vpc_endpoint" "ssm_messages" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.region}.ssmmessages"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ssmmessages-endpoint" })
}

resource "aws_vpc_endpoint" "ec2_messages" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.region}.ec2messages"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ec2messages-endpoint" })
}

resource "aws_vpc_endpoint" "s3_gateway" {
  vpc_id            = aws_vpc.private_runtime.id
  service_name      = "com.amazonaws.${data.aws_region.current.region}.s3"
  vpc_endpoint_type = "Gateway"
  route_table_ids   = aws_route_table.private[*].id
  tags              = merge(local.common_tags, { Name = "${var.run_id}-s3-gateway-endpoint" })
}

resource "aws_key_pair" "operator_break_glass" {
  count = var.operator_ssh_public_key == null ? 0 : 1

  key_name   = "${var.run_id}-operator-break-glass"
  public_key = var.operator_ssh_public_key

  tags = merge(local.common_tags, {
    Name                   = "${var.run_id}-operator-break-glass"
    "adl:credential_class" = "public-key-only"
  })
}

resource "aws_vpc_security_group_ingress_rule" "operator_break_glass_ssh" {
  count = var.operator_ssh_ingress_cidr == null ? 0 : 1

  security_group_id = aws_security_group.runtime_instance.id
  cidr_ipv4         = var.operator_ssh_ingress_cidr
  ip_protocol       = "tcp"
  from_port         = 22
  to_port           = 22
  description       = "Operator break-glass SSH restricted to one /32 CIDR for #268 parity."

  lifecycle {
    precondition {
      condition     = var.operator_ssh_public_key != null
      error_message = "operator_ssh_public_key is required when operator_ssh_ingress_cidr is set."
    }
  }
}

resource "aws_iam_role" "runtime_host" {
  name = "${var.run_id}-runtime-host-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
        Action = "sts:AssumeRole"
      }
    ]
  })

  managed_policy_arns = [
    "arn:${data.aws_partition.current.partition}:iam::aws:policy/AmazonSSMManagedInstanceCore"
  ]

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-runtime-host-role"
  })
}

resource "aws_iam_role_policy" "runtime_host_bootstrap_read" {
  name = "${var.run_id}-immutable-bootstrap-read"
  role = aws_iam_role.runtime_host.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "ReadPinnedBootstrapObjects"
        Effect = "Allow"
        Action = [
          "s3:GetObject",
          "s3:GetObjectVersion"
        ]
        Resource = "arn:${data.aws_partition.current.partition}:s3:::${var.bootstrap_bucket}/${var.bootstrap_prefix}*"
      }
    ]
  })
}

resource "aws_iam_instance_profile" "runtime_host" {
  name = "${var.run_id}-runtime-host-profile"
  role = aws_iam_role.runtime_host.name

  tags = merge(local.common_tags, {
    Name = "${var.run_id}-runtime-host-profile"
  })
}

resource "aws_instance" "runtime_host" {
  ami                         = var.runtime_ami_id
  instance_type               = var.qualification_instance_type
  subnet_id                   = aws_subnet.private[0].id
  vpc_security_group_ids      = [aws_security_group.runtime_instance.id]
  iam_instance_profile        = aws_iam_instance_profile.runtime_host.name
  associate_public_ip_address = false
  key_name                    = var.operator_ssh_public_key == null ? null : aws_key_pair.operator_break_glass[0].key_name

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
    instance_metadata_tags      = "enabled"
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_size           = 64
    volume_type           = "gp3"
  }

  user_data = <<-EOT
    #!/bin/bash
    set -euo pipefail
    install -d -m 0755 /var/lib/adl /opt/adl-build-cache /opt/adl-runtime /etc/adl
    exec > >(tee -a /var/log/adl-issue268-bootstrap.log) 2>&1
    cat >/etc/adl/issue268-runtime.env <<'EOF'
    ADL_RUNTIME_CONTINUITY_ROOT=/opt/adl-runtime/runtime
    ADL_ISSUE268_RETAINED_RUNTIME_ROOT=/opt/adl-runtime/runtime/state/${var.run_id}
    ADL_ISSUE268_BUILD_CACHE_ROOT=/opt/adl-build-cache
    ADL_ISSUE268_BOOTSTRAP_BUCKET=${var.bootstrap_bucket}
    ADL_ISSUE268_BOOTSTRAP_PREFIX=${var.bootstrap_prefix}
    OLLAMA_MODELS=/opt/adl-runtime/runtime/install/current/ollama-models
    EOF
    chmod 0644 /etc/adl/issue268-runtime.env
    if command -v systemctl >/dev/null 2>&1; then
      systemctl enable --now amazon-ssm-agent || true
    fi
    touch /var/lib/adl/issue268-bootstrap-ready
  EOT

  user_data_replace_on_change = true

  tags = merge(local.common_tags, {
    Name                    = "${var.run_id}-runtime-host"
    "adl:source_issue"      = "268"
    "adl:purchase_option"   = "on_demand"
    "adl:runtime_mount"     = "/opt/adl-runtime"
    "adl:bootstrap_marker"  = "/var/lib/adl/issue268-bootstrap-ready"
    "adl:bootstrap_log"     = "/var/log/adl-issue268-bootstrap.log"
    "adl:qualification_ttl" = var.ttl_expires_at
  })

  lifecycle {
    precondition {
      condition     = var.qualification_instance_type == "r7i.2xlarge"
      error_message = "Issue #495 preserves #268's on-demand r7i.2xlarge qualification shape; change requires a new reviewed denominator."
    }
  }
}

resource "aws_instance" "optional_voter" {
  count = var.launch_voters ? 2 : 0

  ami                         = var.runtime_ami_id
  instance_type               = var.runtime_instance_type
  subnet_id                   = aws_subnet.private[count.index].id
  vpc_security_group_ids      = [aws_security_group.runtime_instance.id]
  iam_instance_profile        = aws_iam_instance_profile.runtime_host.name
  associate_public_ip_address = false
  key_name                    = var.operator_ssh_public_key == null ? null : aws_key_pair.operator_break_glass[0].key_name

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
    instance_metadata_tags      = "enabled"
  }

  root_block_device {
    delete_on_termination = true
    encrypted             = true
    volume_size           = 32
    volume_type           = "gp3"
  }

  user_data = <<-EOT
    #!/bin/bash
    set -euo pipefail
    install -d -m 0755 /var/lib/adl/issue194
    printf '%s %s\n' '${var.run_id}' 'aws-voter-${count.index == 0 ? "a" : "b"}' >/var/lib/adl/issue194/node.txt
  EOT

  user_data_replace_on_change = true

  tags = merge(local.common_tags, {
    Name                          = "${var.run_id}-aws-voter-${count.index == 0 ? "a" : "b"}"
    "adl:source_issue"            = "194"
    "adl:node_id"                 = "aws-voter-${count.index == 0 ? "a" : "b"}"
    "adl:component"               = "private-wuji-aws-recovery"
    "adl:public_runtime_exposure" = "false"
    "adl:hosted_model_fallback"   = "false"
  })

  depends_on = [
    aws_vpc_endpoint.ssm,
    aws_vpc_endpoint.ssm_messages,
    aws_vpc_endpoint.ec2_messages,
    aws_vpc_endpoint.s3_gateway
  ]
}

resource "aws_volume_attachment" "retained_runtime" {
  count = var.runtime_volume_id == null ? 0 : 1

  device_name = "/dev/sdf"
  instance_id = aws_instance.runtime_host.id
  volume_id   = var.runtime_volume_id
}

data "aws_region" "current" {}
