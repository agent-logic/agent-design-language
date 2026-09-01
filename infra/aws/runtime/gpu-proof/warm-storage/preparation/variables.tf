variable "aws_account_id" {
  type = string
}

variable "aws_profile" {
  type    = string
  default = "agent-logic-admin"
}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "run_id" {
  type = string
}

variable "owner_token" {
  type      = string
  sensitive = true
}

variable "termination_at" {
  type = string
}

variable "ami_id" {
  type = string
}

variable "instance_type" {
  type    = string
  default = "m7i.2xlarge"
}

variable "vpc_id" {
  type = string
}

variable "subnet_id" {
  type = string
}

variable "ssh_ingress_cidr" {
  type = string
}

variable "ssh_public_key" {
  type      = string
  sensitive = true
}

variable "artifact_bucket" {
  type = string
}

variable "artifact_read_prefix" {
  type = string
}

variable "receipt_write_prefix" {
  type = string
}

variable "runtime_volume_id" {
  type = string
}

variable "gpu_volume_id" {
  type = string
}

variable "user_data" {
  type      = string
  sensitive = true
}

variable "kms_key_arn" {
  type = string
}

variable "root_volume_size_gib" {
  type    = number
  default = 80
}
