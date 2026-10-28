# AWS Account Move-In and Normalization

v0.92.1 promotes the company AWS account from historically accumulated use to explicit corporate operating authority. The work preserves existing website Terraform, classifies every live resource before mutation, separates state and data classes, and admits no cleanup or import without exact ownership, retention, recovery, rollback, and deletion authority.

The lane is seven ordered, independently finishable issues:

1. **AWS-A — resource ownership inventory:** reconcile all-region/global resources, state backends, tags, billing, scripts, CloudFormation, website ownership, and retained evidence.
2. **AWS-B — access and billing baseline:** prove corporate recovery, human/workload separation, billing visibility, budgets, anomaly handling, export, and cost attribution without prematurely removing the proven administrator path. Configure [Agent Toolkit for AWS](https://aws.amazon.com/products/developer-tools/agent-toolkit-for-aws/) for the approved Codex path, require AWS CLI 2.35 or newer, default agent access to read-only, constrain agent calls with IAM context policy, and prove CloudWatch/CloudTrail attribution.
3. **AWS-C — Terraform bootstrap:** establish a separate locked, versioned, recoverable account-foundation backend and least-privilege deployment identity without absorbing existing website or workload states.
4. **AWS-D — audit and security baseline:** establish owned audit, configuration, detection, access-analysis, encryption, logging, retention, and alert authority.
5. **AWS-E — resource adoption register:** assign every durable resource exactly one retain/import/replace/retire-later/ephemeral/frozen-unknown management disposition and prohibit dual ownership.
6. **AWS-F — Runtime platform modules:** deliver reviewed private network, edge, build, and independently scalable Runtime node Terraform modules; existing #122 continues to own public Route53/ACM exposure.
7. **AWS-G — CloudFormation retirement decision:** decide the exact issue-#194/#268 template denominator only after XCL-01 proves cross-cloud Terraform parity and rollback.

Existing #345 remains the GPU Shepherd hardening input. Existing #122 is not recreated. Multi-account Organizations/Control Tower rollout and production traffic are separately authorized future work, not implied by move-in completion.

The source denominator is the operator-authored `AWS_ACCOUNT_MOVE_IN_AND_NORMALIZATION_PLAN.md`, promoted into the issue contracts above. Planning grants no AWS mutation, paid-run, DNS, IAM, billing, Terraform-apply, cleanup, or production authority.

Agent Toolkit is available without an additional toolkit charge, but every AWS resource it provisions or interacts with remains standard billable AWS usage. Installation does not authorize infrastructure mutation; AWS-B retains exact IAM, audit, cost, and operator-approval boundaries.
