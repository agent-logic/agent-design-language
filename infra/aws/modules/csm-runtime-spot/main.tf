data "aws_ami" "amazon_linux_2023" {
  count       = var.ami_id == null ? 1 : 0
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["al2023-ami-2023*-x86_64"]
  }

  filter {
    name   = "architecture"
    values = ["x86_64"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }
}

locals {
  ami_id = var.ami_id == null ? data.aws_ami.amazon_linux_2023[0].id : var.ami_id
  tags = merge(var.tags, {
    Name      = "${var.name_prefix}-runtime"
    Component = "runtime-spot"
  })
}

resource "aws_security_group" "runtime" {
  name        = "${var.name_prefix}-runtime"
  description = "Disposable CSM Runtime host ingress"
  vpc_id      = var.vpc_id
  tags        = local.tags
}

resource "aws_vpc_security_group_ingress_rule" "runtime_from_alb" {
  count                        = var.alb_security_group_id == null ? 0 : 1
  security_group_id            = aws_security_group.runtime.id
  referenced_security_group_id = var.alb_security_group_id
  ip_protocol                  = "tcp"
  from_port                    = var.runtime_port
  to_port                      = var.runtime_port
  description                  = "Runtime HTTPS from ALB"
}

resource "aws_vpc_security_group_ingress_rule" "runtime_from_operator" {
  for_each          = toset(var.operator_ingress_cidrs)
  security_group_id = aws_security_group.runtime.id
  cidr_ipv4         = each.value
  ip_protocol       = "tcp"
  from_port         = var.runtime_port
  to_port           = var.runtime_port
  description       = "Operator Runtime HTTPS smoke access"
}

resource "aws_vpc_security_group_ingress_rule" "ssh_from_operator" {
  for_each          = toset(var.ssh_ingress_cidrs)
  security_group_id = aws_security_group.runtime.id
  cidr_ipv4         = each.value
  ip_protocol       = "tcp"
  from_port         = 22
  to_port           = 22
  description       = "Optional operator SSH"
}

resource "aws_vpc_security_group_egress_rule" "all" {
  security_group_id = aws_security_group.runtime.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
  description       = "Runtime outbound"
}

resource "aws_instance" "runtime" {
  ami                         = local.ami_id
  instance_type               = var.instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  vpc_security_group_ids      = [aws_security_group.runtime.id]
  key_name                    = var.key_name
  iam_instance_profile        = var.iam_instance_profile
  user_data                   = var.user_data

  instance_market_options {
    market_type = "spot"

    spot_options {
      max_price                      = var.spot_max_price
      spot_instance_type             = "one-time"
      instance_interruption_behavior = "terminate"
    }
  }

  root_block_device {
    volume_size           = var.root_volume_size_gb
    volume_type           = "gp3"
    encrypted             = true
    delete_on_termination = true
  }

  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 2
  }

  tags = local.tags
}
