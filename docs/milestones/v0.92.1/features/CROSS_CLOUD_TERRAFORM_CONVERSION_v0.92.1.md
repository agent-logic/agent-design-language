# Cross-Cloud Runtime Terraform Conversion

XCL-01 converts the exact existing issue-#194 and issue-#268 CloudFormation-template behavior into one provider-neutral Runtime workload contract with explicit AWS and GCP Terraform implementations.

The portable contract covers admitted inputs, topology, identities, tags, deadlines, artifacts, outputs, failure behavior, cleanup, and rollback. Provider-specific IAM, network, image, storage, and cost behavior remains explicit; the work must not pretend one Terraform resource graph can span both providers or that the providers are interchangeable.

Completion requires static and saved-plan parity, disposable deployments where separately authorized, exact Runtime artifact identity, provider-specific positive and negative proof, rollback, and independent zero-resource cleanup on both clouds. Existing CloudFormation remains retained rollback authority until AWS-G separately accepts retirement for every consumer and retained evidence path.
