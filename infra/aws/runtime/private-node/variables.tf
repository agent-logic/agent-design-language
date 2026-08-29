variable "aws_region" {
  description = "AWS region for the private Runtime node."
  type        = string
  default     = "us-west-2"
}

variable "expected_aws_account_id" {
  description = "Expected Agent Logic AWS account id. Terraform checks fail closed when the active profile resolves to a different account."
  type        = string

  validation {
    condition     = can(regex("^[0-9]{12}$", var.expected_aws_account_id))
    error_message = "expected_aws_account_id must be a 12 digit AWS account id."
  }
}

variable "expected_terraform_workspace" {
  description = "Expected Terraform workspace for this root, used to keep private-node state separate from ALB-origin state."
  type        = string

  validation {
    condition     = length(trimspace(var.expected_terraform_workspace)) > 0 && var.expected_terraform_workspace != "default"
    error_message = "expected_terraform_workspace must be explicit and must not be default."
  }
}

variable "environment" {
  description = "Deployment environment."
  type        = string
  default     = "dev"
}

variable "csm_name" {
  description = "CSM instance name, for example wuji."
  type        = string
}

variable "vpc_id" {
  description = "VPC id."
  type        = string
}

variable "private_subnet_id" {
  description = "Private subnet id for the Runtime node."
  type        = string
}

variable "alb_security_group_id" {
  description = "ALB security group id allowed to reach runtime_port."
  type        = string
}

variable "ami_id" {
  description = "Optional AMI id."
  type        = string
  default     = null
}

variable "instance_type" {
  description = "Small instance type for the disposable Runtime host."
  type        = string
  default     = "t3.micro"
}

variable "spot_max_price" {
  description = "Optional Spot max price."
  type        = string
  default     = null
}

variable "runtime_port" {
  description = "Runtime HTTPS port."
  type        = number
  default     = 20997
}

variable "key_name" {
  description = "Optional EC2 key pair name. This does not open SSH ingress."
  type        = string
  default     = null
}

variable "iam_instance_profile" {
  description = "Optional IAM instance profile name."
  type        = string
  default     = null
}

variable "user_data" {
  description = "Optional cloud-init/user-data for bootstrapping Runtime. Keep secret values outside tracked files."
  type        = string
  default     = null
  sensitive   = true
}

variable "root_volume_size_gb" {
  description = "Root EBS volume size in GiB."
  type        = number
  default     = 20
}
