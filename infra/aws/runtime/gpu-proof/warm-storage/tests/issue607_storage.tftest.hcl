mock_provider "aws" {}

run "retained_storage_plan" {
  command = plan

  variables {
    aws_account_id      = "123456789012"
    availability_zone   = "us-west-2a"
    storage_id          = "adl-issue607-test-storage"
    owner_token         = "0123456789abcdef0123456789abcdef"
    kms_key_arn         = "arn:aws:kms:us-west-2:123456789012:key/01234567-89ab-cdef-0123-456789abcdef"
    artifact_generation = "issue607-test-generation"
    runtime_seal_sha256 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    gpu_seal_sha256     = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
  }

  assert {
    condition     = aws_ebs_volume.runtime.encrypted && aws_ebs_volume.gpu.encrypted
    error_message = "retained warm volumes must both be encrypted"
  }

  assert {
    condition     = aws_ebs_volume.runtime.throughput == 250 && aws_ebs_volume.gpu.throughput == 500
    error_message = "warm volume throughput fell below the reviewed profiles"
  }

  assert {
    condition     = aws_ebs_volume.runtime.tags["adl:compute-owned"] == "false" && aws_ebs_volume.gpu.tags["adl:compute-owned"] == "false"
    error_message = "retained volumes became compute-owned"
  }
}
