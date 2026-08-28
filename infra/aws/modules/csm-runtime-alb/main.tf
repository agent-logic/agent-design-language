locals {
  lookup_existing_certificate = var.certificate_arn == null && var.reuse_existing_certificate
  certificate_arn = one(concat(
    var.certificate_arn == null ? [] : [var.certificate_arn],
    data.aws_acm_certificate.existing_origin[*].arn
  ))
  tags = merge(var.tags, {
    Name      = "${var.name_prefix}-runtime-alb"
    Component = "runtime-alb"
  })
}

data "aws_acm_certificate" "existing_origin" {
  count       = local.lookup_existing_certificate ? 1 : 0
  domain      = coalesce(var.certificate_lookup_domain, var.origin_fqdn)
  statuses    = ["ISSUED"]
  types       = ["AMAZON_ISSUED"]
  most_recent = true
}

resource "aws_security_group" "alb" {
  name        = "${var.name_prefix}-runtime-alb"
  description = "CSM Runtime origin ALB ingress"
  vpc_id      = var.vpc_id
  tags        = local.tags
}

resource "aws_vpc_security_group_ingress_rule" "https" {
  for_each          = toset(var.allowed_ingress_cidrs)
  security_group_id = aws_security_group.alb.id
  cidr_ipv4         = each.value
  ip_protocol       = "tcp"
  from_port         = 443
  to_port           = 443
  description       = "HTTPS ingress"
}

resource "aws_vpc_security_group_egress_rule" "all" {
  security_group_id = aws_security_group.alb.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
  description       = "ALB outbound"
}

resource "aws_lb" "runtime" {
  name               = "${var.name_prefix}-runtime"
  load_balancer_type = "application"
  internal           = false
  security_groups    = [aws_security_group.alb.id]
  subnets            = var.subnet_ids
  idle_timeout       = 3600
  tags               = local.tags
}

resource "aws_lb_target_group" "runtime" {
  name        = "${var.name_prefix}-runtime"
  port        = var.runtime_port
  protocol    = "HTTPS"
  target_type = "instance"
  vpc_id      = var.vpc_id

  health_check {
    enabled             = true
    protocol            = "HTTPS"
    path                = var.health_check_path
    matcher             = "200-399"
    interval            = 15
    timeout             = 5
    healthy_threshold   = 2
    unhealthy_threshold = 2
  }

  tags = local.tags
}

resource "aws_lb_target_group_attachment" "runtime" {
  count            = var.target_instance_id == null ? 0 : 1
  target_group_arn = aws_lb_target_group.runtime.arn
  target_id        = var.target_instance_id
  port             = var.runtime_port
}

resource "aws_lb_listener" "https" {
  load_balancer_arn = aws_lb.runtime.arn
  port              = 443
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-TLS13-1-2-2021-06"
  certificate_arn   = local.certificate_arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.runtime.arn
  }
}
