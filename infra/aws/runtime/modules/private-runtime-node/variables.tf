variable "name_prefix" {
  description = "Stable name prefix for the private Runtime node."
  type        = string
}

variable "vpc_id" {
  description = "VPC id that owns the selected private subnet."
  type        = string
}

variable "private_subnet_id" {
  description = "Private subnet id for the Runtime node. The module never requests a public IPv4 address."
  type        = string
}

variable "ami_id" {
  description = "Optional AMI id. When null, the latest Amazon Linux 2023 x86_64 AMI is selected."
  type        = string
  default     = null
}

variable "instance_type" {
  description = "Small instance type for the disposable Runtime host."
  type        = string
  default     = "t3.micro"
}

variable "spot_max_price" {
  description = "Optional Spot max price. Null lets AWS use the current Spot market price up to On-Demand."
  type        = string
  default     = null
}

variable "runtime_port" {
  description = "Runtime HTTPS port exposed only to the ALB security group."
  type        = number
  default     = 20997
}

variable "alb_security_group_id" {
  description = "ALB security group id allowed to reach runtime_port."
  type        = string

  validation {
    condition     = length(trimspace(var.alb_security_group_id)) > 0
    error_message = "alb_security_group_id is required; private Runtime nodes do not expose direct public ingress."
  }
}

variable "key_name" {
  description = "Optional EC2 key pair name. This does not open SSH ingress."
  type        = string
  default     = null
}

variable "iam_instance_profile" {
  description = "Optional existing IAM instance profile name."
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

variable "tags" {
  description = "Additional tags."
  type        = map(string)
  default     = {}
}
