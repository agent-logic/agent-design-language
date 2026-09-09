# Issue 770 local validation

Terraform 1.15.3 / AWS provider 5.100.0.

`bash infra/aws/csm-runtime-spot/tests/run_contract.sh`: PASS, 10 mocked plan tests; fmt and validate pass. Provider installation uses network; mocked plans create no resources.

`bash -n infra/aws/csm-runtime-spot/tests/run_live_proof.sh` and `--help`: PASS, syntax/interface only, not live proof.

Independent static security re-review: no remaining actionable findings. Resolved three P2 probe findings: unique listener nonce, verified cleanup required for success, and local socket errors fail closed. Five deterministic offline probe verdict tests pass (success, listener collision, cleanup failure, local socket error, exposed port). Live proof has not run.

Live SSH/isolation/disposal proof remains pending key identity-file path and explicit approval of saved AWS plan. No paid resources created.
