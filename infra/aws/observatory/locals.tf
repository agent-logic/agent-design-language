locals {
  name_prefix = "adl-observatory"
  tags = merge(var.tags, {
    Application = "adl-observatory"
    ManagedBy   = "terraform"
    Issue       = "679"
  })
  connect_src = join(" ", concat(["'self'"], var.runtime_connect_origins))
}
