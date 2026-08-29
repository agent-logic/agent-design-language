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

# The three interface endpoints and the S3 gateway endpoint preserve #194's
# private management/artifact path without creating an internet gateway or NAT.
resource "aws_vpc_endpoint" "ssm" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.name}.ssm"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ssm-endpoint" })
}

resource "aws_vpc_endpoint" "ssm_messages" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.name}.ssmmessages"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ssmmessages-endpoint" })
}

resource "aws_vpc_endpoint" "ec2_messages" {
  vpc_id              = aws_vpc.private_runtime.id
  service_name        = "com.amazonaws.${data.aws_region.current.name}.ec2messages"
  vpc_endpoint_type   = "Interface"
  private_dns_enabled = true
  subnet_ids          = aws_subnet.private[*].id
  security_group_ids  = [aws_security_group.ssm_endpoint.id]
  tags                = merge(local.common_tags, { Name = "${var.run_id}-ec2messages-endpoint" })
}

resource "aws_vpc_endpoint" "s3_gateway" {
  vpc_id            = aws_vpc.private_runtime.id
  service_name      = "com.amazonaws.${data.aws_region.current.name}.s3"
  vpc_endpoint_type = "Gateway"
  route_table_ids   = aws_route_table.private[*].id
  tags              = merge(local.common_tags, { Name = "${var.run_id}-s3-gateway-endpoint" })
}

data "aws_region" "current" {}
