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
    cloudformation_rollback_retained = true
  }
}
