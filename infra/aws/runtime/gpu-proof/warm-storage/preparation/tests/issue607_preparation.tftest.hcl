mock_provider "aws" {}

override_data {
  target = data.aws_subnet.selected
  values = {
    availability_zone = "us-west-2a"
  }
}

override_data {
  target = data.aws_ebs_volume.runtime
  values = {
    encrypted         = true
    kms_key_id        = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
    availability_zone = "us-west-2a"
    tags = {
      "adl:issue"         = "607"
      "adl:compute-owned" = "false"
    }
  }
}

override_data {
  target = data.aws_ebs_volume.gpu
  values = {
    encrypted         = true
    kms_key_id        = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
    availability_zone = "us-west-2a"
    tags = {
      "adl:issue"         = "607"
      "adl:compute-owned" = "false"
    }
  }
}

run "two_image_preparation_plan" {
  command = plan

  variables {
    aws_account_id               = "123456789012"
    run_id                       = "adl-issue607-test-preparation"
    owner_token                  = "0123456789abcdef0123456789abcdef"
    termination_at               = "2030-01-01T00:00:00Z"
    runtime_ami_id               = "ami-0123456789abcdef0"
    gpu_ami_id                   = "ami-0123456789abcdef1"
    vpc_id                       = "vpc-0123456789abcdef0"
    subnet_id                    = "subnet-0123456789abcdef0"
    ssh_ingress_cidr             = "192.0.2.10/32"
    ssh_public_key               = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAITestKey issue607"
    artifact_bucket              = "adl-test-artifacts"
    artifact_read_keys           = ["a/one", "a/two", "a/three", "a/four", "a/five"]
    receipt_write_prefix         = "shepherd/issue-607/test/preparation/"
    runtime_volume_id            = "vol-0123456789abcdef0"
    gpu_volume_id                = "vol-0123456789abcdef1"
    availability_zone            = "us-west-2a"
    artifact_generation          = "issue607-test-generation"
    source_commit                = "0123456789abcdef0123456789abcdef01234567"
    source_archive_key           = "a/source"
    source_archive_version_id    = "source-version"
    source_archive_sha256        = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    artifact_manifest_key        = "a/manifest"
    artifact_manifest_version_id = "manifest-version"
    artifact_manifest_sha256     = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    kms_key_arn                  = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
  }

  assert {
    condition     = aws_instance.runtime_preparation.ami == "ami-0123456789abcdef0" && aws_instance.gpu_preparation.ami == "ami-0123456789abcdef1"
    error_message = "preparation did not bind the distinct exact Runtime and GPU AMIs"
  }

  assert {
    condition     = strcontains(nonsensitive(aws_instance.runtime_preparation.user_data), "cargo build --locked") && strcontains(nonsensitive(aws_instance.gpu_preparation.user_data), "nvidia-smi")
    error_message = "preparation templates did not render their complete build and GPU-facility work"
  }

  assert {
    condition     = aws_volume_attachment.runtime.volume_id == "vol-0123456789abcdef0" && aws_volume_attachment.gpu.volume_id == "vol-0123456789abcdef1"
    error_message = "preparation attached a warm volume to the wrong node"
  }
}
