variable "name_prefix" {
  description = "Stable name prefix for the disposable Runtime host."
  type        = string
}

variable "vpc_id" {
  description = "VPC id that owns the selected subnet."
  type        = string
}

variable "subnet_id" {
  description = "Subnet id for the disposable Runtime host. Use any public subnet for the fast dev path."
  type        = string
}

variable "ami_id" {
  description = "Optional AMI id. When null, the latest Amazon Linux 2023 x86_64 AMI is used."
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
  description = "Runtime HTTPS port on the instance."
  type        = number
  default     = 20997
}

variable "alb_security_group_id" {
  description = "Optional ALB security group allowed to reach runtime_port."
  type        = string
  default     = null
}

variable "operator_ingress_cidrs" {
  description = "Optional direct operator CIDRs allowed to reach runtime_port for smoke tests."
  type        = list(string)
  default     = []
}

variable "ssh_ingress_cidrs" {
  description = "Optional SSH CIDRs. Empty disables SSH ingress."
  type        = list(string)
  default     = []
}

variable "key_name" {
  description = "Optional EC2 key pair name."
  type        = string
  default     = null
}

variable "iam_instance_profile" {
  description = "Optional existing IAM instance profile name."
  type        = string
  default     = null
}

variable "user_data" {
  description = "Optional cloud-init/user-data for bootstrapping Runtime."
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
