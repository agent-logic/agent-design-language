variable "company_account_id" {
  description = "Account returned by aws sts get-caller-identity using agent-logic-admin; retain in private tfvars."
  type        = string
  sensitive   = true
  validation {
    condition     = can(regex("^[0-9]{12}$", var.company_account_id))
    error_message = "A verified 12-digit company account identifier is required."
  }
}
variable "vpc_id" {
  description = "Reviewed company VPC in us-west-2."
  type        = string
}
variable "subnet_id" {
  description = "Reviewed company public subnet with an Internet gateway route in us-west-2."
  type        = string
}
variable "ubuntu_ami_id" {
  description = "Exact reviewed Canonical Ubuntu 24.04 amd64 image; never resolve latest during apply."
  type        = string
}
data "aws_subnet" "selected" {
  id = var.subnet_id
}
data "aws_ami" "pinned" {
  owners = ["099720109477"]
  filter {
    name   = "image-id"
    values = [var.ubuntu_ami_id]
  }
  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-amd64-server-*"]
  }
  filter {
    name   = "architecture"
    values = ["x86_64"]
  }
  filter {
    name   = "state"
    values = ["available"]
  }
}
