# Terraform >= 1.7 mock providers. Plans only; no AWS API or paid resources.
mock_provider "aws" {}

variables {
  name_prefix       = "issue-770-test"
  vpc_id            = "vpc-0123456789abcdef0"
  subnet_id         = "subnet-0123456789abcdef0"
  ami_id            = "ami-0123456789abcdef0"
  key_name          = "existing-approved-key"
  ssh_ingress_cidrs = ["203.0.113.10/32"]
}

run "reject_null_key" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { key_name = null }
  expect_failures = [aws_instance.runtime]
}

run "reject_blank_key" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { key_name = "  " }
  expect_failures = [aws_instance.runtime]
}

run "reject_empty_ssh" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { ssh_ingress_cidrs = [] }
  expect_failures = [aws_instance.runtime]
}

run "reject_null_ssh" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { ssh_ingress_cidrs = null }
  expect_failures = [aws_instance.runtime]
}

run "reject_world_ssh" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { ssh_ingress_cidrs = ["0.0.0.0/0"] }
  expect_failures = [var.ssh_ingress_cidrs]
}

run "reject_invalid_ssh" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables { ssh_ingress_cidrs = ["not-a-cidr"] }
  expect_failures = [var.ssh_ingress_cidrs]
}

run "recoverable_public_node" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  assert {
    condition     = aws_instance.runtime.key_name == "existing-approved-key"
    error_message = "Exactly the selected existing key must be forwarded."
  }
  assert {
    condition     = aws_instance.runtime.associate_public_ip_address && aws_instance.runtime.metadata_options[0].http_tokens == "required" && aws_instance.runtime.root_block_device[0].encrypted && aws_instance.runtime.root_block_device[0].delete_on_termination
    error_message = "Public addressing, IMDSv2 and encrypted disposable root storage must remain intact."
  }
  assert {
    condition     = length(aws_vpc_security_group_ingress_rule.ssh_from_operator) == 1 && aws_vpc_security_group_ingress_rule.ssh_from_operator["203.0.113.10/32"].from_port == 22 && aws_vpc_security_group_ingress_rule.ssh_from_operator["203.0.113.10/32"].to_port == 22 && length(aws_vpc_security_group_ingress_rule.runtime_from_operator) == 0 && length(aws_vpc_security_group_ingress_rule.runtime_from_alb) == 0
    error_message = "SSH recovery must not open application ingress."
  }
}

run "public_root_forwards_recovery_inputs" {
  command = plan
  variables { csm_name = "issue-770-test" }
}

run "application_callers_stay_separate" {
  command = plan
  module { source = "../modules/csm-runtime-spot" }
  variables {
    alb_security_group_id  = "sg-0123456789abcdef0"
    operator_ingress_cidrs = ["198.51.100.20/32"]
  }
  assert {
    condition     = aws_vpc_security_group_ingress_rule.runtime_from_operator["198.51.100.20/32"].from_port == 20997 && aws_vpc_security_group_ingress_rule.runtime_from_operator["198.51.100.20/32"].to_port == 20997 && aws_vpc_security_group_ingress_rule.runtime_from_alb[0].referenced_security_group_id == "sg-0123456789abcdef0" && aws_vpc_security_group_ingress_rule.runtime_from_alb[0].from_port == 20997 && length(aws_vpc_security_group_ingress_rule.ssh_from_operator) == 1
    error_message = "Application ingress must retain only its explicit callers and port."
  }
}
