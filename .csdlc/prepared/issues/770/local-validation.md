# Issue 770 local validation

Terraform 1.15.3 / AWS provider 5.100.0.

`bash infra/aws/csm-runtime-spot/tests/run_contract.sh`: PASS, 10 mocked plan tests; fmt and validate pass. Provider installation uses network; mocked plans create no resources.

`bash -n infra/aws/csm-runtime-spot/tests/run_live_proof.sh` and `--help`: PASS, syntax/interface only, not live proof.

Independent static security re-review: no remaining actionable findings. Resolved three P2 probe findings: unique listener nonce, verified cleanup required for success, and local socket errors fail closed. Five deterministic offline probe verdict tests pass (success, listener collision, cleanup failure, local socket error, exposed port). The subsequent authorized live run passed; see live-proof.json.

Authorized live SSH/isolation/disposal proof passed; see live-proof.json. Initial state-path mismatch was recovered without reapplying. AWS verified termination and absent issue-owned volume/security group. Existing key preserved.
