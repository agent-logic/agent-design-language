terraform {
  required_version = ">= 1.7.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.95"
    }
  }
}

# Supply the account returned by the approved business profile STS verification.
provider "aws" {
  profile             = "agent-logic-admin"
  region              = "us-west-2"
  allowed_account_ids = [var.company_account_id]
  default_tags {
    tags = {
      Project     = "codefriend"
      Environment = "invited-beta"
      Coordinator = "ADL-936"
      ManagedBy   = "Terraform"
    }
  }
}

resource "aws_security_group" "beta" {
  name_prefix = "codefriend-beta-"
  description = "HTTPS beta ingress and HTTP ACME; administration through SSM"
  vpc_id      = var.vpc_id
  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  ingress {
    description = "ACME HTTP challenge and HTTPS redirect"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  egress {
    description = "Package updates, SSM, OAuth, certificate and model services"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_iam_role" "beta" {
  name_prefix = "codefriend-beta-"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "ec2.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "ssm" {
  role       = aws_iam_role.beta.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "beta" {
  name_prefix = "codefriend-beta-"
  role        = aws_iam_role.beta.name
}

resource "aws_instance" "beta" {
  # Canonical Ubuntu 24.04 amd64; owner 099720109477 verified by DescribeImages.
  ami                                  = data.aws_ami.pinned.id
  instance_type                        = "t3.small"
  instance_initiated_shutdown_behavior = "stop"
  subnet_id                            = var.subnet_id
  associate_public_ip_address          = false
  vpc_security_group_ids               = [aws_security_group.beta.id]
  iam_instance_profile                 = aws_iam_instance_profile.beta.name
  depends_on                           = [aws_iam_role_policy_attachment.ssm]
  credit_specification {
    cpu_credits = "standard"
  }
  metadata_options {
    http_endpoint               = "enabled"
    http_tokens                 = "required"
    http_put_response_hop_limit = 1
    instance_metadata_tags      = "disabled"
  }
  root_block_device {
    volume_type           = "gp3"
    volume_size           = 32
    encrypted             = true
    delete_on_termination = false
  }
  tags = { Name = "codefriend-invited-beta" }
  lifecycle {
    precondition {
      condition     = data.aws_subnet.selected.vpc_id == var.vpc_id
      error_message = "The selected subnet must belong to the reviewed VPC."
    }
    prevent_destroy = true
  }
}

resource "aws_eip" "beta" {
  domain = "vpc"
  tags   = { Name = "codefriend-invited-beta" }
}

resource "aws_eip_association" "beta" {
  instance_id   = aws_instance.beta.id
  allocation_id = aws_eip.beta.id
}

output "instance_id" { value = aws_instance.beta.id }
output "public_ip" { value = aws_eip.beta.public_ip }

# No DNS, provider invocation, secret, certificate or application activation here.
