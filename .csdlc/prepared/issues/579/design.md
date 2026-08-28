# Issue #579 design: AWS-F corrective runtime platform edge and proof repair

## Outcome

Issue #579 repairs the post-merge AWS-F findings from #489 / PR #577 without rewriting terminal #489 history. The corrective narrows public-edge ownership back to #122, makes AWS-F proof claims match evidence, repairs the security validator false-pass surface, records enforceable state-isolation requirements, and bounds Spot node resilience as a non-production one-time-node limitation unless a later issue owns replacement or managed target behavior.

## Provenance

- Source issue: #489
- Source PR: #577
- Reviewed PR head: `485b4197908231bb2065e1e29c7c5013536e1975`
- Merged PR #577 merge SHA: `69ba35e066d1389a9f194659acb066a7dca82a40`
- Operator-supplied post-merge review verdict: FAIL

## Corrective findings

1. AWS-F exposed executable Route53/ACM options and examples even though #122 owns public exposure.
2. Deployment proof was overstated or deferred without enough truth around saved-plan identity, artifact wiring, observability, cost/deadline controls, rollback, disposable deployment, and zero-residue cleanup.
3. The security validator could falsely pass forbidden world-open ingress because its regex was overescaped and its egress exclusion was line-oriented rather than structurally sound.
4. State isolation was advisory only; remote backend, locking, account identity, workspace, and key isolation were not enforced.
5. A Spot instance attached by exact instance ID is not production-resilient after interruption.

## Owned paths

- `infra/aws/runtime/**`
- `docs/operations/cloud/aws/runtime-platform/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-f/**`
- `.csdlc/prepared/issues/579/**`
- `.csdlc/evidence/579/**`

## Non-goals

- Do not absorb #122 public edge ownership.
- Do not create live paid AWS resources without explicit operator approval.
- Do not rewrite, reopen, or mutate terminal #489 state.
- Do not implement production cutover, #496 CloudFormation retirement, #495 cross-cloud conversion, or a production node-replacement architecture beyond explicitly bounded Spot truth.

## Corrective contract

The repaired AWS-F package must:

- remove or disable executable Route53/ACM public-exposure creation from AWS-F-owned modules and examples, retaining only read-only consumption of #122 edge outputs where needed;
- make proof documents and validators distinguish static/local proof from any future approved live disposable AWS proof;
- reject direct public Runtime ingress through structural Terraform inspection, including world-open ingress that appears in multiline ingress blocks;
- require or validate backend/workspace/account/key isolation for runtime state before claiming reusable deployment safety;
- describe one-time Spot instance target behavior as bounded and non-production-resilient unless replacement is explicitly owned elsewhere;
- preserve zero credential disclosure and avoid paid AWS mutation in local validation.

## Validation lanes

Local lanes are non-mutating:

- `579-terraform-static` runs focused Terraform formatting/init/validation for touched AWS-F Terraform roots/modules without backends.
- `579-security-validator-regression` proves forbidden world-open Runtime ingress is rejected and egress-only blocks do not mask ingress.
- `579-proof-truth` checks evidence/runbook language for public-edge ownership, proof-status, backend/state-isolation, cleanup, and Spot resilience boundaries.
- `579-diff-hygiene` rejects conflict markers and whitespace errors.
- `exact-head-review` records fresh exact-head review before publication.

Any live AWS deployment, saved remote plan, disposable path proof, or cleanup proof remains deferred unless the operator explicitly authorizes the paid AWS action and profile context.

## Stop conditions

Stop before publication if:

- AWS-F still creates Route53 zones/records or ACM certificates instead of consuming #122 outputs;
- the validator can pass prohibited `0.0.0.0/0` or `::/0` Runtime ingress;
- proof claims live deployment, cleanup, rollback, observability, or artifact wiring beyond actual evidence;
- state isolation remains advisory for reusable runtime state;
- Spot target resilience is presented as production-ready when it is only exact-instance attachment;
- cloud proof would require credentials, paid mutation, production traffic, or sensitive output without explicit operator approval.
