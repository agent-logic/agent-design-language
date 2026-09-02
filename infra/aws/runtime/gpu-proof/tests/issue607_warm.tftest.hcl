mock_provider "aws" {}

override_data {
  target = data.aws_subnet.selected
  values = {
    availability_zone = "us-west-2a"
  }
}

override_data {
  target = data.aws_ebs_volume.runtime_warm[0]
  values = {
    encrypted         = true
    kms_key_id        = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
    availability_zone = "us-west-2a"
    tags = {
      "adl:issue"               = "607"
      "adl:compute-owned"       = "false"
      "adl:artifact-generation" = "issue607-test-generation"
      "adl:seal-sha256"         = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    }
  }
}

override_data {
  target = data.aws_ebs_volume.gpu_warm[0]
  values = {
    encrypted         = true
    kms_key_id        = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
    availability_zone = "us-west-2a"
    tags = {
      "adl:issue"               = "607"
      "adl:compute-owned"       = "false"
      "adl:artifact-generation" = "issue607-test-generation"
      "adl:seal-sha256"         = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    }
  }
}

run "warm_launch_apply" {
  command = apply

  variables {
    issue_number                  = 607
    aws_account_id                = "123456789012"
    run_id                        = "adl-issue607-test-launch"
    owner_token                   = "0123456789abcdef0123456789abcdef"
    runtime_ami_id                = "ami-0123456789abcdef0"
    gpu_ami_id                    = "ami-0123456789abcdef1"
    vpc_id                        = "vpc-0123456789abcdef0"
    subnet_id                     = "subnet-0123456789abcdef0"
    ssh_ingress_cidr              = "192.0.2.10/32"
    ssh_public_key                = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAITestKey issue607"
    authorized_max_hourly_usd     = 1.55
    authorized_max_total_usd      = 20
    artifact_bucket               = "adl-test-artifacts"
    artifact_prefix               = "shepherd/issue-607/"
    artifact_read_keys            = ["a/one", "a/two", "a/three", "a/four", "a/five"]
    gpu_user_data                 = "cold-path-disabled"
    runtime_user_data             = "__GPU_PRIVATE_IP__"
    warm_volume_availability_zone = "us-west-2a"
    runtime_warm_volume_id        = "vol-0123456789abcdef0"
    gpu_warm_volume_id            = "vol-0123456789abcdef1"
    runtime_warm_seal_sha256      = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    gpu_warm_seal_sha256          = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    warm_artifact_generation      = "issue607-test-generation"
    warm_source_commit            = "0123456789abcdef0123456789abcdef01234567"
    warm_kms_key_arn              = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
  }

  assert {
    condition     = strcontains(nonsensitive(aws_instance.gpu.user_data), "veritysetup open") && strcontains(nonsensitive(aws_instance.runtime.user_data), "veritysetup open")
    error_message = "warm launch did not render both sealed-volume activation templates"
  }

  assert {
    condition = (
      !strcontains(nonsensitive(aws_instance.gpu.user_data), "apt-get") &&
      !strcontains(nonsensitive(aws_instance.runtime.user_data), "apt-get") &&
      !strcontains(nonsensitive(aws_instance.gpu.user_data), "cargo build") &&
      !strcontains(nonsensitive(aws_instance.runtime.user_data), "cargo build")
    )
    error_message = "normal launch rendered cold installation or compilation work"
  }

  assert {
    condition     = aws_instance.runtime.tags["adl:issue"] == "607" && aws_instance.gpu.tags["adl:issue"] == "607"
    error_message = "warm compute resources lost issue-607 ownership tags"
  }
}
