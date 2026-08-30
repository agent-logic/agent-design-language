# Issue 495 denominator inventory

This inventory is the pre-bind denominator authority for #495. It names the
two admitted CloudFormation templates and maps their required behaviors into
the portable Runtime workload contract plus provider-specific Terraform
surfaces. It is evidence for conversion only; #496 owns the later
CloudFormation retirement decision.

## Source templates

| Source issue | Template | Disposition in #495 |
| --- | --- | --- |
| #194 | `adl/tools/issue194_private_network.cloudformation.json` | Preserve private Runtime network semantics as AWS Terraform and provider-neutral rows. |
| #268 | `adl/tools/issue268_runtime_qualification.cloudformation.yaml` | Preserve one on-demand Runtime host qualification shape as AWS Terraform and provider-neutral rows. |

## Required behavior rows

| Source | Required behavior | Portable row | Provider mapping |
| --- | --- | --- | --- |
| #194 | VPC `10.194.0.0/16` with two private subnets | `network_attachment`, `private_subnet_ids` | AWS VPC/subnets; GCP VPC/subnet with private Google access. |
| #194 | No public subnet, public IP mapping, internet gateway, or NAT gateway | `launch_flags`, `cleanup_labels` | AWS route tables without IGW/NAT; GCP private subnet/firewall posture. |
| #194 | No public Runtime ingress and private voter mesh only | `runtime_security_identity` | AWS security groups; GCP firewall tags/service account. |
| #194 | SSM, SSM messages, EC2 messages, and S3 private artifact access | `private_service_endpoints`, `artifact_source` | AWS interface/gateway endpoints; GCP private Google APIs/artifact bucket access. |
| #194 | Optional voter instances with encrypted gp3 root volumes | `runtime_node_ids`, `runtime_shape` | AWS EC2 optional/host primitives; GCP compute instances with provider-native disks. |
| #268 | On-demand `r7i.2xlarge` Runtime host (`on_demand_r7i_2xlarge_runtime_host`) | `runtime_shape` | AWS `aws_instance.runtime_host` fixed to `r7i.2xlarge`; GCP maps shape explicitly and does not claim equivalence. |
| #268 | Retained Runtime EBS volume attachment | `retained_runtime_volume`, `retained_volume_id`, `runtime_mount_path` | AWS `aws_volume_attachment.retained_runtime`; GCP uses provider-native persistent disk attachment semantics. |
| #268 | Operator break-glass SSH public key and single-host CIDR | `operator_break_glass_ssh` | AWS key pair plus /32 ingress when configured; GCP metadata/OS Login equivalent is provider-specific. |
| #268 | Amazon SSM managed instance role and pinned S3 object-version read | `service_identity`, `artifact_source` | AWS IAM role/profile with SSM managed policy and `s3:GetObjectVersion`; GCP service account/IAM binding. |
| #268 | IMDSv2 required with hop limit one | `launch_flags` | AWS EC2 metadata options; GCP metadata posture documented separately. |
| #268 | Encrypted gp3 root disk deleted on termination | `runtime_shape`, `cleanup_labels` | AWS root block device; GCP boot disk encryption/delete policy. |
| #268 | Bootstrap log, readiness marker, and readiness command | `bootstrap_log_path`, `readiness_marker_path`, `readiness_command` | AWS user data writes `/var/log/adl-issue268-bootstrap.log` and `/var/lib/adl/issue268-bootstrap-ready`; GCP startup script maps same observable contract. |

## Non-retirement note

The CloudFormation templates remain rollback authority until #496 accepts a
separate retirement decision. #495 does not delete historical evidence, apply
paid cloud resources, or claim live parity proof.
