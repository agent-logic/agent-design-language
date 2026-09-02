# Runtime portable workload contract

Issue #495 defines the XCL-01 portable Runtime workload denominator shared by
the AWS and GCP Terraform implementations.

The contract is intentionally small:

- preserve the exact admitted #194 private Runtime recovery network behavior;
- preserve the exact admitted #268 single-host Runtime qualification behavior;
- expose provider-specific identity, network, storage, metadata, bootstrap,
  cleanup, and output differences instead of hiding them behind one false
  abstraction;
- keep CloudFormation rollback authority until #496 retires it;
- keep live apply/destroy proof gated on explicit operator authorization.

The machine-readable contract is
[`runtime-workload-contract.v1.json`](runtime-workload-contract.v1.json).
