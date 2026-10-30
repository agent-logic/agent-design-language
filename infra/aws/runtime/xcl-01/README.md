# AWS XCL-01 Runtime Terraform conversion

This root is the AWS-side conversion target for issue #495. It keeps the
admitted #194 and #268 CloudFormation behavior visible and does not retire the
source templates; #496 owns that retirement decision.

Static proof for #495 checks that this root preserves the denominator named in
`infra/runtime-portable/runtime-workload-contract.v1.json`:

- private #194-style Runtime network: VPC `10.194.0.0/16`, private subnets, no
  public subnet/NAT/internet gateway, no public ingress, SSM/EC2Messages/S3
  private service access including S3 prefix-list egress to the gateway
  endpoint, real optional voters behind `launch_voters`, cleanup tags, and
  outputs;
- #268-style Runtime host: on-demand `r7i.2xlarge`, retained Runtime EBS
  attachment, IMDSv2, SSM role, bootstrap artifact read policy, readiness
  marker, bootstrap log, cleanup tags, and outputs.

Live AWS plan/apply/destroy proof is intentionally not run by this issue unless
the operator explicitly authorizes paid/cloud mutation.
