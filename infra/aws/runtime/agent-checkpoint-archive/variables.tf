variable "aws_region" {
  type    = string
  default = "us-west-2"
}
variable "environment" {
  type = string
  validation {
    condition     = can(regex("^(dev|prod)$", var.environment))
    error_message = "environment must be dev or prod."
  }
}
variable "runtime_role_name" { type = string }
variable "restore_role_name" { type = string }
variable "bucket_name" {
  type    = string
  default = null
}
variable "tags" {
  type    = map(string)
  default = {}
}
