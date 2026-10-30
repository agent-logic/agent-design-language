# Runtime Cloud Control CloudFront Proof

Issue `#4915` adds a read-only runtime-owned CloudFront/cloud-control hook.
The supported operator path is:

```bash
ADL_AWS_CLOUD_CONTROL_ACCOUNT_SHA256=<approved-agent-logic-account-sha256> \
bash adl/tools/run_wp08_cloudfront_control_proof.sh \
  --out docs/milestones/v0.91.7/review/runtime/wp08_cloudfront_4915 \
  --profile agent-logic-admin \
  --region us-west-2 \
  --csm-bin adl/target/debug/csm
```

The wrapper runs `csm cloud-control cloudfront-status`. The historical
`adl csm` compatibility path intentionally rejects this runtime-owned AWS
surface.

The proof is read-only. It verifies the approved Agent Logic account hash before
CloudFront inspection, lists distributions, describes one selected distribution,
classifies a nonexistent-distribution negative case, and retains only redacted
hashes, counts, state, and policy classifications.

Required AWS permission shape is least-authority read access:

- `sts:GetCallerIdentity`
- `cloudfront:ListDistributions`
- `cloudfront:GetDistribution`

No CloudFront mutation, invalidation, DNS cutover, or customer traffic routing is
performed by this hook.
