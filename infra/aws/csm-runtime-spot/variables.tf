variable "aws_region" {
  description = "AWS region for the disposable Runtime host."
  type        = string
  default     = "us-west-2"
}

variable "environment" {
  description = "Deployment environment."
  type        = string
  default     = "dev"
}

variable "csm_name" {
  description = "CSM instance name."
  type        = string
}

variable "vpc_id" {
  description = "VPC id."
  type        = string
}

variable "subnet_id" {
  description = "Public subnet id for the disposable Runtime host."
  type        = string
}

variable "ami_id" {
  description = "Optional AMI id."
  type        = string
  default     = null
}

variable "instance_type" {
  description = "Small Spot instance type."
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

variable "alb_security_group_id" {
  description = "Optional ALB security group id allowed to reach runtime_port."
  type        = string
  default     = null
}

variable "operator_ingress_cidrs" {
  description = "Optional direct operator CIDRs for Runtime smoke tests."
  type        = list(string)
  default     = []
}

variable "ssh_ingress_cidrs" {
  description = "Optional SSH CIDRs. Empty disables SSH."
  type        = list(string)
  default     = []
}

variable "key_name" {
  description = "Optional EC2 key pair name."
  type        = string
  default     = null
}

variable "iam_instance_profile" {
  description = "Optional IAM instance profile name."
  type        = string
  default     = null
}

variable "user_data" {
  description = "Optional user-data. Keep secret values outside tracked files."
  type        = string
  default     = null
  sensitive   = true
}

variable "user_data_file" {
  description = "Optional local user-data file path. Prefer this for smoke scripts so script contents stay outside Terraform var files."
  type        = string
  default     = null
}

variable "root_volume_size_gb" {
  description = "Root EBS volume size in GiB."
  type        = number
  default     = 20
}
