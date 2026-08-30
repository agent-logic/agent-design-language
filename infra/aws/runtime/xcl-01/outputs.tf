output "portable_contract" {
  value = {
    provider                = "aws"
    vpc_id                  = aws_vpc.private_runtime.id
    private_subnet_ids      = aws_subnet.private[*].id
    runtime_security_group  = aws_security_group.runtime_instance.id
    endpoint_security_group = aws_security_group.ssm_endpoint.id
    private_service_endpoints = {
      ssm         = aws_vpc_endpoint.ssm.id
      ssmmessages = aws_vpc_endpoint.ssm_messages.id
      ec2messages = aws_vpc_endpoint.ec2_messages.id
      s3_gateway  = aws_vpc_endpoint.s3_gateway.id
    }
    runtime_node_ids = [
      aws_instance.runtime_host.id
    ]
    retained_volume_id               = var.runtime_volume_id
    runtime_mount_path               = "/opt/adl-runtime"
    bootstrap_log_path               = "/var/log/adl-issue268-bootstrap.log"
    readiness_marker_path            = "/var/lib/adl/issue268-bootstrap-ready"
    readiness_command                = "test -f /var/lib/adl/issue268-bootstrap-ready"
    iam_role_name                    = aws_iam_role.runtime_host.name
    iam_instance_profile             = aws_iam_instance_profile.runtime_host.name
    cloudformation_rollback_retained = true
  }
}
