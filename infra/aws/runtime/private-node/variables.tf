variable "aws_region" {
  description = "AWS region for the private Runtime node."
  type        = string
  default     = "us-west-2"
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
