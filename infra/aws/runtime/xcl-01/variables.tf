variable "run_id" {
  description = "Issue-scoped run identity. Use issue-194-* for private network parity and issue268-* for Runtime host qualification parity."
  type        = string
}

variable "ttl_expires_at" {
  description = "Cleanup TTL propagated to created resources that support tags."
  type        = string
}

variable "availability_zones" {
  description = "Two AWS availability zones for the #194 private network denominator."
  type        = list(string)
}

variable "runtime_ami_id" {
  description = "AMI for optional #194 voters or #268 Runtime host."
  type        = string
}

variable "runtime_instance_type" {
  description = "Runtime instance type for #194 voters."
  type        = string
  default     = "m7i.large"
}

variable "launch_voters" {
  description = "Whether to launch both optional #194 private voter instances."
  type        = bool
  default     = false
}

variable "qualification_instance_type" {
  description = "#268 Runtime qualification shape retained from the admitted CloudFormation template."
  type        = string
  default     = "r7i.2xlarge"
}

variable "runtime_volume_id" {
  description = "Retained Runtime EBS volume for the #268 qualification host."
  type        = string
  default     = null
}

variable "operator_ssh_public_key" {
  description = "Operator break-glass SSH public key for #268 parity. Do not pass private key material."
  type        = string
  default     = null
  sensitive   = true
}

variable "operator_ssh_ingress_cidr" {
  description = "Single-host /32 SSH ingress CIDR for #268 parity."
  type        = string
  default     = null
}

variable "bootstrap_bucket" {
  description = "S3 bucket containing Shepherd bootstrap artifacts."
  type        = string
  default     = "adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2"
}

variable "bootstrap_prefix" {
  description = "S3 prefix containing Shepherd bootstrap artifacts."
  type        = string
  default     = "shepherd/"
}

variable "s3_prefix_list_id" {
  description = "AWS-managed S3 prefix list id for the selected region; used for private HTTPS egress to the S3 gateway endpoint."
  type        = string
}

variable "common_tags" {
  description = "Additional resource tags."
  type        = map(string)
  default     = {}
}
