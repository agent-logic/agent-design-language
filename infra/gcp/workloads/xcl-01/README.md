# GCP XCL-01 Runtime Terraform conversion

This root is the GCP-side conversion target for issue #495. It consumes the
reviewed GCP-D platform foundation and maps the same portable Runtime workload
contract used by the AWS implementation.

Provider differences are intentional and explicit:

- GCP service accounts are not AWS IAM roles or SSM instance profiles.
- GCP firewall rules and Private Google Access are not AWS security groups and
  VPC endpoints.
- GCP bootstrap artifact access is represented by an optional
  `roles/storage.objectViewer` binding on `artifact_bucket`; no bucket is
  created by this workload root.
- Cleanup deadline truth is carried by `ttl_expires_at` labels/metadata and the
  portable output contract.
- Persistent disks and startup scripts preserve the retained Runtime volume and
  readiness-marker shape without claiming byte-for-byte EC2/EBS equivalence.

Live GCP plan/apply/destroy proof is gated on explicit operator authorization.
